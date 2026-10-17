// The README's screenshots (test/screenshots.js), taken in a running VS Code:
// the extension's own commands set each scene, the Chrome DevTools protocol
// (VS Code started with --remote-debugging-port) sizes the workbench, drags
// its sashes and captures it, title bar cropped.

import * as fs from "fs";
import * as path from "path";
import { commands, DiagnosticSeverity, extensions, languages, Range, Selection, TextEditorRevealType, Uri, window, workspace, WorkspaceEdit } from "vscode";
import type { Api } from "../src/extension";

// The extension host's Node has WebSocket; @types/node 18 does not declare it.
declare const WebSocket: new (url: string) => {
  onopen: () => void;
  onerror: (e: unknown) => void;
  onmessage: (m: { data: string }) => void;
  send(data: string): void;
};

const out = process.env.CIMOXIDE_SHOTS as string;
const data = process.env.CIMOXIDE_SHOTS_DATA as string;
const port = process.env.CIMOXIDE_SHOTS_PORT as string;
const take = (process.env.CIMOXIDE_SHOTS_TAKE as string).split(",");
/** The workbench's size in CSS pixels, captured at twice that. */
const W = 1400;
const H = 860;
const SCALE = 2;

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

/** `f`'s first defined answer, asked every 250 ms for at most `ms`. */
async function until<T>(what: string, f: () => Promise<T | undefined> | T | undefined, ms = 60000): Promise<T> {
  const end = Date.now() + ms;
  for (;;) {
    const v = await f();
    if (v !== undefined) return v;
    if (Date.now() > end) throw new Error(`timed out waiting for ${what}`);
    await sleep(250);
  }
}

type Send = (method: string, params?: object) => Promise<{ result?: any; error?: { message: string } }>;

/** A DevTools session with the workbench's page. */
async function workbench(): Promise<Send> {
  type Target = { type: string; url: string; webSocketDebuggerUrl: string };
  const targets = (await (await fetch(`http://127.0.0.1:${port}/json`)).json()) as Target[];
  const page = targets.find((t) => t.type === "page" && t.url.includes("workbench"));
  if (!page) throw new Error(`no workbench page among ${targets.map((t) => t.url).join(", ")}`);
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise<void>((resolve, reject) => ((ws.onopen = resolve), (ws.onerror = reject)));
  let id = 0;
  const pending = new Map<number, (r: any) => void>();
  ws.onmessage = (m) => {
    const d = JSON.parse(m.data);
    pending.get(d.id)?.(d);
    pending.delete(d.id);
  };
  return (method, params = {}) =>
    new Promise((r) => {
      pending.set(++id, r);
      ws.send(JSON.stringify({ id, method, params }));
    });
}

let send: Send;

/** The bounding box of the first element `selector` matches whose text has `text`. */
async function box(selector: string, text = ""): Promise<{ top: number; right: number; bottom: number }> {
  const expression = `(() => {
    const e = [...document.querySelectorAll(${JSON.stringify(selector)})].find((e) => e.textContent.includes(${JSON.stringify(text)}));
    const r = e && e.getBoundingClientRect();
    return r && { top: r.top, right: r.right, bottom: r.bottom };
  })()`;
  const r = await send("Runtime.evaluate", { expression, returnByValue: true });
  const v = r.result?.result?.value;
  if (!v) throw new Error(`no ${selector} ${text}`);
  return v;
}

/** The middle of the horizontal sash in `scope` nearest to `y`: where to drag a view's top edge. */
async function sashAt(scope: string, y: number): Promise<number> {
  const expression = `[...document.querySelectorAll("${scope} .monaco-sash.horizontal")]
    .map((e) => e.getBoundingClientRect()).filter((r) => r.height > 0).map((r) => r.top + r.height / 2)`;
  const r = await send("Runtime.evaluate", { expression, returnByValue: true });
  const middles: number[] = r.result?.result?.value ?? [];
  if (!middles.length) throw new Error(`no sash in ${scope}`);
  return middles.reduce((a, b) => (Math.abs(b - y) < Math.abs(a - y) ? b : a));
}

