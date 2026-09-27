# NC SHACL validation as a data table + interpreter

> **Implemented**, `24eb07d` → `7f27c8d` on `ncp`. What shipped differs from the
> plan in four places, all noted inline below: paths are not verified against
> the class table; `Path::RefType` was added for the 170 `rdf:type`-terminated
> chains, which the plan did not anticipate; resolution moved into `cimschema`
> so there is no second implementation to drift; and the measured costs are in
> the README, not here.

> Follows the NCP decoder work (`ed0f14f` → `7be0463` on `ncp`), and reuses its shape:
> a generated data table, a generic consumer, and an optional runtime loader that
> takes precedence over the table.

## Context

CGMES validation is **generated code**: 260,900 lines across 44 `generated_*_shacl.rs`
files, each check a hand-shaped function that `downcast_ref::<cimstructs::Terminal>()`
and reads a typed field. That works because every CGMES class is a compiled struct.

NC classes are not. They decode into `GenericElement` property bags, so there is no
struct to downcast to and no typed field to read. The generated-code strategy is not
merely inconvenient here — it has nothing to generate against.

This is the same fork the decoder reached, and it takes the same answer: **generate a
data table of shapes, interpret it against the bag.** The table is emitted by `cimgen`
as `cimvalidation/src/nc_shapes.rs` (the analogue of `cimstructs/src/nc_classes.rs`),
and can be replaced at runtime from the SHACL TTL files behind a feature flag (the
analogue of `CIMOXIDE_RDFS_DIR`).

**Scope**: NC only. CGMES keeps its generated validators untouched, so the validated
CGMES path is provably unaffected — the same containment that made the decoder change
safe.

### The asymmetry that makes this worth doing

Running the existing SHACL importer over the NCP TTL against the CGMES spec (see below)
skips 2,747 constraints as *"simplified — the Rust type system guarantees it"*: 1,816
`sh:nodeKind` and 930 `sh:datatype`. That reasoning is sound for a typed struct, where
`f64` cannot hold `"abc"`.

It is **false for a property bag**, which holds `FieldValue::Text(String)`. In the bag
representation those 2,747 constraints stop being tautologies and become real checks.
`sh:closed` (823 shapes) goes further: it is not expressible against typed structs at
all, because unknown properties are dropped at decode — but a bag keeps every key the
XML carried, so closedness is a set difference over `GenericElement::fields()`.

So the bag representation does not just *cope* with validation, it checks ~3,500
constraints the generated path structurally cannot. That is the headline claim and the
thing to verify first.

## What the assessment established

Measured, not assumed.

### The TTL parser already handles NCP

`cargo run -p cimoxide-gen -- --shacl 'application-profiles-library/NCP/SHACL/*.ttl'`
parses **all 32 top-level NCP SHACL files with zero warnings**. The hand-rolled Turtle
lexer/parser in `cimgen/src/shacl/ttl_import.rs` (1,337 lines) needs no syntax work.
Against the CGMES spec it yields 59 checks and 3,329 skips, which break down as:

| Skip reason | Count | What it becomes in a bag |
|---|---:|---|
| `sh:nodeKind` simplified (type-system guarantee) | 1,816 | **a real check** |
| `sh:datatype` simplified (native Rust type) | 930 | **a real check** |
| attribute not found in hierarchy | 219 | resolves against the NC table |
| `sh:targetSubjectsOf` not implemented | 357 | trivial — "does this field exist?" |
| SPARQL-derived | 6 | still out of scope |
| `sh:minCount=0` vacuously true | 1 | stays vacuous |

### Source inventory

`application-profiles-library/NCP/SHACL/`: 50 TTL files, 4.5 MB — 32 constraint files
plus 18 in `Validation/`. 2,232 `sh:NodeShape`, 4,189 `sh:PropertyShape`.

Constraint budget by predicate:

```
maxCount 2188   nodeKind 1818   datatype 1093   minCount 871   closed 823
class 390       in 340          sparql 35       alternativePath 27
or 8            not 7           qualifiedValueShape 3          maxLength 2
deactivated 2
targetClass 1873   targetSubjectsOf 357   sh:target/SPARQLTarget 2   inversePath 168
```

Two things to read off this. First, NCP has **no numeric range constraints at all** —
no `minInclusive`, `maxInclusive`, `minExclusive`, `maxExclusive`, `lessThan`,
`lessThanOrEquals`, `pattern`, `hasValue`, all of which CGMES uses heavily. The NCP
constraint palette is narrower on the value side and wider on the structural side.
Second, five components are **NCP-only and unparsed today**: `sh:closed`,
`sh:ignoredProperties`, `sh:alternativePath`, `sh:qualifiedValueShape`/
`sh:qualifiedMinCount`, and `sh:deactivated`.

