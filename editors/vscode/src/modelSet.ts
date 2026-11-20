// The model set the user chose: files, from any folders, that form one set,
// taking them out of their folders' sets. Without one, a model set is every
// CIM file of a folder, as before.
//
// Remembered per workspace. The language server hears of it (`cimoxide/setFiles`);
// `cimmcp` reads it from a file whose path it is given once (`--set-file`), as
// the set a tool reads when told none, so chat answers about the same files.

import * as fs from "fs";
import * as path from "path";
import { commands, EventEmitter, ExtensionContext, StatusBarAlignment, StatusBarItem, Uri, window } from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { activeFile } from "./open";

/** The language server's notification of the files chosen; none, folders again. */
const SET_FILES = "cimoxide/setFiles";
/** Where the workspace remembers them. */
const STATE = "cimoxide.chosenFiles";

export class ChosenSet {
  private readonly changed = new EventEmitter<void>();
  /** The set chose, or was cleared. */
  readonly onDidChange = this.changed.event;
  private readonly status: StatusBarItem;
  /** The files chosen, sorted; none, folders. */
  files: Uri[];
  /** The file `cimmcp` reads them from. */
  readonly setFile: string;

  constructor(
    private readonly context: ExtensionContext,
    private readonly client: () => LanguageClient | undefined,
  ) {
    this.files = (context.workspaceState.get<string[]>(STATE) ?? []).map((u) => Uri.parse(u));
    const dir = (context.storageUri ?? context.globalStorageUri).fsPath;
    this.setFile = path.join(dir, "model-set.json");
    this.status = window.createStatusBarItem("cimoxide.modelSet", StatusBarAlignment.Left, 10);
    this.status.name = "CIM model set";
    this.status.command = "cimoxide.modelSet.menu";
    context.subscriptions.push(this.changed, this.status);
    this.write();
    this.show();
  }

  /** The set `uri` belongs to: `"chosen"`, else its folder. */
  keyOf(uri: Uri): string {
    return this.has(uri) ? "chosen" : path.dirname(uri.fsPath);
  }

  has(uri: Uri): boolean {
    return this.files.some((f) => f.fsPath === uri.fsPath);
  }

  /** Make `uris` the set; none, folders again. */
  async set(uris: Uri[]): Promise<void> {
    const unique = new Map(uris.filter((u) => u.scheme === "file").map((u) => [u.fsPath, u]));
    this.files = [...unique.values()].sort((a, b) => a.fsPath.localeCompare(b.fsPath));
    await this.context.workspaceState.update(STATE, this.files.length ? this.files.map((f) => f.toString()) : undefined);
    this.write();
    await this.send();
    this.show();
    this.changed.fire();
  }

  /** Files picked in a dialog, from the active file's folder: the set, or with `add`, added to it. */
  async choose(add = false): Promise<void> {
    const from = this.files[0] ?? activeFile();
    const picked = await window.showOpenDialog({
      canSelectMany: true,
      canSelectFolders: false,
      defaultUri: from ? Uri.file(path.dirname(from.fsPath)) : undefined,
      filters: { "CIM RDF/XML": ["xml"] },
      openLabel: add ? "Add to model set" : "Use as model set",
      title: add ? "Add files to the CIM model set" : "Choose the files of the CIM model set",
    });
    if (!picked?.length) return;
    await this.set(add ? [...this.files, ...picked] : picked);
  }

  /** Tell the language server; on its start too. */
  async send(): Promise<void> {
    await this.client()?.sendNotification(SET_FILES, { files: this.files.map((f) => f.toString()) });
  }

  /** The list `cimmcp` reads, `{ "files": [path] }`. */
  private write(): void {
    try {
      fs.mkdirSync(path.dirname(this.setFile), { recursive: true });
      fs.writeFileSync(this.setFile, JSON.stringify({ files: this.files.map((f) => f.fsPath) }));
    } catch (e) {
      window.showWarningMessage(`cimoxide: could not write the model set for chat (${e}).`);
    }
  }

  private show(): void {
    const n = this.files.length;
    if (!n) {
      this.status.hide();
      return;
    }
    this.status.text = `$(files) CIM set: ${n} file${n === 1 ? "" : "s"}`;
    this.status.tooltip = `The CIM model set chosen:\n${this.files.map((f) => f.fsPath).join("\n")}\n\nClick to change it, or to use folders again.`;
    this.status.show();
  }
}

export function registerModelSet(context: ExtensionContext, client: () => LanguageClient | undefined): ChosenSet {
  const chosen = new ChosenSet(context, client);
  context.subscriptions.push(
    commands.registerCommand("cimoxide.modelSet.choose", () => chosen.choose()),
    commands.registerCommand("cimoxide.modelSet.add", () => chosen.choose(true)),
    commands.registerCommand("cimoxide.modelSet.clear", () => chosen.set([])),
    // From the Explorer: the file clicked, or every file selected.
    commands.registerCommand("cimoxide.modelSet.use", (uri?: Uri, uris?: Uri[]) => {
      const files = (uris?.length ? uris : uri ? [uri] : []).filter((u) => u.fsPath.toLowerCase().endsWith(".xml"));
      return files.length ? chosen.set(files) : chosen.choose();
    }),
    commands.registerCommand("cimoxide.modelSet.menu", async () => {
      const items = [
        { label: "$(files) Choose other files…", run: () => chosen.choose() },
        { label: "$(add) Add files…", run: () => chosen.choose(true) },
        { label: "$(folder) Use folders again", run: () => chosen.set([]) },
        ...chosen.files.map((f) => ({ label: `$(file) ${path.basename(f.fsPath)}`, description: path.dirname(f.fsPath), run: () => commands.executeCommand("vscode.open", f) })),
      ];
      const pick = await window.showQuickPick(items, { placeHolder: "The CIM model set chosen" });
      await pick?.run();
    }),
  );
  return chosen;
}
