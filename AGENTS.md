# AGENTS.md

This file provides guidance to AI coding agents (Claude Code, Codex, Cursor and others) when working with code in this repository.

## What This Project Does

cimoxide is a Rust monorepo providing tooling for ENTSO-E CGMES (Common Information Model Exchange Standard) data used in power system modeling. It reads IEC 61970/61968 RDF/SHACL schemas, generates class tables that RDF/XML decodes against and SHACL shape tables that one interpreter validates against, and provides efficient RDF/XML deserialization.

## Common Commands

```bash
# Build all crates
cargo build

# Run all tests (submodules must be initialized)
cargo test

# Run tests for a specific crate
cargo test -p cimoxide-gen
cargo test -p cimoxide-model

# Run a single test by name
cargo test -p cimoxide-gen --test codegen cimmodel_codegen_stable

# Run benchmarks
cargo bench -p cimoxide-model

# Regenerate cimmodel and cimvalidation from schemas
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

**Data flow**: RDF/SHACL schemas → `cimgen` (code generator) → `cimmodel` (class tables, decoder, converter) + `cimvalidation` (shape tables)

### `cimgen` — Code Generator CLI
Parses RDF schema files and SHACL TTL constraint files, then generates Rust code. Key modules:
- `schema/` — RDF schema parsing and internal model (`CimType`, `CimAttribute`, `CimDatatype`)
- `generator/rust_gen.rs` — Writes `cimmodel`'s generated module: a class table per family
  (`generator/classes_gen.rs`), the CGMES version constants and the profile tables
- `shacl/` — SHACL TTL parsing, constraint model, and validation code generation

### `cimmodel` — Data Model, Decoder and Converter
One crate for CIM data and its RDF/XML in both directions. **There are no generated types.**
Every element of every family is an `Element`: its class (a `&'static ClassDef`), its mRID,
its fields as written (`Class.attr` → `FieldValue`, values unparsed) and which fields were
repeated. What a class declares is data — a class table per family — so an attribute the
class does not declare and a value that does not parse are kept, not dropped.

The generated part lives in `src/generated/` (do not hand-edit): `cgmes_classes.rs`,
`nc_classes.rs`, `constants.rs`, `profile_meta.rs` and a `mod.rs`. `.gitignore` and
`make clean` cover that directory alone; a new hand-written module goes directly in `src/`
and is declared in the hand-written `lib.rs`.

Up to 0.3.3 this was three crates, `cimoxide-structs`, `cimoxide-decoder` and
`cimoxide-convert`, and CGMES decoded into 446 generated structs plus a raw field map
(`RdfBlock`) kept beside each; those crates stay on crates.io at 0.3.x. Dropping the structs
cut RealGrid decoding by 22% and peak memory by 26%, because nothing was built twice.
Field keys are `&'static str` (`FieldMap = FastMap<&'static str, FieldValue>`): the decoder
takes a declared attribute's key from the class table and passes any other through
`base::intern`, which leaks each distinct name once. That cut another 13% off decoding and
56 MB off RealGrid's peak; a `String` per field was ~900k allocations to build and free.

Core files:
- `base.rs` — hand-written. `Element`, `FieldValue`, the `FastMap`/`FieldMap` hashing,
  `ClassDef`/`AttrDef`, and the `TypeRegistry`: `(namespace, local name)` → class, a
  bare-name fallback (CGMES only), and every attribute a class declares, inherited ones
  included (`attr(class, id)`, `chain(class)`)
- `registry.rs` — hand-written. `type_registry()`, built once from both class tables (each
  generated, or read from RDFS at runtime; see "Loading the class tables from RDFS")
- `decode.rs` — hand-written streaming XML parser that produces `CimDataset`
  (re-exported at the crate root with `Element`):
  - `CimDataset::decode_file(path)` / `decode_files(paths)` / `decode_str(content)` — Entry points
  - `CimDataset::merge(other)` — Combine multiple datasets: a later scalar wins, reference
    lists are joined; duplicate tracking stays per file
  - `CimDataset { entries: FastMap<mrid, Element>, by_type: FastMap<type_name, Vec<mrid>> }`
  - `FastMap`/`FieldMap` (in `cimmodel::base`) use a hand-written Fx-style hasher instead of
    SipHash: −12% decode. It has a fixed seed, so map iteration — and therefore `by_type`
    order after `merge` and violation order — is reproducible run to run. Core crates take
    no dependency for it; do not pull in crates that are only in `Cargo.lock` via oxigraph
