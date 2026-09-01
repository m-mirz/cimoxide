//! End-to-end SPARQL queries over decoded CGMES datasets.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use cimdecoder::CimDataset;
use cimsparql::{CimStore, GraphOptions, QueryResults};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn full_grid(names: &[&str]) -> CimDataset {
    let base = repo_root().join("CGMES-Test-Configurations/v3.0/FullGrid/FullGrid-Merged");
    let paths: Vec<PathBuf> = names.iter().map(|n| base.join(n)).collect();
    if let Some(missing) = paths.iter().find(|p| !p.exists()) {
        panic!("missing test data {} — run `git submodule update --init`", missing.display());
    }
    let refs: Vec<&Path> = paths.iter().map(AsRef::as_ref).collect();
    CimDataset::decode_files(&refs).expect("decode")
}

/// Values of the first selected variable, as plain strings.
fn column(store: &CimStore, query: &str, var: &str) -> Vec<String> {
    let QueryResults::Solutions(solutions) = store.query(query).expect("query") else {
        panic!("expected SELECT results for: {query}");
    };
    solutions
        .map(|s| {
            let s = s.expect("solution");
            let term = s.get(var).unwrap_or_else(|| panic!("no binding for ?{var}"));
            match term {
                cimsparql::Term::NamedNode(n) => n.as_str().to_string(),
                cimsparql::Term::Literal(l) => l.value().to_string(),
                other => other.to_string(),
            }
        })
        .collect()
}

#[test]
fn type_index_parity() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    let subjects = column(&store, "SELECT ?s WHERE { ?s a cim:ACLineSegment }", "s");
    let expected = ds.by_type.get("ACLineSegment").expect("ACLineSegments in FullGrid");
    assert_eq!(subjects.len(), expected.len());

    // Every subject IRI resolves back to an MRID the decoder actually holds.
    let recovered: HashSet<String> = subjects
        .iter()
        .map(|iri| {
            cimsparql::iri::iri_to_mrid(iri, &ds)
                .unwrap_or_else(|| panic!("{iri} does not map back to an entry"))
        })
        .collect();
    assert_eq!(recovered, expected.iter().cloned().collect::<HashSet<_>>());
}

/// The decoder drops XML prefixes, so `cim:` and `eu:` attributes of the same element are
/// indistinguishable in `RdfBlock.fields`. `ATTR_RDF` is what puts them back; if it
/// regresses, every `eu:` predicate silently moves into the `cim:` namespace.
#[test]
fn namespace_fidelity() {
    let ds = full_grid(&["FullGrid_EQBD.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    let eu = column(
        &store,
        "SELECT ?n WHERE { ?s a eu:BoundaryPoint ; eu:BoundaryPoint.toEndName ?n }",
        "n",
    );
    assert!(!eu.is_empty(), "expected eu:BoundaryPoint.toEndName triples");

    // The same attribute must NOT be reachable under cim:.
    let wrong_ns = column(
        &store,
        "SELECT ?n WHERE { ?s cim:BoundaryPoint.toEndName ?n }",
        "n",
    );
    assert!(wrong_ns.is_empty(), "eu: attribute leaked into the cim: namespace");

    // ...while a cim: attribute on the very same eu:-typed subject stays under cim:.
    let cim = column(
        &store,
        "SELECT ?d WHERE { ?s a eu:BoundaryPoint ; cim:IdentifiedObject.description ?d }",
        "d",
    );
    assert!(!cim.is_empty(), "expected cim:IdentifiedObject.description on eu:BoundaryPoint");
}

/// Literals must carry their XSD datatype, or `FILTER(?r > 0.1)` compares lexically and
/// silently returns the wrong rows.
#[test]
fn literals_are_typed_numerically() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    let expected = ds
        .by_type
        .get("ACLineSegment")
        .unwrap()
        .iter()
        .filter(|m| {
            ds.entries[*m]
                .element
                .as_any()
                .downcast_ref::<cimstructs::ACLineSegment>()
                .and_then(|a| a.r)
                .is_some_and(|r| r > 0.1)
        })
        .count();
    assert!(expected > 0 && expected < ds.by_type["ACLineSegment"].len(), "need a mixed fixture");

    let n = column(
        &store,
        "SELECT (COUNT(*) AS ?n) WHERE { ?s cim:ACLineSegment.r ?r FILTER(?r > 0.1) }",
        "n",
    );
    assert_eq!(n, vec![expected.to_string()]);

    // A lexical comparison would also match "0.05"; a numeric one must not.
    let datatypes = column(
        &store,
        "SELECT ?t WHERE { ?s cim:ACLineSegment.r ?r BIND(DATATYPE(?r) AS ?t) } LIMIT 1",
        "t",
    );
    assert_eq!(datatypes, vec!["http://www.w3.org/2001/XMLSchema#double"]);
}

