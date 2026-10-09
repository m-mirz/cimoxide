#!/usr/bin/env bash
#
# Measures NC validation: what it costs to run, and what loading the shapes
# from SHACL at runtime adds on top of using the table cimgen generated.
#
# The sibling of scripts/bench_schema_source.sh, and it answers a different
# question. There, a decoder already existed and the question was whether the
# dynamic path made it slower. Here there was no NC validation at all, so the
# first number to establish is the absolute cost of interpreting ~14,800 checks
# — specifically whether it lands near CGMES validation or an order off. (It
# did, and CGMES has since moved to a table too, replacing ~250,000 lines of
# generated per-check functions.)
#
# Five sections:
#
#   1. validation throughput  — the absolute cost (criterion)
#   2. CGMES comparison       — same machine, CGMES FullGrid through its table
#   3. startup wall clock     — what a one-shot CLI run pays for the TTL load
#   4. peak RSS               — the generated table is rodata, the loaded one heap
#   5. table size             — generated source, and whether the binary moves
#
# The A/B in section 3 runs both halves back to back. Comparing separate runs on
# a loaded machine is not reliable: doing exactly that earlier in this project's
# history produced a 20% phantom regression that vanished under control.
#
# Usage:
#   scripts/bench_shape_source.sh [-n repeat] [-s shacl-dir] [--skip-criterion]

set -euo pipefail

repeat=5
shacl_dir=""
skip_criterion=0

while [ $# -gt 0 ]; do
	case "$1" in
	-n)
		repeat="$2"
		shift 2
		;;
	-s)
		shacl_dir="$2"
		shift 2
		;;
	--skip-criterion)
		skip_criterion=1
		shift
		;;
	-h | --help)
		echo "Usage: $0 [-n repeat] [-s shacl-dir] [--skip-criterion]"
		exit 0
		;;
	*)
		echo "Unknown argument: $1" >&2
		exit 1
		;;
	esac
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [ -z "$shacl_dir" ]; then
	shacl_dir="$repo_root/application-profiles-library/NCP/SHACL"
fi

if [ ! -d "$shacl_dir" ]; then
	echo "NCP SHACL directory not found: $shacl_dir" >&2
	echo "(did you run 'git submodule update --init --recursive'?)" >&2
	exit 1
fi

cd "$repo_root"

ttl_bytes=$(find "$shacl_dir" -maxdepth 1 -name '*.ttl' -printf '%s\n' | paste -sd+ | bc)
ttl_files=$(find "$shacl_dir" -maxdepth 1 -name '*.ttl' | wc -l)
echo "SHACL source: $ttl_files files, $ttl_bytes bytes"
echo

# --- 1. validation throughput ----------------------------------------------

if [ "$skip_criterion" -eq 0 ]; then
	echo "=== 1. NC validation throughput ==="
	cargo bench -p cimoxide-validation --bench nc_validate 2>&1 |
		grep -E 'benchmarking|nc_validate|time:|thrpt:' || true
	echo

	echo "=== 1a. validation throughput, generated vs SHACL-loaded (A/B) ==="
	echo "--- baseline: generated table ---"
	env -u CIMOXIDE_SHACL_DIR cargo bench -p cimoxide-validation --features dynamic-shapes \
		--bench nc_validate -- --save-baseline generated 2>&1 |
		grep -E 'time:|thrpt:' || true
	echo "--- comparison: SHACL-loaded table ---"
	CIMOXIDE_SHACL_DIR="$shacl_dir" cargo bench -p cimoxide-validation --features dynamic-shapes \
		--bench nc_validate -- --baseline generated 2>&1 |
		grep -E 'time:|thrpt:|change:|Performance|No change' || true
	echo

	echo "=== 1b. shape load, split by stage ==="
	cargo bench -p cimoxide-validation --features dynamic-shapes --bench shape_load 2>&1 |
		grep -E 'SHACL files|parse_ttl|resolve_shapes|full_load|time:' || true
	echo
fi

# --- 2. the CGMES comparison ------------------------------------------------

