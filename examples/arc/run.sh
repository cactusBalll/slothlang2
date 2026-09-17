#!/bin/bash
# ARC stress runner. Each `arc_*.sl` prints `OK <name>` / `LEAK <name> <d>`
# per scenario and a final `sink=<live>`; a run passes when every scenario is
# OK, the overall baseline is zero and the process exits cleanly (no crash).
#
# The programs use `sloth_rc_live()` deltas, so they detect missing release
# counts (leaks) as well as over-release (which crashes the allocator).
#
# Usage:  examples/arc/run.sh
# Env:    SLOTHC=/path/to/slothc   (defaults to target/debug/slothc)
set -u
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
SLOTHC="${SLOTHC:-$root/target/debug/slothc}"
if [ ! -x "$SLOTHC" ]; then
    echo "slothc not found at $SLOTHC (run 'cargo build' first)"
    exit 2
fi

pass=0
fail=0
for f in "$here"/arc_*.sl; do
    name="$(basename "$f" .sl)"
    out="$("$SLOTHC" run "$f" 2>&1)"
    rc=$?
    leaks="$(printf '%s\n' "$out" | grep -c '^LEAK ')"
    oks="$(printf '%s\n' "$out" | grep -c '^OK ')"
    overall="$(printf '%s\n' "$out" | grep -c '^OK   overall$')"
    sink="$(printf '%s\n' "$out" | grep '^sink=' | tail -1)"
    if [ "$rc" -ne 0 ] || [ "$leaks" -ne 0 ] || [ "$overall" -ne 1 ]; then
        echo "$name: FAIL (rc=$rc leaks=$leaks ok=$oks $sink)"
        printf '%s\n' "$out" | grep -E '^LEAK |panic|free\(\)|malloc|corrupt|timeout' | head -10
        fail=$((fail + 1))
    else
        echo "$name: OK ($oks scenarios, $sink)"
        pass=$((pass + 1))
    fi
done

echo "arc stress: $pass passed, $fail failed"
exit $((fail > 0))