/// A two-hop join through an association must agree with walking `MridRef`s by hand.
#[test]
fn association_traversal() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    let via_sparql: HashSet<String> = column(
        &store,
        "SELECT ?e WHERE { ?t a cim:Terminal ; cim:Terminal.ConductingEquipment ?e .
                           ?e a cim:ACLineSegment }",
        "e",
    )
    .into_iter()
    .collect();

    let lines: HashSet<&String> = ds.by_type["ACLineSegment"].iter().collect();
    let via_structs: HashSet<String> = ds.by_type["Terminal"]
        .iter()
        .filter_map(|m| {
            ds.entries[m].element.as_any().downcast_ref::<cimstructs::Terminal>()
        })
        .filter_map(|t| t.conducting_equipment.as_ref())
        .map(|r| r.mrid.trim_start_matches('#').to_string())
        .filter(|m| lines.contains(m))
        .map(|m| cimsparql::iri::mrid_to_iri(&m))
        .collect();

    assert!(!via_structs.is_empty());
    assert_eq!(via_sparql, via_structs);
}

/// Enum values arrive fragment-stripped (`UnitSymbol.W`); the enum namespace must rebuild
/// the full IRI, otherwise no query can match one.
#[test]
fn enum_values_are_full_iris() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    let units = column(
        &store,
        "SELECT DISTINCT ?u WHERE { ?s cim:BasicIntervalSchedule.value1Unit ?u }",
        "u",
    );
    assert!(!units.is_empty());
    assert!(units.iter().all(|u| u.starts_with("http://iec.ch/TC57/CIM100#UnitSymbol.")));

    // Matching a specific enum value by IRI has to work.
    let QueryResults::Boolean(found) = store
        .query("ASK { ?s cim:BasicIntervalSchedule.value1Unit cim:UnitSymbol.W }")
        .unwrap()
    else {
        panic!("expected ASK result");
    };
    assert!(found);
}

/// `eu:LimitKind` and `eu:SVCControlMode` are enumerations that upstream `cims:stereotype`
/// parsing classifies as plain classes, so their values reach the graph as bare
/// `LimitKind.tatl` references. They must still land on the eu: namespace.
#[test]
fn misclassified_eu_enums_recover_their_namespace() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    let kinds = column(
        &store,
        "SELECT DISTINCT ?k WHERE { ?s eu:OperationalLimitType.kind ?k }",
        "k",
    );
    assert!(!kinds.is_empty());
    assert!(
        kinds.iter().all(|k| k.starts_with("http://iec.ch/TC57/CIM100-European#LimitKind.")),
        "got {kinds:?}"
    );
}

#[test]
fn ask_and_construct() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    let QueryResults::Boolean(yes) = store.query("ASK { ?s a cim:ACLineSegment }").unwrap() else {
        panic!("expected ASK result");
    };
    assert!(yes);

    let QueryResults::Boolean(no) = store.query("ASK { ?s a cim:NoSuchClass }").unwrap() else {
        panic!("expected ASK result");
    };
    assert!(!no);

    let QueryResults::Graph(triples) = store
        .query("CONSTRUCT { ?s a cim:Line } WHERE { ?s a cim:ACLineSegment }")
        .unwrap()
    else {
        panic!("expected CONSTRUCT results");
    };
    let n = triples.count();
    assert_eq!(n, ds.by_type["ACLineSegment"].len());
}

