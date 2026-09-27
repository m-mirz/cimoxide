# AGENTS.md

This file provides guidance to AI coding agents (Claude Code, Codex, Cursor and others) when working with code in this repository.

## What This Project Does

cimoxide is a Rust monorepo providing tooling for ENTSO-E CGMES (Common Information Model Exchange Standard) data used in power system modeling. It reads IEC 61970/61968 RDF/SHACL schemas, generates strongly-typed Rust structs and validation functions, and provides efficient RDF/XML deserialization.

## Common Commands

```bash
# Build all crates
cargo build

# Run all tests (submodules must be initialized)
cargo test

# Run tests for a specific crate
cargo test -p cimoxide-gen
cargo test -p cimoxide-decoder

# Run a single test by name
cargo test -p cimoxide-gen --test codegen cimstructs_codegen_stable

# Run benchmarks
cargo bench -p cimoxide-decoder

# Regenerate cimstructs and cimvalidation from schemas
cargo run -p cimoxide-gen
```

## Submodule Setup

Tests and code generation require Git submodules:

```bash
git submodule update --init --recursive
```

- `application-profiles-library/` — ENTSO-E RDF and SHACL schema files
- `CGMES-Test-Configurations/` — Real-world test datasets

## Crate Architecture

**Data flow**: RDF/SHACL schemas → `cimgen` (code generator) → `cimstructs` + `cimvalidation` (generated) ← `cimdecoder` (deserializer)

### `cimgen` — Code Generator CLI
Parses RDF schema files and SHACL TTL constraint files, then generates Rust code. Key modules:
- `schema/` — RDF schema parsing and internal model (`CimType`, `CimAttribute`, `CimDatatype`)
- `generator/rust_gen.rs` — Emits struct definitions
- `shacl/` — SHACL TTL parsing, constraint model, and validation code generation

### `cimstructs` — Generated Typed Structs (do not hand-edit)
Contains one file per CGMES class. Core files in `src/`:
- `base.rs` — hand-written. Traits `CimElement`, `RdfBlock`, `FieldValue`, `MridRef`;
  the `TypeRegistry`; and `ClassDef`/`AttrDef`/`GenericElement` for property-bag families
- `registry.rs` — `TYPE_ROWS` keyed by `(namespace, local name)`, plus a bare-name
  fallback table and the legacy `registry()`
- `constants.rs` — CGMES version constants
- `nc_classes.rs` — the NC family's 596-class table (generated)

### `cimdecoder` — RDF/XML Deserializer
Streaming XML parser that produces `CimDataset`:
- `CimDataset::decode_file(path)` / `decode_files(paths)` / `decode_str(content)` — Entry points
- `CimDataset::merge(other)` — Combine multiple datasets (re-parses conflicts)
- `CimDataset::drop_blocks()` — Free `RdfBlock` memory after final merge
- `CimDataset { entries: HashMap<mrid, CimEntry>, by_type: HashMap<type_name, Vec<mrid>> }`

### `cimvalidation` — SHACL Validators (`generated_*.rs`, `nc_shapes.rs`, `nc_profiles.rs` are generated; do not hand-edit those)
Only `src/generated_*.rs`, `src/nc_shapes.rs` and `src/nc_profiles.rs` are generated. `src/sparql/` (hand-written reimplementations of the
`sh:sparql` constraints), `helpers.rs`, `violation.rs`, `detect.rs` and `lib.rs` are
hand-written. Entry points:
- `validate_files(per_file, cfg)` — full two-phase run: per-file checks in parallel, then
  cross-profile checks on the merged dataset
- `validate_header`, `validate_profile_local(dataset, profile, cfg)`, `validate_crossprofile`
- `combined_config(...)` builds the `Config`
- Profiles: `"EQ"`, `"OP"`, `"DY"`, `"SV"`, `"SSH"`, `"SC"`, `"GL"`, `"DL"`, `"EQBD"`

### `cimsparql` — SPARQL 1.1 Querying
Materialises a `CimDataset` into an in-memory oxigraph store (`default-features = false`, so
no RocksDB/C++ toolchain) and queries it:
- `CimStore::from_dataset(&ds)` / `from_dataset_with(&ds, &GraphOptions)` / `.query(sparql)`
- `quads(&ds, &opts, &mut stats)` streams quads without a store
- Relies on the `TYPE_NS` / `ATTR_RDF` tables `cimgen` emits into `cimstructs`, which are the
  only runtime source of per-attribute IRIs — the decoder resolves an element's class by
  namespace but keeps field keys as bare `Class.attr`
- `cimoxide-cli` (`cimcli query`) and `cimoxide-py` (`dataset.query(...)`) depend on it
  behind a default-on `sparql` feature

## Profile Families

Two schema trees are read from `application-profiles-library/`: CGMES (`CGMES/RDFS`,
446 typed structs) and NC / Network Codes (`NCP/RDFS`, 596 classes as a data table
decoded into `GenericElement` property bags). A `Family` in
`cimgen/src/schema/family.rs` holds everything that differs between them.

