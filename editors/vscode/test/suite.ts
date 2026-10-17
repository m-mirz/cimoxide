// Smoke tests in a running VS Code (test/run.js): the extension as a user
// meets it — the server's requests through the client, the model tree as the
// view asks for it, code actions and rename through VS Code's own commands,
// the grid panel. What the server's own tests cannot see: how the client
// sends a request, where VS Code loads an icon from, what a webview gets.

import * as assert from "assert";
import * as fs from "fs";
import * as os from "os";
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

test("a CIM file opens as CIM XML, highlighted as XML; another XML file stays XML", async () => {
  await until("the EQ file as CIM XML", () =>
    workspace.textDocuments.find((d) => d.uri.toString() === eq.toString() && d.languageId === "cimxml"));
  const grammar = (extensions.getExtension("m-mirz.cimoxide")?.packageJSON.contributes.grammars ?? [])
    .find((g: { language: string }) => g.language === "cimxml");
  const root = extensions.getExtension("m-mirz.cimoxide")?.extensionPath as string;
  const rules = JSON.parse(fs.readFileSync(path.join(root, grammar.path), "utf8"));
  assert.deepStrictEqual(rules.patterns, [{ include: "text.xml" }]);
  const plain = path.join(fs.mkdtempSync(path.join(os.tmpdir(), "cimoxide-plain-")), "plain.xml");
  fs.writeFileSync(plain, '<?xml version="1.0"?>\n<project><name>not CIM</name></project>\n');
  const doc = await workspace.openTextDocument(Uri.file(plain));
  await new Promise((r) => setTimeout(r, 1000));
  assert.strictEqual(doc.languageId, "xml");
  assert.ok(!doc.isClosed, "the plain file was switched");
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

type Hit = { id: string; label: string; class: string; place: string };

test("deleting is the tree's: it reaches every file; the editor offers none", async () => {
  const doc = await workspace.openTextDocument(eq);
  const at = idBefore(doc, "<cim:IdentifiedObject.name>110.0<");
  const actions = await commands.executeCommand<CodeAction[]>("vscode.executeCodeActionProvider", eq, new Range(at, at));
  assert.ok(!actions.some((a) => /^(Delete|Add…)/.test(a.title)), actions.map((a) => a.title).join(" | "));
  const vl = (await api.request<Hit[]>("cimoxide/search", { uri: eq.toString(), query: "110.0" }))
    .find((h) => h.class === "VoltageLevel" && h.place === "PP_Brussels");
  assert.ok(vl, "no 110 kV level in PP_Brussels");
  const del = await api.request<{ edit: { documentChanges: unknown[] } }>("cimoxide/delete", { uri: eq.toString(), id: vl.id, dependents: true });
  assert.ok(del.edit.documentChanges.length >= 3, `the deletion reaches ${del.edit.documentChanges.length} file(s)`);
});

test("Referenced by lists what refers to an object", async () => {
  const tr = (await api.request<Hit[]>("cimoxide/search", { uri: eq.toString(), query: "BE-TR2_1" })).find((h) => h.class === "PowerTransformer");
  assert.ok(tr);
  const refs = await api.request<(Hit & { key: string })[]>("cimoxide/referrers", { uri: eq.toString(), id: tr.id });
  const classes = new Set(refs.map((r) => r.class));
  assert.ok(classes.has("Terminal") && classes.has("PowerTransformerEnd"), refs.map((r) => `${r.class} ${r.key}`).join(", "));
});

test("a reference to nothing gets the likeliest objects to name, from the tree and in the editor", async () => {
  const doc = await workspace.openTextDocument(eq);
  const text = doc.getText();
  const field = '<cim:Terminal.ConnectivityNode rdf:resource="#';
  const start = text.indexOf(field) + field.length;
  const end = text.indexOf('"', start);
  const at = doc.positionAt(start);
  const terminal = idBefore(doc, text.slice(text.indexOf(field), text.indexOf(field) + 10));
  const id = doc.lineAt(terminal.line).text.match(/rdf:ID="([^"]+)"/)?.[1];
  assert.ok(id);
  const blank = new WorkspaceEdit();
  blank.delete(eq, new Range(at, doc.positionAt(end)));
  assert.ok(await workspace.applyEdit(blank));
  await doc.save();
  const fixes = await until("the tree's fixes for the emptied reference", async () => {
    const f = await api.request<{ title: string }[]>("cimoxide/fixes", { uri: eq.toString(), id });
    return f.some((x) => x.title.startsWith("Set Terminal.ConnectivityNode to ConnectivityNode")) ? f : undefined;
  });
  assert.ok(fixes.length >= 1);
  const actions = await commands.executeCommand<CodeAction[]>("vscode.executeCodeActionProvider", eq, new Range(at, at));
  assert.ok(actions.some((a) => a.title.startsWith("Set Terminal.ConnectivityNode to ConnectivityNode")), actions.map((a) => a.title).join(" | "));
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

test("the tree's fixes and rename come from the server, for files of any size", async () => {
  // A line of the NL file, changed on disk earlier, lacks its r: its fixes.
  const nl = file("_NL_EQ_");
  const code = (d: Diagnostic) => (typeof d.code === "object" ? d.code.value : d.code);
  const finding = languages.getDiagnostics(nl).find((d) => code(d) === "eq:ACLineSegment.r-cardinality");
  // The object, as the message names it: VS Code keeps a diagnostic's data from extensions.
  const id = (finding?.message.match(/\(ACLineSegment (\S+)\)/) ?? [])[1];
  assert.ok(id, "no finding on the NL file's line");
  const fixes = await api.request<{ title: string }[]>("cimoxide/fixes", { uri: nl.toString(), id });
  assert.ok(fixes.some((f) => f.title === "Add ACLineSegment.r"), fixes.map((f) => f.title).join(" | "));
  const renamed = await api.request<{ changes: Record<string, unknown[]> }>("cimoxide/rename", { uri: nl.toString(), id, newName: "_renamed" });
  assert.ok(Object.keys(renamed.changes).length >= 2, Object.keys(renamed.changes).join(", "));
});

test("search finds any object of the set, and the tree reaches it", async () => {
  const hits = await api.request<{ id: string; label: string; class: string; place: string }[]>(
    "cimoxide/search", { uri: eq.toString(), query: "be-tr2" });
  assert.ok(hits.length >= 3 && hits.every((h) => h.label.toLowerCase().includes("be-tr2")), hits.map((h) => h.label).join(", "));
  const transformer = hits.find((h) => h.class === "PowerTransformer");
  assert.strictEqual(transformer?.place, "PP_Brussels");
  const at = await api.modelTree.locateId(transformer.id);
  assert.strictEqual(at?.key, `c/${transformer.id}`);
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