- `convert.rs` — hand-written RDF/XML export (`dataset_to_xml`, `dataset_to_xml_for_profile`)
  and JSON (`dataset_to_json`, `dataset_from_json`): `{mrid: {"_type", "id", "Class.attr":
  "value" | ["value", ...]}}`, values as written. Import tells references from text by the
  attribute's declaration in the class table
- `schema_source.rs` — where each family's class table comes from (generated or RDFS)

### `cimvalidation` — SHACL Validators (`cgmes_shapes.rs`, `cgmes_profiles.rs`, `nc_shapes.rs`, `nc_profiles.rs` are generated; do not hand-edit those)
Both families validate through one interpreter, `bag.rs`, over a shape table:
`cgmes_shapes.rs` (CGMES) and `nc_shapes.rs`, each reading its own family's elements.
See "Shape tables" below. CGMES used to have ~250k lines of generated per-check
validators (123 s to compile the crate, against 9.5 s now); they are gone.

Generated: `src/cgmes_shapes.rs`, `src/cgmes_profiles.rs`, `src/nc_shapes.rs`, `src/nc_profiles.rs`. `src/sparql/` (hand-written reimplementations of the
`sh:sparql` constraints), `helpers.rs`, `violation.rs`, `detect.rs` and `lib.rs` are
hand-written. Entry points:
- `validate_files(per_file, cfg)` — full two-phase run: per-file checks in parallel, then
  cross-profile checks on the merged dataset
- `validate_header`, `validate_profile_local(dataset, profile, cfg)`, `validate_crossprofile`
- `validate_profile_shacl` / `validate_crossprofile_shacl` — the CGMES table half alone,
  without the hand-written SPARQL rules
- The hand-written rules read attributes through `sparql::Fields` (`f64("Class.attr")`,
  `reference(..)`, `enumeration(..)`, …). Its accessors keep the values the generated
  structs once held — last value of a repeated scalar, `true` only for the text `true`,
  numbers parsed into the attribute's own type, a single-valued reference written twice is
  absent — and the key is the attribute's declaring class (`Equipment.inService` on a
  Breaker). An absent value is not a value: the SPARQL binds `$this $PATH ?value`
  and `?x > ?max` is false when `?max` is unbound, so a rule must not default a
  missing number to 0 — the CsConverter angle ranges did, and fired on every
  rectifier and inverter in SSH files, whose limits are EQ attributes. About 170
  `unwrap_or(0.0)` calls remain in `sparql/`; each is suspect. Three rules (float special values and mRID uniqueness in `common.rs`, dangling
  references in `common_solved_mas.rs`) read `sparql::view`, which rebuilds the structs'
  view — declared attributes only, in their typed form, iterated in the order a struct's
  map produced — from the class table and `profile_meta::ATTR_RDF`
- `combined_config(...)` builds the `Config`
- Profiles: `"EQ"`, `"OP"`, `"DY"`, `"SV"`, `"SSH"`, `"SC"`, `"GL"`, `"DL"`, `"EQBD"`

### `cimsparql` — SPARQL 1.1 Querying
Materialises a `CimDataset` into an in-memory oxigraph store (`default-features = false`, so
no RocksDB/C++ toolchain) and queries it:
- `CimStore::from_dataset(&ds)` / `from_dataset_with(&ds, &GraphOptions)` / `.query(sparql)`
- `quads(&ds, &opts, &mut stats)` streams quads without a store
- Relies on the `TYPE_NS` / `ATTR_RDF` tables `cimgen` emits into `cimmodel`, which are the
  only runtime source of per-attribute IRIs — the decoder resolves an element's class by
  namespace but keeps field keys as bare `Class.attr`
- `cimoxide-cli` (`cimcli query`) and `cimoxide-py` (`dataset.query(...)`) depend on it
  behind a default-on `sparql` feature

## Profile Families

Two schema trees are read from `application-profiles-library/`: CGMES (`CGMES/RDFS`,
446 classes) and NC / Network Codes (`NCP/RDFS`, 596 classes), each a class table that
elements decode against. A `Family` in
`cimgen/src/schema/family.rs` holds everything that differs between them.

