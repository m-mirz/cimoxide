// Opening a place in a file. Through the `vscode.open` command, not
// `workspace.openTextDocument`: VS Code hands no document over 50 MB to an
// extension (RealGrid's EQ file is 80 MB), but opens it in an editor.

import { commands, Range, Uri, ViewColumn } from "vscode";

/** A range as the server sends it. */
export interface Span {
  start: { line: number; character: number };
  end: { line: number; character: number };
}

/** Open `uri` in the first editor column with `span` selected. */
export async function openAt(uri: string, span: Span, preserveFocus = false): Promise<void> {
  const selection = new Range(span.start.line, span.start.character, span.end.line, span.end.character);
  await commands.executeCommand("vscode.open", Uri.parse(uri), { viewColumn: ViewColumn.One, selection, preserveFocus });
}
