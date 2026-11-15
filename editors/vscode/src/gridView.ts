// The grid view: a model set's substations and the lines between them, with
// each one's findings, drawn in a webview from the language server's
// `cimoxide/grid` request, and what a substation holds from
// `cimoxide/substation`. Clicking an object opens where it is written.

import * as crypto from "crypto";
import * as path from "path";
import {
  Disposable,
  ExtensionContext,
  languages,
  Uri,
  ViewColumn,
  WebviewPanel,
  window,
  workspace,
} from "vscode";
import { LanguageClient, WorkspaceEdit as LspWorkspaceEdit } from "vscode-languageclient/node";
import { applyEdit } from "./applyEdit";
import { activeFile, openAt, validateSet } from "./open";

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

/**
 * Where the grid shows an object, as the server says: `select`, the node,
 * edge (`e:<id>`) or item to select; `enter`, the substation it is inside;
 * `inside`, a substation's own id.
 */
export interface Reveal {
  select?: string | null;
  enter?: string;
  inside?: string | null;
}

const GRID_REQUEST = "cimoxide/grid";
const SUBSTATION_REQUEST = "cimoxide/substation";
/** What the webview may ask of the server while editing inside a substation. */
const EDITING_REQUESTS = new Set(["cimoxide/reroute", "cimoxide/diagram", "cimoxide/palette", "cimoxide/topology"]);

/** One panel per model set, by directory. */
const panels = new Map<string, GridPanel>();

/** Show the grid of `uri`'s model set (the editor title button passes it), else the active tab's. */
export async function showGrid(context: ExtensionContext, client: LanguageClient | undefined, uri?: Uri): Promise<void> {
  const file = uri?.scheme === "file" ? uri : activeFile();
  if (!client || !file) {
    window.showInformationMessage("cimoxide: open a CGMES or NC file to show its model set's grid.");
    return;
  }
  await openGrid(context, client, file);
}

/** Show the grid of `uri`'s model set with an object selected, or inside its substation. */
export async function revealInGrid(context: ExtensionContext, client: LanguageClient, uri: Uri, at: Reveal): Promise<void> {
  (await openGrid(context, client, uri))?.reveal(at);
}

/** The grid panel of `uri`'s model set, opened if need be. */
async function openGrid(context: ExtensionContext, client: LanguageClient, uri: Uri): Promise<GridPanel | undefined> {
  const dir = path.dirname(uri.fsPath);
  const open = panels.get(dir);
  if (open) {
    open.panel.reveal(undefined, true);
    return open;
  }
  // A set is validated shortly after one of its files opens; one whose file
  // VS Code keeps from the server (over 50 MB) only when asked.
  let grid: Grid | null = await client.sendRequest<Grid | null>(GRID_REQUEST, { uri: uri.toString() });
  if (!grid) {
    await validateSet(client, uri);
  }
  for (let attempt = 0; attempt < 60 && !grid; attempt++) {
    await new Promise((resolve) => setTimeout(resolve, 500));
    grid = await client.sendRequest<Grid | null>(GRID_REQUEST, { uri: uri.toString() });
  }
  if (!grid) {
    window.showInformationMessage("cimoxide: this file's model set has not been validated; is it a CGMES or NC file?");
    return undefined;
  }
  const panel = new GridPanel(context, client, uri, dir, grid);
  panels.set(dir, panel);
  return panel;
}

class GridPanel {
  readonly panel: WebviewPanel;
  private files: Set<string>;
  /** The substation shown inside, if any. */
  private inside: string | undefined;
  private refresh: NodeJS.Timeout | undefined;
  /** Whether the page has drawn the grid, and what to show once it has. */
  private ready = false;
  private pending: Reveal | undefined;
  /** What a topology edit added: selected and opened once the substation shows it. */
  private opening: string | undefined;
  private readonly disposables: Disposable[] = [];