They overlap on 164 class names and both spell the XML prefix `cim`, so **only the
namespace tells them apart** — `http://iec.ch/TC57/CIM100#` vs
`https://cim.ucaiug.io/ns#`. The decoder resolves `(namespace, local name)` with a
per-element fallback to the bare name (real CGMES files bind unexpected prefixes and
write classes under namespaces they were not declared in).

NC elements carry a `nc:` prefix on `type_name` and thus on `by_type` keys. `:` cannot
occur in a CIM class name, so bare-name consumers are unaffected. NC supports decoding
only: validation, encoding and SPARQL are CGMES-only, and the encoder skips NC elements
rather than emit malformed XML.

The two families must be imported into **separate** `CimSpecification`s —
`specification_namespaces` is one prefix-to-IRI map filled with `or_insert`, and both
bind `cim` and `base` differently.

### Loading the NC table from RDFS

`cimstructs::schema_source` can build the NC class table from RDFS at runtime
instead of using the generated one, behind the `dynamic-schema` feature (off by
default; on for `cimoxide-cli` and `cimoxide-py`). Resolution: explicit
`load_from` > `CIMOXIDE_RDFS_DIR` > generated table.

The parser lives in `cimschema/` (package `cimoxide-schema`) so both `cimgen`
at build time and `cimstructs` at runtime use the same code. `postprocess` is
**not** optional for either: attribute classification, profile origins and
namespace fill-in all happen there.

`cimstructs/tests/dynamic_schema.rs` compares the runtime table against the
generated one field by field. The two are built by separate code paths — the
loader in `schema_source.rs` and `classes_gen.rs` in cimgen — and nothing else
stops them drifting.

Measured cost (`scripts/bench_schema_source.sh`): +25 ms per process, +3.5%
decode, +6.2 MB RSS. See the README table.

### NC validation: a shape table, not generated checks

CGMES validation is generated code — one function per check, each downcasting
to a concrete struct and reading a typed field. NC classes decode into
`GenericElement` bags, so there is no struct to downcast to and nothing for
that strategy to reference. The shapes are a data table instead:

- `cimschema/src/shacl/` — the TTL parser, shared by `cimgen` at build time
  and (later) `cimvalidation` at runtime, the same split the RDFS parser uses
- `cimvalidation/src/shapes.rs` — hand-written IR (`ShapeDef`, `PropShape`,
  `Check`, `Constraint`, `Path`)
- `cimvalidation/src/nc_shapes.rs` — generated, 1,966 shapes / 14,839 checks
- `cimvalidation/src/nc_profiles.rs` — generated, profile IRI → short code
- `cimvalidation/src/bag.rs` — hand-written interpreter

**The bag checks more than the generated path can, not less.** `sh:datatype`
and `sh:nodeKind` are tautologies against an `f64` and real checks against a
`FieldValue::Text` — 2,747 NCP constraints that `simplify` used to discard as
"type-system guarantees", which is why that stage is now gated on
`Family.typed`. `sh:closed` (823 shapes) is inexpressible against generated
structs at all, because unknown properties are dropped at decode.

Resolution happens once, when the table is built, never per element: target
classes become family-qualified `by_type` keys with abstract classes expanded
to concrete descendants, and paths become the field keys the decoder stores
(the local XML name, prefix dropped).

`sh:path ( nc:X.y rdf:type )` becomes `Path::RefType` — follow the association,
read the referenced element's class. 170 of NCP's 178 chains are this, and
their allowed lists name CGMES classes beside NC ones, so class resolution
spans both families. This is the only place NC validation reaches across the
family boundary, and it is safe precisely because it reads the referenced
element's *type* rather than an attribute the decoder dropped.

Paths are **not** verified against the class table. A bag reads whatever key
the XML carried, so a shape whose path the imported table lacks still validates
real data — `dcterms:spatial` on `dcat:Dataset` is exactly that. Verifying
rejected 43 DatasetMetadata shapes whose data is present.

Profile dispatch is read rather than written. `NCP/SHACL/Validation/` ships one
manifest per profile whose `owl:imports` names the files that apply, and
`NCP/PROF/` maps a dataset's `dcterms:conformsTo` IRI to a short code. NC
announces its profiles with a DCAT header (`dcat:Dataset`), not CGMES's
`md:FullModel` — without one, no NC profile is detected and nothing runs.

NC leans on `sh:Info` far more than CGMES: 842 occurrences against 7. `cimcli
validate` therefore treats `sh:Info` as advisory and excludes it from the exit
code.

#### Loading the shapes from SHACL at runtime

`cimvalidation::shape_source` can build the shape table and profile index from
a SHACL directory instead of using the generated ones, behind the
`dynamic-shapes` feature (off by default; on for `cimoxide-cli` and
`cimoxide-py`). Resolution: explicit `load_from` > `CIMOXIDE_SHACL_DIR` >
generated. A bad path warns and falls back.

