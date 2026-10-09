# cimoxide for VS Code

Language support for ENTSO-E **CGMES** and **Network Code (NC)** RDF/XML, powered by
[cimoxide](https://github.com/m-mirz/cimoxide).

- **Validation** — the CGMES and NC SHACL rules (shape tables plus the hand-written
  `sh:sparql` rules) as diagnostics, the same findings as `cimcli validate`. A model is
  usually several files (EQ, SSH, SV, TP, …), so the unit of validation is a *model set*:
  every CIM XML file in the document's directory. Cross-profile findings appear on every
  file that writes the object.
- **Hover** — a class's inheritance chain and profiles, an attribute's type, multiplicity
  and profiles, and the class and name of the object an `rdf:resource` points to.
- **Go to definition / find references** — follow `rdf:resource="#_…"` across the files
  of the model set.
- **Outline** — every object in the document, labelled by `IdentifiedObject.name`.
- **Completion** — after `<`: the attributes of the enclosing object's class (inherited ones
  included), or the classes of the document's namespace between objects.

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

The language server, `cimlsp`, is bundled for Linux (x64, arm64), macOS (x64, arm64) and
Windows (x64). It speaks LSP over stdio, so other editors can use it too.
