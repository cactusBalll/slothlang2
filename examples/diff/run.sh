#!/bin/bash
# Differential runner: slothlang2 JIT (`slothc run`) vs `gcc -O2` for the
# classic-algorithm suite. Integer algorithms must match byte-for-byte; the
# float reproducers are informational (see PLAN §8).
#
# Usage:  examples/diff/run.sh
# Env:    SLOTHC=/path/to/slothc   (defaults to target/debug/slothc)
set -u
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
SLOTHC="${SLOTHC:-$root/target/debug/slothc}"
if [ ! -x "$SLOTHC" ]; then
    echo "slothc not found at $SLOTHC (run 'cargo build' first)"
    exit 2
fi
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cd "$here"

match=0
fail=0
for algo in kmp nqueens dijkstra matmul quicksort edge corner json; do
    if ! gcc -O2 -o "$tmp/$algo.bin" "$algo.c" 2> "$tmp/$algo.cc.log"; then
        echo "$algo: GCC FAIL"
        cat "$tmp/$algo.cc.log"
        fail=$((fail + 1))
        continue
    fi
    "$tmp/$algo.bin" > "$tmp/$algo.c.out" 2>&1
    "$SLOTHC" run "$algo.sl" > "$tmp/$algo.sl.raw" 2>&1
    grep -v '^invokePacked' "$tmp/$algo.sl.raw" | grep -v '^SLOTH' > "$tmp/$algo.sl.out"
    if diff -q "$tmp/$algo.c.out" "$tmp/$algo.sl.out" > /dev/null; then
        echo "$algo: MATCH ($(wc -l < "$tmp/$algo.c.out") lines)"
        match=$((match + 1))
    else
        echo "$algo: DIFF"
        diff "$tmp/$algo.c.out" "$tmp/$algo.sl.out" | head -20
        fail=$((fail + 1))
    fi
done
echo "integer differentials: $match match, $fail differ"

echo "-- float reproducers (expected to differ: word-plane f64 LSB loss) --"
for algo in floatdiff fmatmul; do
    gcc -O2 -o "$tmp/$algo.bin" "$algo.c" 2> /dev/null
    "$tmp/$algo.bin" > "$tmp/$algo.c.out" 2>&1
    "$SLOTHC" run "$algo.sl" > "$tmp/$algo.sl.raw" 2>&1
    grep -v '^invokePacked' "$tmp/$algo.sl.raw" | grep -v '^SLOTH' > "$tmp/$algo.sl.out"
    nd="$(diff "$tmp/$algo.c.out" "$tmp/$algo.sl.out" | grep -c '^<')"
    echo "$algo: $nd differing lines / $(wc -l < "$tmp/$algo.c.out")"
done

exit $fail