`sh:deactivated` matters out of proportion to its count: two shapes carry it, and a
parser that ignores it enforces two rules the schema explicitly switched off.

### The NC class table cooperates

596 classes — 510 concrete, 86 abstract. Checked for the two collisions that would
have forced a namespace-qualified key everywhere:

- **no local class name appears in two namespaces**
- **no attribute id appears in two namespaces**

So the decoder's bare `Class.attr` field keys are unambiguous within NC, and a shape
path `cim:IdentifiedObject.mRID` can be resolved to the field key
`IdentifiedObject.mRID` by dropping the prefix. `AttrDef.origins` already carries the
profile codes (`["ER", "RA", "SSI"]`), matching the profile keywords in the SHACL
manifests exactly.

### Profile dispatch is data, not code

`cimvalidation/src/lib.rs` hardcodes profile dispatch as ten hand-written functions
(`validate_eq`, `validate_ssh`, …), each a literal list of generated module calls. NCP
ships that mapping as data:

- `NCP/SHACL/Validation/*.ttl` — 18 manifests, one per profile, each an `owl:imports`
  list naming exactly which constraint files apply, tagged `dcat:keyword "CO"`.
- `NCP/PROF/*.rdf` — profile identity: `about="https://ap.cim4.eu/Contingency"`,
  `owl:versionIRI https://ap.cim4.eu/Contingency/2.3`, `dcat:keyword CO`.

18 profiles: AE AS CO ER GD IAM MA OR PS PSP RA RAS SAR SM SIS SHS SSI, plus ALL.
Every manifest imports 5 files; the `ALL` one imports 12. Four files are shared by
17–18 manifests (`NC-AP-Con-PrefixDeclaration`, `ClassCount`,
`IdentifiedObjecStringLength`, `DatasetMetadata`).

Dataset-side detection has a landing place too: `nc:Dataset` is already in the class
table with `Resource.conformsTo` (dcterms, list-valued) — the NC analogue of
`md:Model.profile`. So `conformsTo → https://ap.cim4.eu/Contingency/2.3 → "CO"` is a
table lookup built from `NCP/PROF`.

### The one genuinely awkward finding

**119 of 619 distinct `sh:targetClass` tokens do not resolve in the NC table.** They
use `cim16:` (`http://iec.ch/TC57/2013/CIM-schema-cim16#`) and `cim17:`
(`http://iec.ch/TC57/CIM100#`) — legacy *CGMES* namespaces. 862 occurrences,
concentrated in `EquipmentReliability-Simple` (440), `GridDisturbance-Simple` (210) and
`SteadyStateInstruction-Simple` (210).

They look like this:

```turtle
er:PhaseTapChangerTabular  a  sh:NodeShape ;
        sh:property     er:TapChanger.TapChangerController-valueType ,
                        er:TapChanger.TapChangerController-cardinality ;
        sh:targetClass  cim17:PhaseTapChangerTabular , cim16:PhaseTapChangerTabular .
```

An NC shape constraining an **NC attribute** (`nc:TapChanger.TapChangerController`) on
a **CGMES class**. `http://iec.ch/TC57/CIM100#PhaseTapChangerTabular` is in
`registry.rs:285`, so the decoder builds a typed `cimstructs::PhaseTapChangerTabular`
— which has no field for the NC attribute, so the decoder **already drops it**.

This is a pre-existing decoder gap that validation surfaces rather than creates, and
there is no honest way to check these shapes while the data is being discarded upstream.
Attempting it would produce false `minCount` violations on attributes that were present
in the XML. **These shapes are therefore recorded as skipped with a named reason and a
count**, the same way `codegen.rs` records its skips today, and the plan does not
pretend otherwise. Fixing it properly means teaching the decoder to retain unmodelled
fields on typed structs — a separate change with its own cost, out of scope here.

---

## Design

### 1. Move the SHACL front end into `cimschema`

`cimgen/src/shacl/{model.rs, ttl_import.rs, simplify.rs}` (1,612 lines) move to
`cimschema/src/shacl/`, so `cimgen` at build time and `cimvalidation` at runtime parse
with the same code. `codegen.rs`, `skip.rs`, `sparql_report.rs` and `ttl_report.rs`
stay in `cimgen` — they are generator concerns.

