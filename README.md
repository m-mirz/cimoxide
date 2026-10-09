# cimoxide

Rust tooling for CGMES CIM data: decoding, SHACL validation, conversion and SPARQL, driven by class and shape tables generated from the ENTSO-E schemas.

## Usage

### `cimoxide-cli`

```bash
# Decode RDF/XML and print element counts (--json for machine-readable output)
cimoxide-cli import [--json] <xml-files...>

# Convert RDF/XML <-> JSON; --profile splits XML output per CGMES profile into a directory
cimoxide-cli convert --to json <xml-files...> [--out <output.json>]
cimoxide-cli convert --to xml <input.json> [--profile EQ,SSH] [--out <output.xml|dir/>]

# Run SHACL + SPARQL validation
cimoxide-cli validate [--profiles EQ,SSH,...] [--solved] [--not-solved]
                      [--common] [--quality] [--silence rule1,rule2]
                      [--format text|json|sarif] <xml-files...>

# Run a SPARQL 1.1 query over the merged input files
cimoxide-cli query --query "SELECT ..." | --file <query.rq>
                   [--format text|json|csv|tsv] <xml-files...>
```

| `validate` flag | Effect |
|---|---|
| `--profiles EQ,SSH,...` | override the auto-detected profile list |
| `--solved` / `--not-solved` | force the solved/unsolved variant of cross-checks |
| `--common` | enable cross-profile common checks |
| `--quality` | enable the 14 CIMdesk quality checks (see below) |
| `--silence rule1,rule2` | suppress specific `rule_id`s |
| `--format text\|json\|sarif` | output format (default `text`) |

In `text` mode the exit status is `2` if any violation or warning is found (`sh:Info` is
advisory); `json` and `sarif` always exit `0`. `sarif` writes
[SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/) for code-scanning tools: one
result per finding, the object as a logical location and every file and line that writes it
as a physical location (`sh:Violation` → `error`, `sh:Warning` → `warning`, `sh:Info` → `note`).

### Editor support: `cimlsp` and the VS Code extension

`cimlsp` (crate `cimoxide-lsp`) is a language server over stdio for CGMES and NC RDF/XML:

- **Diagnostics** — the findings of `cimcli validate`, over the *model set* a document
  belongs to: every CIM XML file in its directory, open buffers in place of the files on
  disk. A finding shows on every file that writes its object, at the field it is about.
  Runs on open and save, and while typing with `onType`.
- **Hover** on a class, an attribute or an `rdf:resource`; **go to definition** and **find
  references** across the model set; a document **outline**; **completion** of a class's
  attributes (inherited included) and of classes.

Initialization options: `{"common": bool, "quality": bool, "silence": [rule ids],
"onType": bool}`; schemas come from `CIMOXIDE_RDFS_DIR` / `CIMOXIDE_SHACL_DIR` as for
`cimcli`. Any LSP client can run it (`cargo install cimoxide-lsp`, or the binaries on each
GitHub release).

The VS Code extension in `editors/vscode/` bundles `cimlsp` per platform and is published to
the Visual Studio Marketplace by `.github/workflows/vscode.yml`. To try it
locally:

```bash
cargo build --release -p cimoxide-lsp -p cimoxide-mcp
cd editors/vscode && npm ci && mkdir -p server && cp ../../target/release/{cimlsp,cimmcp} server/
npx vsce package --target linux-x64      # then: code --install-extension cimoxide-*.vsix
```

or open `editors/vscode/` in VS Code and press F5 (with `cimoxide.server.path` pointing at
`target/release/cimlsp`).

### Chat about a model: `cimmcp`

