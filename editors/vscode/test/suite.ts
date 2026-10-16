// Smoke tests in a running VS Code (test/run.js): the extension as a user
// meets it — the server's requests through the client, the model tree as the
// view asks for it, code actions and rename through VS Code's own commands,
// the grid panel. What the server's own tests cannot see: how the client
// sends a request, where VS Code loads an icon from, what a webview gets.

import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import {
  CodeAction,
  commands,
  Diagnostic,
  extensions,
  languages,
  Position,
  Range,
  TabInputWebview,
  TextDocument,
  Uri,
  window,
  workspace,
  WorkspaceEdit,
} from "vscode";
import type { Api } from "../src/extension";
import type { Entry } from "../src/modelTree";

const data = process.env.CIMOXIDE_TEST_DATA as string;

/** The set's file whose name has `part`. */
function file(part: string): Uri {
  const name = fs.readdirSync(data).find((f) => f.includes(part));
  assert.ok(name, `no file with ${part} in ${data}`);
  return Uri.file(path.join(data, name));
}

/** `f`'s first defined answer, asked every 250 ms for at most `ms`. */
async function until<T>(what: string, f: () => Promise<T | undefined> | T | undefined, ms = 60000): Promise<T> {
  const end = Date.now() + ms;
  for (;;) {
    const v = await f();
    if (v !== undefined) return v;
    if (Date.now() > end) throw new Error(`timed out waiting for ${what}`);
    await new Promise((r) => setTimeout(r, 250));
  }
}

/** The position of the start tag of the element before the line holding `text`, on its `rdf:ID` value. */
function idBefore(doc: TextDocument, text: string): Position {
  const at = doc.getText().indexOf(text);
  assert.ok(at >= 0, `no ${text}`);
  for (let line = doc.positionAt(at).line; line >= 0; line--) {
    const l = doc.lineAt(line).text;
    const id = l.indexOf('rdf:ID="');
    if (id >= 0) return new Position(line, id + 'rdf:ID="'.length + 1);
  }
  throw new Error(`no rdf:ID before ${text}`);
}

const tests: [string, () => Promise<void>][] = [];
const test = (name: string, f: () => Promise<void>) => tests.push([name, f]);

let api: Api;
let eq: Uri;

test("activates, validates the set and lists it in the tree", async () => {
  eq = file("_BE_EQ_");
  await window.showTextDocument(eq);
  api = (await extensions.getExtension("m-mirz.cimoxide")?.activate()) as Api;
  const sections = await until("the tree's sections", async () => {
    const s = await api.modelTree.getChildren();
    return s.length ? s : undefined;
  });
  assert.deepStrictEqual(sections.map((s) => s.label), ["Geographical", "Operational", "Not contained", "By class"]);
});

test("the tree's equipment icons are files in the extension", async () => {
  const byClass = (await api.modelTree.getChildren()).find((s) => s.label === "By class") as Entry;
  const lines = (await api.modelTree.getChildren(byClass)).find((g) => g.label === "ACLineSegment");
  assert.ok(lines, "no ACLineSegment group");
  const icon = api.modelTree.getTreeItem(lines).iconPath as { light: Uri; dark: Uri };
  const root = extensions.getExtension("m-mirz.cimoxide")?.extensionPath as string;
  for (const u of [icon.light, icon.dark]) {
    assert.ok(u.fsPath.startsWith(root), `${u.fsPath} is not in the extension, where VS Code loads icons from`);
    assert.ok(fs.existsSync(u.fsPath), `${u.fsPath} missing`);
  }
});

