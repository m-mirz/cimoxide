// The model view: the objects of the active editor's model set by region,
// substation and voltage level, by area and by class, from the language
// server's `cimoxide/tree`, a level at a time; each with the findings of what
// it holds, as a badge in the colour of the worst.

import * as fs from "fs";
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
  ThemeColor,
  ThemeIcon,
  TreeDataProvider,
  TreeItem,
  TreeItemCollapsibleState,
  TreeView,
  Uri,
  window,
  workspace,
} from "vscode";
import { LanguageClient, WorkspaceEdit as LspWorkspaceEdit } from "vscode-languageclient/node";
import { Reveal, revealInGrid } from "./gridView";
import { activeFile, openAt, openEdited, validateSet } from "./open";
import { symbolOf } from "./symbols";

const TREE_REQUEST = "cimoxide/tree";
const PATH_REQUEST = "cimoxide/treePath";
const PATH_OF_REQUEST = "cimoxide/treePathOf";
const SEARCH_REQUEST = "cimoxide/search";
const REFERRERS_REQUEST = "cimoxide/referrers";
const DELETE_REQUEST = "cimoxide/delete";
const ADDABLE_REQUEST = "cimoxide/addable";
const ADD_REQUEST = "cimoxide/add";
const FIXES_REQUEST = "cimoxide/fixes";
const RENAME_REQUEST = "cimoxide/rename";
/** What VS Code hands an extension at most of a file. */
const SYNC_LIMIT = 50 * 1024 * 1024;
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
export interface Entry {
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
  DCLine: "dash",
  Terminal: "plug",
  DCTerminal: "plug",
  ACDCConverterDCTerminal: "plug",
  PowerTransformerEnd: "debug-stackframe-dot",
  RatioTapChanger: "settings",
  PhaseTapChanger: "settings",
  PhaseTapChangerTabular: "settings",
  PhaseTapChangerLinear: "settings",
  PhaseTapChangerSymmetrical: "settings",
  PhaseTapChangerAsymmetrical: "settings",
  GeneratingUnit: "flame",
  ThermalGeneratingUnit: "flame",
  HydroGeneratingUnit: "flame",
  WindGeneratingUnit: "flame",
  SolarGeneratingUnit: "flame",
  NuclearGeneratingUnit: "flame",
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

/**
 * Where the equipment symbols are as icon files, light and dark: written at
 * build time (build-symbols.js) into the extension, as VS Code loads a tree
 * icon only from where it trusts.
 */
let symbolDir: Uri | undefined;

/** Equipment by its symbol (as the grid view draws it), else a codicon. */
function icon(e: Entry): ThemeIcon | { light: Uri; dark: Uri } {
  if (e.kind === "section") return new ThemeIcon(SECTION_ICONS[e.key] ?? "folder");
  if (e.kind === "range") return new ThemeIcon("list-flat");
  if (e.kind === "missing") return new ThemeIcon("question");
  const cls = e.class ?? "";
  const symbol = symbolOf(cls);
  if (symbol && symbolDir) {
    return { light: Uri.joinPath(symbolDir, `${symbol}-light.svg`), dark: Uri.joinPath(symbolDir, `${symbol}-dark.svg`) };
  }
  if (e.kind === "group") return new ThemeIcon("symbol-class");
  return new ThemeIcon(CLASS_ICONS[cls] ?? (cls.startsWith("Sv") ? "graph" : cls.endsWith("Limit") ? "dashboard" : "symbol-object"));
}

export class ModelTree implements TreeDataProvider<Entry> {
  private readonly changed = new EventEmitter<Entry | undefined>();
  readonly onDidChangeTreeData = this.changed.event;
  /** A document of the model set shown. */
  uri: Uri | undefined;
  onlyFindings = false;

  constructor(private readonly client: () => LanguageClient | undefined) {}

