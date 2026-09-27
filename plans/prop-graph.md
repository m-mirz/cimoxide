# cimoxide: Graph Representation, Schema Flexibility & Persistence — Design Summary

Context: cimoxide is a Rust CGMES/CIM parser (PyPI-distributed) used alongside gridoxide
(Rust power grid calculation library). This doc summarizes design decisions from a
discussion on moving from RDF-only handling toward a property-graph-style in-memory
model with flexible schema handling, SHACL validation, and S3-backed persistence.

## 1. In-memory representation: petgraph over RDF triple stores

- Use `petgraph` (index-based, `Vec`-backed) for the actual solver-facing graph:
  topology traversal, power flow, state estimation, contingency analysis.
- Rationale: RDF stores (Jena/Oxigraph) pay per-triple dictionary/index indirection
  even for simple attribute access; petgraph gives direct index-based access.
  Expect roughly 1–2 orders of magnitude speedup for traversal-heavy workloads.
- Keep a triple-store / SPARQL layer (Oxigraph preferred over Jena — see below) only
  for workloads that genuinely need ad-hoc, unanticipated query patterns: external
  tooling, LLM-assisted exploratory queries, standards-compliant interop/export.
- CGMES's schema is closed-but-versioned (not open-world/arbitrary), which is why a
  typed/native graph structure fits better than RDF's genuinely-flexible-schema model.

## 2. Schema flexibility without full recompilation

Currently: codegen for schema types AND for SHACL checks, both build-time (`build.rs`).
Goal: keep near-codegen performance while making new CGMES profile versions a data
load, not a recompile.

Key recommended pattern — **property-bag with fast-path promotion**:
```rust
struct CimNode {
    class: ClassId,                         // interned, versioned, not a Rust enum
    mrid: Mrid,
    // promoted hot attributes as real typed fields (only for classes/attrs
    // your solvers actually touch — profile before promoting)
    rated_voltage: Option<f64>,
    // everything else:
    extra: HashMap<AttrId, CimValue>,
}
enum CimValue { Float(f64), Str(String), Enum(EnumId), Ref(NodeIndex), ... }
```
- Class hierarchy (multi-inheritance) stored as a lookup table (`ClassId -> Vec<ClassId>`
  ancestors) built from the profile at load time, not encoded via Rust enum/trait nesting.
- Edge/association types: use an interned `RelationId` (not a hardcoded `enum EdgeKind`),
  resolved from the profile at load time; promote hot associations to direct
  `petgraph` edge indices for solver hot paths.
- **Default `Vec` (not `Option`) for association multiplicity everywhere**, even for
  currently-1:1 associations — CGMES has historically changed 1:1 → 1:many between
  versions (e.g. multi-terminal equipment, three-winding transformers, HVDC).
- Version everything by `(CgmesProfileVersion, ClassId)` — schema, shapes, and relation
  tables. CGMES evolution happens as discrete "load a new profile version" events, not
  live schema drift, so this versioning is a natural load-time checkpoint.
- Historical evidence that relationship-level changes matter (not just attributes):
  profile restructuring across EQ/SSH/TP/SV/DY, multiplicity changes, new association
  classes for dynamics/HVDC, association role renames.

## 3. SHACL validation: load-time IR compilation instead of build-time codegen

- Split constraints by cost: cheap ones (`minCount`/`maxCount`, `datatype`, `sh:in`,
  range checks) are ~O(1) against a property-bag lookup — interpret generically, codegen
  buys little. Only `sh:sparql` and `sh:class`-with-traversal are expensive enough to
  warrant hand-optimization.
- Parse SHACL shapes graph into an IR (`NodeShape { class, properties: Vec<PropertyShape> }`)
  **once at profile-load time**, not at `cargo build` time. New/updated `.ttl` shapes
  file = no recompilation.
- Compile the IR into closures (or enum-dispatched function table) at load time — pay
  compilation cost once per profile load, not per validation call. Gets close to
  generated-code speed without the build-time coupling.
- Only promote specific hot shapes to hand-tuned/generated code after profiling shows
  the closure-dispatch path is a real bottleneck — not as a default.
