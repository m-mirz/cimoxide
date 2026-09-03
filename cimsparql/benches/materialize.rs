//! Cost of turning a decoded dataset into a queryable RDF graph.
//!
//! Methodology follows `cimdecoder/benches/real_grid.rs`: the dataset is decoded once
//! outside the timed loop, so only quad generation and store loading are measured.
//!
//! Materialisation holds the graph *in addition to* the typed structs and `RdfBlock`s it was
//! built from, so the quad count reported here is the number to reason about for memory.

use std::path::Path;

use cimdecoder::CimDataset;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use cimsparql::{CimStore, GraphOptions, Stats, quads};

const BASE: &str = "../CGMES-Test-Configurations/v3.0/RealGrid/RealGrid-Merged";

fn load() -> CimDataset {
    let names = ["RealGrid_EQ.xml", "RealGrid_SSH.xml", "RealGrid_TP.xml", "RealGrid_SV.xml"];
    let paths: Vec<std::path::PathBuf> = names.iter().map(|n| Path::new(BASE).join(n)).collect();
    let refs: Vec<&Path> = paths.iter().map(AsRef::as_ref).collect();
    CimDataset::decode_files_parallel(&refs).expect("decode RealGrid")
}

fn bench_quads(c: &mut Criterion) {
    let ds = load();
    let opts = GraphOptions::new();

    let mut probe = Stats::default();
    let count = quads(&ds, &opts, &mut probe).count() as u64;
    eprintln!("RealGrid: {} elements -> {count} quads", ds.entries.len());

    let mut group = c.benchmark_group("materialize");
    group.sample_size(10);
    group.throughput(Throughput::Elements(count));

    // Quad generation alone: the mapping cost, without the store's indexing.
    group.bench_function("quads_only", |b| {
        b.iter(|| {
            let mut stats = Stats::default();
            std::hint::black_box(quads(&ds, &opts, &mut stats).count())
        })
    });

    // The full path a caller pays for: generate, intern, and index.
    group.bench_function("into_store", |b| {
        b.iter(|| std::hint::black_box(CimStore::from_dataset(&ds).unwrap().len().unwrap()))
    });

    group.finish();
}

fn bench_query(c: &mut Criterion) {
    let ds = load();
    let store = CimStore::from_dataset(&ds).expect("materialize");

    let mut group = c.benchmark_group("query");
    group.sample_size(10);

    group.bench_function("count_by_type", |b| {
        b.iter(|| {
            std::hint::black_box(
                store.query("SELECT (COUNT(?s) AS ?n) WHERE { ?s a cim:ACLineSegment }").unwrap(),
            )
        })
    });

    // A two-hop join, the shape most CGMES conformance queries take.
    group.bench_function("terminal_join", |b| {
        b.iter(|| {
            std::hint::black_box(
                store
                    .query(
                        "SELECT (COUNT(*) AS ?n) WHERE {
                           ?t a cim:Terminal ; cim:Terminal.ConductingEquipment ?e .
                           ?e a cim:ACLineSegment }",
                    )
                    .unwrap(),
            )
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_quads, bench_query
}
criterion_main!(benches);