test("an object's code actions: add in it, delete it (resolved: every file)", async () => {
  const doc = await workspace.openTextDocument(eq);
  const at = idBefore(doc, "<cim:IdentifiedObject.name>110.0<");
  const actions = await commands.executeCommand<CodeAction[]>("vscode.executeCodeActionProvider", eq, new Range(at, at), undefined, 10);
  const titles = actions.map((a) => a.title);
  assert.ok(titles.includes("Add… in VoltageLevel 110.0"), titles.join(" | "));
  const del = actions.find((a) => a.title.startsWith("Delete VoltageLevel 110.0 and"));
  assert.ok(del?.edit, titles.join(" | "));
  assert.ok(del.edit.entries().length >= 3, `the deletion reaches ${del.edit.entries().length} file(s)`);
  // And then opens what it changed, for review.
  assert.strictEqual(del.command?.command, "cimoxide.openEdited");
  assert.strictEqual((del.command.arguments?.[0] as string[]).length, del.edit.entries().length);
});

test("a file changed on disk, unopened, is revalidated", async () => {
  const nl = file("_NL_EQ_");
  assert.ok(!workspace.textDocuments.some((d) => d.uri.toString() === nl.toString()), "the NL file is open");
  const text = fs.readFileSync(nl.fsPath, "utf8");
  const at = text.indexOf("<cim:ACLineSegment.r>");
  const start = text.lastIndexOf("\n", at);
  fs.writeFileSync(nl.fsPath, text.slice(0, start) + text.slice(text.indexOf("\n", at)));
  const code = (d: Diagnostic) => (typeof d.code === "object" ? d.code.value : d.code);
  await until("the finding on the file changed on disk", () =>
    languages.getDiagnostics(nl).find((d) => code(d) === "eq:ACLineSegment.r-cardinality"));
});

test("a quick fix adds a missing field and selects its value", async () => {
  const doc = await workspace.openTextDocument(eq);
  const r = doc.getText().indexOf("<cim:ACLineSegment.r>");
  const line = doc.positionAt(r).line;
  const cut = new WorkspaceEdit();
  cut.delete(eq, new Range(line, 0, line + 1, 0));
  assert.ok(await workspace.applyEdit(cut));
  await doc.save();
  const code = (d: Diagnostic) => (typeof d.code === "object" ? d.code.value : d.code);
  const finding = await until("the missing field's finding", () =>
    languages.getDiagnostics(eq).find((d) => code(d) === "eq:ACLineSegment.r-cardinality"));
  const actions = await commands.executeCommand<CodeAction[]>("vscode.executeCodeActionProvider", eq, finding.range);
  const fix = actions.find((a) => a.title === "Add ACLineSegment.r");
  assert.ok(fix?.edit && fix.command, actions.map((a) => a.title).join(" | "));
  assert.ok(await workspace.applyEdit(fix.edit));
  assert.ok(doc.getText().includes("<cim:ACLineSegment.r>0</cim:ACLineSegment.r>"));
  await commands.executeCommand(fix.command.command, ...(fix.command.arguments ?? []));
  const editor = window.activeTextEditor;
  assert.strictEqual(editor && editor.document.getText(editor.selection), "0");
});

test("rename changes an mRID in every file that names it", async () => {
  const doc = await workspace.openTextDocument(eq);
  const at = idBefore(doc, "<cim:IdentifiedObject.name>BE-Line_1<");
  const edit = await commands.executeCommand<WorkspaceEdit>("vscode.executeDocumentRenameProvider", eq, at, "_renamed-line");
  const files = edit.entries().map(([u]) => path.basename(u.fsPath));
  assert.ok(files.length >= 4, files.join(", "));
});

test("Show grid opens its panel", async () => {
  await commands.executeCommand("cimoxide.showGrid", eq);
  await until("the grid panel", () =>
    window.tabGroups.all.flatMap((g) => g.tabs).find((t) => t.input instanceof TabInputWebview && t.label.startsWith("Grid:")));
});

export async function run(): Promise<void> {
  let failed = 0;
  for (const [name, f] of tests) {
    try {
      await f();
      console.log(`  ✓ ${name}`);
    } catch (e) {
      failed++;
      console.error(`  ✗ ${name}\n    ${e instanceof Error ? e.stack ?? e.message : e}`);
    }
  }
  if (failed) throw new Error(`${failed} of ${tests.length} test(s) failed`);
}
