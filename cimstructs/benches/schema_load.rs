//! What loading the NC class table from RDFS costs.
//!
//! Run with the feature the loader lives behind:
//!
//!     cargo bench -p cimoxide-structs --features dynamic-schema --bench schema_load
//!
//! `full_load` interns and leaks a table per iteration, which is why its sample
//! size is pinned low: criterion's default would leak hundreds of tables and
//! perturb the allocator it is trying to measure.

use std::path::Path;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

const RDFS_DIR: &str = "../application-profiles-library/NCP/RDFS";

fn rdfs_bytes() -> u64 {
    std::fs::read_dir(RDFS_DIR)
        .expect("NCP RDFS not found — did you init the submodules?")
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "rdf"))
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

fn bench(c: &mut Criterion) {
    let bytes = rdfs_bytes();
    let mut group = c.benchmark_group("schema_load");
    group.throughput(Throughput::Bytes(bytes));

    // Parse + postprocess only. Allocates but does not leak, so this one can
    // run at criterion's normal sample size.
    group.bench_function("import_and_postprocess", |b| {
        b.iter(|| {
            cimschema::import::import_schema_files(
                &format!("{RDFS_DIR}/*-AP-Voc-RDFS2020.rdf"),
                &cimschema::family::NC,
                false,
            )
            .unwrap()
        })
    });

    group.finish();

    // The whole thing, including interning and leaking the ClassDef table. The
    // difference against the above is what building the table costs on top of
    // parsing.
    let mut leaky = c.benchmark_group("schema_load_leaky");
    leaky.throughput(Throughput::Bytes(bytes));
    leaky.sample_size(10);
    leaky.warm_up_time(Duration::from_millis(500));
    leaky.bench_function("full_load", |b| {
        b.iter(|| cimstructs::schema_source::load_table("nc", Path::new(RDFS_DIR)).unwrap())
    });
    leaky.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
