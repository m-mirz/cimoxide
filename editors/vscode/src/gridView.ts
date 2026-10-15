// The grid view: a model set's substations and the lines between them, with
// each one's findings, drawn in a webview from the language server's
// `cimoxide/grid` request. Clicking an object opens where it is written.

import * as crypto from "crypto";
import * as path from "path";
import {
  Disposable,
  ExtensionContext,
  languages,
  Range,
  Uri,
  ViewColumn,
  WebviewPanel,
  window,
  workspace,
} from "vscode";
import { LanguageClient } from "vscode-languageclient/node";

/** A place in a file, as the server sends it. */
interface Location {
  uri: string;
  range: { start: { line: number; character: number }; end: { line: number; character: number } };
}

/** The `cimoxide/grid` response; `null` before the set's first validation. */
interface Grid {
  files: string[];
  nodes: unknown[];
  edges: unknown[];
}

const GRID_REQUEST = "cimoxide/grid";

/** One panel per model set, by directory. */
const panels = new Map<string, GridPanel>();

export async function showGrid(context: ExtensionContext, client: LanguageClient | undefined): Promise<void> {
  const editor = window.activeTextEditor;
  if (!client || !editor || editor.document.uri.scheme !== "file") {
    window.showInformationMessage("cimoxide: open a CGMES or NC file to show its model set's grid.");
    return;
  }
  const uri = editor.document.uri;
  const dir = path.dirname(uri.fsPath);
  const open = panels.get(dir);
  if (open) {
    open.panel.reveal();
    return;
  }
  // A set is validated shortly after one of its files opens.
  let grid: Grid | null = null;
  for (let attempt = 0; attempt < 20 && !grid; attempt++) {
    grid = await client.sendRequest<Grid | null>(GRID_REQUEST, { uri: uri.toString() });
    if (!grid) {
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
  }
  if (!grid) {
    window.showInformationMessage("cimoxide: this file's model set has not been validated; is it a CGMES or NC file?");
    return;
  }
  panels.set(dir, new GridPanel(context, client, uri, dir, grid));
}

class GridPanel {
  readonly panel: WebviewPanel;
  private files: Set<string>;
  private refresh: NodeJS.Timeout | undefined;
  private readonly disposables: Disposable[] = [];

  constructor(
    context: ExtensionContext,
    private readonly client: LanguageClient,
    private readonly uri: Uri,
    private readonly dir: string,
    grid: Grid,
  ) {
    const dist = Uri.joinPath(context.extensionUri, "dist");
    this.panel = window.createWebviewPanel("cimoxide.grid", `Grid: ${path.basename(dir)}`, ViewColumn.Beside, {
      enableScripts: true,
      retainContextWhenHidden: true,
      localResourceRoots: [dist],
    });
    this.files = new Set(grid.files);
    const nonce = crypto.randomBytes(16).toString("base64");
    const script = this.panel.webview.asWebviewUri(Uri.joinPath(dist, "grid.js"));
    const csp = this.panel.webview.cspSource;
    this.panel.webview.html = `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${csp} 'unsafe-inline'; script-src 'nonce-${nonce}';">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Grid</title>
</head>
<body>
<div id="bar"><input id="search" type="search" placeholder="Find a substation"><span id="summary"></span></div>
<div id="main"><div id="graph"></div><div id="info"></div></div>
<script nonce="${nonce}" src="${script}"></script>
</body>
</html>`;
    this.disposables.push(
      this.panel.webview.onDidReceiveMessage((msg) => this.receive(msg)),
      // Findings change with every validation: redraw their counts.
      languages.onDidChangeDiagnostics((e) => {
        if (e.uris.some((u) => this.files.has(u.toString()))) {
          clearTimeout(this.refresh);
          this.refresh = setTimeout(() => this.update(), 300);
        }
      }),
    );
    // The webview asks for the grid once its script runs (`ready`).
    this.panel.onDidDispose(() => this.dispose(), null, this.disposables);
  }

  private post(grid: Grid): void {
    this.panel.webview.postMessage({ type: "grid", grid });
  }

  private async update(): Promise<void> {
    const grid = await this.client.sendRequest<Grid | null>(GRID_REQUEST, { uri: this.uri.toString() });
    if (grid) {
      this.files = new Set(grid.files);
      this.post(grid);
    }
  }

  private async receive(msg: { type: string; location?: Location }): Promise<void> {
    if (msg.type === "ready") {
      await this.update();
    } else if (msg.type === "open" && msg.location) {
      const { uri, range } = msg.location;
      const doc = await workspace.openTextDocument(Uri.parse(uri));
      const selection = new Range(range.start.line, range.start.character, range.end.line, range.end.character);
      await window.showTextDocument(doc, { viewColumn: ViewColumn.One, selection });
    }
  }

  private dispose(): void {
    clearTimeout(this.refresh);
    panels.delete(this.dir);
    for (const d of this.disposables) {
      d.dispose();
    }
  }
}
