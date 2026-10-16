// The model view: the objects of the active editor's model set by region,
// substation and voltage level, by area and by class, from the language
// server's `cimoxide/tree`, a level at a time; each with the findings of what
// it holds, as a badge in the colour of the worst.

import * as path from "path";
import {
  commands,
  Disposable,
  EventEmitter,
  ExtensionContext,
  FileDecoration,
  languages,
  MarkdownString,
  Position,
  Range,
  ThemeColor,
  ThemeIcon,
  TreeDataProvider,
  TreeItem,
  TreeItemCollapsibleState,
  TreeView,
  Uri,
  ViewColumn,
  window,
  workspace,
} from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { Reveal, revealInGrid } from "./gridView";

const TREE_REQUEST = "cimoxide/tree";
const PATH_REQUEST = "cimoxide/treePath";
const DELETE_REQUEST = "cimoxide/delete";
const VIEW = "cimoxide.model";
/** The scheme of the tree items' `resourceUri`, which carries their findings to the badge. */
const SCHEME = "cimoxide-tree";

interface Findings {
  violation: number;
  warning: number;
  info: number;
}

interface Location {
  uri: string;
  range: { start: { line: number; character: number }; end: { line: number; character: number } };
}

/** A node as the server sends it. */
interface Entry {
  key: string;
  kind: "section" | "group" | "range" | "object" | "missing";
  label: string;
  class?: string;
  id?: string;
  /** How many children it has; 0 for a leaf. */
  children: number;
  findings: Findings;
  location?: Location | null;
  grid?: Reveal | null;
}

const SECTION_ICONS: Record<string, string> = {
  geo: "globe",
  op: "organization",
  other: "inbox",
  class: "symbol-class",
};

const CLASS_ICONS: Record<string, string> = {
  GeographicalRegion: "globe",
  SubGeographicalRegion: "map",
  Substation: "home",
  VoltageLevel: "layers",
  Bay: "symbol-array",
  Line: "dash",
  ACLineSegment: "dash",
  DCLine: "dash",
  DCLineSegment: "dash",
  Terminal: "plug",
  DCTerminal: "plug",
  ACDCConverterDCTerminal: "plug",
  ConnectivityNode: "circle-small-filled",
  TopologicalNode: "circle-filled",
  DCNode: "circle-small-filled",
  BusbarSection: "remove",
  Breaker: "debug-disconnect",
  Disconnector: "debug-disconnect",
  LoadBreakSwitch: "debug-disconnect",
  Switch: "debug-disconnect",
  Fuse: "debug-disconnect",
  GroundDisconnector: "debug-disconnect",
  Jumper: "debug-disconnect",
  PowerTransformer: "arrow-swap",
  PowerTransformerEnd: "debug-stackframe-dot",
  RatioTapChanger: "settings",
  PhaseTapChanger: "settings",
  PhaseTapChangerTabular: "settings",
  PhaseTapChangerLinear: "settings",
  PhaseTapChangerSymmetrical: "settings",
  PhaseTapChangerAsymmetrical: "settings",
  SynchronousMachine: "zap",
  AsynchronousMachine: "zap",
  GeneratingUnit: "flame",
  ThermalGeneratingUnit: "flame",
  HydroGeneratingUnit: "flame",
  WindGeneratingUnit: "flame",
  SolarGeneratingUnit: "flame",
  NuclearGeneratingUnit: "flame",
  EnergyConsumer: "arrow-down",
  ConformLoad: "arrow-down",
  NonConformLoad: "arrow-down",
  EnergySource: "arrow-up",
  ExternalNetworkInjection: "arrow-up",
  EquivalentInjection: "arrow-up",
  LinearShuntCompensator: "symbol-constant",
  NonlinearShuntCompensator: "symbol-constant",
  StaticVarCompensator: "symbol-constant",
  OperationalLimitSet: "dashboard",
  CurrentLimit: "dashboard",
  VoltageLimit: "dashboard",
  ActivePowerLimit: "dashboard",
  ApparentPowerLimit: "dashboard",
  RegulatingControl: "target",
  TapChangerControl: "target",
  ControlArea: "organization",
  LoadArea: "organization",
  SubLoadArea: "organization",
  ConformLoadGroup: "group-by-ref-type",
  NonConformLoadGroup: "group-by-ref-type",
  TieFlow: "arrow-both",
  BaseVoltage: "symbol-numeric",
  FullModel: "file",
};

function total(f: Findings): number {
  return f.violation + f.warning + f.info;
}

function counts(f: Findings): string {
  const parts: string[] = [];
  if (f.violation) parts.push(`${f.violation} error${f.violation === 1 ? "" : "s"}`);
  if (f.warning) parts.push(`${f.warning} warning${f.warning === 1 ? "" : "s"}`);
  if (f.info) parts.push(`${f.info} info`);
  return parts.join(", ");
}