async function drag(x: number, y: number, x2: number, y2: number): Promise<void> {
  await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y });
  await sleep(300);
  await send("Input.dispatchMouseEvent", { type: "mousePressed", x, y, button: "left", buttons: 1, clickCount: 1 });
  for (let i = 1; i <= 10; i++) {
    await send("Input.dispatchMouseEvent", { type: "mouseMoved", x: x + ((x2 - x) * i) / 10, y: y + ((y2 - y) * i) / 10, button: "left", buttons: 1 });
  }
  await send("Input.dispatchMouseEvent", { type: "mouseReleased", x: x2, y: y2, button: "left", buttons: 0, clickCount: 1 });
  // Out of the way, where it hovers nothing.
  await send("Input.dispatchMouseEvent", { type: "mouseMoved", x: W / 2, y: H - 4 });
  await sleep(500);
}

async function shot(name: string): Promise<void> {
  await sleep(800);
  const title = (await box(".part.titlebar")).bottom;
  const r = await send("Page.captureScreenshot", { format: "png", clip: { x: 0, y: title, width: W, height: H - title, scale: 1 } });
  if (!r.result) throw new Error(`capturing ${name}: ${r.error?.message}`);
  fs.writeFileSync(path.join(out, `${name}.png`), Buffer.from(r.result.data, "base64"));
  console.log(`  ${name}.png`);
}

/** The set's file whose name has `part`. */
function file(part: string): Uri {
  const name = fs.readdirSync(data).find((f) => f.includes(part));
  if (!name) throw new Error(`no file with ${part} in ${data}`);
  return Uri.file(path.join(data, name));
}

/** `uri` in an editor, the cursor `after` characters past the first `text`, centred. */
async function cursorAt(uri: Uri, text: string, after = 0) {
  const editor = await window.showTextDocument(uri, { preview: false });
  const at = editor.document.positionAt(editor.document.getText().indexOf(text) + after);
  editor.selection = new Selection(at, at);
  editor.revealRange(new Range(at, at), TextEditorRevealType.InCenter);
  return editor;
}

type Hit = { id: string; class: string; place: string };

