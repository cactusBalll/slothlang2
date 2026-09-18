#!/bin/bash
# Coroutine (fiber) example runner: JIT-runs the typed 1.0 walkthrough and
# checks the expected interleaving plus clean ARC teardown.
#
# Usage:  examples/fiber/run.sh
# Env:    SLOTHC=/path/to/slothc   (defaults to target/debug/slothc)
set -u
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
SLOTHC="${SLOTHC:-$root/target/debug/slothc}"
if [ ! -x "$SLOTHC" ]; then
    echo "slothc not found at $SLOTHC (run 'cargo build' first)"
    exit 2
fi

out="$("$SLOTHC" run "$here/fibers.sl" 2>&1)"
rc=$?
expect=(
    "resumable: true"
    "got i from fiber: 0"
    "got i from fiber: 1"
    "got i from fiber: 5"
    "got i from fiber: -1"
    "error in fiber"
    "fiber OK"
)
fail=0
if [ "$rc" -ne 0 ]; then
    echo "fibers: FAIL (rc=$rc)"
    printf '%s\n' "$out" | tail -20
    fail=1
fi
for line in "${expect[@]}"; do
    if ! printf '%s\n' "$out" | grep -qxF "$line"; then
        echo "fibers: FAIL (missing '$line')"
        fail=1
    fi
done
if [ "$fail" -eq 0 ]; then
    echo "fiber example: OK"
fi
exit $fail
