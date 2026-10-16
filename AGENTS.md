# AGENTS.md

Guidance for AI coding agents (Claude Code, Codex, Cursor and others) working in this repository.

## What This Project Does

cimoxide is a Rust workspace for ENTSO-E CGMES and NC (Network Codes) data. It reads the
IEC 61970/61968 RDFS and SHACL schemas, generates class tables that RDF/XML decodes against
and shape tables that one interpreter validates against, and converts and queries the data.

## Common Commands

```bash
git submodule update --init --recursive   # tests and codegen need the submodules
cargo build
cargo test                                # all crates
cargo test -p cimoxide-gen --test codegen cimmodel_codegen_stable   # one test
cargo bench -p cimoxide-model
cargo run -p cimoxide-gen                 # regenerate cimmodel and cimvalidation
```

Submodules: `application-profiles-library/` (ENTSO-E RDFS and SHACL) and
`CGMES-Test-Configurations/` (real test datasets). `relicapgrid/` (ENTSO-E's ReliCapGrid,
for `scripts/relicapgrid`'s harness) is `update = none`: fetch it with
`git submodule update --init --checkout relicapgrid`.

## Crates

Data flow: RDFS/SHACL → `cimgen` → `cimmodel` (class tables) + `cimvalidation` (shape tables).

| Directory / `use` | crates.io package | Role |
|---|---|---|
| `cimgen/` | `cimoxide-gen` (binary `cimgen`) | code generator |
| `cimschema/` | `cimoxide-schema` | RDFS and SHACL parser, shared by `cimgen` and the runtime loaders |
| `cimmodel/` | `cimoxide-model` | class tables, decoder, RDF/XML and JSON conversion |
| `cimvalidation/` | `cimoxide-validation` | shape-table interpreter plus hand-written `sh:sparql` rules |
| `cimsparql/` | `cimoxide-sparql` | SPARQL 1.1 over an in-memory oxigraph store |
| `cimoxide/` | `cimoxide` | facade re-exporting the above |
| `cimoxide-cli/` | `cimoxide-cli` (binary `cimcli`) | CLI |
| `cimoxide-lsp/` | `cimoxide-lsp` (binary `cimlsp`) | language server; VS Code extension in `editors/vscode/` |
| `cimoxide-mcp/` | `cimoxide-mcp` (binary `cimmcp`) | MCP server for chat clients; bundled and registered by the extension |
| `cimoxide-py/` | — | PyO3 bindings, outside the workspace |

`cargo -p` takes the package name. Each crate sets an explicit `[lib] name`.

### Generated files — do not hand-edit

- `cimmodel/src/generated/` (`cgmes_classes.rs`, `nc_classes.rs`, `mod.rs`). `.gitignore` and
  `make clean` cover that directory alone; new hand-written modules go directly in `src/`.
- `cimvalidation/src/{cgmes_shapes,cgmes_profiles,nc_shapes,nc_profiles}.rs`.

Everything else (`base.rs`, `registry.rs`, `decode.rs`, `convert.rs`, `schema_source.rs`,
`bag.rs`, `shapes.rs`, `sparql/`, `detect.rs`, …) is hand-written.

### `cimmodel`

**There are no generated types.** Every element of every family is an `Element`: its class
(`&'static ClassDef`), its mRID, its fields as written (`Class.attr` → `FieldValue`, values
unparsed) and which fields were repeated. Undeclared attributes and unparseable values are
kept, not dropped.

Each family's generated file holds one `SCHEMA: Schema` — classes (`ClassDef`/`AttrDef`),
profiles (code → URI) and namespace bindings. That is everything the crates know about a
family: the decoder, validation, export and SPARQL all read it through
`registry::type_registry()` (`schema(family)`, `attr_of`, `family_attr`). `AttrDef` carries
`used` (an association end some profile exchanges), `xsd` and `value_ns`; classes and
attributes carry the vocabulary's `rdfs:comment` as `comment`, and `Schema::enums` lists each
enumeration with its values and theirs (`Schema::enum_value("WindingConnection.D")`) (first non-empty profile in
file-name order wins; `cimlsp` hover and `cimmcp describe_class` show it); both
`classes_gen.rs` and the runtime loader derive them through `cimschema::table`.

- `base.rs` — `Element`, `FieldValue`, `FastMap`/`FieldMap`, `ClassDef`/`AttrDef`,
  `TypeRegistry` (`(namespace, local name)` → class, bare-name fallback for CGMES, inherited
  attributes via `attr(class, id)`, `chain(class)`)
- `decode.rs` — streaming parser → `CimDataset { entries, by_type }`; `decode_file`,
  `decode_files`, `decode_str`. `merge`: a later scalar wins, reference lists join, duplicate
  tracking stays per file. An object typed differently by different files keeps every type
  (`Element::types`); its class and `by_type` bucket are the most specific one.
  `CimDataset::type_conflicts` lists objects whose types are unrelated (neither an ancestor of the
  other).
- `convert.rs` — RDF/XML export (`dataset_to_xml`, `dataset_to_xml_for_profile`) and JSON
  (`{mrid: {"_type", "id", "Class.attr": "value" | [...], "_lang"?: {key: [tags]}}}`).
  JSON cannot say whether a string is a reference; `CimDataset::element_from_json` (used by
  cimoxide-py's `__setitem__`) holds a field as the dataset holds it on objects of the same
  class, else as the class table declares it. `convert_nc.rs` — one NC
  profile's export (DCAT header, elements, fields), which `dataset_to_xml_for_profile`
  dispatches to for an NC code, and the NC and foreign part of `dataset_to_xml`, which
  writes the whole dataset as read. An unknown profile code is an error.
- An object of a class no table knows (reference data's SKOS/PROV/DCAT) goes to
  `CimDataset::foreign`, keyed by its full IRI (`_:label` for an `rdf:nodeID`), with a
  `base::foreign_class` whose `qualified` name is the class IRI. Apart from `entries`, so
  validation, SPARQL and CIM lookups never see one; only the encoder and `foreign()` in
  cimoxide-py do.
- What the decoded value loses of how it was written is kept per dataset for the encoder:
  `field_forms` by (class, key) — a field's namespace where not its declaration's (the DCAT
  header's `dcterms:`/`prov:` properties), its `rdf:datatype`, the namespace a non-local
  reference was cut from; `type_ns` — an element's namespace where not its class's;
  `prefixes` — the documents' prefix bindings. JSON carries none of them.
- `schema_source.rs` — generated vs. RDFS-loaded class tables

Performance invariants:
- Field keys are `&'static str`: declared attributes use the class table's key, others go
  through `base::intern` (leaks each distinct name once). Don't reintroduce `String` keys.
- `FastMap` uses a hand-written Fx-style hasher with a fixed seed, so `by_type` order and
  violation order are reproducible. Core crates take no dependency for it; never pull in
  crates that are only in `Cargo.lock` via oxigraph.

### `cimvalidation`

Entry points: `validate_files(per_file, cfg)` (per-file checks in parallel, then
cross-profile on the merged dataset), `validate_header`, `validate_profile_local`,
`validate_crossprofile`, `validate_profile_shacl` / `validate_crossprofile_shacl` (table
only, no SPARQL rules), `validate_nc_profile` / `validate_nc_merged`, `combined_config`.
CGMES profiles: `EQ OP DY SV SSH SC GL DL EQBD`.

Hand-written SPARQL rules (`src/sparql/`) read attributes through `sparql::Fields`
(`f64("Class.attr")`, `reference`, `enumeration`, …): last value of a repeated scalar,
`true` only for the text `true`, a single-valued reference written twice is absent, and the
key is the declaring class (`Equipment.inService` on a Breaker). Rules for writing them,
audited against `application-profiles-library/validate/shacl-sparql/<rule>.rq`:
- **An absent value is not a value.** Never default a missing number to 0; a pattern-bound
  value must be present, an `OPTIONAL` one compared only when given. `bound(?x)` is
  `Fields::has`; a flag is `== Some(true)`.
- Where the SPARQL cannot report (misspelt property, contradictory bindings, …), implement
  its `sh:description` and say so at the rule.
- Don't hand-write a rule the shape table already runs (`sh:and`/`sh:xone` node shapes).
- `Fields::of_class`/`get` match the exact class; a lookup through a reference uses
  `Fields::of`.
- The Complex `*SolvedMAS`/`*NotSolvedMAS` rules (`sparql::mas_groups`) and the `!NS`
  table shapes run on the merged dataset, so dataset-wide counts span every file given.
- A rule in one profile's file that reads another profile's attributes (check the
  attribute's `origins`) runs on the merged dataset too, from `sparql::across_groups`:
  per file it never fires (CsConverter ranges: SSH mode vs. EQ limits) or misfires.
- `cimvalidation/tests/sparql_presence.rs` tests each rule with values present, absent and zero.

### `cimsparql`

`CimStore::from_dataset(&ds)` / `from_dataset_with(&ds, &GraphOptions)` / `.query(..)`;
`quads(..)` streams without a store. oxigraph is used with `default-features = false` (no
RocksDB / C++). Needs the CGMES schema table for IRIs, XSD types and enum namespaces.
`cimcli` and `cimoxide-py` depend on it behind a default-on `sparql` feature.

## Profile Families

CGMES (`CGMES/RDFS`, 446 classes) and NC (`NCP/RDFS`, 596 classes); `Family` in
`cimgen/src/schema/family.rs` holds what differs. They share 164 class names and both use
the prefix `cim`, so **only the namespace tells them apart** (`http://iec.ch/TC57/CIM100#`
vs `https://cim.ucaiug.io/ns#`). The decoder resolves `(namespace, local name)` with a
bare-name fallback.

- NC `type_name`s and `by_type` keys carry an `nc:` prefix.
- NC exports per profile (`dataset_to_xml_for_profile`) or as part of the whole dataset
  (`dataset_to_xml`). Ids are written in the form read (`rdf:ID`, `rdf:about="#…"`,
  `urn:uuid:`). `scripts/relicapgrid`'s export tests compare each file and its exports as
  RDF graphs.
- Import the two families into **separate** `CimSpecification`s (shared `or_insert`
  prefix map; both bind `cim` and `base` differently).

### Runtime loading (RDFS and SHACL)

Behind features `dynamic-schema` (`cimmodel::schema_source`) and `dynamic-shapes`
(`cimvalidation::shape_source`, which enables `dynamic-schema`); both off by default, on for
`cimcli` and `cimoxide-py`. Resolution: explicit `load_from` > `CIMOXIDE_RDFS_DIR` /
`CIMOXIDE_SHACL_DIR` > generated. Each variable is a `PATH`-style list; `family_of_dir`
assigns each directory to a family (CGMES's `61970-600-2_*` pattern is tried first). A
SHACL load finds RDFS at `<dir>/../RDFS`. Bad paths warn and fall back.

`postprocess` in `cimschema` is mandatory for both build-time and runtime parsing.
`cimmodel/tests/dynamic_schema.rs` and `cimvalidation/tests/dynamic_shapes.rs` compare
runtime and generated tables entry by entry — the class-table loader is a second code path
that can drift from `classes_gen.rs`.

### Shape tables

Both families validate through `bag.rs` over a shape table; a `bag::Source` keeps each
family's shapes on its own elements. CGMES ships no per-profile manifests, so
`cimschema::shacl::cgmes_manifest` writes the file → tag mapping (`"EQ"`, `"SSH!NS"` for
not-solved on the merged dataset, `"X:SV"` for cross-profile, `"HDR"`, `"COMMON"` under
`--common`). NC reads its manifests from `NCP/SHACL/Validation/`. Profiles are detected from
the header (`md:Model.profile` / `dcterms:conformsTo`) via `PROF/`; no header, nothing runs.

Semantics to preserve:
- A concrete `sh:targetClass` matches that class only; an abstract one expands to concrete
  descendants (`Resolver::targets_of`). The APL is written for literal matching; widening
  concrete targets added ~14k spurious findings.
- `sh:closed` on an abstract class governs descendants except those below a class with its
  own closed shape in the same file (`Resolver::closed_concrete`).
- `sh:datatype` and `sh:nodeKind` are real checks against the text as written.
- `sh:in` values are keyed like a decoded `rdf:resource` (after the last `#`).
- `sh:class` holds if any type of a merged element is an instance; `sh:in` on an `rdf:type`
  path (`RefClass`) tests every type.
- In the per-file phase a class check reads the referenced object's types from the file;
  if the file lacks it, NC shapes read them from the other files given (`validate_files`
  passes the set), CGMES shapes stay silent. NCP's value types list every reachable class
  (`cim16:`/`cim17:`/`cim:`/`nc:`); CGMES's per-profile ones list only their own profile's
  classes (`sv:SvStatus.ConductingEquipment`: the two converters) and fail ~31k times on
  the test configurations when read across files. The cross-profile files check CGMES types.
- Paths are **not** verified against the class table (data may carry keys the table lacks).
- Stepping a `Path::Chain` from an absent element makes the checks silent.
- `sh:minCount` on `IdentifiedObject.mRID` skips a CGMES element no file defined (`rdf:ID`),
  only described (`rdf:about`; `Element::defined`): the SSH SHACL asks every `cim:Equipment`
  for an mRID the EQ file writes. NC defines objects with `rdf:about`, so NC is exempt.
- A finding is reported once per element and rule. Its `property` is the path's first field
  key, `^Field` when inverse. `sh:targetNode` + `[ sh:inversePath rdf:type ]` (instance
  counts) is hand-written in `sparql/`, labelled `^rdf:type`.
- Resolution happens once at table build, never per element.
- `sh:Info` is advisory and excluded from `cimcli validate`'s exit code.
- Not covered (reported as skips): the 119 `cim16:`/`cim17:` NC target classes.

Walks over ≥20,000 elements run on threads (`cimvalidation/src/par.rs`, `std::thread::scope`)
in contiguous runs concatenated in order, so output is byte-identical to one thread. Unit
tests on small fixtures don't exercise the split; compare `cimcli` output on real data.

NC specifics: the Complex files are imported only by the `ALL` manifest
(`resolve::MERGED_PROFILE`) and run once on the merged dataset (`validate_nc_merged`). NCP's
35 `sh:sparql` constraints and two `sh:SPARQLTarget` shapes are hand-written in
`sparql/nc.rs`, with departures from the SPARQL commented at each rule.
`cimvalidation/tests/nc_sparql.rs` breaks and fixes each. `ClassCount` is `sh:Info` on
every class, so tests asserting a clean NC dataset filter `sh:Info`.

### Checking a validation change

Compare `cimcli validate --common --quality --format json` before and after as multisets of
`(object_id, rule_id, property)` across the CGMES test configurations, and on copies mutated
to trigger the touched rules. Time with alternating before/after runs in one session; a
difference under ~5% means nothing until an independent A/B reproduces it (cross-run drift
has been measured at 20% for identical code).

### `cimoxide-lsp`

`cimlsp` over stdio (`lsp-server`/`lsp-types`, no async runtime). A *model set* is every CIM
XML file in a document's directory, open buffers taking precedence over disk; it is
validated with `validate_files` + `combined_config`, as `cimcli validate` does, on a worker
thread with a 300 ms debounce. The decoder keeps no positions, so `src/index.rs` makes its
own quick-xml pass per document (ranges of elements, fields, `rdf:resource` values; mRID =
text after the last `#`, as in `decode.rs`). A per-file finding is placed only in the file it
was found in (`validate_files_traced`). Each diagnostic's `data` carries `object_id`
and `property`; its `code` is the `rule_id`. Diagnostics must stay equal to
`cimcli validate --format json` as sets of `(object_id, rule_id, property)` per directory.

Rename (an mRID) and the delete code actions (`codeAction/resolve` computes the edit;
`cimoxide/delete` for the tree view) are text edits over the set's files as they are now: an open
buffer, else the file on disk with the set's index of it (`src/edit.rs`). An mRID is the end of
the value written, so only that suffix is replaced; a deletion takes whole lines and is annotated
`needsConfirmation`, so the client previews it. Deleting with dependents takes the object's
subtree in the tree's containment (`Tree::descendants`). A finding whose `property` the object
writes in no file gets a quick fix adding it (`edit::add_missing`): to the file writing the object
whose header profile is among the attribute's `origins`, after its last field, in its line ending;
the action's command `cimoxide.select` (registered by the extension) selects the value. For a
reference, `edit::candidates` ranks the objects of the range class: linked to the object first
(naming what it names with its other fields, or named by it), then by containers shared with the
object (where the candidate is, or the objects naming it with the same field), then use; the same
ranking feeds the `rdf:resource` completion (`candidates_rank_what_the_data_names` pins the
MicroGrid hit rates). Adding an object (`cimoxide/addable`, `cimoxide/add`, `edit::add`): the
classes of the parent's family with a `CONTAINED` field ranging over the parent's class, of the
profiles the set declares; written with a v4 UUID (from `RandomState`, no dependency) in the
non-SSH file whose profile carries the class (the parent's own first), after the last element
there of an object the parent holds, plus an empty `rdf:about` in the SSH file when the class is
in SSH. No mandatory fields: the findings ask for them, and the quick fixes add them.

`cimoxide/grid` (`src/grid.rs`) answers the set's grid at substation level as last validated:
substations, and boundary points no substation holds, as nodes; equipment whose terminals reach
two of them as edges; each finding counted on its object's node or edge (`elsewhere` if none).
It is built from the set's datasets before validation consumes them. Each DL diagram and each GL
coordinate system is a layout: a node is at the mean of the points of the objects it holds (else of
its lines' ends), an edge follows its line's points. The automatic layout is computed here too (`src/layout.rs`:
each component by PivotMDS, which keeps graph distances, then local Fruchterman–Reingold with
repulsion only between nodes in neighbouring grid cells; linear, deterministic; the tests check
distance faithfulness on a ring and a lattice), as Cytoscape's `cose` is quadratic and hangs on
RealGrid, and a force layout alone, multilevel included, leaves it folded; inside a
substation the view still runs `cose`, which boxes the voltage levels. `cimoxide/substation` answers what one
substation holds, built with the grid: its equipment and the connectivity (else topological) nodes
their terminals end at, by voltage level, a terminal as a link; equipment reaching other nodes as
an item with its far ends; switch states and `inService`. A finding counted on the substation's node
goes to the item its object stands for (the reference `place` followed), else to the substation
itself, so the two add up to the node's count (`micro_grid_inside` checks this).

`cimoxide/tree` (`src/tree.rs`) answers the set's objects as a tree, a level at a time by key, built
with the grid from the same model: Geographical, each object under the target of the first
`CONTAINED` field it writes, from the regions and containers no container holds; Operational, the
areas; Not contained, the rest of that forest by class, so Geographical and Not contained hold each
object once; By class, every object. A parent the set lacks is an object of its own. Each node
counts its subtree's findings; a level of more than `PAGE` children is split by class, then in
pages. Each object carries `Grid::reveal`, where the grid view shows it. `Tree::path` gives the keys
down to an object, those class groups and pages included (`cimoxide/treePath`, for the view to
reveal the object at the cursor), and `Tree::containers` the outline's container labels
(`micro_grid_tree`).

The extension (`editors/vscode/`, TypeScript, `vscode-languageclient`) passes settings as
initialization options and schema directories as `CIMOXIDE_*` environment variables, and
restarts the server on any `cimoxide.*` change. *Show grid* draws `cimoxide/grid` in a webview
(`src/gridView.ts`; the page is `src/webview/grid.ts`, Cytoscape.js bundled by esbuild into
`dist/grid.js`, as webviews load nothing from the network). The *CIM Model* Explorer view
(`src/modelTree.ts`) draws `cimoxide/tree` for the active editor's set; its findings badges are
a `FileDecorationProvider` on the items' `cimoxide-tree:` resource URIs, which carry the counts.

### `cimoxide-mcp`

`cimmcp`: MCP over stdio as newline-delimited JSON-RPC, hand-written in `src/main.rs` (no
async runtime, no MCP SDK). Tools in `src/tools.rs`, output plain text with a `limit`; all read
but `export` and `edit`, which write `to_xml_for_profile`/`to_xml` output to a file inside the
workspace (never over one unless `overwrite`, never over a model set's own file), or return it
when short; `format: "json"` writes instead every object of `entries` and `foreign`, as
cimoxide-py gives them. `edit` applies its changes to a fresh decode of the set, in JSON element form through
`CimDataset::element_from_json` as cimoxide-py's `__setitem__` does; edits never live in the session. `src/session.rs` caches each model set (a directory's CIM files, as for `cimlsp`, or the
files and directories a tool's `files` lists) until a file's mtime changes; the SPARQL store, the
files that wrote each object (`rdf:ID` or `rdf:about`, for `get_object` and `conflicts`) and each
validation configuration's findings are built on first use. `validate` reports what `cimcli validate` does
(`tests/protocol.rs` compares them). `cimmodel::decode::is_cim` decides which files belong,
shared with `cimlsp`. The extension registers it through
`lm.registerMcpServerDefinitionProvider` with `--dir <first workspace folder>` (VS Code
≥ 1.101).

## Codegen Stability Tests

`cimgen/tests/codegen.rs` hashes generated output: `cimmodel_codegen_stable`,
`nc_classes_codegen_stable`, `cgmes_shapes_codegen_stable`, `nc_shapes_codegen_stable`.
For an intentional generator or schema change:
1. Run the tests; the failure message shows the new hash
2. Update the `assert_eq!` hashes
3. Regenerate with `cargo run -p cimoxide-gen`
4. Commit hashes and generated files together

## Releasing

Version, license and internal dependency versions live once in the root `Cargo.toml`
(`[workspace.package]`, `[workspace.dependencies]`). Pushing a `v*` tag runs `release.yml`
(cimcli, cimlsp and cimmcp binaries), `pypi.yml` (the `cimoxide` bindings, and the
`cimoxide-mcp` wheels: `cimmcp` as a maturin `bindings = "bin"` wheel per platform, for
`uvx --from cimoxide-mcp cimmcp`; its version comes from the workspace) and `crates-io.yml` (needs `CARGO_REGISTRY_TOKEN` in the
`crates-release` environment; runs `make generate`, hence `--allow-dirty`; dry-runnable via
`workflow_dispatch`). Publishing goes through `scripts/publish-workspace.sh`, which skips
`name@version` pairs already on crates.io and sleeps through 429 rate limits, so a stalled
run can be re-run as-is.

`vscode.yml` builds `cimlsp` and `cimmcp` for three targets (Linux x64 as static musl, Windows x64, macOS arm64), packages one VSIX per
platform and publishes them to the Visual Studio Marketplace and Open VSX (`VSCE_PAT` and
`OVSXAT` in the `vscode-release` environment). `editors/vscode/package.json`'s version is
bumped by hand with the workspace's; the workflow fails when they differ.