function icon(e: Entry): ThemeIcon {
  if (e.kind === "section") return new ThemeIcon(SECTION_ICONS[e.key] ?? "folder");
  if (e.kind === "range") return new ThemeIcon("list-flat");
  if (e.kind === "group") return new ThemeIcon("symbol-class");
  if (e.kind === "missing") return new ThemeIcon("question");
  const cls = e.class ?? "";
  return new ThemeIcon(CLASS_ICONS[cls] ?? (cls.startsWith("Sv") ? "graph" : cls.endsWith("Limit") ? "dashboard" : "symbol-object"));
}

class ModelTree implements TreeDataProvider<Entry> {
  private readonly changed = new EventEmitter<Entry | undefined>();
  readonly onDidChangeTreeData = this.changed.event;
  /** A document of the model set shown. */
  uri: Uri | undefined;
  onlyFindings = false;

  constructor(private readonly client: () => LanguageClient | undefined) {}

  /** The node listing each node fetched, by key, for `reveal`. */
  private readonly parents = new Map<string, Entry | undefined>();

  refresh(): void {
    this.parents.clear();
    this.changed.fire(undefined);
  }

  getParent(e: Entry): Entry | undefined {
    return this.parents.get(e.key);
  }

  async getChildren(entry?: Entry): Promise<Entry[]> {
    const client = this.client();
    if (!client || !this.uri) return [];
    // No token argument: an `undefined` one would send the params as `[params, null]`.
    const result = await client.sendRequest<Entry[] | null>(TREE_REQUEST, {
      uri: this.uri.toString(),
      key: entry?.key,
      onlyFindings: this.onlyFindings,
    });
    for (const child of result ?? []) {
      this.parents.set(child.key, entry);
    }
    return result ?? [];
  }

  /**
   * The node of the object at `position` in `doc`, its ancestors fetched on
   * the way; `undefined` off an object, or if the filter hides it.
   */
  async locate(doc: Uri, position: Position): Promise<Entry | undefined> {
    const client = this.client();
    if (!client) return undefined;
    const found = await client.sendRequest<{ id: string; keys: string[] } | null>(PATH_REQUEST, {
      uri: doc.toString(),
      position,
      onlyFindings: this.onlyFindings,
    });
    let at: Entry | undefined;
    for (const key of found?.keys ?? []) {
      at = (await this.getChildren(at)).find((e) => e.key === key);
      if (!at) return undefined;
    }
    return at;
  }

  getTreeItem(e: Entry): TreeItem {
    const item = new TreeItem(e.label, e.children ? TreeItemCollapsibleState.Collapsed : TreeItemCollapsibleState.None);
    item.id = e.key;
    item.iconPath = icon(e);
    const f = e.findings;
    item.resourceUri = Uri.from({ scheme: SCHEME, path: `/${encodeURIComponent(e.key)}`, query: `${f.violation},${f.warning},${f.info}` });
    const object = e.kind === "object" || e.kind === "missing";
    item.description = object ? (e.kind === "missing" ? "not in the set" : e.class) : String(e.children);
    const tip = new MarkdownString();
    tip.appendMarkdown(object ? `**${e.class || "?"}** ${e.label}` : `**${e.label}**: ${e.children}`);
    if (e.id && e.id !== e.label) tip.appendMarkdown(`\n\n\`${e.id}\``);
    if (e.kind === "missing") tip.appendMarkdown("\n\nReferred to, but no file of the set writes it.");
    if (total(f)) tip.appendMarkdown(`\n\n${counts(f)}${object && e.children ? " here and below" : ""}`);
    item.tooltip = tip;
    if (e.location) {
      item.command = { command: "cimoxide.model.open", title: "Open definition", arguments: [e.location] };
    }
    const context: string[] = [e.kind];
    if (e.grid) context.push("grid");
    if (e.grid?.inside) context.push("inside");
    if (e.kind === "object" && e.children) context.push("holds");
    item.contextValue = context.join(" ");
    return item;
  }
}

/** The findings badge: the count, in the colour of the worst. */
function decoration(uri: Uri): FileDecoration | undefined {
  if (uri.scheme !== SCHEME) return undefined;
  const [violation, warning, info] = uri.query.split(",").map(Number);
  const n = violation + warning + info;
  if (!n) return undefined;
  const color = violation ? "list.errorForeground" : warning ? "list.warningForeground" : "editorInfo.foreground";
  return new FileDecoration(n < 100 ? String(n) : "+", undefined, new ThemeColor(color));
}

function isCim(uri: Uri, languageId: string): boolean {
  return uri.scheme === "file" && languageId === "xml";
}