`cimschema` already depends on `quick-xml` and `glob`; the TTL parser is hand-rolled and
adds no dependency.

**Invariant**: pure relocation. All three codegen hashes
(`cimstructs_codegen_stable`, `cimvalidation_codegen_stable`, `nc_classes_codegen_stable`)
unchanged. A moved hash means something leaked into the move. This is the same proof
obligation as step 1 of the RDFS plan, and it held there.

**One behaviour change is unavoidable in this step**: `simplify.rs` must become
family-aware. Its rules 1 and 2 drop `sh:nodeKind` and `sh:datatype` as "type-system
guarantees", which is true only for generated structs. Gate them on `Family.typed`, so
CGMES keeps today's behaviour byte for byte and NC retains the 2,747 constraints. Since
`simplify` currently runs before `codegen` with no family in scope, it takes the
`&'static Family` the rest of the pipeline already threads.

### 2. Extend the TTL importer for the five NCP-only components

In `cimschema/src/shacl/ttl_import.rs`, alongside the existing components:

| Predicate | Model addition |
|---|---|
| `sh:closed` + `sh:ignoredProperties` | `ShapeInfo.closed: Option<Vec<String>>` — allowed paths from the shape's own `sh:property` entries plus the ignored list |
| `sh:alternativePath` | a third `Path` variant beside forward and `sh:inversePath` |
| `sh:qualifiedValueShape` + `sh:qualifiedMinCount` | `ConstraintInfo` carrying a nested branch, like the existing `sh:or` handling |
| `sh:deactivated` | `ShapeInfo.deactivated: bool` |

The closed shapes enumerate their allowed set in full, so no inference is needed:

```turtle
co:OrdinaryContingency-AllowedProperties
        sh:closed             true ;
        sh:ignoredProperties  ( rdf:type ) ;
        sh:property  [ sh:path nc:Contingency.normalProbability ] ;
        sh:property  [ sh:path cim:IdentifiedObject.mRID ] ;      # … 7 total
        sh:severity  sh:Info ;
        sh:targetClass  nc:OrdinaryContingency .
```

Note these `sh:property` objects are **blank nodes**, not IRIs. `extract_shape` reads
`sh:property` via `collect_iri_list`; whether that already follows blank nodes is
unverified and is the first thing to check in this step.

Note also the severity: closed shapes are `sh:Info`, not `sh:Violation`. 842 `sh:Info`
occurrences in NCP against 7 in CGMES — NC leans on advisory severity in a way CGMES
does not, so the CLI's exit-code logic needs to keep Info out of the failure count.

### 3. Shape IR: a table, keyed the way the decoder is keyed

Generated into `cimvalidation/src/nc_shapes.rs` as `pub static SHAPES: &[ShapeDef]`,
with the IR hand-written in `cimvalidation/src/shapes.rs`:

```rust
pub enum Path {
    Forward(&'static str),                  // field key, "Contingency.normalProbability"
    Inverse(&'static str),                  // the field key on the far end
    Alternative(&'static [&'static str]),
}

pub enum NodeKind { Iri, Literal, BlankNode }

pub enum Constraint {
    MinCount(u32),
    MaxCount(u32),
    Datatype(&'static str),                  // xsd local name
    NodeKind(NodeKind),
    Class(&'static str),                     // family-qualified, "nc:Equipment"
    In(&'static [&'static str]),
    MaxLength(u32),
    Not(&'static Constraint),
    Or(&'static [&'static [Constraint]]),
    QualifiedMinCount { count: u32, shape: &'static [Constraint] },
}

pub struct PropShape {
    pub path: Path,
    pub constraints: &'static [Constraint],
    pub rule_id: &'static str,
    pub name: &'static str,
    pub message: &'static str,
    pub description: &'static str,
    pub severity: &'static str,
}

pub enum Target {
    /// Family-qualified class name, pre-expanded to concrete descendants at build time.
    Class(&'static [&'static str]),
    SubjectsOf(&'static str),
    Node(&'static str),
}

pub struct ShapeDef {
    pub targets: &'static [Target],
    pub props: &'static [PropShape],
    /// `Some(allowed field keys)` when `sh:closed`.
    pub closed: Option<&'static [&'static str]>,
    pub deactivated: bool,
    /// NC profile code owning the file this shape came from.
    pub profile: &'static str,
    pub file: &'static str,
}
```

Three resolution decisions, all made at table-build time so the interpreter stays a
straight walk:

- **Target class → `by_type` key.** A prefixed name is resolved to `(namespace, local)`
  through the TTL file's own `@prefix` map, then to `ClassDef.qualified` — `nc:` on the
  front, whatever the namespace was. This is the decoder's identity model, and it is
  what keeps `cim:ContingencyEquipment` in an NCP file from colliding with CGMES's
  `cim:` (the two bind different IRIs; see `testdata/test_nc_CO_001.xml`).
- **Abstract targets expanded to concrete descendants.** Three of 619 target tokens
  are abstract (`nc:EnergyComponent`, `nc:GridStateAlteration`,
  `nc:RemedialActionImpact`). Exact `by_type` lookup would silently check nothing.
  `ClassDef.super_class` makes the expansion a walk over 596 rows.
- **Path → field key.** Drop the prefix: `cim:IdentifiedObject.mRID` →
  `"IdentifiedObject.mRID"`. Safe because no attribute id appears in two namespaces.

### 4. The interpreter

`cimvalidation/src/bag.rs`, hand-written, taking `&CimDataset` and `&[ShapeDef]` and
returning `Vec<Violation>` — the same signature shape as every generated validator, so
it slots into the existing two-phase API without touching it.

```rust
pub fn validate_bag_profile(ds: &CimDataset, profile: &str, cfg: &Config) -> Vec<Violation>;
```

Per shape: skip if `deactivated`; collect target mRIDs; for each, `downcast_ref::<GenericElement>()`
and walk `props`. Values come from `GenericElement::get`/`get_refs`/`fields`, all of
which exist. `sh:closed` is `fields().keys()` minus the allowed set.

**The one real cost is `sh:inversePath`** (168 shapes). Answering "how many X point at
me" needs a reverse index, so build one per dataset before the walk: for every element,
for every `AttrKind::Association` field, record `target_mrid → (source_mrid, attr_id)`.
That is O(associations) once, versus O(elements × shapes) if done naively, and it is the
only part of this design with a performance question attached. Build it lazily, only if
some active shape uses an inverse or alternative path.

`sh:datatype` checking is where the bag earns its keep and also where it can produce
noise: the value is a `String`, so the check is a parse attempt against the xsd type.
The existing `helpers.rs` already has `is_xsd_datetime`, `is_xsd_date`,
`is_xsd_gmonthday`, `is_xsd_anyuri`; numeric and boolean types need `parse::<f64>()`
and the `"true"|"1"` set `GenericElement::get_bool` already uses. Reuse both rather
than writing a third spelling.

### 5. Profile dispatch from the manifests

`cimgen` reads `NCP/SHACL/Validation/*.ttl` for the `owl:imports` lists and
`NCP/PROF/*.rdf` for `versionIRI → keyword`, and emits two small tables beside the
shapes:

```rust
pub static NC_PROFILES: &[(&str, &str)] = &[("https://ap.cim4.eu/Contingency/2.3", "CO"), …];
pub static NC_PROFILE_FILES: &[(&str, &[&str])] = &[("CO", &["Contingency-AP-Con-Simple-SHACL", …]), …];
```

This also finally discharges the `nc_meta.rs` item from the NCP plan, which was listed
and never implemented — under a different name and with real consumers, rather than as
two unused constants.

Detection extends `detect.rs` with an NC branch: `by_type["nc:Dataset"]` →
`Resource.conformsTo` → profile codes. Keep it beside the existing `FullModel` /
`DifferenceModel` path rather than replacing it; a dataset can be CGMES, NC, or both,
and `Config.profiles` already carries a flat list of codes. NC codes (`CO`, `ER`, …)
cannot collide with CGMES codes (`EQ`, `SSH`, …).

### 6. Runtime loading from TTL

`cimvalidation/src/shape_source.rs`, mirroring `cimstructs/src/schema_source.rs`
closely enough that the two should be read side by side:

```rust
pub fn resolve(family: &'static str, generated: &'static [ShapeDef]) -> &'static [ShapeDef];

#[cfg(feature = "dynamic-shapes")]
pub use dynamic::{load_from, load_table, ShapeError, SHACL_DIR_ENV};  // "CIMOXIDE_SHACL_DIR"
```

Precedence explicit `load_from` > `CIMOXIDE_SHACL_DIR` > generated table. A missing or
malformed directory **warns and falls back**, naming the glob it tried — silently
serving stale shapes because a path had a typo is the worst outcome available. A
`load_from` after the first validation returns `ShapeError::TooLate` rather than being
quietly ignored; as in `schema_source`, that guard is best-effort, with a narrow race
between the check and the insert, and is documented rather than locked.

