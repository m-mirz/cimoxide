//! What loading the NC shape table from SHACL costs.
//!
//! Split so the answer is actionable: parsing 4.5 MB of Turtle, resolving the
//! shapes against the class table, and interning the result are three
//! different costs with three different fixes.
//!
//! Interning **leaks**, once per load, which is what makes the `&'static` IR
//! possible. Criterion runs hundreds of iterations, so the full load is
//! measured with a small sample size and the rest on owned data — leaking per
//! iteration would leak gigabytes and perturb the allocator, producing numbers
//! that say nothing about a real process.

use std::path::{Path, PathBuf};

use cimschema::family;
use cimschema::shacl::skip::SkipCollector;
use cimschema::shacl::{resolve, ttl_import};
use criterion::{criterion_group, criterion_main, Criterion};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn shacl_dir() -> PathBuf {
    root().join("application-profiles-library/NCP/SHACL")
}

fn ttl_paths() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(shacl_dir())
        .expect("NCP SHACL directory not found — run `git submodule update --init`")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("ttl"))
        .collect();
    paths.sort();
    paths
}

fn bench(c: &mut Criterion) {
    let paths = ttl_paths();
    let bytes: u64 = paths.iter().filter_map(|p| p.metadata().ok()).map(|m| m.len()).sum();
    eprintln!("{} SHACL files, {bytes} bytes", paths.len());

    // 1. Turtle parse alone — expected to dominate.
    let mut group = c.benchmark_group("shape_load");
    group.sample_size(20);
    group.bench_function("parse_ttl", |b| {
        b.iter(|| {
            let mut n = 0;
            for p in &paths {
                n += ttl_import::import_ttl_file(p).expect("parse failed").shapes.len();
            }
            n
        })
    });

    // 2. Resolution against the class table, with parsing hoisted out. Needs
    //    the NC spec, which is itself a 3.65 MB RDFS parse — measured by
    //    cimstructs' schema_load bench, not repeated here.
    let spec = cimschema::import::import_schema_files(
        &root()
            .join("application-profiles-library/NCP/RDFS/*-AP-Voc-RDFS2020.rdf")
            .to_string_lossy(),
        &family::NC,
        false,
    )
    .expect("NC RDFS import failed");
    let cgmes = cimschema::import::import_schema_files(
        &root()
            .join("application-profiles-library/CGMES/RDFS/61970-600-2_*-AP-Voc-RDFS2020.rdf")
            .to_string_lossy(),
        &family::CGMES,
        false,
    )
    .expect("CGMES RDFS import failed");
    let others = vec![&cgmes];

    let profiles_of =
        resolve::profile_files(&shacl_dir().join("Validation")).expect("manifests failed");

    group.bench_function("resolve_shapes", |b| {
        b.iter_batched(
            || {
                let mut files: Vec<_> = paths
                    .iter()
                    .map(|p| ttl_import::import_ttl_file(p).expect("parse failed"))
                    .collect();
                cimschema::shacl::simplify::simplify(&mut files, &family::NC);
                files
            },
            |files| {
                let mut c = SkipCollector::new();
                resolve::resolve_shapes(&spec, &others, &files, &profiles_of, &mut c)
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();

    // 3. The whole thing, including the leak. Bounded to a handful of
    //    iterations: this is the number a one-shot CLI run actually pays.
    #[cfg(feature = "dynamic-shapes")]
    {
        let mut group = c.benchmark_group("shape_load_leaky");
        group.sample_size(10);
        group.warm_up_time(std::time::Duration::from_millis(500));
        group.bench_function("full_load", |b| {
            b.iter(|| {
                cimvalidation::shape_source::load_table("nc", &shacl_dir()).expect("load failed")
            })
        });
        group.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