They overlap on 164 class names and both spell the XML prefix `cim`, so **only the
namespace tells them apart** — `http://iec.ch/TC57/CIM100#` vs
`https://cim.ucaiug.io/ns#`. The decoder resolves `(namespace, local name)` with a
per-element fallback to the bare name (real CGMES files bind unexpected prefixes and
write classes under namespaces they were not declared in).

NC elements carry a `nc:` prefix on `type_name` and thus on `by_type` keys. `:` cannot
occur in a CIM class name, so bare-name consumers are unaffected. NC supports decoding
and validation; encoding and SPARQL are CGMES-only, and the encoder skips NC elements
rather than emit malformed XML.

The two families must be imported into **separate** `CimSpecification`s —
`specification_namespaces` is one prefix-to-IRI map filled with `or_insert`, and both
bind `cim` and `base` differently.

### Loading the class tables from RDFS

`cimmodel::schema_source` can build a family's class table — CGMES or NC — from
RDFS at runtime instead of using the generated one, behind the `dynamic-schema`
feature (off by default; on for `cimoxide-cli` and `cimoxide-py`). Resolution:
explicit `load_from` > `CIMOXIDE_RDFS_DIR` > generated table. The variable may list
several directories, separated as in `PATH`; `schema_source::family_of_dir` assigns
each to the family whose vocabularies it holds (CGMES's `61970-600-2_*` pattern is
tried first, because NC's `*-AP-Voc-RDFS2020.rdf` matches CGMES's files too), so NC's
directory alone leaves CGMES on its generated table.

The parser lives in `cimschema/` (package `cimoxide-schema`) so both `cimgen`
at build time and `cimmodel` at runtime use the same code. `postprocess` is
**not** optional for either: attribute classification, profile origins and
namespace fill-in all happen there.

`cimmodel/tests/dynamic_schema.rs` compares each family's runtime table against
the generated one field by field. The two are built by separate code paths — the
loader in `schema_source.rs` and `classes_gen.rs` in cimgen — and nothing else
stops them drifting.

Measured cost (`scripts/bench_schema_source.sh`): +25 ms per process, +3.5%
decode, +6.2 MB RSS. See the README table.

### Shape tables: how both families validate

CGMES validation used to be generated code — one function per check, each
downcasting to a generated struct and reading a typed field. NC never had
structs, so there was nothing for that strategy to reference; NC got a data
table instead, and CGMES now uses the same table and interpreter (and has no
structs either). A `bag::Source` keeps each family's
shapes on its own elements (NC targets `IdentifiedObject.name` via
`sh:targetSubjectsOf`, which would otherwise reach every CGMES element).

CGMES ships no per-profile manifests, so `cimschema::shacl::cgmes_manifest`
writes the file → tag mapping out (`"EQ"`, `"SSH!NS"` for not-solved,
`"X:SV"` for cross-profile, `"HDR"`), referenced from `Family::shacl_manifest`.

CGMES targets follow SHACL: an abstract `sh:targetClass` expands to its
concrete subclasses. The generated validators looked the literal class up and
so checked nothing for those — e.g. SSH `IdentifiedObject.mRID-cardinality`
and `Equipment.inService`.

The SSH mRID rule is kept as written on purpose. It fires on every Equipment
element of nearly every SSH test configuration (18,761 on RealGrid), which
leaves the attribute out; the SSH vocabulary says `1..1` and the SHACL agrees.
Do not skip or downgrade it. Its rule id `ido:IdentifiedObject.mRID-cardinality`
is shared by all ten profiles' constraint files, so `--silence` hides it in EQ
too.

Beyond NC's constraints the IR has numeric ranges, `sh:length`,
`sh:lessThan(OrEquals)`, `sh:not [sh:class]`, sequence paths (`Path::Chain`,
incl. inverse and `rdf:type` steps; stepping *from* an element absent from the
dataset makes the result unknown and the checks silent, while a final
`rdf:type` step yields the elements reached and class checks skip absent ones),
node-level `sh:and`/`sh:or`/`sh:xone` (`ShapeDef::logic`; a branch may be
negated — `[ sh:not X ]`; all-or-nothing: a branch the importer cannot fully
represent skips the whole combination) and `sh:qualifiedValueShape` over a value
list (`Constraint::QualifiedIn`). NCP's DatasetMetadata material implications,
`sh:or ( [ sh:not dm:conformsToNCProfile ] [ P required ] )`, run through those.