Interning is the same `Box::leak` / `Vec::leak` pattern, for the same reason: the IR is
`&'static` throughout because relaxing it would put a lifetime parameter on `ShapeDef`,
`PropShape` and `Constraint` and thread it through every `Violation`-producing
signature.

Feature `dynamic-shapes` on `cimvalidation`, off by default, enabling an optional
`cimoxide-schema` dep; on by default for `cimoxide-cli` and `cimoxide-py`. Watch for
the dev-dependency feature unification that bit the decoder work: a dev-dependency
enabling the feature turns it on for the whole graph in test builds, which can make a
"feature off" test pass for the wrong reason.

### 7. Equivalence test

`cimvalidation/tests/dynamic_shapes.rs`, modelled on
`cimstructs/tests/dynamic_schema.rs`: build the table from TTL and compare it against
the generated one constraint by constraint. The loader and the generator are separate
code paths and **nothing else stops them drifting** — this is the same argument that
made `runtime_table_matches_generated` the load-bearing test of the last change, and it
caught real divergence there.

---

## Measurement

| | What | Where | Expectation |
|---|---|---|---|
| **M1** | shapes parsed / constraints active / skipped, by reason | `--rule-report` extension | ~3,500 constraints the generated path cannot express |
| **M2** | TTL parse + table build, 4.5 MB | `cimvalidation/benches/shape_load.rs` | dominated by the Turtle parse; compare against the 23 ms RDFS parse |
| **M3** | validation throughput on a synthetic NC dataset | `cimvalidation/benches/nc_validate.rs` | **no baseline exists — this establishes one** |
| **M4** | reverse-index build cost, and validation with/without inverse shapes | same bench | isolates the one part with a complexity question |
| **M5** | generated vs TTL-loaded, criterion A/B named baselines | M3 bench | startup delta, like the decoder's +25 ms |
| **M6** | `nc_shapes.rs` size, compile time | `scripts/bench_shape_source.sh` | measure; do not guess from `nc_classes.rs` |

M3 differs in kind from the decoder's M3. There the question was "does the dynamic path
cost anything", against a known 87 ms baseline. Here there is no prior implementation,
so the number to establish is whether interpreting ~7,600 constraints over a large NC
dataset is in the same range as the generated CGMES validators or an order off. If it is
close, that is evidence the 260,900 generated lines could eventually become a table too —
worth knowing, and explicitly **not** in this plan's scope.

Two traps carried over from the last round, both of which silently produce meaningless
numbers:

- **Leak amplification.** Criterion runs hundreds of iterations; leaking per iteration
  leaks gigabytes and perturbs the allocator. Build owned structures in the loop and
  measure the leak step separately, once, with `sample_size(10)`.
- **A/B across separate runs.** Use criterion named baselines with `--bench <name>` so
  the args reach the criterion target and not the lib harness. Comparing separate runs
  on a loaded machine produced a 20% phantom regression earlier in this branch's
  history, and re-measuring under control made it vanish.

A third, specific to this change: **the synthetic dataset must actually trigger the
shapes.** `testdata/test_nc_CO_001.xml` has three elements. A generated document that
happens to satisfy every constraint would benchmark the empty path. Assert a violation
count, not just that it ran — the decoder bench's "all 50k elements decoded" assertion
is what caught 238 silently dropped elements, and the equivalent here is cheap.

---

## Files

