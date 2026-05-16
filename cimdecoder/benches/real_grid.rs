//! Benchmark methodology matches the Go BenchmarkRealGridLoad* benchmarks in
//! cimgo/validation/cgmes_config_test.go: files are pre-loaded into memory
//! before the timed loop so only parse/decode time is measured, not I/O.

use std::path::Path;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use cimdecoder::CimDataset;

const BASE: &str = "../cimgo/CGMES-Test-Configurations/v3.0/RealGrid/RealGrid-Merged";

fn load_files() -> Vec<(String, String)> {
    ["RealGrid_EQ.xml", "RealGrid_SSH.xml", "RealGrid_TP.xml", "RealGrid_SV.xml"]
        .iter()
        .map(|name| {
            let content = std::fs::read_to_string(Path::new(BASE).join(name))
                .unwrap_or_else(|e| panic!("cannot read {name}: {e}"));
            (name.to_string(), content)
        })
        .collect()
}

/// Decode each file independently (sequential, like BenchmarkRealGridLoadSequential).
fn bench_sequential(c: &mut Criterion) {
    let blobs = load_files();
    let total_bytes: u64 = blobs.iter().map(|(_, s)| s.len() as u64).sum();

    let mut group = c.benchmark_group("decode_sequential");
    group.throughput(Throughput::Bytes(total_bytes));

    group.bench_function("all_four_profiles", |b| {
        b.iter(|| {
            let mut ds = CimDataset::new();
            for (_, content) in &blobs {
                ds.merge(CimDataset::decode_str(content).unwrap());
            }
            ds
        })
    });

    group.finish();
}

/// Decode individual profiles to show per-file throughput.
fn bench_per_profile(c: &mut Criterion) {
    let blobs = load_files();
    let mut group = c.benchmark_group("decode_single");

    for (name, content) in &blobs {
        let label = name.trim_end_matches(".xml").trim_start_matches("RealGrid_");
        let bytes = content.len() as u64;
        group.throughput(Throughput::Bytes(bytes));
        group.bench_with_input(
            BenchmarkId::new("profile", label),
            content,
            |b, c| b.iter(|| CimDataset::decode_str(c).unwrap()),
        );
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_sequential, bench_per_profile
}
criterion_main!(benches);