/// A `sh:sparql` constraint taken verbatim from the CGMES SHACL files, with SHACL's `$this`
/// and `$PATH` substituted the way a validator would. This is the shape `cimgen` currently
/// discards and `cimvalidation/src/sparql/` reimplements by hand.
#[test]
fn entsoe_sparql_constraint_shape() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let store = CimStore::from_dataset(&ds).unwrap();

    // equ:Equipment.aggregate-notUsedSparql, C:301:EQ:Equipment.aggregate:notUsed —
    // "the attribute is not used for EquivalentBranch, EquivalentShunt, EquivalentInjection".
    let query = |class: &str| {
        format!(
            "SELECT ?this ?value
             WHERE {{
               ?this a {class} .
               OPTIONAL {{ ?this cim:Equipment.aggregate ?value }} .
               FILTER (bound(?value)) .
             }}"
        )
    };

    // Same semantics as the hand-written check: no violations on this fixture.
    for class in ["cim:EquivalentBranch", "cim:EquivalentShunt", "cim:EquivalentInjection"] {
        assert!(
            column(&store, &query(class), "this").is_empty(),
            "unexpected violation for {class}"
        );
    }

    // Point the same query shape at a class that does carry the attribute, so the empty
    // results above are evidence the query works rather than that it matches nothing.
    //
    // Note the constraint's actual semantics: `OPTIONAL { ?this $PATH ?value } FILTER(bound(?value))`
    // flags the attribute being *present*, whatever its value. The hand-written counterpart
    // in `cimvalidation/src/sparql/equipment.rs` instead tests the decoded boolean
    // (`aggregate.unwrap_or(false)`), so the two disagree wherever the attribute is present
    // and `false` — 122 of the 125 occurrences in this fixture. They agree on the three
    // classes the rule actually targets here, but this is the kind of divergence a migration
    // to engine-evaluated `sh:sparql` would have to reconcile.
    let flagged: HashSet<String> =
        column(&store, &query("cim:ACLineSegment"), "this").into_iter().collect();
    let expected: HashSet<String> = ds.by_type["ACLineSegment"]
        .iter()
        .filter(|m| {
            ds.entries[*m]
                .element
                .as_any()
                .downcast_ref::<cimstructs::ACLineSegment>()
                .is_some_and(|a| a.base.base.base.aggregate.is_some())
        })
        .map(|m| cimsparql::iri::mrid_to_iri(m))
        .collect();
    assert!(!expected.is_empty(), "fixture should have ACLineSegments carrying the attribute");
    assert_eq!(flagged, expected);
}

/// `drop_blocks()` frees the lossless field maps; materialisation must fall back to
/// `CimElement::to_block()` rather than producing an empty graph.
#[test]
fn works_after_drop_blocks() {
    let mut ds = full_grid(&["FullGrid_EQ.xml"]);
    let before = CimStore::from_dataset(&ds).unwrap().len().unwrap();

    ds.drop_blocks();
    let store = CimStore::from_dataset(&ds).unwrap();
    assert_eq!(store.stats().rebuilt_blocks, ds.entries.len());

    let after = store.len().unwrap();
    assert!(after > 0);
    // The typed structs cannot round-trip predicates they never modelled, so the rebuilt
    // graph is a subset — but it must still answer the same type query.
    assert!(after <= before);
    assert_eq!(
        column(&store, "SELECT ?s WHERE { ?s a cim:ACLineSegment }", "s").len(),
        ds.by_type["ACLineSegment"].len()
    );
}

/// Scoping is the main lever on memory for large datasets.
#[test]
fn include_types_scopes_the_graph() {
    let ds = full_grid(&["FullGrid_EQ.xml"]);
    let opts = GraphOptions::new().with_types(["ACLineSegment"]);
    let store = CimStore::from_dataset_with(&ds, &opts).unwrap();

    assert_eq!(store.stats().elements, ds.by_type["ACLineSegment"].len());
    assert!(column(&store, "SELECT ?s WHERE { ?s a cim:Terminal }", "s").is_empty());
    assert_eq!(
        column(&store, "SELECT ?s WHERE { ?s a cim:ACLineSegment }", "s").len(),
        ds.by_type["ACLineSegment"].len()
    );
}

/// Every predicate in the CGMES test configurations should be known to `ATTR_RDF`. A
/// non-zero count means the generated tables and the data have drifted apart.
#[test]
fn all_predicates_are_mapped() {
    let ds = full_grid(&[
        "FullGrid_EQ.xml",
        "FullGrid_EQBD.xml",
        "FullGrid_SSH.xml",
        "FullGrid_SV.xml",
        "FullGrid_TP.xml",
        "FullGrid_OP.xml",
        "FullGrid_SC.xml",
    ]);
    let store = CimStore::from_dataset(&ds).unwrap();
    assert_eq!(store.stats().unmapped_predicates, 0, "unmapped predicates: {:?}", store.stats());
}