- After applying a diff (see §5), re-validate only the touched mRIDs/relations, not the
  whole graph — cost proportional to diff size, not graph size.

## 4. Rust crates worth adopting

- **Replace the hand-rolled TTL parser** with `oxttl` (Turtle/TriG/N-Triples/N-Quads,
  extracted from Oxigraph, actively maintained, zero-copy where possible).
- **`oxrdf`** — underlying term/triple model (interned IRI/Literal/BlankNode) worth
  using directly or as a reference design even independent of oxttl adoption.
- Check whether inputs are actually **RDF/XML** rather than Turtle — CGMES profile
  exchange has traditionally used RDF/XML; if so, `sophia` covers more serializations.
- **No mature native-Rust SHACL engine exists** (checked as of this conversation) —
  keep the custom IR/closure validator; only outsource *parsing* (via oxttl/oxrdf), not
  validation logic.
- Prefer **Oxigraph over Jena** if/when a SPARQL layer is wanted: Rust-native (fits the
  existing stack), generally faster on comparable SPARQL workloads (no JVM/GC overhead),
  avoids introducing a second language/runtime into the pipeline.
- Bus-factor note: both Jena (heavily `afs`/Andy Seaborne-driven, despite ASF formal
  governance) and Oxigraph (single primary maintainer) carry similar single-maintainer
  risk — this favors keeping cimoxide's own logic decoupled from either (thin parsing
  dependency only), not picking one over the other for "community safety."
- Caution: a GitHub fork (`sparkling/oxigraph`) surfaced during research with
  suspicious marketing-style claims about "Jena parity" — treat as unreliable, use only
  the canonical `oxigraph/oxigraph` repo.

## 5. Applying CGMES diff models to the graph

CGMES `dm:DifferenceModel` = `dm:forwardDifferences` (add) + `dm:reverseDifferences`
(remove) triple sets.

- Attribute triple (fwd) → set/create property; (rev) → remove/reset property.
- Association triple (fwd) → add typed edge (create endpoint nodes first if new);
  (rev) → remove the *specific* edge instance (respect multiplicity — don't nuke all
  edges of that relation type from a node).
- Full object deletion = all outgoing triples for an mRID appear in reverse-diff only —
  detect this pattern and cascade-delete incident edges (RDF doesn't need this; a real
  PG with edge objects does).
- **Apply node/attribute changes before edge changes** within one diff (unordered
  triples otherwise risk referencing not-yet-created nodes).
- **Apply each diff atomically** — buffer operations, commit only if all referenced
  mRIDs resolve; otherwise reject with a clear error.
- Reverse-differences double as a rollback path (apply reverse-as-forward on validation
  failure) — pairs naturally with post-diff SHACL re-validation (§3) for atomic
  accept/reject.
- Validate a diff's declared base profile/model version against the graph's current
  tracked version before applying (mis-ordered/duplicate diffs are a known real-world
  failure mode in TSO/ISO pipelines).
- **Open question to confirm before building general diff machinery**: does the actual
  use case need arbitrary forward/reverse diffs, or is it closer to "SSH/SV profiles
  are usually fully replaced," in which case a simpler "replace this profile's
  contribution" operation may suffice instead of full DifferenceModel semantics.

## 6. Long-term persistence (S3)

Don't use one blob format for everything — split by access pattern:

- **Full/base snapshots → Parquet, one table per CIM class** (not one heterogeneous
  node table, and *not* a single "all attributes as JSON blob" column — this defeats
  column pruning, predicate pushdown, and per-column compression, which are Parquet's
  entire reason for existing). Edges as a separate table
  `(from_mrid, to_mrid, rel_type, properties...)`, possibly split per relation type.
  - Gives free interoperability: DuckDB/Athena/Spark can query snapshots directly
    without cimoxide.
- **If per-class-table management is too much overhead**, use a hybrid schema instead
  of full JSON-blob: promote known-hot attributes to real typed Parquet columns
  (mirrors the in-memory fast-path-promotion pattern), everything else into an
  `extra: BINARY` overflow column. Promote more columns over time via Parquet's native
  schema evolution.