`sh:datatype` and `sh:nodeKind` are kept for both families: the interpreter
reads the text as written, where a malformed number is real. `sh:in` values are
keyed the way the decoder stores an `rdf:resource` — after the last `#`, else
the whole IRI, with prefixed names expanded first.

`sh:closed` is the one exception to that expansion. The APL writes one closed
`AllowedProperties` shape per class, listing its properties with inherited
ones, so a closed shape governs its class's concrete descendants *except* those
below a class with a closed shape of its own in the same file
(`Resolver::closed_concrete`); otherwise every property a subclass adds would be
reported against its superclass's list (`PinTerminal.kind` against
`GateInputPin`). A closed shape that also carries checks would be split in two so
the checks keep the full targets; none in the APL does.

The interpreter walks element-major (each target once, every shape on its
class) and reports a finding once per element and rule: a property shape shared
by two node shapes reaches a subclass instance through both once abstract
targets are expanded.

A finding's `property` is the path's first field key, written `^Field` when
that step is inverse, as SHACL writes it: `^Terminal.ConductingEquipment` on a
Switch means the terminals pointing at it. `sh:targetNode cim:X` with
`[ sh:inversePath rdf:type ]` counts the instances of class X — the focus node
is the class, not an element — so the resolver skips that path and the rules
using it are hand-written (`sparql/`), labelled `^rdf:type`.

Walks over a large dataset run on threads (`std::thread::scope`, no
dependency; `cimvalidation/src/par.rs`): the interpreter's class and subject
walks and its index pass, the per-element SPARQL walks, and the independent
SPARQL rule groups. Validation is memory-bound, a cache miss per element, and
threads overlap those. Work is cut into contiguous runs whose results are
concatenated in run order, so the output is byte-identical to a single thread
(GENC1's findings are sorted, since its hash buckets depend on the thread
count). Below 20,000 elements a walk stays on one thread, so unit tests on
small fixtures do not exercise the split — compare `cimcli` output on the real
configurations for that.

The manifest tag `"COMMON"` (IdentifiedObject string lengths) runs on the merged
dataset under `--common`. CGMES profile codes are checked before the NC index in
`validate_profile_local`, so CGMES data never loads the NC table.

To check a validation change, compare `cimcli validate --common --quality
--format json` output before and after as multisets of `(object_id, rule_id,
property)` across the CGMES test configurations — and on copies mutated to
trigger the rules touched, since most real configurations violate few of them.
Time with alternating before/after runs in the same session.

The pieces:

- `cimschema/src/shacl/` — the TTL parser, shared by `cimgen` at build time
  and (later) `cimvalidation` at runtime, the same split the RDFS parser uses
- `cimvalidation/src/shapes.rs` — hand-written IR (`ShapeDef`, `PropShape`,
  `Check`, `Constraint`, `Path`)
- `cimvalidation/src/nc_shapes.rs` — generated, 1,973 shapes / 14,842 checks
- `cimvalidation/src/cgmes_shapes.rs` — generated, 849 shapes / 18,681 checks plus
  node-level logic
- `cimvalidation/src/nc_profiles.rs`, `cgmes_profiles.rs` — generated from each
  family's `PROF/` descriptors, profile IRI → short code
- `cimvalidation/src/bag.rs` — hand-written interpreter

**The table checks more than generated code could, not less.** `sh:datatype`
and `sh:nodeKind` are tautologies against an `f64` and real checks against the
text as written — `simplify` used to discard them as "type-system guarantees"
and no longer does, for either family. `sh:closed` (823 NC shapes) was
inexpressible against generated structs, which dropped unknown properties at
decode; an element keeps them.

Resolution happens once, when the table is built, never per element: target
classes become family-qualified `by_type` keys with abstract classes expanded
to concrete descendants, and paths become the field keys the decoder stores
(the local XML name, prefix dropped).

