# cimoxide

Rust tooling for CGMES CIM data: struct generation, SHACL validation, decoding, and protobuf conversion.

## Usage

### `cimoxide-cli`

```bash
# Decode RDF/XML and print element counts (--json for machine-readable output)
cimoxide-cli import [--json] <xml-files...>

# Convert RDF/XML -> JSON
cimoxide-cli convert --to json <xml-files...> [--out <output.json>]

# Convert JSON -> RDF/XML
cimoxide-cli convert --to xml <input.json> [--out <output.xml>]

# Convert JSON -> RDF/XML, split per CGMES profile into a directory
cimoxide-cli convert --to xml <input.json> --profile EQ,SSH [--out <dir/>]

# Run SHACL + SPARQL validation
cimoxide-cli validate [--profiles EQ,SSH,...] [--solved] [--not-solved]
                      [--common] [--quality] [--silence rule1,rule2]
                      [--format json|text] <xml-files...>

# Run a SPARQL 1.1 query over the merged input files
cimoxide-cli query --query "SELECT ..." | --file <query.rq>
                   [--format text|json|csv|tsv] <xml-files...>
```

`validate` flags:

| Flag | Effect |
|---|---|
| `--profiles EQ,SSH,...` | override the auto-detected profile list |
| `--solved` / `--not-solved` | force the solved/unsolved variant of cross-checks |
| `--common` | enable cross-profile common checks |
| `--quality` | enable the 14 CIMdesk quality checks (see "CIMdesk quality checks" below) |
| `--silence rule1,rule2` | suppress specific `rule_id`s from the output |
| `--format json\|text` | output format (default `text`); in `text` mode, exits with status `2` if any violation is found |

## Repository layout

| Crate | Description |
|---|---|
| `cimgen` | Code generator — reads ENTSO-E RDF/SHACL schemas and emits Rust source |
| `cimmodel` | The data model: **generated** typed structs for every CGMES class and the NC class table (see "Profile families" below); the hand-written decoder (`decode.rs`, `CimDataset`), which reads CGMES and NC RDF/XML and resolves classes by XML namespace; and the hand-written JSON/RDF-XML conversion (`convert.rs`: `dataset_to_json`, `dataset_to_xml`, `dataset_to_xml_for_profile`) |
| `cimschema` | RDFS and SHACL parser, shared by `cimgen` at build time and the runtime loaders |
| `cimvalidation` | SHACL validation for both families through one interpreter (`bag.rs`) over **generated** shape tables (`src/cgmes_shapes.rs`, `src/nc_shapes.rs`); `src/sparql/` is hand-written (the `sh:sparql` constraints, see "SHACL Validation" below) |
| `cimsparql` | SPARQL 1.1 querying over a decoded dataset, backed by an in-memory oxigraph store (see "SPARQL" below) |
| `cimoxide` | Facade re-exporting `cimmodel` (as `model`), `cimvalidation` and `cimsparql` under one dependency |
| `cimoxide-cli` | `cimoxide-cli` binary — import/convert/validate over the command line |
| `cimoxide-py` | Python bindings (PyO3) exposing decode/convert/validate as a `cimoxide` package; built with `maturin`, excluded from the Cargo workspace |

In `cimmodel/src/`, everything but `base.rs`, `schema_source.rs`, `decode.rs` and
`convert.rs` is generated — do not hand-edit it. The decoder and converter used to be the
crates `cimoxide-decoder` and `cimoxide-convert`, and the model `cimoxide-structs`; those
stay on crates.io at 0.3.x. In `cimvalidation/`, only `cgmes_shapes.rs`, `nc_shapes.rs` and `nc_profiles.rs` are generated; everything else
(`src/sparql/`, `bag.rs`, `shapes.rs`, `helpers.rs`, `violation.rs`, `detect.rs`, `lib.rs`)
is hand-written.


## Profile families

cimoxide reads two ENTSO-E schema trees from `application-profiles-library/`:

| Family | Schemas | Representation | Status |
|---|---|---|---|
| CGMES | `CGMES/RDFS` | 446 typed structs | decode, validate, convert, query |
| NC (Network Codes) | `NCP/RDFS` | 596-row class table + property bags | decode only |

They overlap on 164 class names — `Terminal`, `Equipment`, `ACLineSegment`,
`Substation` and more — and these are *not* duplicates. NCP's re-declared CIM
classes carry 55 attributes CGMES does not have at all, such as
`Equipment.networkAnalysisEnabled` and `Contingency.mustStudy`. NC's `Equipment`
is a different type from CGMES's.

Only the XML namespace distinguishes them, because both families spell the
prefix `cim`:

```xml
<!-- CGMES -->                            <!-- NC -->
xmlns:cim="http://iec.ch/TC57/CIM100#"    xmlns:cim="https://cim.ucaiug.io/ns#"
```

So the decoder resolves `(namespace, local name)`, falling back to the bare
local name when a document binds a namespace it does not recognise — which real
files do, and which is how every release before this behaved.

### Working with NC data

NC elements are property bags rather than typed structs: adding a second family
as codegen would have meant ~700 more struct files and a naming rule to break
ties that the namespace already breaks. They key off a `nc:` prefix:

```rust
let ds = CimDataset::decode_file(Path::new("contingencies.xml"))?;
for mrid in &ds.by_type["nc:OrdinaryContingency"] {
    let c = ds.entries[mrid].element.as_any()
        .downcast_ref::<cimmodel::base::GenericElement>().unwrap();
    println!("{} {:?}", mrid, c.get_str("IdentifiedObject.name"));
}
```

