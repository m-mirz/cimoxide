// Applying the server's workspace edits. VS Code passes no document over 50 MB to extensions, and an
// edit it is asked to make to one is not reliably written: so an edit to such a file is made on disk
// here, and the server, which reads that file from disk, sees it at once. Every other file takes
// the edit as VS Code makes it, unsaved and undoable, as its refactorings are.

import * as fs from "fs";
import { Uri, window, workspace } from "vscode";
import { LanguageClient, TextEdit as LspTextEdit, WorkspaceEdit as LspWorkspaceEdit } from "vscode-languageclient/node";

/** VS Code's limit for passing a document to extensions. */
export const SYNC_LIMIT = 50 * 1024 * 1024;

/** Whether `uri` names a file on disk over [`SYNC_LIMIT`]. */
export function overLimit(uri: string): boolean {
  try {
    return fs.statSync(Uri.parse(uri).fsPath).size > SYNC_LIMIT;
  } catch {
    return false;
  }
}

/** `text` with `edits` made: positions in lines and UTF-16 code units, as LSP's. */
export function edited(text: string, edits: LspTextEdit[]): string {
  const starts = [0];
  for (let i = text.indexOf("\n"); i >= 0; i = text.indexOf("\n", i + 1)) starts.push(i + 1);
  const offset = (p: { line: number; character: number }) =>
    Math.min(text.length, (p.line < starts.length ? starts[p.line] : text.length) + p.character);
  const spans = edits
    .map((e) => ({ from: offset(e.range.start), to: offset(e.range.end), text: e.newText }))
    .sort((a, b) => a.from - b.from || a.to - b.to);
  const parts: string[] = [];
  let at = 0;
  for (const s of spans) {
    parts.push(text.slice(at, s.from), s.text);
    at = Math.max(at, s.to);
  }
  parts.push(text.slice(at));
  return parts.join("");
}

/**
 * Makes the server's `edit`: on disk for each file over 50 MB, through VS Code for the rest. The
 * files it changed, or `undefined` if VS Code did not make its part.
 */
export async function applyEdit(client: LanguageClient, edit: LspWorkspaceEdit, refactoring = false): Promise<Uri[] | undefined> {
  const onDisk = new Map<string, LspTextEdit[]>();
  const take = (uri: string, edits: LspTextEdit[]) => onDisk.set(uri, [...(onDisk.get(uri) ?? []), ...edits]);
  const rest: LspWorkspaceEdit = {};
  if (edit.changes) {
    for (const [uri, edits] of Object.entries(edit.changes)) {
      if (overLimit(uri)) take(uri, edits);
      else (rest.changes ??= {})[uri] = edits;
    }
  }
  if (edit.documentChanges) {
    for (const change of edit.documentChanges) {
      if ("textDocument" in change && overLimit(change.textDocument.uri)) take(change.textDocument.uri, change.edits as LspTextEdit[]);
      else (rest.documentChanges ??= []).push(change);
    }
  }
  const changed: Uri[] = [];
  for (const [uri, edits] of onDisk) {
    const path = Uri.parse(uri).fsPath;
    fs.writeFileSync(path, edited(fs.readFileSync(path, "utf8"), edits));
    changed.push(Uri.parse(uri));
  }
  if (onDisk.size) {
    window.setStatusBarMessage(`cimoxide: written straight to disk (over 50 MB, so VS Code cannot undo it): ${[...onDisk.keys()].map((u) => Uri.parse(u).path.split("/").pop()).join(", ")}`, 8000);
  }
  if (rest.changes || rest.documentChanges) {
    const converted = await client.protocol2CodeConverter.asWorkspaceEdit(rest);
    if (!converted || !(await workspace.applyEdit(converted, { isRefactoring: refactoring }))) return onDisk.size ? changed : undefined;
    changed.push(...converted.entries().map(([u]) => u));
  }
  return changed;
}