export async function run(): Promise<void> {
  send = await workbench();
  await send("Emulation.setDeviceMetricsOverride", { width: W, height: H, deviceScaleFactor: SCALE, mobile: false });
  const main = file(process.env.CIMOXIDE_SHOTS_MAIN as string);
  await window.showTextDocument(main);
  const api = (await extensions.getExtension("m-mirz.cimoxide")?.activate()) as Api;
  await until("the tree's sections", async () => ((await api.modelTree.getChildren()).length ? true : undefined));
  // The Explorer as the CIM Model view: the others hidden, it dragged to the top, the sidebar wider.
  for (const view of ["outline", "timeline"]) await commands.executeCommand(`${view}.toggleVisibility`);
  await commands.executeCommand("cimoxide.model.focus");
  await sleep(500);
  const sash = await sashAt(".part.sidebar", (await box(".pane-header", "CIM Model")).top);
  await drag(200, sash, 200, 100);
  const sidebar = (await box(".part.sidebar")).right;
  await drag(sidebar + 1, H / 2, 400, H / 2);
  // The first validation.
  await sleep(4000);

  if (take.includes("model-tree")) {
    await cursorAt(main, "<cim:IdentifiedObject.name>BE-G1<");
    await commands.executeCommand("cimoxide.model.reveal");
    await sleep(1500);
    await shot("model-tree");
  }
  if (take.includes("validation")) {
    // A finding: its squiggle, its message on hover, and the Problems panel listing the set's.
    await commands.executeCommand("workbench.action.closeSidebar");
    const [uri, diagnostics] = await until("a finding", () => {
      const all = languages.getDiagnostics().filter(([, d]) => d.length);
      return all.find(([, d]) => d.some((x) => x.severity === DiagnosticSeverity.Error)) ?? all[0];
    });
    const finding = diagnostics.find((x) => x.severity === DiagnosticSeverity.Error) ?? diagnostics[0];
    const editor = await window.showTextDocument(uri, { preview: false });
    editor.selection = new Selection(finding.range.start, finding.range.start);
    editor.revealRange(finding.range, TextEditorRevealType.InCenter);
    await commands.executeCommand("workbench.actions.view.problems");
    await window.showTextDocument(editor.document, editor.viewColumn);
    await commands.executeCommand("editor.action.showHover");
    await sleep(1500);
    await shot("validation");
    await commands.executeCommand("workbench.action.closePanel");
  }
  if (take.includes("hover")) {
    await commands.executeCommand("workbench.action.closeSidebar");
    await commands.executeCommand("workbench.action.closeAllEditors");
    await cursorAt(main, '<cim:SynchronousMachine rdf:ID="', "<cim:Sync".length);
    await commands.executeCommand("editor.action.showHover");
    await sleep(1500);
    await shot("hover");
  }
  if (take.includes("delete-preview")) {
    // Deleting a voltage level with what it holds, in every file, previewed; then discarded.
    await commands.executeCommand("workbench.action.closeSidebar");
    await commands.executeCommand("workbench.action.closeAllEditors");
    const hits = await api.request<Hit[]>("cimoxide/search", { uri: main.toString(), query: "10.5" });
    const level = hits.find((h) => h.class === "VoltageLevel" && h.place === "PP_Brussels");
    if (!level) throw new Error("no 10.5 kV level in PP_Brussels");
    // Settles only once the preview is applied or discarded: not awaited.
    void commands.executeCommand("cimoxide.model.deleteAll", await api.modelTree.locateId(level.id));
    await until("the preview", async () => ((await box(".part.panel")).bottom > 0 ? true : undefined));
    await sleep(2000);
    // The panel taller, and the first file's first change open in a diff above it.
    await drag(W / 2, await sashAt("", (await box(".part.panel")).top), W / 2, H * 0.45);
    await commands.executeCommand("refactorPreview.focus");
    for (const c of ["list.focusFirst", "list.expand", "list.focusDown", "list.select"]) {
      await commands.executeCommand(c);
      await sleep(300);
    }
    await sleep(1500);
    await shot("delete-preview");
    await commands.executeCommand("refactorPreview.discard");
    await commands.executeCommand("workbench.action.closePanel");
  }
  if (take.includes("grid-realgrid")) {
    await commands.executeCommand("workbench.action.closeSidebar");
    await commands.executeCommand("workbench.action.closeAllEditors");
    await commands.executeCommand("cimoxide.showGrid", main);
    await sleep(6000);
    await shot("grid-realgrid");
  }
  if (take.includes("grid-substation")) {
    await commands.executeCommand("workbench.action.closeSidebar");
    await commands.executeCommand("workbench.action.closeAllEditors");
    const hits = await api.request<Hit[]>("cimoxide/search", { uri: main.toString(), query: "PP_Brussels" });
    const substation = hits.find((h) => h.class === "Substation");
    if (!substation) throw new Error("no PP_Brussels");
    await commands.executeCommand("cimoxide.model.showInside", await api.modelTree.locateId(substation.id));
    await sleep(5000);
    await shot("grid-substation");
  }
  if (take.includes("quick-fix")) {
    // A reference to nothing, which the quick fixes complete with the likeliest objects.
    await commands.executeCommand("workbench.action.closeSidebar");
    await commands.executeCommand("workbench.action.closeAllEditors");
    const doc = await workspace.openTextDocument(main);
    const field = '<cim:Terminal.ConnectivityNode rdf:resource="#';
    const start = doc.getText().indexOf(field) + field.length;
    const blank = new WorkspaceEdit();
    blank.delete(main, new Range(doc.positionAt(start), doc.positionAt(doc.getText().indexOf('"', start))));
    await workspace.applyEdit(blank);
    await doc.save();
    await sleep(4000);
    await cursorAt(main, field, field.length);
    await commands.executeCommand("editor.action.quickFix");
    await sleep(2500);
    await shot("quick-fix");
  }
}