  constructor(
    context: ExtensionContext,
    private readonly client: LanguageClient,
    private readonly uri: Uri,
    private readonly dir: string,
    grid: Grid,
  ) {
    const dist = Uri.joinPath(context.extensionUri, "dist");
    this.panel = window.createWebviewPanel("cimoxide.grid", `Grid: ${path.basename(dir)}`, { viewColumn: ViewColumn.Beside, preserveFocus: true }, {
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
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${csp} 'unsafe-inline'; script-src 'nonce-${nonce}'; img-src data:;">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Grid</title>
</head>
<body>
<div id="bar"><button id="back" title="Back to the grid (Escape)">&larr; Grid</button><select id="layout" title="Layout"></select><select id="colour" title="Colour by" style="display:none"><option value="voltage">Voltage level</option><option value="state">State (SV)</option></select><select id="values" title="The SV's values beside what they describe: all, those that need attention (loaded from 80 %, 5 % off nominal), or none" style="display:none"><option value="exceptions">Values: exceptions</option><option value="all">Values: all</option><option value="off">Values: off</option></select><input id="search" type="search" placeholder="Find a substation"><span id="edit"></span><span id="summary"></span></div>
<div id="palette" style="display:none"></div>
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
    if (this.inside) {
      await this.updateInside();
    }
  }

  /** What the substation shown holds; `null` once it is gone from the set. */
  private async updateInside(): Promise<void> {
    const inside = await this.client.sendRequest<unknown>(SUBSTATION_REQUEST, { uri: this.uri.toString(), id: this.inside });
    if (!inside) {
      this.inside = undefined;
    }
    this.panel.webview.postMessage({ type: "inside", inside });
    const added = this.opening && (inside as { items?: { id: string; location?: Location | null }[] } | null)?.items?.find((it) => it.id === this.opening);
    if (added) {
      this.opening = undefined;
      this.panel.webview.postMessage({ type: "select", id: added.id });
      // Where it is written, for what it still needs: validation's findings and their quick fixes.
      if (added.location) await openAt(added.location.uri, added.location.range, true);
    }
  }

  /** Select an object, entering its substation first, or leaving the one shown. */
  async reveal(at: Reveal): Promise<void> {
    if (!this.ready) {
      this.pending = at;
      return;
    }
    if (at.enter) {
      if (this.inside !== at.enter) {
        this.inside = at.enter;
        await this.updateInside();
      }
    } else if (this.inside) {
      this.inside = undefined;
      this.panel.webview.postMessage({ type: "leave" });
    }
    if (at.select) {
      this.panel.webview.postMessage({ type: "select", id: at.select });
    }
  }

  /**
   * An editing request from the webview, for the server: the edit it answers applied (left
   * unsaved, as VS Code's refactorings are), the rest replied. What it added is opened once the
   * set is validated again.
   */
  private async ask(n: number, method: string, params: Record<string, unknown>): Promise<void> {
    const reply = (value: unknown, error?: string) => this.panel.webview.postMessage({ type: "reply", n, value, error });
    if (!EDITING_REQUESTS.has(method)) {
      reply(null, `no request ${method}`);
      return;
    }
    try {
      const answer = await this.client.sendRequest<Record<string, unknown> | null>(method, { uri: this.uri.toString(), ...params });
      if (answer && typeof answer === "object" && "edit" in answer) {
        if (!(await applyEdit(this.client, answer.edit as LspWorkspaceEdit))) {
          reply(null, "VS Code did not apply the edit");
          return;
        }
        const { edit: _, ...rest } = answer;
        if (typeof rest.select === "string") this.opening = rest.select;
        reply(rest);
        return;
      }
      reply(answer);
    } catch (e) {
      reply(null, e instanceof Error ? e.message : String(e));
    }
  }

  private async receive(msg: { type: string; location?: Location; id?: string; n?: number; method?: string; params?: Record<string, unknown> }): Promise<void> {
    if (msg.type === "ask" && msg.n !== undefined && msg.method) {
      await this.ask(msg.n, msg.method, msg.params ?? {});
      return;
    }
    if (msg.type === "ready") {
      await this.update();
      this.ready = true;
      if (this.pending) {
        await this.reveal(this.pending);
        this.pending = undefined;
      }
    } else if (msg.type === "enter" && msg.id) {
      this.inside = msg.id;
      await this.updateInside();
    } else if (msg.type === "leave") {
      this.inside = undefined;
    } else if (msg.type === "open" && msg.location) {
      await openAt(msg.location.uri, msg.location.range);
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
