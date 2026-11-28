//! Several datasets in one store, each in its own named graph; NC's data in NC's namespaces.

use std::path::Path;

use cimmodel::CimDataset;
use cimsparql::{CimStore, QueryResults};

fn decode(name: &str) -> CimDataset {
    CimDataset::decode_file(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../testdata")
            .join(name),
    )
    .unwrap()
}

fn count(store: &CimStore, sparql: &str, graphs: &[&str]) -> usize {
    match store.query_in(sparql, graphs).unwrap() {
        QueryResults::Solutions(s) => s.count(),
        _ => panic!("not a SELECT"),
    }
}

const EQ: &str = "urn:test:eq";
const CO: &str = "urn:test:co";

#[test]
fn a_query_reads_the_graphs_it_is_given() {
    let (eq, co) = (
        decode("test_sparql_EQ_001.xml"),
        decode("test_nc_CO_001.xml"),
    );
    let store = CimStore::from_datasets(&[(Some(EQ), &eq), (Some(CO), &co)]).unwrap();
    let all = "SELECT ?s WHERE { ?s a ?type }";
    let (in_eq, in_co, both) = (
        count(&store, all, &[EQ]),
        count(&store, all, &[CO]),
        count(&store, all, &[]),
    );
    assert_eq!(in_eq, eq.entries.len());
    assert_eq!(in_co, co.entries.len());
    assert_eq!(both, in_eq + in_co);
}

/// NC binds `cim:` to `https://cim.ucaiug.io/ns#` and keeps its own classes in `nc:`; a
/// query written for NC data finds it only if the store writes it there.
#[test]
fn nc_data_lands_in_nc_namespaces() {
    let co = decode("test_nc_CO_001.xml");
    let store = CimStore::from_datasets(&[(None, &co)]).unwrap();
    let contingencies = "PREFIX nc: <https://cim4.eu/ns/nc#>
        PREFIX cim: <https://cim.ucaiug.io/ns#>
        SELECT ?c WHERE { ?c a nc:OrdinaryContingency ; cim:IdentifiedObject.name ?n }";
    assert_eq!(count(&store, contingencies, &[]), 1);
    let cgmes = "PREFIX cim: <http://iec.ch/TC57/CIM100#>
        SELECT ?c WHERE { ?c cim:IdentifiedObject.name ?n }";
    assert_eq!(count(&store, cgmes, &[]), 0);
}

/// A triple two files both state is one triple of the merge, as a query counting it
/// expects; read as a union of the files' graphs it would match twice.
#[test]
fn the_merge_holds_a_triple_once() {
    let eq = decode("test_sparql_EQ_001.xml");
    let store = CimStore::from_datasets(&[(Some(EQ), &eq), (Some("urn:test:again"), &eq)]).unwrap();
    let all = "SELECT ?s WHERE { ?s a ?type }";
    assert_eq!(count(&store, all, &[]), eq.entries.len());
    assert_eq!(count(&store, all, &[EQ]), eq.entries.len());
}
