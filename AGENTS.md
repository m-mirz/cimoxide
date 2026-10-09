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
`CGMES-Test-Configurations/` (real test datasets).

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
`used` (an association end some profile exchanges), `xsd` and `value_ns`; both
`classes_gen.rs` and the runtime loader derive them through `cimschema::table`.

- `base.rs` — `Element`, `FieldValue`, `FastMap`/`FieldMap`, `ClassDef`/`AttrDef`,
  `TypeRegistry` (`(namespace, local name)` → class, bare-name fallback for CGMES, inherited
  attributes via `attr(class, id)`, `chain(class)`)
- `decode.rs` — streaming parser → `CimDataset { entries, by_type }`; `decode_file`,
  `decode_files`, `decode_str`. `merge`: a later scalar wins, reference lists join, duplicate
  tracking stays per file. An object typed differently by different files keeps every type
  (`Element::types`); its class and `by_type` bucket are the most specific one.
- `convert.rs` — RDF/XML export (`dataset_to_xml`, `dataset_to_xml_for_profile`) and JSON
  (`{mrid: {"_type", "id", "Class.attr": "value" | [...]}}`)
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
- NC supports decode and validation only; the encoder skips NC elements.
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
- Paths are **not** verified against the class table (data may carry keys the table lacks).
- Stepping a `Path::Chain` from an absent element makes the checks silent.
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
text after the last `#`, as in `decode.rs`). Each diagnostic's `data` carries `object_id`
and `property`; its `code` is the `rule_id`. Diagnostics must stay equal to
`cimcli validate --format json` as sets of `(object_id, rule_id, property)` per directory.

The extension (`editors/vscode/`, TypeScript, `vscode-languageclient`) passes settings as
initialization options and schema directories as `CIMOXIDE_*` environment variables, and
restarts the server on any `cimoxide.*` change.

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
(cimcli binaries), `pypi.yml` and `crates-io.yml` (needs `CARGO_REGISTRY_TOKEN` in the
`crates-release` environment; runs `make generate`, hence `--allow-dirty`; dry-runnable via
`workflow_dispatch`). Publishing goes through `scripts/publish-workspace.sh`, which skips
`name@version` pairs already on crates.io and sleeps through 429 rate limits, so a stalled
run can be re-run as-is.

`vscode.yml` builds `cimlsp` for five targets (static musl on Linux), packages one VSIX per
platform and publishes them to the Visual Studio Marketplace (`VSCE_PAT` in the
`vscode-release` environment). `editors/vscode/package.json`'s version is
bumped by hand with the workspace's; the workflow fails when they differ.