`sh:path ( nc:X.y rdf:type )` is a two-step `Path::Chain` — follow the
association, read the referenced element's class (the interpreter has a
zero-allocation fast path for it). 170 of NCP's 178 chains are this, and
their allowed lists name CGMES classes beside NC ones, so class resolution
spans both families. This is the only place NC validation reaches across the
family boundary, and it is safe precisely because it reads the referenced
element's *type* rather than an attribute the decoder dropped.

Paths are **not** verified against the class table. A bag reads whatever key
the XML carried, so a shape whose path the imported table lacks still validates
real data — `dcterms:spatial` on `dcat:Dataset` is exactly that. Verifying
rejected 43 DatasetMetadata shapes whose data is present.

Profile identity is read rather than written, for both families: each `PROF/`
directory maps the profile IRI a dataset declares to a short code — NC's
`dcterms:conformsTo` in a DCAT header (`dcat:Dataset`), CGMES's
`md:Model.profile` in an `md:FullModel` (`detect.rs` looks it up in
`cgmes_profile_index()`). Without a header, no profile is detected and nothing
runs. CGMES's PROF also names `FH` (file header), which no test configuration
declares and which has no shapes of its own; the header rules run for every file.

Which files apply to which profile is read for NC (`NCP/SHACL/Validation/` ships
one manifest per profile, whose `owl:imports` names the files) but written out
for CGMES in `cgmes_manifest`, since CGMES ships no manifests. CGMES's PROF files
list constraint resources per standard part and solved/notSolved, but not the
cross-profile split the manifest's `X:` tags carry.

NC leans on `sh:Info` far more than CGMES: 842 occurrences against 7. `cimcli
validate` therefore treats `sh:Info` as advisory and excludes it from the exit
code.

#### Loading the shapes from SHACL at runtime

`cimvalidation::shape_source` can build the shape table and profile index from
a SHACL directory instead of using the generated ones, behind the
`dynamic-shapes` feature (off by default; on for `cimoxide-cli` and
`cimoxide-py`), for NC and CGMES alike. Resolution: explicit `load_from` >
`CIMOXIDE_SHACL_DIR` > generated. A bad path warns and falls back.

`CIMOXIDE_SHACL_DIR` may list several directories, separated as in `PATH`;
`shape_source::family_of_dir` assigns each to the family whose files it holds
(NC: a `Validation/` subdirectory; CGMES: files its manifest names), so an NC
directory alone never makes CGMES try to load from it. A shape table and a class
table from different releases disagree about what a class is, so load the CGMES
RDFS from the same release (`CIMOXIDE_RDFS_DIR`, or `<dir>/../RDFS` as below).

The feature also enables `cimmodel/dynamic-schema`, because the shapes
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
RSS, and no measurable change in validation throughput. See the README table.

**Reading that bench:** criterion baselines are rigorous within a run, but the
two halves of a generated-vs-loaded comparison are separate processes, and
cross-run drift here has been measured at 20% for byte-identical code. A
difference under ~5% means nothing until a second independent A/B reproduces
it — a 14.9% "speedup" was reported from a single pair of runs and vanished on
re-measurement.

**Not covered**, and reported as skips rather than silently dropped: NCP's 35
`sh:sparql` constraints; the 119 `cim16:`/`cim17:` target classes (NC shapes on
CGMES classes). They were skipped because CGMES structs discarded the NC attributes
they check; elements now keep those, so enabling them is possible but would change
validation results. One logical shape in
`RemedialActionSchedule-AP-Con-Complex-SHACL.ttl` resolves but never runs: only
the combined "ALL" manifest imports that file.


## Codegen Stability Tests

`cimgen/tests/codegen.rs` contains four hash-based tests that detect unintended generator drift:
- `cimmodel_codegen_stable` — Hashes `cimmodel`'s regenerated module (class tables, constants,
  profile tables) against a stored SHA-256
- `cgmes_shapes_codegen_stable` — Same for the CGMES shape table, which validation runs,
  and its profile index
- `nc_classes_codegen_stable` — Hashes the NC class table alone, so a CGMES-only change
  cannot mask an NC change
- `nc_shapes_codegen_stable` — Same for the NC shape table and its profile index

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
| `cimmodel/` | `cimoxide-model` |
| `cimvalidation/` | `cimoxide-validation` |
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