export function registerModelTree(context: ExtensionContext, client: () => LanguageClient | undefined): void {
  const tree = new ModelTree(client);
  const view: TreeView<Entry> = window.createTreeView(VIEW, { treeDataProvider: tree, showCollapseAll: true });
  let refresh: NodeJS.Timeout | undefined;

  const follow = (uri: Uri | undefined, languageId: string | undefined): void => {
    if (!uri || !languageId || !isCim(uri, languageId)) return;
    if (tree.uri && path.dirname(tree.uri.fsPath) === path.dirname(uri.fsPath)) return;
    tree.uri = uri;
    view.description = path.basename(path.dirname(uri.fsPath));
    commands.executeCommand("setContext", "cimoxide.model.active", true);
    tree.refresh();
  };
  const filter = (on: boolean): void => {
    tree.onlyFindings = on;
    commands.executeCommand("setContext", "cimoxide.model.onlyFindings", on);
    view.message = on ? "Only objects with findings" : undefined;
    tree.refresh();
  };
  // The object at the cursor, selected in the tree.
  let shown: string | undefined;
  const revealCursor = async (focus: boolean): Promise<void> => {
    const editor = window.activeTextEditor;
    if (!editor || !isCim(editor.document.uri, editor.document.languageId)) return;
    follow(editor.document.uri, editor.document.languageId);
    const at = await tree.locate(editor.document.uri, editor.selection.active);
    if (!at) {
      if (focus) window.showInformationMessage("cimoxide: no object of the model tree here (or the filter hides it).");
      return;
    }
    if (!focus && at.key === shown) return;
    shown = at.key;
    await view.reveal(at, { select: true, focus, expand: false });
  };
  let cursor: NodeJS.Timeout | undefined;
  // Deleting goes through the refactoring preview: the server marks its edit as needing confirmation.
  const remove = async (e: Entry | undefined, dependents: boolean): Promise<void> => {
    const c = client();
    if (!c || !tree.uri || !e?.id) return;
    const found = await c.sendRequest<{ edit: unknown } | null>(DELETE_REQUEST, { uri: tree.uri.toString(), id: e.id, dependents });
    if (!found) return;
    const edit = await c.protocol2CodeConverter.asWorkspaceEdit(found.edit as never);
    if (edit) await workspace.applyEdit(edit, { isRefactoring: true });
  };
  // Renaming is the editor's: put the cursor on the object's start tag and ask for it.
  const rename = async (e: Entry | undefined): Promise<void> => {
    if (!e?.location) return;
    const { uri, range } = e.location;
    const doc = await workspace.openTextDocument(Uri.parse(uri));
    const at = new Position(range.start.line, range.start.character + 1);
    await window.showTextDocument(doc, { viewColumn: ViewColumn.One, selection: new Range(at, at) });
    await commands.executeCommand("editor.action.rename");
  };
  const reveal = async (e: Entry | undefined, inside: boolean): Promise<void> => {
    const c = client();
    if (!c || !tree.uri || !e?.grid) return;
    await revealInGrid(context, c, tree.uri, inside ? { enter: e.grid.inside ?? undefined } : e.grid);
  };

  follow(window.activeTextEditor?.document.uri, window.activeTextEditor?.document.languageId);
  context.subscriptions.push(
    view,
    { dispose: () => (clearTimeout(refresh), clearTimeout(cursor)) } as Disposable,
    // With `cimoxide.model.followCursor`, the tree follows the cursor while it is visible.
    window.onDidChangeTextEditorSelection((e) => {
      if (!view.visible || !workspace.getConfiguration("cimoxide").get<boolean>("model.followCursor")) return;
      if (e.textEditor !== window.activeTextEditor) return;
      clearTimeout(cursor);
      cursor = setTimeout(() => revealCursor(false), 300);
    }),
    commands.registerCommand("cimoxide.model.reveal", () => revealCursor(true)),
    commands.registerCommand("cimoxide.model.delete", (e?: Entry) => remove(e, false)),
    commands.registerCommand("cimoxide.model.deleteAll", (e?: Entry) => remove(e, true)),
    commands.registerCommand("cimoxide.model.rename", (e?: Entry) => rename(e)),
    window.registerFileDecorationProvider({ provideFileDecoration: decoration }),
    window.onDidChangeActiveTextEditor((ed) => follow(ed?.document.uri, ed?.document.languageId)),
    // Findings change with every validation, and the set is there after the first.
    languages.onDidChangeDiagnostics((e) => {
      const dir = tree.uri && path.dirname(tree.uri.fsPath);
      if (dir && e.uris.some((u) => u.scheme === "file" && path.dirname(u.fsPath) === dir)) {
        clearTimeout(refresh);
        refresh = setTimeout(() => tree.refresh(), 300);
      }
    }),
    commands.registerCommand("cimoxide.model.refresh", () => tree.refresh()),
    commands.registerCommand("cimoxide.model.onlyFindings", () => filter(true)),
    commands.registerCommand("cimoxide.model.allObjects", () => filter(false)),
    commands.registerCommand("cimoxide.model.showInGrid", (e?: Entry) => reveal(e, false)),
    commands.registerCommand("cimoxide.model.showInside", (e?: Entry) => reveal(e, true)),
    commands.registerCommand("cimoxide.model.open", async (location: Location) => {
      const doc = await workspace.openTextDocument(Uri.parse(location.uri));
      const { start, end } = location.range;
      await window.showTextDocument(doc, {
        viewColumn: ViewColumn.One,
        selection: new Range(start.line, start.character, end.line, end.character),
        preserveFocus: true,
      });
    }),
  );
}
