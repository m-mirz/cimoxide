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

### `cimvalidation` — SHACL Validators (`generated_*.rs` only; do not hand-edit those)
Only `src/generated_*.rs` is generated. `src/sparql/` (hand-written reimplementations of the
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

## Codegen Stability Tests

`cimgen/tests/codegen.rs` contains three hash-based tests that detect unintended generator drift:
- `cimstructs_codegen_stable` — Hashes regenerated struct output against a stored SHA-256
- `cimvalidation_codegen_stable` — Same for validation code
- `nc_classes_codegen_stable` — Hashes the NC class table alone, so a CGMES-only change
  cannot mask an NC change

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
