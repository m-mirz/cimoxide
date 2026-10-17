// CGMES and NC files claimed as CIM XML (`cimoxide.claimFiles`): they are
// `.xml`, which VS Code opens as XML for every extension of XML in general
// (schema validation, formatting, …). A language of their own keeps those
// off; its grammar includes XML's, so they are highlighted as before. No file
// pattern tells them apart, so each XML document is looked at as it opens and
// switched: it opens as XML first, briefly.

import { ExtensionContext, languages, Range, TextDocument, workspace } from "vscode";
import { CIM_XML } from "./open";

/** As `cimmodel::decode::is_cim`: near the top, a CIM namespace, an NC profile or ENTSO-E reference data. */
const MARKERS = ["http://iec.ch/TC57/", "https://cim.ucaiug.io/ns#", "https://ap.cim4.eu/", "https://energy.referencedata.eu/"];
const HEAD = 8192;

export function registerClaim(context: ExtensionContext): void {
  // Switched this session, by URI: one set back to XML by hand stays XML
  // until it is closed (switching closes it as XML and opens it as CIM XML).
  const switched = new Set<string>();
  const consider = (doc: TextDocument): void => {
    if (doc.languageId !== "xml" || doc.uri.scheme !== "file") return;
    if (!workspace.getConfiguration("cimoxide").get<boolean>("claimFiles", true)) return;
    const key = doc.uri.toString();
    if (switched.has(key)) return;
    const head = doc.getText(new Range(doc.positionAt(0), doc.positionAt(HEAD)));
    if (!MARKERS.some((m) => head.includes(m))) return;
    switched.add(key);
    void languages.setTextDocumentLanguage(doc, CIM_XML);
  };
  workspace.textDocuments.forEach(consider);
  context.subscriptions.push(
    workspace.onDidOpenTextDocument(consider),
    workspace.onDidCloseTextDocument((doc) => {
      if (doc.languageId === "xml") switched.delete(doc.uri.toString());
    }),
  );
}
