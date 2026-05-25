//! Benchmark methodology matches the Go BenchmarkRealGridLoad* benchmarks in
//! cimgo/validation/cgmes_config_test.go: files are pre-loaded into memory
//! before the timed loop so only parse/decode time is measured, not I/O.

use std::path::Path;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use cimdecoder::CimDataset;

const BASE: &str = "../CGMES-Test-Configurations/v3.0/RealGrid/RealGrid-Merged";

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

/// Decode all four files in parallel using one thread per file, then merge sequentially.
/// Uses pre-loaded in-memory blobs to measure only parse/decode time, not I/O.
fn bench_parallel(c: &mut Criterion) {
    let blobs = load_files();
    let total_bytes: u64 = blobs.iter().map(|(_, s)| s.len() as u64).sum();

    let mut group = c.benchmark_group("decode_parallel");
    group.throughput(Throughput::Bytes(total_bytes));

    group.bench_function("all_four_profiles", |b| {
        b.iter(|| {
            let datasets: Vec<CimDataset> = std::thread::scope(|s| {
                blobs
                    .iter()
                    .map(|(_, content)| s.spawn(|| CimDataset::decode_str(content).unwrap()))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|h| h.join().expect("decode thread panicked"))
                    .collect()
            });
            datasets
                .into_iter()
                .reduce(|mut a, b| {
                    a.merge(b);
                    a
                })
                .unwrap()
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

/// Decode all four files in parallel from disk using actual file paths, matching
/// what `cimoxide-cli import` does via `decode_files_parallel`.
/// Files land in the OS page cache after the warmup phase so I/O is warm but
/// the full decode-from-disk path (open, BufRead, parse, instantiate) is exercised.
fn bench_import(c: &mut Criterion) {
    let base = Path::new("../CGMES-Test-Configurations/v3.0/RealGrid/RealGrid-Merged");
    let names = ["RealGrid_EQ.xml", "RealGrid_SSH.xml", "RealGrid_TP.xml", "RealGrid_SV.xml"];
    let paths: Vec<std::path::PathBuf> = names.iter().map(|n| base.join(n)).collect();
    let total_bytes: u64 = paths.iter()
        .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
        .sum();

    let mut group = c.benchmark_group("import");
    group.throughput(Throughput::Bytes(total_bytes));

    group.bench_function("parallel_four_profiles", |b| {
        b.iter(|| {
            let refs: Vec<&Path> = paths.iter().map(|p| p.as_path()).collect();
            CimDataset::decode_files_parallel(&refs).unwrap()
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_parallel, bench_per_profile, bench_import
}
criterion_main!(benches);