  /** The directories whose sets the tree asked the server to validate. */
  private readonly asked = new Set<string>();
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
    // Not validated yet: perhaps a file VS Code keeps from the server (over 50 MB).
    const dir = path.dirname(this.uri.fsPath);
    if (!result && !entry && !this.asked.has(dir)) {
      this.asked.add(dir);
      await validateSet(client, this.uri);
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
    return this.walk(found?.keys ?? []);
  }

  /** The node of object `id` of the set shown, its ancestors fetched on the way. */
  async locateId(id: string): Promise<Entry | undefined> {
    const client = this.client();
    if (!client || !this.uri) return undefined;
    const found = await client.sendRequest<{ id: string; keys: string[] } | null>(PATH_OF_REQUEST, {
      uri: this.uri.toString(),
      id,
      onlyFindings: this.onlyFindings,
    });
    return this.walk(found?.keys ?? []);
  }

  /** Down `keys` from the sections, each level fetched. */
  private async walk(keys: string[]): Promise<Entry | undefined> {
    let at: Entry | undefined;
    for (const key of keys) {
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

export function registerModelTree(context: ExtensionContext, client: () => LanguageClient | undefined): ModelTree {
  symbolDir = Uri.joinPath(context.extensionUri, "dist", "symbols");
  const tree = new ModelTree(client);
  const view: TreeView<Entry> = window.createTreeView(VIEW, { treeDataProvider: tree, showCollapseAll: true });
  let refresh: NodeJS.Timeout | undefined;

  const follow = (uri: Uri | undefined, languageId: string | undefined): void => {
    // A tab with no text editor (a file over 50 MB): an XML file by its name.
    if (!uri || !(languageId ? isCim(uri, languageId) : activeFile()?.toString() === uri.toString())) return;
    if (tree.uri && path.dirname(tree.uri.fsPath) === path.dirname(uri.fsPath)) return;
    tree.uri = uri;
    view.description = path.basename(path.dirname(uri.fsPath));
    commands.executeCommand("setContext", "cimoxide.model.active", true);
    tree.refresh();
  };
  // A file VS Code keeps from extensions: say so once, and where its model is.
  const noticed = new Set<string>();
  const notice = (uri: Uri | undefined): void => {
    if (!uri || noticed.has(uri.toString())) return;
    let size = 0;
    try {
      size = fs.statSync(uri.fsPath).size;
    } catch {
      return;
    }
    if (size <= SYNC_LIMIT) return;
    noticed.add(uri.toString());
    const name = path.basename(uri.fsPath);
    window
      .showInformationMessage(
        `cimoxide: ${name} is over 50 MB, so VS Code does not pass it to extensions: no hover, completion, ` +
          "quick fixes or rename in its editor. Its model set is validated, and can be browsed, fixed and edited " +
          "from the CIM Model view.",
        "Show CIM Model",
      )
      .then((choice) => choice && commands.executeCommand(`${VIEW}.focus`));
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
  // An object found: where it is written, and its node in the tree.
  const revealHit = async (hit: { id: string; label: string; location: Location | null }): Promise<void> => {
    if (hit.location) await openAt(hit.location.uri, hit.location.range, true);
    const at = await tree.locateId(hit.id);
    if (at) await view.reveal(at, { select: true, focus: true, expand: true });
    else window.showInformationMessage(`cimoxide: ${hit.label} is hidden by the filter on findings.`);
  };
  // What refers to an object (the tree's Find references): pick one to go to it.
  const referrers = async (e: Entry | undefined): Promise<void> => {
    const c = client();
    if (!c || !tree.uri || !e?.id) return;
    type Referrer = { id: string; label: string; class: string; key: string; place: string; location: Location | null };
    const found = await c.sendRequest<Referrer[]>(REFERRERS_REQUEST, { uri: tree.uri.toString(), id: e.id });
    if (!found.length) {
      window.showInformationMessage(`cimoxide: nothing in the model set refers to ${e.class} ${e.label}.`);
      return;
    }
    const pick = await window.showQuickPick(
      found.map((r) => ({ label: r.label, description: `${r.class} · ${r.key}`, detail: r.place || undefined, r })),
      { placeHolder: `${found.length} object(s) refer to ${e.class} ${e.label}`, matchOnDescription: true },
    );
    if (pick) await revealHit(pick.r);
  };
  // Search the whole set, not only what the tree has loaded: the server
  // matches names and mRIDs as one types.
  const search = (): void => {
    const c = client();
    const uri = tree.uri;
    if (!c || !uri) {
      window.showInformationMessage("cimoxide: open a CGMES or NC file to search its model set.");
      return;
    }
    type Hit = { id: string; label: string; class: string; place: string; location: Location | null };
    const pick = window.createQuickPick<{ label: string; description: string; detail?: string; alwaysShow: boolean; hit: Hit }>();
    pick.placeholder = `Find an object of ${path.basename(path.dirname(uri.fsPath))} by name or mRID`;
    let typing: NodeJS.Timeout | undefined;
    pick.onDidChangeValue((value) => {
      clearTimeout(typing);
      typing = setTimeout(async () => {
        pick.busy = true;
        const hits = await c.sendRequest<Hit[]>(SEARCH_REQUEST, { uri: uri.toString(), query: value });
        // The server has filtered: keep its order, and what matched by mRID.
        if (pick.value === value) {
          pick.items = hits.map((h) => ({ label: h.label, description: h.class, detail: h.place || undefined, alwaysShow: true, hit: h }));
        }
        pick.busy = false;
      }, 150);
    });
    pick.onDidAccept(async () => {
      const hit = pick.selectedItems[0]?.hit;
      pick.hide();
      if (hit) await revealHit(hit);
    });
    pick.onDidHide(() => {
      clearTimeout(typing);
      pick.dispose();
    });
    pick.show();
  };
  // Deleting goes through the refactoring preview: the server marks its edit as needing confirmation.
  const remove = async (e: Entry | undefined, dependents: boolean): Promise<void> => {
    const c = client();
    if (!c || !tree.uri || !e?.id) return;
    const found = await c.sendRequest<{ edit: unknown } | null>(DELETE_REQUEST, { uri: tree.uri.toString(), id: e.id, dependents });
    if (!found) return;
    // Applied once confirmed in the preview; then what it changed, for review.
    await openEdited(await apply(found.edit as LspWorkspaceEdit, true));
  };
  // Adding asks for the class, from those the containment puts in the
  // object, then for a name; the server writes the object, and validation
  // asks for what else it needs, which the quick fixes add.
  const addIn = async (arg?: { id?: string; uri?: string; label?: string; class?: string }): Promise<void> => {
    const c = client();
    const uri = arg?.uri ?? tree.uri?.toString();
    if (!c || !uri || !arg?.id) return;
    const where = arg.class ? `${arg.class} ${arg.label}` : (arg.label ?? arg.id);
    const classes = await c.sendRequest<{ class: string; key: string; comment: string }[]>(ADDABLE_REQUEST, { uri, id: arg.id });
    if (!classes.length) {
      window.showInformationMessage(`cimoxide: nothing is held in ${where}.`);
      return;
    }
    const pick = await window.showQuickPick(
      classes.map((x) => ({ label: x.class, description: x.key, detail: x.comment || undefined })),
      { placeHolder: `Add in ${where}`, matchOnDescription: true },
    );
    if (!pick) return;
    const name = await window.showInputBox({
      prompt: `IdentifiedObject.name of the new ${pick.label}`,
      placeHolder: "Empty: name it later",
    });
    if (name === undefined) return;
    let added: { edit: unknown; uri: string; select: Location["range"] };
    try {
      added = await c.sendRequest(ADD_REQUEST, { uri, parent: arg.id, class: pick.label, name });
    } catch (e) {
      window.showErrorMessage(`cimoxide: ${e instanceof Error ? e.message : e}`);
      return;
    }
    const edit = await c.protocol2CodeConverter.asWorkspaceEdit(added.edit as LspWorkspaceEdit);
    if (!edit || !(await workspace.applyEdit(edit))) return;
    await openAt(added.uri, added.select);
  };
  // Renaming is the editor's: put the cursor on the object's start tag and ask for it.
  // The tree's edits are the server's, from the files as they are: so they
  // work where the editor's do not, in a file over 50 MB.
  const apply = async (edit: LspWorkspaceEdit, refactoring = false): Promise<Uri[]> => {
    const c = client();
    const converted = c && (await c.protocol2CodeConverter.asWorkspaceEdit(edit));
    if (!converted || !(await workspace.applyEdit(converted, { isRefactoring: refactoring }))) return [];
    return converted.entries().map(([u]) => u);
  };
  const rename = async (e: Entry | undefined): Promise<void> => {
    const c = client();
    if (!c || !tree.uri || !e?.id) return;
    const newName = await window.showInputBox({ prompt: `New mRID of ${e.class} ${e.label}`, value: e.id, valueSelection: [0, e.id.length] });
    if (!newName || newName === e.id) return;
    try {
      const edit = await c.sendRequest<LspWorkspaceEdit>(RENAME_REQUEST, { uri: tree.uri.toString(), id: e.id, newName });
      await openEdited(await apply(edit, true));
    } catch (err) {
      window.showErrorMessage(`cimoxide: ${err instanceof Error ? err.message : err}`);
    }
  };
  // The quick fixes of an object's findings, as the editor offers them.
  const fix = async (e: Entry | undefined): Promise<void> => {
    const c = client();
    if (!c || !tree.uri || !e?.id) return;
    const found = await c.sendRequest<{ title: string; detail: string; edit: LspWorkspaceEdit; uri: string; select: Location["range"] | null }[]>(
      FIXES_REQUEST, { uri: tree.uri.toString(), id: e.id });
    if (!found.length) {
      window.showInformationMessage(
        `cimoxide: nothing to fix on ${e.class} ${e.label}: fixes add a missing field, or name an object where a reference names none.`);
      return;
    }
    const pick = await window.showQuickPick(found.map((f) => ({ label: f.title, detail: f.detail, f })), { placeHolder: `Fix ${e.class} ${e.label}` });
    if (!pick) return;
    const changed = await apply(pick.f.edit);
    if (pick.f.select) await openAt(pick.f.uri, pick.f.select);
    else await openEdited(changed);
  };
  const reveal = async (e: Entry | undefined, inside: boolean): Promise<void> => {
    const c = client();
    if (!c || !tree.uri || !e?.grid) return;
    await revealInGrid(context, c, tree.uri, inside ? { enter: e.grid.inside ?? undefined } : e.grid);
  };

  follow(window.activeTextEditor?.document.uri, window.activeTextEditor?.document.languageId);
  if (!window.activeTextEditor) {
    follow(activeFile(), undefined);
    notice(activeFile());
  }
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
    commands.registerCommand("cimoxide.model.fix", (e?: Entry) => fix(e)),
    commands.registerCommand("cimoxide.model.search", search),
    commands.registerCommand("cimoxide.model.referrers", (e?: Entry) => referrers(e)),
    commands.registerCommand("cimoxide.model.add", (e?: Entry | { id: string; uri: string; label: string }) => addIn(e)),
    window.registerFileDecorationProvider({ provideFileDecoration: decoration }),
    window.onDidChangeActiveTextEditor((ed) => follow(ed?.document.uri, ed?.document.languageId)),
    window.tabGroups.onDidChangeTabs(() => {
      if (!window.activeTextEditor) {
        follow(activeFile(), undefined);
        notice(activeFile());
      }
    }),
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
      await openAt(location.uri, location.range, true);
    }),
  );
  return tree;
}