`:` cannot occur in a CIM class name, so `by_type["Equipment"]` (CGMES) and
`by_type["nc:Equipment"]` never collide, and existing CGMES code is unaffected.

### Loading the class table from RDFS

The NC class table is generated into the binary, but `cimcli` and the Python
bindings can load it from the ENTSO-E RDFS files at runtime instead, so a new
profile version is a data load rather than a recompile:

```bash
CIMOXIDE_RDFS_DIR=application-profiles-library/NCP/RDFS cimcli import model.xml
```

Resolution is explicit `cimmodel::schema_source::load_from` > `CIMOXIDE_RDFS_DIR`
> the generated table. A directory that is missing or fails to parse warns,
naming the glob it tried, and falls back to the generated table. In library
crates this lives behind the `dynamic-schema` feature, off by default, so
nothing that merely wants the types pulls in an XML parser.

**What it costs** (18 files, 3.65 MB of RDFS; `scripts/bench_schema_source.sh`):

| | Generated | From RDFS | Delta |
|---|---|---|---|
| Startup, per process | 7 ms | 32 ms | **+25 ms** |
| Decode, 50k NC elements | 87.4 ms | 90.4 ms | **+3.5%** |
| Peak RSS | 7.2 MB | 13.5 MB | +6.2 MB |
| Binary size | — | — | unchanged |

The startup cost splits into 23 ms of XML parsing and ~8 ms of building and
interning the table. It is paid once per process, so it is noise for a
long-running service and dominates a one-shot CLI run on a small file.

The 3.5% decode cost was not expected — both paths do identical hash lookups
once the table exists. The likely cause is memory layout rather than extra
work: the generated table's strings sit contiguously in rodata, while interned
ones are scattered heap allocations. That explanation is inferred from the
shape of the change, not profiled.


### NC validation: a shape table and an interpreter

CGMES validation used to be generated code — 260,900 lines, one function per
check, each downcasting to a concrete struct and reading a typed field. NC
classes are property bags, so there was no struct to downcast to and nothing for
that strategy to reference. Its shapes became a data table instead, interpreted
at run time: 1,973 shapes and 14,842 checks in a 2.4 MB generated table. CGMES
has since moved to the same table and interpreter (see "SHACL Validation").