| File | Change |
|---|---|
| `cimschema/src/shacl/{model,ttl_import,simplify}.rs` | moved from `cimgen/src/shacl/` |
| `cimschema/src/shacl/ttl_import.rs` | + `closed`, `ignoredProperties`, `alternativePath`, `qualifiedValueShape`, `deactivated` |
| `cimschema/src/shacl/simplify.rs` | nodeKind/datatype rules gated on `Family.typed` |
| `cimgen/src/shacl/{codegen,skip,sparql_report,ttl_report}.rs` | `super::model` → `cimschema::shacl::model` |
| `cimgen/src/generator/shapes_gen.rs` | **new** — emits `nc_shapes.rs` + the two profile tables |
| `cimgen/src/main.rs` | `--nc-shacl <glob>`, NC branch in `run_shacl` |
| `cimvalidation/src/shapes.rs` | **new**, hand-written — the IR |
| `cimvalidation/src/bag.rs` | **new**, hand-written — the interpreter |
| `cimvalidation/src/shape_source.rs` | **new**, hand-written — precedence, interning, `load_from` |
| `cimvalidation/src/nc_shapes.rs` | **new**, generated |
| `cimvalidation/src/{lib,detect}.rs` | NC profile branch; bag validation in the two-phase API |
| `cimvalidation/Cargo.toml` | `dynamic-shapes` feature, optional `cimoxide-schema` |
| `cimoxide-cli`, `cimoxide-py` | enable `dynamic-shapes`; keep `sh:Info` out of the exit code |
| `cimgen/tests/codegen.rs` | + `nc_shapes_codegen_stable` |
| `cimvalidation/tests/dynamic_shapes.rs` | **new** — runtime vs generated, constraint by constraint |
| `cimvalidation/benches/{shape_load,nc_validate}.rs`, `scripts/bench_shape_source.sh` | **new** |
| `AGENTS.md`, `README.md` | the NC validation section, precedence, env var, measured numbers |

## Commits

1. **Move the SHACL front end into `cimschema`.** Pure relocation; all three hashes
   unchanged, which is the proof.
2. **Parse the five NCP-only components**, and make `simplify` family-aware. CGMES
   hashes unchanged — `cimvalidation_codegen_stable` is the check that the family gate
   holds, since every one of those 2,747 simplifications is a CGMES behaviour too.
3. **The IR and the interpreter**, driven by a table `cimgen` now emits. Riskiest
   commit: a resolution mistake shows up as a rule that silently checks nothing, not as
   a compile error. Hence the skip accounting in M1 — every constraint is either active
   or skipped with a reason, and the two must sum to the parsed total.
4. **Profile dispatch from the manifests** + `detect.rs` NC branch, so `cimcli validate`
   on an NC dataset runs the right shapes.
5. **Runtime loading**, feature-gated off, plus the equivalence test.
6. **Benches, the script, and the measured numbers in the docs.**

Steps 3 and 4 are separable and should stay separate: 3 can be exercised by passing an
explicit profile code, which keeps a detection bug from looking like an interpreter bug.

## Verification

```bash
cargo test --workspace
cargo test -p cimoxide-gen --test codegen                    # step 1: three hashes unchanged
cargo run -q -p cimoxide-cli -- validate testdata/test_nc_CO_001.xml
CIMOXIDE_SHACL_DIR=application-profiles-library/NCP/SHACL \
  cargo run -q -p cimoxide-cli -- validate testdata/test_nc_CO_001.xml
```

Both invocations must report **identical violations** — same rules, same order, same
severities — since the TTL and the generated table describe the same shapes. A diff is
a loader bug, and it is the strongest single check that the runtime path reproduces the
generated one. That is exactly how the decoder's dynamic path was validated.

Then:

- The CO fixture is too small to exercise much. Extend it, or add a second fixture,
  with a **known** violation of each implemented component — a duplicated property for
  `maxCount`, a missing required association for `minCount`, a non-numeric literal for
  `datatype`, an undeclared property for `closed`. A test that asserts "no violations"
  cannot distinguish a clean file from an interpreter that checks nothing.
- `cargo run -p cimoxide-cli -- validate` on the CGMES corpus produces **byte-identical**
  output to `main`. NC is additive; any CGMES diff is a regression.
- A corrupt or missing `CIMOXIDE_SHACL_DIR` warns and falls back, not panics.
- `load_from` after a validation returns `TooLate`.
- `cargo tree -p cimoxide-validation` without the feature pulls no `cimoxide-schema`.
- Editing a TTL constraint (tighten a `maxCount`) changes `cimcli validate` output with
  **no rebuild** — the actual point of the exercise, and the check that proved the
  decoder's dynamic path was real.

## Explicitly out of scope

- **35 `sh:sparql` constraints.** CGMES's 191 are hand-reimplemented in
  `cimvalidation/src/sparql/` (~7,000 lines). NC's would be the same kind of work and
  belongs in its own change. Counted and reported as skipped.
- **The 119 `cim16:`/`cim17:` target classes** (862 occurrences), for the reason given
  above: the decoder discards the data these shapes constrain, so checking them would
  emit false violations. Reported as skipped with a named reason.
- **CGMES validation.** Untouched. The interpreter is reached only through NC profile
  codes, so no CGMES dataset can enter it.
- **Replacing the generated CGMES validators** with this machinery. M3 will produce
  evidence about whether that is viable; acting on it is a separate decision.
