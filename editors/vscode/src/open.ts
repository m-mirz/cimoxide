// Opening a place in a file. Through the `vscode.open` command, not
// `workspace.openTextDocument`: VS Code hands no document over 50 MB to an
// extension (RealGrid's EQ file is 80 MB), but opens it in an editor.

import { commands, Range, TabInputText, Uri, ViewColumn, window } from "vscode";
import { LanguageClient } from "vscode-languageclient/node";

/** The language CGMES and NC files are claimed as (`claim.ts`), highlighted with XML's grammar. */
export const CIM_XML = "cimxml";

/** Whether a document of `languageId` may be a CGMES or NC file: XML, or claimed as CIM XML. */
export function isXmlLanguage(languageId: string): boolean {
  return languageId === "xml" || languageId === CIM_XML;
}

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

/**
 * The XML file of the active editor tab: its text editor's, else the tab's
 * own, as VS Code gives an extension no text editor of a file over 50 MB.
 */
export function activeFile(): Uri | undefined {
  const editor = window.activeTextEditor;
  const uri = editor ? editor.document.uri : (() => {
    const input = window.tabGroups.activeTabGroup.activeTab?.input;
    return input instanceof TabInputText ? input.uri : undefined;
  })();
  return uri?.scheme === "file" && uri.path.toLowerCase().endsWith(".xml") ? uri : undefined;
}

/** Ask the server to validate `uri`'s model set, though it may not have the file open. */
export async function validateSet(client: LanguageClient, uri: Uri): Promise<void> {
  await client.sendRequest("workspace/executeCommand", { command: "cimoxide.revalidate", arguments: [uri.toString()] });
}

/**
 * Open the files an edit changed, each at its first change, in tabs behind
 * the active one: an edit to a file no editor shows leaves it unsaved and
 * unseen (and VS Code's Save All passes over one it never handed an
 * extension, over 50 MB).
 */
export async function openEdited(uris: Uri[], at?: (uri: Uri) => Range | undefined): Promise<void> {
  for (const uri of uris) {
    await commands.executeCommand("vscode.open", uri, { preview: false, preserveFocus: true, selection: at?.(uri) });
  }
}
