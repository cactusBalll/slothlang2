#!/bin/bash
# TE-P2 matvec benchmark: sloth2 AOT (`slothc build`, clang -O3) vs the
# `gcc -O3` reference in `matvec.c`. Reports the sloth/reference ratio against
# the design §8.2 target (>= 0.7x).
#
# Usage:  examples/tensor/run.sh
# Env:    SLOTHC=/path/to/slothc   (defaults to target/debug/slothc)
#         REPEATS=n               (wall-time runs to take the min of)
set -u
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
SLOTHC="${SLOTHC:-$root/target/debug/slothc}"
REPEATS="${REPEATS:-3}"
if [ ! -x "$SLOTHC" ]; then
    echo "slothc not found at $SLOTHC (run 'cargo build' first)"
    exit 2
fi
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cd "$here"

gcc -O3 -o "$tmp/matvec.c.bin" matvec.c || exit 1
"$SLOTHC" build matvec.sl "$tmp/matvec.sl.bin" > /dev/null || exit 1

timeit() {
    local best=""
    for _ in $(seq "$REPEATS"); do
        local s e
        s=$(date +%s%N)
        "$1" > /dev/null
        e=$(date +%s%N)
        local d=$(( (e - s) / 1000000 ))
        if [ -z "$best" ] || [ "$d" -lt "$best" ]; then best="$d"; fi
    done
    echo "$best"
}

ref_ms=$(timeit "$tmp/matvec.c.bin")
sloth_ms=$(timeit "$tmp/matvec.sl.bin")
echo "reference (gcc -O3): ${ref_ms} ms"
echo "sloth2 AOT (clang -O3): ${sloth_ms} ms"

ratio=$(awk -v r="$ref_ms" -v s="$sloth_ms" 'BEGIN { printf "%.2f", r / s }')
echo "matvec ratio: ${ratio}x (target >= 0.70x)"

# fusion benchmark: fused rmsnorm vs the naive multi-op decomposition (§8.2)
"$SLOTHC" build rmsnorm_fused.sl "$tmp/fused.bin" > /dev/null || exit 1
"$SLOTHC" build rmsnorm_naive.sl "$tmp/naive.bin" > /dev/null || exit 1
fused_ms=$(timeit "$tmp/fused.bin")
naive_ms=$(timeit "$tmp/naive.bin")
echo "rmsnorm fused: ${fused_ms} ms, naive: ${naive_ms} ms"
fratio=$(awk -v n="$naive_ms" -v f="$fused_ms" 'BEGIN { printf "%.2f", n / f }')
echo "fusion speedup: ${fratio}x (target >= 1.30x)"

awk -v r="$ratio" -v f="$fratio" \
    'BEGIN { exit (r >= 0.70 && f >= 1.30) ? 0 : 1 }'