**The bag checks more than the generated path can.** `sh:datatype` and
`sh:nodeKind` are tautologies against an `f64` and real checks against a
`FieldValue::Text` — 2,747 NCP constraints that used to be discarded as
type-system guarantees. `sh:closed` (823 shapes, "this property is not in the
profile") cannot be expressed against generated structs at all, because unknown
properties are dropped at decode; a bag still has them.

Profiles come from the schema rather than from code: `NCP/SHACL/Validation/`
names which constraint files each of the 18 profiles uses, and `NCP/PROF/` maps
a dataset's `dcterms:conformsTo` IRI to a short code. NC announces itself with a
DCAT header (`dcat:Dataset`), not CGMES's `md:FullModel` — with no header, no NC
profile is detected and nothing runs.

NC leans on advisory severity far more than CGMES: 842 `sh:Info` occurrences
against 7. `cimcli validate` therefore reports `sh:Info` findings but does not
fail on them.

```bash
cimcli validate model.xml                                    # generated tables
CIMOXIDE_SHACL_DIR=application-profiles-library/NCP/SHACL \
  cimcli validate model.xml                                  # NC from SHACL
CIMOXIDE_SHACL_DIR=application-profiles-library/NCP/SHACL:application-profiles-library/CGMES/SHACL \
  cimcli validate model.xml                                  # both from SHACL
```

`CIMOXIDE_SHACL_DIR` takes one directory or several, separated as in `PATH`; each
serves the family whose files it holds (NC: a `Validation/` subdirectory; CGMES: the
constraint files its manifest names). A directory holding neither is reported and
ignored. CGMES classes are compiled structs, so a CGMES table loaded from another release
can name classes this build does not decode; shapes on those match nothing. Loading the
CGMES table adds about 165 ms per process.

**What it costs** (32 files, 4.55 MB of Turtle; `scripts/bench_shape_source.sh`;
20,000 synthetic CO elements producing 45,000 violations):

| | Generated | From SHACL | Delta |
|---|---|---|---|
| Startup, per process | 8 ms | 171 ms | **+163 ms** |
| Validate, CO profile | 40.8 ms | 40.8 ms | none (p = 0.83) |
| Validate, all 18 profiles | 290 ms | 296 ms | +2.0% |
| Peak RSS | 11.4 MB | 62.7 MB | +51 MB |
| Binary size | — | — | unchanged |

The startup cost splits into 54 ms of Turtle parsing, 22 ms of resolving the
shapes against the class table, and the rest re-importing both families' RDFS —
the shapes have to agree with the classes they constrain, so the dynamic path
pays for that import too. At 192 ms for the load it is far more expensive than
the class table's 25 ms, and it dominates any one-shot run.

Steady-state throughput is flat, which is what the identical tables predict. The
decoder's equivalent measurement showing +3.5% remains the odd one out.

**Indexing `sh:targetSubjectsOf` mattered more than anything else here.**
Resolving those targets by scanning the dataset per shape cost 10.8 ms on a
profile whose shapes matched *nothing*, because the scan happens before anything
can be ruled out. One pass over the fields any active shape asks about took that
to 6.3 ms, the CO profile from 47.4 to 40.8 ms, and all 18 profiles from 469 to
290 ms. Measured twice, in independent A/Bs, agreeing to within a point.

CGMES now validates the same way: its shapes are a table (`cgmes_shapes.rs`, 849 shapes,
18,681 checks) run by the same interpreter, replacing ~250,000 lines of generated per-check
functions. On RealGrid (189,000 elements) `cimcli validate` went from 2,304 ms to about
1,370 ms while checking more (1,435 ms with `--common --quality`), and `cimoxide-validation`
compiles in 9.5 s instead of 123.5 s. The interpreter walks element-major — each target element once, with every shape
that targets its class — because reaching an element's fields is the cost of a check, not
the comparison itself.

A note on method, since this bench is easy to misread: criterion baselines give
rigour *within* a run, but the two halves of a generated-vs-loaded comparison
are separate processes, and cross-run drift on this machine has been measured at
20% for byte-identical code. Any difference under about 5% needs a second
independent A/B before it means anything.

Not covered for NC, and reported as skips rather than dropped silently: the 35
`sh:sparql` constraints, and 119 `cim16:`/`cim17:` target classes, which are NC
shapes on CGMES classes whose NC attributes the decoder discards.

DatasetMetadata's material implications are checked —
`sh:or ( [ sh:not dm:conformsToNCProfile ] [ sh:path P ; sh:minCount 1 ] )`, "a dataset
declaring an NC profile must carry P", seven of them — through negated logical branches
and `sh:qualifiedValueShape`. One more logical shape, in
`RemedialActionSchedule-AP-Con-Complex-SHACL.ttl`, resolves too but never runs: only the
combined "ALL" manifest imports that file, not the RemedialActionSchedule profile's own.

Still not supported for NC: RDF/XML encoding and SPARQL. The encoder skips NC
elements rather than emit a malformed document.

## Setup

Clone with submodules (the ENTSO-E RDF schema and SHACL files are required):

```bash
git clone --recurse-submodules <url>
# or after a plain clone:
git submodule update --init --recursive
```

## Commands

```bash
# Build everything
cargo build -v

# Run all tests (requires submodules)
cargo test

# Regenerate cimmodel and cimvalidation from schema files
cargo run -p cimoxide-gen
```

### `cimgen` flags

All flags are optional; omitting them uses the defaults below. Pass extra args after `--`,
e.g. `cargo run -p cimoxide-gen -- --verbose --rule-report`.

| Flag | Default | Effect |
|---|---|---|
| `--schema <glob>` | `application-profiles-library/CGMES/RDFS/61970-600-2_*-AP-Voc-RDFS2020.rdf` | RDF/RDFS schema files to import |
| `--output <dir>` | `cimmodel/src` | struct output directory |
| `--shacl <glob>` | `application-profiles-library/CGMES/SHACL/*.ttl` | SHACL TTL files to import |
| `--shacl-output <dir>` | `cimvalidation/src` | shape table output directory (`cgmes_shapes.rs`, `nc_shapes.rs`, `nc_profiles.rs`) |
| `--python-stubs-output <dir>` | `cimoxide-py/python/cimoxide` | `.pyi` type stub output directory |
| `--verbose` / `-v` | off | print parse diagnostics |
| `--skip-report` | off | print the full per-entry + global skipped-constraint breakdown (see "Skipped constraints" below) |
| `--rule-report` | off | print the rule-count tables used in this README (see "SHACL Validation" below) |

`--shacl`, `--shacl-output`, and `--python-stubs-output` disable that generation stage
entirely if passed as the very last argument with no value following.

### Makefile targets

| Target | Effect |
|---|---|
| `make generate` | create generated-output directories if missing, then `cargo run -p cimoxide-gen` |
| `make build` | `cargo build --workspace` |
| `make test` | `cargo test --workspace` |
| `make all` | `generate` + `build` + `test` |
| `make python-dev` | `maturin develop --release` in `cimoxide-py` (local editable install) |
| `make python-build` | `maturin build --release` in `cimoxide-py` (build a distributable wheel) |
| `make clean` | `cargo clean`, plus remove generated `cimmodel`/`cimvalidation` files (keeps the hand-written ones) |

## Benchmarks

```bash
# Run all benchmarks
cargo bench -p cimoxide-model

# Run a specific group (e.g. the import benchmark)
cargo bench -p cimoxide-model --bench real_grid -- import
```

Benchmarks are in `cimmodel/benches/real_grid.rs` and use the RealGrid test dataset
from the `CGMES-Test-Configurations` submodule (must be initialised). The `import`
group measures `decode_files_parallel` with real file paths, matching what
`cimoxide-cli import` does. Add debug symbols to get useful flamegraphs:

```toml
# Cargo.toml
[profile.bench]
debug = true
```

## Codegen stability tests

`cimgen/tests/codegen.rs` contains four hash-based tests that detect unintended drift in
the generated output:

- `cimmodel_codegen_stable` — runs the RDF struct generator and hashes the generated part of `cimmodel/src/`
- `nc_classes_codegen_stable` — hashes the NC class table on its own
- `cgmes_shapes_codegen_stable` — hashes the CGMES shape table (`cimvalidation/src/cgmes_shapes.rs`)
- `nc_shapes_codegen_stable` — hashes the NC shape table (`cimvalidation/src/nc_shapes.rs`)

Each test regenerates into a temporary directory under `target/` and compares the
directory hash against a stored expected value. A mismatch means either the generator
logic or the schema files changed and the checked-in generated code needs to be updated.

### Bootstrapping the hashes (first run)

After the initial generation, or after any intentional schema/generator change, update
the stored hashes:

1. Run the tests and let them fail — the actual hash is printed in the failure message:
   ```bash
   cargo test -p cimoxide-gen --test codegen -- --nocapture
   ```
2. Copy the printed hash into the corresponding `assert_eq!` in `cimgen/tests/codegen.rs`.
3. Regenerate the checked-in files so they match:
   ```bash
   cargo run -p cimoxide-gen
   ```
4. Commit both the updated test hashes and the regenerated source files together.

## SPARQL

`cimsparql` materialises a decoded `CimDataset` into an in-memory RDF graph and runs SPARQL
1.1 over it — SELECT, ASK, CONSTRUCT and DESCRIBE, including joins across profiles.

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

From the command line and from Python:

```bash
cimoxide-cli query --format csv \
  --query 'SELECT ?name ?p WHERE { ?s a cim:EnergyConsumer ;
                                      cim:IdentifiedObject.name ?name ;
                                      cim:EnergyConsumer.p ?p }' \
  FullGrid_EQ.xml FullGrid_SSH.xml
```

```python
rows = dataset.query("SELECT ?s WHERE { ?s a cim:ACLineSegment }")
```

The CGMES namespaces (`cim:`, `eu:`, `md:`, `dm:`, `rdf:`) and `xsd:` are pre-bound via
oxigraph's parser, so queries need no prologue and a `sh:select` body from the SHACL files
can be pasted in unchanged. A query that declares a prefix itself wins. `query_raw` binds
nothing.

### Engine

`oxigraph` is pulled in with `default-features = false`. That is load-bearing: oxigraph's
default feature is `rocksdb`, which requires `oxrocksdb-sys` and a C++ toolchain. Disabled,
the dependency tree is pure Rust and `Store::new()` is the in-memory store.

`cimsparql` is a separate crate rather than a feature on `cimmodel`, so none of the crates
`gridoxide` consumes as a git dependency gain an optional-dependency edge. `cimoxide-cli` and
`cimoxide-py` depend on it behind a `sparql` feature that is **on by default**; build with
`--no-default-features` to drop it.

### How decoded data becomes RDF

The decoder is namespace-blind by construction: `local_name()` drops the XML prefix and
`strip_fragment()` drops the IRI base, so `RdfBlock.fields` keys are bare
`IdentifiedObject.name` strings and every value is an untyped `FieldValue::Text`. Two tables
generated into `cimmodel/src/profile_meta.rs` put that back:

| Table | Contents |
|---|---|
| `TYPE_NS` | `(type_name, namespace)` — the RDF namespace of each CIM class |
| `ATTR_RDF` | `(attr_id, namespace, range, kind)` keyed by `RdfBlock.fields` key; `kind` is literal / association / enum, and `range` is the XSD datatype or the enum namespace |

This is what keeps `eu:` attributes out of the `cim:` namespace — a single `eu:BoundaryPoint`
carries both `eu:BoundaryPoint.toEndName` and `cim:IdentifiedObject.description`, and they
must land in different namespaces. The converter's XML writer (`cimmodel::convert`) reads the same two tables on the
way out (see ["RDF/XML export"](#rdfxml-export) below).

Other mapping rules:

- **Subjects.** `_bd8b27aa-…` (from `rdf:ID`) and `urn:uuid:2b6b90c0-…` (from `rdf:about`)
  both normalise onto `urn:uuid:`, so `iri_to_mrid` is a pure inverse and no side table is
  held. Non-UUID identifiers fall back to `http://cimoxide/id/`.
- **Literals** are typed from `ATTR_RDF`. CGMES `Float` becomes `xsd:double`, not
  `xsd:float`: the decoder parses into `f64`, and claiming single precision would make
  `FILTER(?r > 0.1)` disagree with the value that was decoded. Without typing, that filter
  would compare lexically.
- **Enum values** arrive fragment-stripped (`UnitSymbol.W`) and are rebuilt against the
  enum's namespace. Two `eu:` enumerations — `LimitKind` and `SVCControlMode` — are
  generated as marker structs rather than enums, because `cims:stereotype` parsing is
  last-write-wins and their `European` stereotype overwrites the `enumeration` one; their
  values are recovered from `TYPE_NS` instead.
- **Fields** come from `entry.block` where it is populated, which is the lossless source and
  retains predicates the typed struct's catch-all arm discarded. After `drop_blocks()`,
  materialisation falls back to `CimElement::to_block()` and the graph becomes a subset.
  `CimStore::stats()` reports how many elements took that path.
- **Graphs.** Everything goes in the default graph. Per-profile named graphs are not
  possible: `CimEntry` records no source-file provenance.

### Cost

The graph is held *in addition to* the typed structs and `RdfBlock`s it was built from, so
materialisation roughly doubles memory for a dataset.

The merged RealGrid configuration (117 MB of RDF/XML) decodes to **188,551 elements** and
materialises to **1,302,191 quads**. From `cargo bench -p cimoxide-sparql`:

| | Time | Rate |
|---|---:|---:|
| `quads_only` — build the quads, no store | 804 ms | 1.62 M quads/s |
| `into_store` — build, intern and index them | 2.67 s | 488 K quads/s |
| `query/count_by_type` — `COUNT` over one `rdf:type` | 810 µs | |
| `query/terminal_join` — two-hop join, the shape most CGMES conformance queries take | 80 ms | |

Peak RSS for the same dataset, measured on `cimoxide-cli` release builds:

| Command | Peak RSS | Wall |
|---|---:|---:|
| `cimcli import` (decode only) | 559 MB | 1.24 s |
| `cimcli query` (decode + materialise + `COUNT(*)`) | 1067 MB | 3.94 s |

So the graph costs roughly what the decoded dataset already costs — about +510 MB and +2.7 s
here, both consistent with the `into_store` figure above.

Two levers if that matters: `GraphOptions::with_types([...])` restricts materialisation to
the classes a query touches, and calling `CimDataset::drop_blocks()` first frees the raw
field maps at the cost of the predicates the typed structs do not model.

## RDF/XML export

`cimmodel::convert` writes CGMES RDF/XML, either as a flat dump (`dataset_to_xml`) or split per
profile (`dataset_to_xml_for_profile`, `cimcli convert --to xml --profile EQ,SSH`).

### Namespaces

Element and field prefixes both come from the generated `TYPE_NS` / `ATTR_RDF` tables (see
["How decoded data becomes RDF"](#how-decoded-data-becomes-rdf) above), never from a
hardcoded prefix. The prefix is decided **per field**, never inherited from the owning class,
because CGMES freely mixes them on one element:

```xml
<eu:BoundaryPoint rdf:ID="_0c26dd2f-…">
  <eu:BoundaryPoint.toEndName>Maribor</eu:BoundaryPoint.toEndName>
  <cim:IdentifiedObject.description>RG CE; 400 kV; OHL (2)</cim:IdentifiedObject.description>
  <eu:IdentifiedObject.energyIdentCodeEic>10T-AT-SI-00001T</eu:IdentifiedObject.energyIdentCodeEic>
  <cim:IdentifiedObject.mRID>0c26dd2f-…</cim:IdentifiedObject.mRID>
</eu:BoundaryPoint>
```

Two `IdentifiedObject.*` fields on the same element, in two different namespaces. Enum values
are written as absolute IRIs (`rdf:resource="http://iec.ch/TC57/CIM100#UnitSymbol.W"`), as
real CGMES files do, while MRID references stay local fragments (`rdf:resource="#…"`).

Re-encoding `FullGrid_EQ.xml` reproduces the source file's prefix distribution exactly — 732
`cim:` / 2 `eu:` / 1 `md:` elements, 5303 `cim:` / 1401 `eu:` / 7 `md:` fields, and all 247
absolute enum references.

### Which elements and fields go in which profile

Purely from the schema, so an attribute set on an object *after* import is routed exactly
like a decoded one:

- an element is written to profile P iff `TYPE_ORIGINS` lists P for its class;
- it is a definition (`rdf:ID`) if P is its class's dominant origin, otherwise a reference
  (`rdf:about="#…"`) carrying only the fields P itself owns.

That second rule needs a profile to be the dominant origin of *something*. **EQBD is the
dominant origin of no type and no attribute** — the RDFS declares its classes and attributes
identically in the Equipment profile, so EQ always outranks it — which made the rule select
nothing and exported every boundary file as a bare header. A profile in that position is
treated as self-defining and falls back to plain profile membership, which is all the schema
actually asserts. The condition is computed from `ATTR_ORIGINS`, not hardcoded:

| profile | dominant for #attrs | behaviour |
|---|---:|---|
| DY, EQ, SC, OP, GL, SV, DL, FH | 2783 … 14 | definitions and references as above |
| SSH, TP | 64, 8 | dominant for no *type* — emit references, as the real files do |
| EQBD | 0 | self-defining: `rdf:ID` plus every member field |

This matches what ENTSO-E ships: `FullGrid_EQBD.xml` is 27 `rdf:ID` elements and no
`rdf:about`, each carrying its name and mRID.

Known limitation: exporting a *merged* dataset to EQBD emits every boundary-class instance it
holds, not just those a boundary file would carry. The EQ/EQBD split is per-instance —
`BaseVoltage`, `ConnectivityNode` and `Line` all appear legitimately in both real files — and
no class-level table can express that. Round-tripping a single EQBD file is exact.

## SHACL Validation

`cimgen` resolves the constraints in the CGMES SHACL Turtle files into a shape table,
`cimvalidation/src/cgmes_shapes.rs`, which `cimvalidation/src/bag.rs` interprets against the
decoded dataset — the same interpreter and table format the NC family uses. Which file
applies to which profile, and when (not solved, cross-profile, header), is written out in
`cimschema/src/shacl/cgmes_manifest.rs`, since CGMES ships no per-profile manifests.

Targets follow SHACL: an abstract `sh:targetClass` applies to all its concrete subclasses.
One consequence: the SSH rule `IdentifiedObject.mRID-cardinality` targets `cim:Equipment`
and now applies to every piece of equipment in an SSH file. Most SSH files in the ENTSO-E
test configurations leave `IdentifiedObject.mRID` out (18,761 findings on RealGrid), while the
SSH vocabulary and SHACL require it exactly once. cimoxide reports it as written. Note that
its rule id, `ido:IdentifiedObject.mRID-cardinality`, is shared by every profile, so
`--silence` would hide a missing mRID in EQ as well.

A finding is reported once even when a property shape reaches an element through more
than one node shape. `sh:datatype` and `sh:nodeKind` are checked: the interpreter reads the
text as written, so a malformed number or a literal where a reference belongs is reported
rather than silently dropped by typed decoding. The IdentifiedObject string-length rules
(`61970-600-2_IdentifiedObjectCommon_AP-Con-Complex-SHACL.ttl`) run from the table under
`--common`.

The 186 SPARQL constraints (all currently implemented, see "SPARQL Check Coverage" below)
are not in the table but implemented as hand-written Rust functions in
`cimvalidation/src/sparql/`.

### Simplification rules applied during import

Before resolution, `cimgen` normalises each property shape's constraint list. Rules
are applied in order; a constraint that matches a rule is either dropped or rewritten and
does not reach the shape table.

*The simplifications applied here do not necessarily work for other validation engines and
are specific to this tool.*

Rules 1, 2 and 7 used to drop `sh:nodeKind` and `sh:datatype` as guaranteed by typed struct
fields; validation now reads the text as written, so both are kept.

| Rule | Constraint removed / rewritten | Reason |
|------|-------------------------------|--------|
| 3 | `sh:minCount 0` | Vacuously true — zero or more values are always acceptable. |
| 4 | `sh:minCount 0` + `sh:maxCount 1` → keep `sh:maxCount 1`, drop `sh:minCount 0` | `sh:minCount 0` is vacuously true (Rule 3); the upper bound is preserved for a duplicate-occurrence check. |
| 5 | `sh:minCount 1` + `sh:maxCount 1` → synthetic `sh:Required` | The pair means "exactly 1 value". Both constraints are collapsed into a single `Required` sentinel, avoiding duplicate presence checks. |
| 6 | `sh:in` with a single value → rewritten as `sh:hasValue` | A one-element allow-list is semantically identical to an exact-value check. |

### Skipped constraints

Some constraints are not in the shape table. The table below summarises the categories and
counts as reported by `cargo run -p cimoxide-gen -- --skip-report`. Its total (196) matches
the "SHACL Rules by Profile" table's `Skipped` column below exactly, since both sum the same
`skip::SkipCollector`-deduped entries — just grouped differently (by reason here, by CGMES
profile group there). Every category is either handled by an alternative method
(hand-written functions), a defect in the upstream ENTSO-E TTL files that only ENTSO-E can
fix, or not observable after decoding.

| Count | Category | Reason |
|------:|---------|--------|
| 178 | SPARQL-derived constraints | `sh:sparql` constraints and `sh:target SPARQLTarget` targets both require evaluating an arbitrary SPARQL query at runtime, so both have a hand-written implementation instead (see "SPARQL Check Coverage" below). `cimsparql` provides an evaluator (see ["SPARQL"](#sparql) above), but validation is deliberately not wired to it. This is not the same count as the 186 in "SPARQL Check Coverage": this one is every distinct `(property, component, sh:name)` skip entry, deduped per TTL file and not split on `sh:name`'s `\|`-joined compound values. |
| 2 | Instance count of a class | `sh:targetNode cim:X` with `[ sh:inversePath rdf:type ]` counts the instances of class X: the focus node is the class itself, which the table cannot express. Both such rules, `eq600:GeographicalRegion-EQ__4` and `sv456:TopologicalIsland-instance`, are hand-written, as is `all600:All-HGEN2`, which counts file headers the same way. |
| 6 | Target class not defined by the schema | Upstream defects: `cim:GovHydroIEEE1` (no such class), `cim:TextDiagramObjectDiagramObject` (two names run together), and node shapes whose "target class" is a rule label rather than a CIM class (`cim:AngleReference`, `cim:AllGeneratingUnit`, `cim:SubstationCount`, `cim:FloatSpecialValues`/`IDuniqueness`/`IDchecks`) — the latter all back `sh:sparql` rules that are hand-written. |
| 3 | Value list or class that does not resolve | Upstream defects: `cim:CSConverter` (should be `cim:CsConverter`) in two `CSCDynamics.CSConverter-valueType` shapes, and the empty `sh:in ()` on `Measurement.Terminal-valueType`. |
| 7 | `sh:nodeKind` on a compound-datatype or `rdf:type` path | `sh:nodeKind sh:BlankNode` on GL's compound `Location.mainAddress` paths and on difference-model header paths: blank-node-ness is not visible after decoding. The class half of each GL rule (`sh:in ( cim:Status )` on the `rdf:type` step, etc.) is checked. |
| **196** | **Total** | |

#### Comparing with cimgo

cimoxide and cimgo both cover the same 74 TTL files at the same `application-profiles-library`
commit (`d8b2d21`), and both report 12,269 non-`sh:sparql` constraints — though not by the
same route: cimgo's `shaclimport` does not recognise `sh:length` and never sees the
`IdentifiedObject.energyIdentCodeEic` shape, while cimoxide's per-pattern counting differs
in deduplication. Compare per file with the `PERFILE` lines (see below). The skip
*categories* are not comparable row by row: cimgo generates per-check code, and its skips
reflect that generator's limits rather than the shape table's.

### Upstream SHACL TTL defects

**Field name typos** — `sh:lessThan` references a misspelled field name; all four are in `61970-302_Dynamics-AP-Con-Complex-SHACL.ttl`:

| Rule (`sh:name`) | Defect |
|------------------|--------|
| `C:302:DY:GovHydroIEEE0.pmin:valueRangePair` | `sh:lessThan cim:GovHydroIEEE.pmax` — class suffix `0` missing; should be `GovHydroIEEE0.pmax` |
| `C:302:DY:PFVArType1IEEEVArController.vvtmin:valueRangePair` | `sh:lessThan cim:PVFArType1IEEEVArController.vvtmax` — prefix transposed; should be `PFVArType1IEEEVArController.vvtmax` |
| `C:302:DY:ExcDC1A.efdmin:valueRangePair` | `sh:lessThan cim:ExcDC1A.edfmax` — letters transposed; should be `efdmax` |
| `C:302:DY:PssIEEE4B.vhmin:valueRangePair` | `sh:lessThan cim:PssIEEE4V.vhmax` — class suffix wrong; should be `PssIEEE4B.vhmax` |

**Class name typo** — one property shape in `61970-600-2_Dynamics-AP-Con-Complex-InverseAssociation-SHACL.ttl` references a misspelled class in its inverse path (reported as one entry covering all 4 concrete target classes):

| Rule (`sh:name`) | Defect |
|------------------|--------|
| `SynchronousMachineDynamics.CrossCompoundTurbineGovernorDyanmics-cardinality` | `sh:inversePath cim:CrossCompoundTurbineGovernorDyanmics.SynchronousMachineDynamics` — "Dynamics" misspelled as "Dyanmics"; applied to `SynchronousMachineEquivalentCircuit`, `SynchronousMachineSimplified`, `SynchronousMachineTimeConstantReactance`, `SynchronousMachineUserDefined` |

**Class name capitalisation mismatch** — two SHACL files (`61970-457_Dynamics-AP-Con-Complex-Explicit-CrossProfile-SHACL.ttl` and `61970-457_Dynamics-AP-Con-Complex-Implicit-CrossProfile-SHACL.ttl`) reference `cim:CSConverter` (capital S), but all RDFS schema files consistently define the class as `cim:CsConverter`:

| Shape | Defect |
|-------|--------|
| `dy457cpe:CSCDynamics.CSConverter-valueType` | `sh:in (cim:CSConverter)` — should be `cim:CsConverter` |
| `dy457cpi:CSCDynamics.CSConverter-valueType` | same defect in the implicit cross-profile file |

**Wrong field names in inverse paths** — four property shapes in `61970-600-2_Dynamics-AP-Con-Complex-InverseAssociation-SHACL.ttl` reference field names that do not match the RDFS schema:

| Rule (`sh:name`) | Defect |
|------------------|--------|
| `SynchronousMachineDynamics.CrossCompoundTurbineGovernorDynamics-cardinality` | `sh:inversePath cim:CrossCompoundTurbineGovernorDynamics.SynchronousMachineDynamics` — no such field; RDFS defines `HighPressureSynchronousMachineDynamics` and `LowPressureSynchronousMachineDynamics`; applies to 4 concrete target classes |
| `CsConverter.CSCDynamics-cardinality` | `sh:inversePath cim:CSCDynamics.CsConverter` — capitalisation wrong; RDFS defines `CSCDynamics.CSConverter` |
| `VCompIEEEType2.GenICompensationForGenJ-cardinality` | `sh:inversePath cim:GenICompensationForGenJ.VCompIEEEType2` — capitalisation wrong; RDFS defines `GenICompensationForGenJ.VcompIEEEType2` |
| `WindContQIEC.WindTurbineType3or4IEC-cardinality` | `sh:inversePath cim:WindTurbineType3or4IEC.WindContQIEC` — capitalisation wrong; RDFS defines `WindTurbineType3or4IEC.WIndContQIEC` |

**Stale field reference** — one property shape in `61970-301_Operation-AP-Con-Complex-SHACL.ttl` references a field removed from the CGMES 3.0 schema:

| Rule (`sh:name`) | Defect |
|------------------|--------|
| `C:301:OP:AccumulatorValue.value:valueRange` | `sh:minExclusive` on `cim:AccumulatorValue.value` — field removed in CGMES 3.0 |

**Non-existent target class** — one property shape in `61970-600-2_Dynamics-AP-Con-Complex-InverseAssociation-SHACL.ttl` lists `cim:GovHydroIEEE1` in its `sh:targetClass` alongside several real classes. No such class exists in the CIM standard or in `cimmodel`; `cimgen` silently skips it when resolving concrete target classes.

**Empty `sh:in` list** — one property shape in `61970-600-2_Operation-AP-Con-Simple-SHACL.ttl` has `sh:in ()` (reported as one entry covering all 4 concrete target classes):

| Rule (`sh:name`) | Defect |
|------------------|--------|
| `Measurement.Terminal-valueType` | `sh:in ()` — empty allow-list; applied to `Accumulator`, `Analog`, `Discrete`, `StringMeasurement` |

### SHACL Rules by Profile

The skipped-constraint counts above are global totals across all 74 TTL files.
`cargo run -p cimoxide-gen -- --rule-report` also breaks the checked-vs-skipped split down by
CGMES profile group, using the same file-to-group classification as the SPARQL Check Coverage
table below (`ttl_group_label` in `cimgen/src/shacl/ttl_report.rs`). "Checks" counts
distinct `(path, component, sh:name)` rule patterns the shape table holds for that group: a
property shape shared by several node shapes, or applied to several classes, counts once,
matching how `skip::SkipCollector` dedups the "Skipped" side. "Total" is their sum — the
number of non-`sh:sparql` SHACL constraints CGMES defines for that profile group,
independent of either tool's capability. Every TTL file is counted, whether or not the
manifest runs it.

The table checks 98.4% of these constraints; the rest are the 196 skips above.

`--rule-report` also prints a per-file breakdown ("=== Per-File Rule Counts ===", one
`PERFILE\t<name>\t<checks>\t<skipped>\t<total>` line per TTL file) in the same format cimgo's
`-rule-report` uses, so a profile-group mismatch between the two tools can be localized to a
specific file with no external script: `grep PERFILE cimoxide.log | sort > a; grep PERFILE
cimgo.log | sort > b; awk -F'\t' '{print $2, $5}' a | diff - <(awk -F'\t' '{print $2, $5}' b)`.

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

Complex constraints defined using `sh:sparql` in the CGMES SHACL files are not in the
shape table. These are instead implemented as hand-written Rust
functions in `cimvalidation/src/sparql/` and wired into the profile validators.

The `cimsparql` crate can now evaluate SPARQL (see ["SPARQL"](#sparql) above), so generating
these checks from the TTL query text is possible in principle. Validation is deliberately
left as it is: `cimgen` discards the `sh:select` text at import, `sh:SPARQLTarget` needs
target resolution before constraint evaluation, and the hand-written checks avoid
materialising a graph per validation run. `cimsparql/tests/query.rs` runs one constraint
verbatim as a starting point for measuring that trade-off — and shows the two are not always
equivalent: the SHACL query flags the attribute being *present*
(`OPTIONAL { … } FILTER(bound(?value))`), while its hand-written counterpart tests the
decoded boolean value.

Each manual validation rule has the ID and name from the source profile for traceability.
Some functions cover several SPARQL rules.

Counts below are generated by `cargo run -p cimoxide-gen -- --rule-report`, which statically
resolves the call graph in `cimvalidation/src/sparql/` from each profile group's
`validate()` entry point(s) and matches the resulting `Violation.name` values against the
`sh:name`s of `sh:sparql` constraint shapes actually defined in the CGMES SHACL TTL files —
re-run it after adding, removing, or renaming checks and update this table to match. Matching
is done on `sh:name` (the CGMES conformance rule name, e.g.
`C:452:EQ:SynchronousMachine:aggregate`) rather than the SHACL shape ID (`rule_id`): `sh:name`
is a plain string with no namespace prefix to normalize, and it's copied verbatim into
`Violation.name` on the hand-written side.

**`Implemented`/`TTL Total` count distinct named conformance rules (`sh:name` values), not
distinct `sh:sparql` shapes.** A single shape's `sh:name` can itself be a `|`-joined compound
of several rule names when one `sh:sparql` query enforces multiple documented conformance
rules at once — both sides are split on `|` before matching, so one shape contributes one
entry to the totals per rule name it names, not one entry per shape. A shape can be partially
covered: if the hand-written check only tags its `Violation.name` with some of the rule names
a shape's `sh:sparql` query is documented to enforce, the rest count as not implemented even
though the shape itself has *a* check.

This table uses the same `ttl_group_label` grouping as "SHACL Rules by Profile"
above, so their rows line up 1:1 (`C:600 conformance` (`prof10.rs`) has no row of its own: like
`Common / AllProfiles`, it's a cross-cutting rule not tied to a single profile, and is folded
in there). `C:600 conformance`'s 9 profile-specific `rule_id`s (`prof10:PROF10-EQ`, `-DY`, ...)
all share one conformance rule name, `C:600:ALL:NA:PROF10`, contributing exactly 1 to
`Common / AllProfiles`'s `Implemented` count — but since PROF10 is a plain SHACL constraint too
complex to auto-generate (not one tagged `sh:SPARQLConstraintComponent`), it has no TTL
backing, so it never affects `Common / AllProfiles`'s `TTL Total`/`Coverage` either.
`CIMdesk quality` still has no `sh:sparql` backing in the TTL files at all, so it keeps its own
row showing `n/a` for TTL Total/Coverage.

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

Every SPARQL constraint defined in the CGMES SHACL TTL files has a matching hand-written check.

### CIMdesk quality checks (`--quality`)

`cimvalidation/src/sparql/quality.rs` implements 14 modeling quality checks (14 distinct
`rule_id`s: 13 `quality:*` plus the pre-existing `eqbd2:EQBD2`) that are not encoded in the
CGMES SHACL TTL files. They are disabled by default and enabled via the `--quality` flag in
`cimoxide-cli validate`. The "CIMdesk quality" row in the SPARQL Check Coverage table above
shows 15, not 14: like every other row it counts distinct `Violation.name` values, not
`rule_id`s, and `check_no_locations_for_conductors` reuses one `rule_id`
(`quality:Conductor.noLocation`) for two differently-named checks — "No Location for
ACLineSegment" and "No Location for DCLineSegment".

| Class | Check |
|-------|-------|
| *(global)* | No `TapChangerControl`s found. |
| *(global)* | No `RegulatingControl`s found. |
| *(global)* | No `ShuntCompensator` objects found. |
| `Substation` | Instance has no child `VoltageLevel`s. |
| `ControlArea` | Instance has no child objects. |
| *(global)* | No `Location` objects associated with line segments. |
| `ACLineSegment` | `x / r` ratio is too large. |
| `BaseVoltage` | Two instances share the same `nominalVoltage`. |
| `PowerTransformer` | Both ends have the same `nominalVoltage`. |
| `ConnectivityNode` | Open-ended node with only one `Terminal` connected. |
| `Disconnector` | The two `ConnectivityNode`s it connects are in different `VoltageLevel`s. |
| `ConformLoad` | The load and its connected `TopologicalNode`s are not in the same `EquipmentContainer`. |
| `RegulatingControl` | Target voltage deviates 10–20 % from the `nominalVoltage` of the regulated node. |
| EQBD `BaseVoltage` | Base voltage MRID is not listed in the equipment boundary dataset. |