The feature also enables `cimstructs/dynamic-schema`, because the shapes
resolve against the class table: a shape table from one release and a class
table from another would disagree about what a class is. The RDFS is found
beside the SHACL directory (`<dir>/../RDFS`, the way `PROF` is), so a load is
self-contained and does not depend on the working directory.
`CIMOXIDE_RDFS_DIR` still takes precedence for the family being loaded.

Unlike the class table, there is **no second implementation to drift**: both
`cimgen` and the runtime loader call `cimschema::shacl::resolve`, and only the
output differs — rendered Rust source versus interned `&'static` data.
`cimvalidation/tests/dynamic_shapes.rs` still compares the two tables shape by
shape, because that rendering-versus-interning step is duplicated, and it is
what caught the sibling families' schemas being resolved against the working
directory and silently dropped — which had emptied every CGMES class out of the
cross-family value-type rules.

Measured cost (`scripts/bench_shape_source.sh`): +163 ms per process, +51 MB
RSS, and validation 14.9% *faster* on the loaded table than the generated one.
See the README table for the breakdown and for why the throughput went the
opposite way from the decoder's.

**Not covered**, and reported as skips rather than silently dropped: NCP's 35
`sh:sparql` constraints; the 119 `cim16:`/`cim17:` target classes (NC shapes on
CGMES classes, whose NC attributes the decoder discards, so checking them would
report absent values the XML carried); and three `sh:qualifiedValueShape`
shapes reachable only through `sh:or ( [ sh:not <named shape> ] .. )` —
material implication, which the IR has no way to express.


## Codegen Stability Tests

`cimgen/tests/codegen.rs` contains four hash-based tests that detect unintended generator drift:
- `cimstructs_codegen_stable` — Hashes regenerated struct output against a stored SHA-256
- `cimvalidation_codegen_stable` — Same for the CGMES validators. Excludes `nc_shapes.rs`
  and `nc_profiles.rs`, which land in the same directory
- `nc_classes_codegen_stable` — Hashes the NC class table alone, so a CGMES-only change
  cannot mask an NC change
- `nc_shapes_codegen_stable` — Same for the NC shape table

**When making intentional generator changes**:
1. Run the codegen tests — the failure message shows the actual new hash
2. Update the `assert_eq!` expected hashes in `cimgen/tests/codegen.rs`
3. Regenerate the checked-in files with `cargo run -p cimoxide-gen -- ...`
4. Commit the updated hashes and generated files together

These tests are CI-critical; a hash mismatch means schema or generator logic changed unexpectedly.

## Crate Names

Directory and `use` names keep the short form; the crates.io package name is prefixed so
the family is recognisable in the registry. `cargo -p` takes the package name.

| Directory / `use` | crates.io package |
|---|---|
| `cimoxide/` | `cimoxide` — facade re-exporting the crates below |
| `cimgen/` | `cimoxide-gen` (binary stays `cimgen`) |
| `cimstructs/` | `cimoxide-structs` |
| `cimdecoder/` | `cimoxide-decoder` |
| `cimvalidation/` | `cimoxide-validation` |
| `cimconvert/` | `cimoxide-convert` |
| `cimsparql/` | `cimoxide-sparql` |
| `cimoxide-cli/` | `cimoxide-cli` (binary `cimcli`) |

Each crate sets an explicit `[lib] name`, so renaming a package never touches source.

## Releasing

Shared package metadata (`version`, `license`, `repository`, keywords) lives in
`[workspace.package]` in the root `Cargo.toml`; internal crate dependencies are declared
once in `[workspace.dependencies]` with both a path and a version. Bumping the version
there bumps every crate and every inter-crate requirement.

Pushing a `v*` tag triggers three workflows: `release.yml` (cimcli binaries + GitHub
release), `pypi.yml` (Python wheels), and `crates-io.yml`. The crates.io job needs a
`CARGO_REGISTRY_TOKEN` secret in the `crates-release` environment, runs `make generate`
first (the generated sources are gitignored, hence `--allow-dirty`), and can be dry-run
from `workflow_dispatch`.

Publishing goes through `scripts/publish-workspace.sh` rather than a bare `cargo publish
--workspace`, because crates.io meters publishes per account: a burst of 5 brand-new
crate *names* and then 1 per 10 minutes, versus a burst of 30 new *versions* of existing
crates and then 1 per minute (<https://crates.io/docs/rate-limits>). Publishing 8 new
names therefore always stalls part-way. The script asks the registry which
`name@version` pairs already exist, passes them as `--exclude`, and sleeps until the
refill time the 429 reports before retrying the rest — so a stalled run is re-runnable
as-is, and `--dry-run` keeps working once some versions are on the registry (cargo fails
a dry run outright otherwise, rust-lang/cargo#14789).

Once every crate name exists on crates.io, later releases only ever spend the
30-per-burst *existing version* allowance, so the whole workspace publishes in one pass.