echo "Building cimoxide-cli (release)..."
cargo build --release -p cimoxide-cli >/dev/null 2>&1
cimcli="$repo_root/target/release/cimcli"

now_ns() { date +%s%N; }

median() {
	printf '%s\n' "$@" | sort -n |
		awk '{a[NR]=$1} END {print (NR%2) ? a[(NR+1)/2] : int((a[NR/2]+a[NR/2+1])/2)}'
}

cgmes_dir="$repo_root/CGMES-Test-Configurations/v3.0/FullGrid/FullGrid-Merged"
echo "=== 2. CGMES validation, for scale ==="
if [ -d "$cgmes_dir" ]; then
	cg_ms=()
	for ((i = 1; i <= repeat; i++)); do
		start=$(now_ns)
		"$cimcli" validate "$cgmes_dir"/*.xml >/dev/null 2>&1 || true
		end=$(now_ns)
		cg_ms+=($(((end - start) / 1000000)))
	done
	echo "  cimcli validate, 7 CGMES files : $(median "${cg_ms[@]}") ms  (runs: ${cg_ms[*]})"
	echo "  (CGMES runs the same interpreter over cgmes_shapes.rs)"
else
	echo "  (CGMES-Test-Configurations not present, skipped)"
fi
echo

# --- 3/4. startup wall clock and peak RSS ----------------------------------

fixture="$repo_root/testdata/test_nc_CO_002.xml"

# A four-element fixture is the point: it isolates the shape load from the
# validation itself.
echo "=== 3. startup wall clock — 'cimcli validate' on a 4-element NC file ==="
gen_ms=()
dyn_ms=()
for ((i = 1; i <= repeat; i++)); do
	start=$(now_ns)
	env -u CIMOXIDE_SHACL_DIR "$cimcli" validate "$fixture" >/dev/null 2>&1 || true
	end=$(now_ns)
	gen_ms+=($(((end - start) / 1000000)))

	start=$(now_ns)
	CIMOXIDE_SHACL_DIR="$shacl_dir" "$cimcli" validate "$fixture" >/dev/null 2>&1 || true
	end=$(now_ns)
	dyn_ms+=($(((end - start) / 1000000)))
done
g=$(median "${gen_ms[@]}")
d=$(median "${dyn_ms[@]}")
echo "  generated table : ${g} ms  (runs: ${gen_ms[*]})"
echo "  SHACL-loaded    : ${d} ms  (runs: ${dyn_ms[*]})"
echo "  difference      : +$((d - g)) ms per process"
echo "  (the loaded path also re-imports the RDFS the shapes resolve against,"
echo "   so this includes the class-table load measured by bench_schema_source.sh)"
echo

echo "=== 4. peak RSS ==="
if command -v /usr/bin/time >/dev/null 2>&1; then
	g_rss=$(env -u CIMOXIDE_SHACL_DIR /usr/bin/time -f '%M' "$cimcli" validate "$fixture" 2>&1 >/dev/null | tail -1)
	d_rss=$(CIMOXIDE_SHACL_DIR="$shacl_dir" /usr/bin/time -f '%M' "$cimcli" validate "$fixture" 2>&1 >/dev/null | tail -1)
	echo "  generated table : ${g_rss} KB"
	echo "  SHACL-loaded    : ${d_rss} KB"
	echo "  difference      : +$((d_rss - g_rss)) KB"
else
	echo "  (/usr/bin/time not available, skipped)"
fi
echo

# --- 5. table and binary size ----------------------------------------------

echo "=== 5. table and binary size ==="
for f in cimvalidation/src/nc_shapes.rs cimvalidation/src/nc_profiles.rs cimvalidation/src/cgmes_shapes.rs; do
	if [ -f "$f" ]; then
		printf '  %-34s %s bytes\n' "$f" "$(stat -c '%s' "$f")"
	fi
done
echo "  cimcli: $(stat -c '%s' "$cimcli") bytes"
echo "  (the generated table stays compiled in as the fallback, so the binary is"
echo "   expected to be unchanged by the dynamic-shapes feature)"
