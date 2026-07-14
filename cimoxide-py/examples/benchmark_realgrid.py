"""Benchmark decode/encode/validate throughput against the RealGrid CGMES dataset.

Usage:
    python examples/benchmark_realgrid.py [iterations]
"""

import os
import sys
import tempfile
import time

import cimoxide

REALGRID = os.path.join(os.path.dirname(__file__), "..", "..", "CGMES-Test-Configurations", "v3.0", "RealGrid", "RealGrid-Merged")
PROFILES = ["EQ", "SSH", "TP", "SV"]
PATHS = [os.path.join(REALGRID, f"RealGrid_{p}.xml") for p in PROFILES]

ITERATIONS = int(sys.argv[1]) if len(sys.argv) > 1 else 3


def total_bytes(paths):
    return sum(os.path.getsize(p) for p in paths)


def timeit(label, fn):
    times = []
    result = None
    for _ in range(ITERATIONS):
        start = time.perf_counter()
        result = fn()
        times.append(time.perf_counter() - start)
    best, mean = min(times), sum(times) / len(times)
    print(f"{label:<10} best={best:7.3f}s  mean={mean:7.3f}s  ({ITERATIONS} run(s))")
    return result, best


def main():
    input_bytes = total_bytes(PATHS)
    print(f"RealGrid input: {len(PATHS)} files, {input_bytes / 1e6:.1f} MB\n")

    ds, decode_best = timeit("decode", lambda: cimoxide.decode_files(PATHS))
    print(f"  -> {len(ds):,} objects, {input_bytes / decode_best / 1e6:.1f} MB/s\n")

    out_dir = tempfile.mkdtemp(prefix="cimoxide-bench-")
    _, encode_best = timeit("encode", lambda: ds.write_xml_files(out_dir, PROFILES))
    output_bytes = sum(os.path.getsize(os.path.join(out_dir, f"{p}.xml")) for p in PROFILES)
    print(f"  -> {output_bytes / 1e6:.1f} MB written, {output_bytes / encode_best / 1e6:.1f} MB/s\n")

    violations, validate_best = timeit("validate", lambda: cimoxide.validate_files(PATHS))
    print(f"  -> {len(violations):,} violation(s), {input_bytes / validate_best / 1e6:.1f} MB/s\n")


if __name__ == "__main__":
    main()
