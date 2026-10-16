# cimoxide for VS Code

Language support for ENTSO-E **CGMES** and **Network Code (NC)** RDF/XML, powered by
[cimoxide](https://github.com/m-mirz/cimoxide).

- **Validation** — the CGMES and NC SHACL rules (shape tables plus the hand-written
  `sh:sparql` rules) as diagnostics, the same findings as `cimcli validate`. A model is
  usually several files (EQ, SSH, SV, TP, …), so the unit of validation is a *model set*:
  every CIM XML file in the document's directory. Cross-profile findings appear on every
  file that writes the object.
- **Hover** — a class's definition from the ENTSO-E vocabulary, inheritance chain and
  profiles, an attribute's definition, type, multiplicity and profiles, an enumeration
  value's definition, and the class and name of the object an `rdf:resource` points to.
- **Go to definition / find references** — follow `rdf:resource="#_…"` across the files
  of the model set.
- **Outline** — every object in the document, labelled by `IdentifiedObject.name`, with its
  class and containers (`Breaker · PP_Brussels › 110.0 › Bay 3`).
- **CIM Model** (in the Explorer) — the active document's model set as a tree: *Geographical*,
  regions down through substations, voltage levels, bays and equipment to terminals, limits and
  state variables; *Operational*, control and load areas; *Not contained*, what no container
  holds (base voltages, curves, …), by class; *By class*, every object. A region whose file is
  missing (an IGM without its boundary set) is listed by its mRID. Each node shows the number of
  findings below it, in the colour of the worst; the filter button keeps only those with findings.
  Click to open where an object is written; *Show in grid* selects it in the grid view, inside its
  substation where it has one. *Reveal in CIM Model* (editor context menu) selects the object at the
  cursor; `cimoxide.model.followCursor` does so as the cursor moves.
- **Rename and delete** — *Rename Symbol* (F2) on an `rdf:ID`, `rdf:about` or `rdf:resource`
  renames the mRID in every file of the model set: definitions, descriptions, references and an
  `IdentifiedObject.mRID` that writes it. *Refactor…* on an object (or the *CIM Model* view's
  context menu) deletes it from every file with the references to it, or with everything it holds
  (a voltage level's bays, equipment, terminals, …); the edit opens in the refactoring preview
  first. Both change the files as written and nothing else, and are undone like any edit.
- **Add an object** — *Add…* on an object in the *CIM Model* view, or under *Refactor…* on its
  element: pick a class among those it can hold (equipment, bays and connectivity nodes in a voltage
  level; terminals, ends, limit sets on equipment, …) and a name. The object is written with a new
  UUID, its mRID, name and container, in the file whose profile carries it, after what the
  container already holds, with an empty element for its SSH part; its name is selected. What else
  it needs shows as findings, which the quick fixes below complete.
- **Quick fixes** — on a finding that a field is missing (*Quick Fix…*, Ctrl+.): add it, to the
  file whose profile carries it (an SSH attribute to the SSH file, wherever the finding shows), with
  a value to type over, which is selected: `false`, `0`, an enumeration's first value or the object's
  own mRID. A missing reference is offered with the five likeliest objects to name (*Add
  Terminal.ConnectivityNode: ConnectivityNode BUS_1 in PP_Brussels › 110.0*), or empty with the
  objects listed to choose from; so is a reference to nothing (`rdf:resource="#"`). With several
  missing on one object, one fix adds them all.
- **Completion** in an `rdf:resource` value: the objects of the set the reference can name, the
  likeliest first (linked to the object through its other references, then nearest in its
  containers), or an enumeration's values.
- **Completion** — after `<`: the attributes of the enclosing object's class (inherited ones
  included), or the classes of the document's namespace between objects.
- **Grid view** — *cimoxide: Show grid* (or the button in the editor title) draws the model
  set as substations, boundary points and the lines between them: as the set's Diagram Layout
  draws them, on its Geographical Locations, or laid out automatically. Lines, substations and voltage
  levels are coloured by voltage (legend in the side panel); a ring and halo mark each one with
  findings, in the colour of the worst and as wide as their number; select one for its voltage
  levels, equipment and findings, double-click a line to open where it is written. Double-click
  a substation to see inside it: its equipment, drawn with single-line symbols (after IEC 60617,
  as the CIM Model view's icons are), and connectivity nodes by voltage level, its switches drawn
  open or closed, out-of-service equipment faded, each with its findings, on
  the Diagram Layout or laid out automatically; double-click a line there to go to its other end,
  Escape to go back. The view follows each revalidation.

- **Chat** — an MCP server, `cimmcp`, offered to chat (Copilot agent mode): ask about the
  model sets of the workspace. Its tools list the model sets, summarise one (headers, object
  counts), find objects by class, name or attribute value, show an object with its references
  resolved, validate, run SPARQL and describe a CIM class. They only read. Turn it off with
  `cimoxide.mcp.enabled`.

Validation runs when a document is opened and saved; turn on
`cimoxide.validation.onType` to also run it while typing.

## Settings

| Setting | |
|---|---|
| `cimoxide.validation.common` | common cross-profile checks (`--common`) |
| `cimoxide.validation.quality` | modelling-quality checks (`--quality`) |
| `cimoxide.validation.silence` | rule ids not to report |
| `cimoxide.validation.onType` | revalidate while typing |
| `cimoxide.schema.rdfsDir` / `cimoxide.schema.shaclDir` | load RDFS / SHACL at runtime instead of the built-in tables |
| `cimoxide.server.path` | use another `cimlsp` binary |
| `cimoxide.model.followCursor` | select the object at the cursor in the *CIM Model* view |
| `cimoxide.mcp.enabled` | offer the `cimmcp` MCP server to chat |
| `cimoxide.mcp.path` | use another `cimmcp` binary |

The language server, `cimlsp`, and the MCP server, `cimmcp`, are bundled for Linux (x64),
macOS (Apple silicon) and Windows (x64). They speak LSP and MCP over stdio, so other
editors and chat clients can use them too — e.g. `claude mcp add cimoxide -- cimmcp --dir .`
for Claude Code. The extension needs VS Code 1.101 or later.