`cimmcp` (crate `cimoxide-mcp`) is a [Model Context Protocol](https://modelcontextprotocol.io)
server over stdio, so an LLM chat can answer questions about a model set — "which generators
are in BE and what is their P?", "why does this line fail validation?". Its tools are
read-only:

| Tool | |
|---|---|
| `list_model_sets` | directories holding CIM files, with each file's profiles |
| `summary` | a set's files, model headers and object counts by class |
| `find_objects` | search by class (abstract classes included), name/mRID text and attribute value |
| `get_object` | one object's fields, references resolved, and what references it |
| `validate` | the findings of `cimcli validate`, by rule, filterable |
| `sparql` | a SPARQL query over the merged CGMES data |
| `describe_class` | a class's, attribute's, enumeration's or enumeration value's definition from the ENTSO-E vocabulary, and what a class carries |

A model set is decoded once and kept until one of its files changes. `--dir <path>` (else
`CIMOXIDE_MODEL_DIR`, else the working directory) is where relative paths resolve. The VS Code
extension bundles `cimmcp` and offers it to chat (Copilot agent mode) for the workspace; any
other MCP client can run it. It is on PyPI as a binary wheel, so with
[uv](https://docs.astral.sh/uv/) a client starts it with nothing installed first:

```bash
claude mcp add cimoxide -- uvx --from cimoxide-mcp cimmcp            # Claude Code
codex mcp add cimoxide -- uvx --from cimoxide-mcp cimmcp --dir /path/to/models   # Codex
```

Clients configured by an `mcpServers` JSON block (Claude Desktop, Cursor, Windsurf,
Antigravity, …) take `"command": "uvx", "args": ["--from", "cimoxide-mcp", "cimmcp", "--dir",
"/path/to/models"]`. `cargo install cimoxide-mcp` and the GitHub release binaries work too;
see [`cimoxide-mcp/README.md`](cimoxide-mcp/README.md).

## Repository layout

| Crate | Description |
|---|---|
| `cimgen` | Code generator — reads ENTSO-E RDF/SHACL schemas and emits Rust source |
| `cimmodel` | **Generated** class tables for CGMES and NC, the decoder (`CimDataset` of `Element`s) and JSON/RDF-XML conversion |
| `cimschema` | RDFS and SHACL parser, shared by `cimgen` and the runtime loaders |
| `cimvalidation` | SHACL validation through one interpreter (`bag.rs`) over **generated** shape tables; hand-written `sh:sparql` rules in `src/sparql/` |
| `cimsparql` | SPARQL 1.1 over a decoded dataset, backed by in-memory oxigraph |
| `cimoxide` | Facade re-exporting `cimmodel` (as `model`), `cimvalidation` and `cimsparql` |
| `cimoxide-cli` | The command-line tool |
| `cimoxide-lsp` | `cimlsp`, the language server; its VS Code extension is `editors/vscode/` |
| `cimoxide-mcp` | `cimmcp`, the MCP server for chat clients |
| `cimoxide-py` | Python bindings (PyO3, built with `maturin`, outside the Cargo workspace) |

Generated, not to be hand-edited: `cimmodel/src/generated/` and
`cimvalidation/src/{cgmes_shapes,cgmes_profiles,nc_shapes,nc_profiles}.rs`. The 0.3.x crates
`cimoxide-structs`, `cimoxide-decoder` and `cimoxide-convert` were merged into `cimmodel`.

## Profile families

| Family | Schemas | Classes | Supports |
|---|---|---|---|
| CGMES | `CGMES/RDFS` | 446 | decode, validate, convert, query |
| NC (Network Codes) | `NCP/RDFS` | 596 | decode, validate |

The two share 164 class names (`Terminal`, `Equipment`, …) but these are different classes —
NC's carry attributes CGMES lacks, such as `Equipment.networkAnalysisEnabled`. Both use the
prefix `cim`, so only the XML namespace tells them apart:

```xml
<!-- CGMES -->                            <!-- NC -->
xmlns:cim="http://iec.ch/TC57/CIM100#"    xmlns:cim="https://cim.ucaiug.io/ns#"
```

The decoder resolves `(namespace, local name)`, falling back to the bare local name for
unrecognised namespaces, as real files need.

### Working with elements

Every element is an `Element`: its class, its mRID, and its fields as written, keyed
`Class.attr`. There are no generated types; attributes a class does not declare and values
that do not parse are kept. NC classes carry an `nc:` prefix, so `by_type["Equipment"]` and
`by_type["nc:Equipment"]` never collide:

```rust
let ds = CimDataset::decode_file(Path::new("contingencies.xml"))?;
for mrid in &ds.by_type["nc:OrdinaryContingency"] {
    println!("{} {:?}", mrid, ds.entries[mrid].get_str("IdentifiedObject.name"));
}
let line = &ds.entries["_line1"];          // a CGMES ACLineSegment
let r: Option<f64> = line.get_f64("ACLineSegment.r");
let terminals = line.get_refs("ACLineSegment.Terminals");
```

On RealGrid (189,000 elements) `cimcli import` takes 0.78 s and 350 MB peak.

### Loading schemas and shapes at runtime

Class and shape tables are compiled in, but `cimcli` and the Python bindings can load either
from the ENTSO-E files instead, so a new profile version is a data load, not a recompile:

```bash
CIMOXIDE_RDFS_DIR=application-profiles-library/CGMES/RDFS:application-profiles-library/NCP/RDFS \
  cimcli validate model.xml
CIMOXIDE_SHACL_DIR=application-profiles-library/NCP/SHACL:application-profiles-library/CGMES/SHACL \
  cimcli validate model.xml
```

Resolution is explicit `load_from` > environment variable > generated table. Each variable
takes one or more directories separated as in `PATH`; each serves the family whose files it
holds, and a family without one keeps its generated table. A directory that holds neither or
fails to parse warns and falls back. A SHACL load reads the RDFS of the same release from
`<dir>/../RDFS`, since shapes and classes must agree. In library crates this sits behind the
`dynamic-schema` / `dynamic-shapes` features, off by default.

| Cost (`scripts/bench_schema_source.sh`, `bench_shape_source.sh`) | From RDFS | From SHACL |
|---|---|---|
| Startup, per process | +25 ms | +163 ms |
| Throughput | decode +3.5% | validate unchanged (±2%) |
| Peak RSS | +6.2 MB | +51 MB |

Cross-run drift in these benches has been measured at 20% for identical code, so a difference
under ~5% needs a second independent A/B before it means anything.

## SHACL Validation

`cimgen` resolves the SHACL Turtle files into shape tables (`cgmes_shapes.rs`: 849 shapes,
18,681 checks; `nc_shapes.rs`: 2,014 shapes, 14,889 checks), which `cimvalidation/src/bag.rs`
interprets element by element. On RealGrid `cimcli validate` takes about 1.4 s. Which file
applies to which profile comes from NCP's manifests for NC and from
`cimschema/src/shacl/cgmes_manifest.rs` for CGMES, which ships none. Profiles are detected
from the file header (`md:Model.profile`, or NC's `dcterms:conformsTo`); a file without one
is not validated.

- A concrete `sh:targetClass` matches that class only; an abstract one (3 of 1,487 CGMES
  targets) matches its concrete subclasses. Data carries no `rdfs:subClassOf`, and the APL
  is written for literal matching.
- `sh:datatype`, `sh:nodeKind` and `sh:closed` are checked against the text as written, so a
  malformed number or an undeclared property is reported.
- A finding is reported once per element and rule.
- The SSH rule `IdentifiedObject.mRID-cardinality` applies only to elements written as
  `cim:Equipment` (9,070 on RealGrid). Its rule id is shared by every profile, so `--silence`
  hides a missing mRID in EQ too.
- The 186 CGMES `sh:sparql` constraints and NCP's 35 are hand-written in
  `cimvalidation/src/sparql/`. NCP's Complex files run once on the merged dataset.
- Not covered for NC, reported as skips: 119 `cim16:`/`cim17:` target classes.

### Simplification rules applied during import

Specific to this tool; they may not hold for other engines.

| Rule | Constraint removed / rewritten | Reason |
|------|-------------------------------|--------|
| 3 | `sh:minCount 0` | Vacuously true |
| 4 | `sh:minCount 0` + `sh:maxCount 1` → `sh:maxCount 1` | Rule 3; the upper bound stays as a duplicate check |
| 5 | `sh:minCount 1` + `sh:maxCount 1` → `sh:Required` | "Exactly one", checked once |
| 6 | single-value `sh:in` → `sh:hasValue` | Equivalent |

(Rules 1, 2 and 7 used to drop `sh:nodeKind`/`sh:datatype`; both are now kept.)

### Skipped constraints

From `cargo run -p cimoxide-gen -- --skip-report`; the total matches the `Skipped` column of
the profile table below.

| Count | Category | Reason |
|------:|---------|--------|
| 178 | SPARQL-derived | `sh:sparql` constraints and `sh:SPARQLTarget` targets; hand-written instead (counted per `(property, component, sh:name)` entry, so not 186) |
| 2 | Instance count of a class | `sh:targetNode` + `[ sh:inversePath rdf:type ]`; hand-written (`GeographicalRegion-EQ__4`, `TopologicalIsland-instance`, plus `All-HGEN2`) |
| 6 | Target class not in the schema | Upstream defects (see below), or rule labels used as classes, all backing hand-written `sh:sparql` rules |
| 3 | Unresolvable value list | Upstream defects: `cim:CSConverter` (×2), empty `sh:in ()` |
| 7 | `sh:nodeKind` on compound/`rdf:type` paths | Blank-node-ness is not visible after decoding; the class half is checked |
| **196** | **Total** | |

cimgo covers the same 74 TTL files and also counts 12,269 non-`sh:sparql` constraints;
compare per file with the `PERFILE` lines of `--rule-report`:
`awk -F'\t' '{print $2, $5}'` on each tool's sorted `grep PERFILE` output, then `diff`.

### Upstream SHACL TTL defects

| File | Rule / shape | Defect |
|------|------|--------|
| 302 Dynamics Complex | `C:302:DY:GovHydroIEEE0.pmin:valueRangePair` | `sh:lessThan cim:GovHydroIEEE.pmax` — should be `GovHydroIEEE0.pmax` |
| 302 Dynamics Complex | `C:302:DY:PFVArType1IEEEVArController.vvtmin:valueRangePair` | `PVFArType1…` — should be `PFVArType1…` |
| 302 Dynamics Complex | `C:302:DY:ExcDC1A.efdmin:valueRangePair` | `ExcDC1A.edfmax` — should be `efdmax` |
| 302 Dynamics Complex | `C:302:DY:PssIEEE4B.vhmin:valueRangePair` | `PssIEEE4V.vhmax` — should be `PssIEEE4B.vhmax` |
| 600-2 Dynamics InverseAssociation | `SynchronousMachineDynamics.CrossCompoundTurbineGovernorDyanmics-cardinality` | class misspelt `Dyanmics` |
| 600-2 Dynamics InverseAssociation | `SynchronousMachineDynamics.CrossCompoundTurbineGovernorDynamics-cardinality` | no such field; RDFS has `High`/`LowPressureSynchronousMachineDynamics` |
| 600-2 Dynamics InverseAssociation | `CsConverter.CSCDynamics-cardinality` | `CSCDynamics.CsConverter` — RDFS spells `CSConverter` |
| 600-2 Dynamics InverseAssociation | `VCompIEEEType2.GenICompensationForGenJ-cardinality` | RDFS spells `VcompIEEEType2` |
| 600-2 Dynamics InverseAssociation | `WindContQIEC.WindTurbineType3or4IEC-cardinality` | RDFS spells `WIndContQIEC` |
| 600-2 Dynamics InverseAssociation | `sh:targetClass` list | `cim:GovHydroIEEE1` does not exist |
| 457 Dynamics CrossProfile (×2) | `CSCDynamics.CSConverter-valueType` | `sh:in (cim:CSConverter)` — class is `CsConverter` |
| 301 Operation Complex | `C:301:OP:AccumulatorValue.value:valueRange` | field removed in CGMES 3.0 |
| 600-2 Operation Simple | `Measurement.Terminal-valueType` | empty `sh:in ()` |
| 600-2 SV Simple | `SvStatus.ConductingEquipment-valueType` | `sh:in` lists only `CsConverter`/`VsConverter`; cimoxide runs it per SV file, where it is silent |

### SHACL Rules by Profile

From `cargo run -p cimoxide-gen -- --rule-report` (grouping: `ttl_group_label` in
`cimgen/src/shacl/ttl_report.rs`). "Checks" counts distinct `(path, component, sh:name)`
patterns in the table; "Total" is every non-`sh:sparql` constraint in the group's TTL files.
The table checks 98.4% of them.

| Profile Group | Checks | Skipped | Total |
|---------------|-------:|--------:|------:|
| Equipment (EQ) | 1135 | 66 | 1201 |
| Steady State Hypothesis (SSH) | 226 | 39 | 265 |
| Dynamics (DY) | 9770 | 43 | 9813 |
| State Variables (SV) | 130 | 12 | 142 |
| Short Circuit (SC) | 326 | 7 | 333 |
| Common / AllProfiles | 161 | 23 | 184 |
| Topology (TP) | 45 | 3 | 48 |
| DiagramLayout (DL) | 87 | 1 | 88 |
| Operation (OP) | 193 | 2 | 195 |
| **Total** | **12073** | **196** | **12269** |

### SPARQL Check Coverage

`sh:sparql` constraints are implemented as hand-written functions in
`cimvalidation/src/sparql/`, each tagged with the source rule's ID and name. `--rule-report`
resolves the call graph from each group's `validate()` and matches the resulting
`Violation.name`s against the TTL files' `sh:name`s (split on `|`, so counts are per rule
name, not per shape). Re-run it after changing checks. `C:600:ALL:NA:PROF10` counts toward
Common without TTL backing.

| Profile Group | Implemented | TTL Total | Coverage |
|---------------|-------------:|----------:|---------:|
| Equipment (EQ) | 68 | 68 | 100.0% |
| Steady State Hypothesis (SSH) | 39 | 39 | 100.0% |
| Dynamics (DY) | 40 | 40 | 100.0% |
| State Variables (SV) | 11 | 11 | 100.0% |
| Short Circuit (SC) | 7 | 7 | 100.0% |
| Common / AllProfiles | 16 | 16 | 100.0% |
| Topology (TP) | 3 | 3 | 100.0% |
| DiagramLayout (DL) | 1 | 1 | 100.0% |
| Operation (OP) | 1 | 1 | 100.0% |
| CIMdesk quality | 15 | n/a | n/a |
| **Total** | **186** | **186** | **100.0%** |

Validation deliberately does not run the TTL's SPARQL through `cimsparql`: it would need a
graph per run, and the two are not always equivalent (`cimsparql/tests/query.rs`).

### CIMdesk quality checks (`--quality`)

`cimvalidation/src/sparql/quality.rs` implements 14 checks not in the CGMES SHACL files
(the coverage table shows 15 names because `quality:Conductor.noLocation` covers AC and DC
lines separately).

| Class | Check |
|-------|-------|
| *(global)* | No `TapChangerControl`s / `RegulatingControl`s / `ShuntCompensator`s found |
| *(global)* | No `Location` objects associated with line segments |
| `Substation` | No child `VoltageLevel`s |
| `ControlArea` | No child objects |
| `ACLineSegment` | `x / r` ratio too large |
| `BaseVoltage` | Two instances share the same `nominalVoltage` |
| `PowerTransformer` | Both ends have the same `nominalVoltage` |
| `ConnectivityNode` | Only one `Terminal` connected |
| `Disconnector` | Connects nodes in different `VoltageLevel`s |
| `ConformLoad` | Not in the same `EquipmentContainer` as its `TopologicalNode`s |
| `RegulatingControl` | Target voltage 10–20 % off the regulated node's `nominalVoltage` |
| EQBD `BaseVoltage` | MRID not listed in the equipment boundary dataset |

## SPARQL

`cimsparql` materialises a `CimDataset` into an in-memory RDF graph and runs SPARQL 1.1 over
it — SELECT, ASK, CONSTRUCT and DESCRIBE, including joins across profiles.

```rust
use cimsparql::{CimStore, QueryResults};

let ds = cimmodel::CimDataset::decode_files(&paths)?;
let store = CimStore::from_dataset(&ds)?;
if let QueryResults::Solutions(solutions) = store.query(
    "SELECT ?name ?r WHERE { ?s a cim:ACLineSegment ;
                                cim:IdentifiedObject.name ?name ;
                                cim:ACLineSegment.r ?r }",
)? {
    for solution in solutions { println!("{:?}", solution?); }
}
```

```python
rows = dataset.query("SELECT ?s WHERE { ?s a cim:ACLineSegment }")
```

`cim:`, `eu:`, `md:`, `dm:`, `rdf:` and `xsd:` are pre-bound, so a `sh:select` body from the
SHACL files can be pasted in unchanged; a query's own prefixes win, and `query_raw` binds
nothing. oxigraph is built with `default-features = false` — pure Rust, no RocksDB. `cimcli`
and `cimoxide-py` include SPARQL behind the default `sparql` feature.

How data becomes RDF: the CGMES schema table restores what the decoder strips — each class's
and attribute's namespace (so `eu:` attributes stay out of `cim:`), a literal's XSD type
(`Float` → `xsd:double`) and an enumeration's value namespace. Subjects normalise onto
`urn:uuid:` (non-UUIDs onto `http://cimoxide/id/`). Every field becomes a quad, including
undeclared ones. Everything goes in the default graph; elements record no source file.

**Cost.** The graph is held in addition to the elements. Merged RealGrid (117 MB RDF/XML,
188,551 elements → 1.3 M quads): building the store takes 2.7 s, `cimcli query` peaks at
900 MB against 350 MB for `import`; a two-hop join takes 80 ms. `GraphOptions::with_types`
restricts materialisation to the classes a query touches.

## RDF/XML export

`cimmodel::convert` writes CGMES RDF/XML as a flat dump (`dataset_to_xml`) or per profile
(`dataset_to_xml_for_profile`, `cimcli convert --to xml --profile EQ,SSH`).

Prefixes come from the schema and are decided **per field**, since CGMES mixes them on one
element (`eu:BoundaryPoint` with `eu:BoundaryPoint.toEndName` and
`cim:IdentifiedObject.description`). Enum values are written as absolute IRIs, mRID
references as local fragments. Re-encoding `FullGrid_EQ.xml` reproduces its prefix
distribution exactly.

Profile routing is purely from the schema: an element goes to profile P iff its class's
`origins` list P, as a definition (`rdf:ID`) if P is the class's dominant origin, otherwise as
a reference (`rdf:about`) with only P's fields. EQBD dominates nothing (EQ outranks it), so it
is treated as self-defining: `rdf:ID` plus every member field, matching `FullGrid_EQBD.xml`.
Known limitation: exporting a *merged* dataset to EQBD emits every boundary-class instance,
since the EQ/EQBD split is per instance. Round-tripping a single EQBD file is exact.

## Development

```bash
git clone --recurse-submodules <url>      # or: git submodule update --init --recursive
cargo build
cargo test                                # requires submodules
cargo run -p cimoxide-gen                 # regenerate cimmodel and cimvalidation
cargo bench -p cimoxide-model [--bench real_grid -- import]
```

Benchmarks (`cimmodel/benches/real_grid.rs`) use RealGrid from `CGMES-Test-Configurations`;
set `debug = true` under `[profile.bench]` for flamegraphs.

### `cimgen` flags

Pass after `--`, e.g. `cargo run -p cimoxide-gen -- --verbose --rule-report`.

| Flag | Default | Effect |
|---|---|---|
| `--schema <glob>` | `application-profiles-library/CGMES/RDFS/61970-600-2_*-AP-Voc-RDFS2020.rdf` | RDF/RDFS schema files |
| `--output <dir>` | `cimmodel/src/generated` | class-table output |
| `--shacl <glob>` | `application-profiles-library/CGMES/SHACL/*.ttl` | SHACL TTL files |
| `--shacl-output <dir>` | `cimvalidation/src` | shape-table output |
| `--python-stubs-output <dir>` | `cimoxide-py/python/cimoxide` | `.pyi` stubs |
| `--verbose` / `-v` | off | parse diagnostics |
| `--skip-report` | off | full skipped-constraint breakdown |
| `--rule-report` | off | the rule-count tables in this README |

`--shacl`, `--shacl-output` and `--python-stubs-output` disable that stage when passed last
with no value.

### Makefile targets

| Target | Effect |
|---|---|
| `make generate` | create output directories, then `cargo run -p cimoxide-gen` |
| `make build` / `make test` | `cargo build` / `cargo test --workspace` |
| `make all` | `generate` + `build` + `test` |
| `make python-dev` / `make python-build` | `maturin develop` / `maturin build --release` in `cimoxide-py` |
| `make clean` | `cargo clean` and remove generated files |

### Codegen stability tests

`cimgen/tests/codegen.rs` regenerates into `target/` and compares hashes:
`cimmodel_codegen_stable`, `nc_classes_codegen_stable`, `cgmes_shapes_codegen_stable`,
`nc_shapes_codegen_stable`. After an intentional schema or generator change:

1. `cargo test -p cimoxide-gen --test codegen -- --nocapture` — the new hash is printed
2. Copy it into the matching `assert_eq!`
3. `cargo run -p cimoxide-gen`
4. Commit hashes and generated files together