- **Diffs → batched record files, not one S3 object per diff** (S3 listing/request
  overhead). Two options, trade-off is Rust ergonomics vs. cross-ecosystem interop:
  - **Protobuf via `prost` + a small custom OCF-style container** (magic bytes +
    embedded schema descriptor + length-delimited compressed blocks) — better Rust
    ergonomics, consistent with the rest of the Rust-native stack, ~a day of extra
    work to build the container format.
  - **Avro (Object Container File) via `apache-avro`** — native schema-embedded,
    splittable file format for free, better if other ecosystems (Spark/Hive/Python)
    need to read diff archives directly; rougher Rust API, smaller/slower-moving
    maintainer base than `prost`.
  - Given the stack is Rust-first throughout (cimoxide, gridoxide, petgraph,
    oxttl/oxrdf) with no existing Spark/Hive dependency, **Protobuf+prost is the
    likely better fit** unless cross-ecosystem consumption becomes a real requirement.
- **Don't use bincode/rkyv as the S3 source of truth** — great for the local
  disposable cache rebuilt from S3, bad for long-term archival since it ties on-disk
  format to current Rust struct layout.
- Maintain a small manifest (JSON or small Parquet index) mapping model versions to
  S3 keys (which snapshot is base, which diffs apply on top, in what order) — don't
  rely on S3 key naming alone for ordering.
- Periodically checkpoint (materialize a new full snapshot) so diff-replay chains
  don't grow unbounded — cadence depends on diff frequency (hourly SSH vs. daily EQ).
- Tag snapshots and diffs with `(CgmesProfileVersion, modelVersion)` metadata,
  consistent with in-memory versioning.
- **If this grows into a multi-writer pipeline** (multiple TSOs pushing updates
  concurrently), consider Apache Iceberg/Delta Lake on S3 instead of hand-rolling
  manifest+checkpoint+ACID logic — not needed for a single-producer archive.

## 7. Scale estimates & reference numbers

- A grid model with **2 million CIM objects** → roughly **24–32 million RDF triples**
  (range 20–40M depending on class mix). Breakdown per object: 1 `rdf:type` triple +
  ~7–10 attribute triples + ~3–5 association triples.
  - `Terminal`/`ConnectivityNode` are often 40–50% of object count and triple-sparse,
    pulling the average down; attribute-rich equipment (transformers, generators)
    pushes it up.
  - Multi-profile exchange (EQ+SSH+TP+SV+DY for one scenario) is additive on top of
    this per-profile estimate.
- Equivalent property-graph representation: ~2M nodes, ~6–10M edges, ~14–20M attribute
  values as node properties (more compact than RDF specifically because attributes
  collapse into node properties instead of remaining separate triples).
- Jena/TDB2 scaling notes (relevant if a triple-store layer is used for interop):
  in-memory Jena graphs (`GraphMemFast`/`GraphMemValue`) become uncomfortable in the
  low-tens-of-millions-of-triples range (GC pressure); TDB2 (disk-backed, six-way
  SPO/SOP/PSO/POS/OSP/OPS indexing) scales to billions of triples but at ~5–6x storage
  overhead and real per-write index-update cost; single-node only (no horizontal
  sharding); single-writer transaction model; property-path/multi-hop traversal
  remains structurally slower than native adjacency (petgraph) regardless of indexing.

## Suggested next implementation steps
1. Swap hand-rolled TTL parser for `oxttl`/`oxrdf`.
2. Define `ClassId`/`AttrId`/`RelationId` interning + `(ProfileVersion, ClassId)`-keyed
   schema/shape caches.
3. Build the SHACL shapes → IR → closure compiler (load-time, not build-time).
4. Migrate node representation to property-bag + fast-path-promoted fields.
5. Implement diff application (attribute/edge add-remove, ordering, atomicity,
   post-diff incremental SHACL re-validation).
6. Design per-class Parquet snapshot schema + Protobuf/prost diff container format;
   build the manifest layer tying snapshot+diff chains to model versions.