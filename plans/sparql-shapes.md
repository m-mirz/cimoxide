# The profiles' SPARQL constraints, run as written

cimoxide checks the `sh:sparql` constraints of ENTSO-E's SHACL with rules written by hand in
`cimvalidation/src/sparql/`. This plan adds a second way: read the constraints' queries from
the shape files and run them with oxigraph, through `cimsparql`. It is a reference for the
hand-written rules and an opt-in mode, not their replacement.

## Why

- **A reference in the workspace.** The hand-written rules are transcriptions of 226 queries
  (192 CGMES, 34 NC), under about 130 rule ids, and a transcription can be wrong in ways
  their unit tests share. The ReliCapGrid harness compares cimoxide with `triplets`, but only
  outside the workspace, on one dataset, and with Python. Running the official queries next
  to the hand-written rules on the test corpus makes the comparison a test.
- **Coverage.** Constraints nobody has written by hand yet, and those of a new profile
  release, run without code. Since #39 and #40 the shape files come from `CIMOXIDE_PROFILES`,
  so the queries would come with them.

## Why not instead of the hand-written rules

- **Cost.** `cimsparql` holds the graph beside the elements. Merged RealGrid (188,551
  elements, 1.3 M quads) takes 2.7 s to load and peaks at 900 MB against 350 MB for
  `import`. Several queries are heavy (`COUNT` subqueries, `FILTER NOT EXISTS`) and run once
  per constraint over the whole graph.
- **The queries have bugs the hand-written rules work around.** Where a query cannot report
  what its `sh:description` says (a misspelt property, contradictory bindings), the
  hand-written rule implements the description (see AGENTS.md). Run as written, those
  queries report what they report.
- **cimlens runs validation as wasm.** oxigraph does not build for `wasm32-unknown-unknown` as
  it is: its `getrandom` needs the `wasm_js` backend switched on. Even then the memory cost
  argues against it in a browser. The mode stays out of cimlens's default build.

## Shape

### Reading the constraints (cimschema)

`ttl_import` keeps a SPARQL constraint only as a marker for the skip report. It keeps the
constraint itself instead, as a `SparqlConstraint`:

- the shape it belongs to: its IRI as the rule id (the property shape
  `equ:ShuntCompensator.maximumSections-numberOfInstances`, not the constraint node
  `…Sparql`), its targets, its `sh:path`, severity, `sh:name`, message and description, and
  whether it is `sh:deactivated`;
- `sh:select` (or `sh:ask`), and the prefixes `sh:prefixes` declares through `sh:declare`;
- the file it came from, so the manifest decides which profiles it runs for, as it does for
  the shape table.

The skip report counts a constraint as skipped only where the mode is off.

### Running them (cimvalidation, feature `sparql-shapes`)

Behind a feature that pulls in `cimoxide-sparql`, off by default and on for `cimox` and
`cimoxide-py`. For each constraint:

1. Substitute `$PATH` with the shape's path (148 uses in the shipped files).
2. Bind `$this` to every focus node: run the query once with `$this` as a variable, joined
   with the shape's targets (`?this a cim:Class`, `sh:targetNode`, subjects or objects of),
   rather than once per node.
3. Map each result to a `Violation`: `$this` back to its mRID (`cimsparql::iri`), `?value`
   and the other bindings into the message template, rule id and severity from the shape.

Which data a constraint runs on follows the hand-written dispatch
(`validate_profile_local`, `validate_crossprofile`, `across_groups`, `mas_groups`), through
named graphs in one store (see [Graphs](#graphs)): a file's own rules on that file's graph, the
rest on the union of the model's graphs.

### Graphs

One store, a named graph per file, loaded once. `validate_files` already receives the files
apart (`per_file: Vec<CimDataset>`), so `cimsparql` loads each into its own graph; nothing has
to record a source file on `Element`.

- **Name**: the file's `md:FullModel` IRI, the model's identity, which `Model.DependentOn` and
  `Model.Supersedes` already point at.
- **Header**: in the file's graph, as in the file, so a header rule sees one file's header.
- **Index**: profile → graphs, from each header's `Model.profile`, for the runner to choose a
  rule's data graph.
- **Choosing the data graph**: none of the 226 queries uses `GRAPH`, `FROM` or `FROM NAMED`;
  they are written for "the data graph", which the caller sets. oxigraph sets a query's default
  graph to chosen named graphs (`query.dataset_mut().set_default_graph(…)`): one graph for a
  file's own rules, all of them for merged ones. A store per file and a merged one would load
  every quad twice.
- **Not per profile.** SOPTIM's OpenCGMES projects data onto fixed graph names per profile
  (`EQ`, `SSH`, and `RDFS_EQ`… for schemas), so queries do not depend on a TSO or timestamp
  (LF Energy Summit 2025, slide 15), and an open PR of theirs names graphs
  `cgmes:model:<keyword>` for one IGM. In a common grid model several files share a profile
  (each TSO's EQ), and a file's own rules (class counts, header rules) must see one file, so
  the graph is the file. Profile names can come later as aliases for users' own `cimox
  query` runs.
- **Difference models**: out of scope; cimoxide applies them before validating.

### Comparing (tests)

A test runs both on the corpus and compares violations by rule id and object:

- `testdata/test_sparql_*.xml`, the hand-written rules' fixtures (53 files, each breaking
  rules on purpose);
- the CGMES conformance sets in `CGMES-Test-Configurations/v3.0` (MicroGrid, MiniGrid,
  SmallGrid, Svedala, PST, PowerFlow; RealGrid and FullGrid behind `#[ignore]` for their
  size).

A difference fails the test unless it is listed, with its reason, in a table of known
divergences: the query bugs AGENTS.md records, and rules whose query and description
disagree. A listed divergence that no longer occurs fails too, so the table cannot go stale.

### Using it (cimox, cimoxide-py)

`cimox validate --sparql` runs the official queries in place of the hand-written rules, and
`--sparql=both` runs both and reports where they differ. The Python bindings take the same
option. The servers (`cimlsp`, `cimmcp`) keep the hand-written rules.

## Phases

One PR each.

1. **Read the constraints.** `SparqlConstraint` in cimschema, with its tests: every
   constraint of `CGMES-AP` and `NC-AP` read, with its rule id, targets and prefixes. No
   change to what runs.
2. **Run them.** `cimsparql` loads a list of datasets into a named graph each, and runs a
   query with chosen graphs as its default graph; the runner behind `sparql-shapes`. Tested per constraint on the
   `test_sparql_*` fixtures: each fires where its hand-written rule's fixture says it should.
3. **Compare.** The corpus test and the table of known divergences. The phase reports what the
   comparison finds; a bug in a hand-written rule is fixed in its own PR.
4. **The option.** `--sparql` in `cimox`, the Python option, and the README.

## Open points

- Whether `--sparql` replaces only the hand-written rules or also the shape table, for
  constraints the table covers too. The shape table is exact for what it covers, so probably
  only the SPARQL ones.
- Time limits: a query that runs for minutes on RealGrid should be reported, not wait.
