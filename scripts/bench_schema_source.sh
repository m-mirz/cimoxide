#!/usr/bin/env bash
#
# Measures what loading the NC class table from RDFS at runtime costs, against
# using the table cimgen generated.
#
# Four numbers, because they answer different questions:
#
#   1. decode throughput   — is *normal operation* slower? (criterion A/B)
#   2. startup wall clock  — what does a one-shot CLI run pay? (the real cost)
#   3. peak RSS            — the generated table is rodata; the loaded one is heap
#   4. binary size         — the generated table stays as the fallback, so this
#                            should not move; measured to confirm that
#
# The decode A/B runs both halves back to back through criterion named
# baselines. Comparing separate runs on a loaded machine is not reliable: doing
# exactly that earlier in this project's history produced a 20% phantom
# regression that vanished under a controlled run.
#
# Usage:
#   scripts/bench_schema_source.sh [-n repeat] [-r rdfs-dir] [--skip-decode]

set -euo pipefail

repeat=5
rdfs_dir=""
skip_decode=0

while [ $# -gt 0 ]; do
	case "$1" in
	-n)
		repeat="$2"
		shift 2
		;;
	-r)
		rdfs_dir="$2"
		shift 2
		;;
	--skip-decode)
		skip_decode=1
		shift
		;;
	-h | --help)
		echo "Usage: $0 [-n repeat] [-r rdfs-dir] [--skip-decode]"
		exit 0
		;;
	*)
		echo "Unknown argument: $1" >&2
		exit 1
		;;
	esac
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [ -z "$rdfs_dir" ]; then
	rdfs_dir="$repo_root/application-profiles-library/NCP/RDFS"
fi

if [ ! -d "$rdfs_dir" ]; then
	echo "NCP RDFS directory not found: $rdfs_dir" >&2
	echo "(did you run 'git submodule update --init --recursive'?)" >&2
	exit 1
fi

cd "$repo_root"

rdfs_bytes=$(find "$rdfs_dir" -maxdepth 1 -name '*.rdf' -printf '%s\n' | paste -sd+ | bc)
rdfs_files=$(find "$rdfs_dir" -maxdepth 1 -name '*.rdf' | wc -l)
echo "RDFS source: $rdfs_files files, $rdfs_bytes bytes"
echo

# --- 1. decode throughput, generated vs RDFS -------------------------------

if [ "$skip_decode" -eq 0 ]; then
	echo "=== 1. decode throughput (criterion A/B, back to back) ==="
	echo "--- baseline: generated table ---"
	env -u CIMOXIDE_RDFS_DIR cargo bench -p cimoxide-decoder --bench nc_decode -- \
		--save-baseline generated 2>&1 | grep -E 'time:|thrpt:' || true
	echo "--- comparison: RDFS-loaded table ---"
	CIMOXIDE_RDFS_DIR="$rdfs_dir" cargo bench -p cimoxide-decoder --bench nc_decode -- \
		--baseline generated 2>&1 | grep -E 'time:|thrpt:|change:|Performance|No change' || true
	echo
fi

# --- 2/3. startup wall clock and peak RSS ----------------------------------

echo "Building cimoxide-cli (release)..."
cargo build --release -p cimoxide-cli >/dev/null 2>&1
cimcli="$repo_root/target/release/cimcli"
fixture="$repo_root/testdata/test_nc_CO_001.xml"

now_ns() { date +%s%N; }

median() {
	printf '%s\n' "$@" | sort -n | awk '{a[NR]=$1} END {print (NR%2) ? a[(NR+1)/2] : int((a[NR/2]+a[NR/2+1])/2)}'
}

# A three-element fixture is the point here: it isolates startup from decoding.
echo "=== 2. startup wall clock — 'cimcli import' on a 3-element file ==="
gen_ms=()
dyn_ms=()
for ((i = 1; i <= repeat; i++)); do
	start=$(now_ns)
	env -u CIMOXIDE_RDFS_DIR "$cimcli" import "$fixture" >/dev/null
	end=$(now_ns)
	gen_ms+=($(((end - start) / 1000000)))

	start=$(now_ns)
	CIMOXIDE_RDFS_DIR="$rdfs_dir" "$cimcli" import "$fixture" >/dev/null
	end=$(now_ns)
	dyn_ms+=($(((end - start) / 1000000)))
done
g=$(median "${gen_ms[@]}")
d=$(median "${dyn_ms[@]}")
echo "  generated table : ${g} ms  (runs: ${gen_ms[*]})"
echo "  RDFS-loaded     : ${d} ms  (runs: ${dyn_ms[*]})"
echo "  difference      : +$((d - g)) ms per process"
echo

echo "=== 3. peak RSS ==="
if command -v /usr/bin/time >/dev/null 2>&1; then
	g_rss=$(env -u CIMOXIDE_RDFS_DIR /usr/bin/time -f '%M' "$cimcli" import "$fixture" 2>&1 >/dev/null | tail -1)
	d_rss=$(CIMOXIDE_RDFS_DIR="$rdfs_dir" /usr/bin/time -f '%M' "$cimcli" import "$fixture" 2>&1 >/dev/null | tail -1)
	echo "  generated table : ${g_rss} KB"
	echo "  RDFS-loaded     : ${d_rss} KB"
	echo "  difference      : +$((d_rss - g_rss)) KB"
else
	echo "  (/usr/bin/time not available, skipped)"
fi
echo

echo "=== 4. binary size ==="
echo "  cimcli: $(stat -c '%s' "$cimcli") bytes"
echo "  (the generated table remains compiled in as the fallback, so this is"
echo "   expected to be unchanged by the feature)"
