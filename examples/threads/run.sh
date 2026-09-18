#!/bin/bash
# TH-P3 multithreading example runner.
#
#   * JIT-runs the channel producer/consumer and thread+fiber demos and checks
#     their deterministic results;
#   * builds the serial and parallel matvec AOT and reports the speedup against
#     the design §8.2 target (>= 2.5x on 4 cores; a conservative 1.5x gate is
#     used here so the check is stable on shared CI machines).
#
# Usage:  examples/threads/run.sh
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

fail=0
expect() {
    local file="$1"; shift
    out="$("$SLOTHC" run "$here/$file" 2>&1)"
    if [ $? -ne 0 ]; then
        echo "$file: FAIL (nonzero exit)"
        printf '%s\n' "$out" | tail -10
        fail=1
        return
    fi
    for line in "$@"; do
        if ! printf '%s\n' "$out" | grep -qxF "$line"; then
            echo "$file: FAIL (missing '$line')"
            printf '%s\n' "$out" | tail -10
            fail=1
        fi
    done
}

expect "producer_consumer.sl" "199980000" "producer/consumer OK"
expect "thread_fiber.sl" "206512" "thread+fiber OK"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cd "$here"

"$SLOTHC" build matvec_serial.sl "$tmp/serial.bin" > /dev/null || exit 1
"$SLOTHC" build matvec_parallel.sl "$tmp/parallel.bin" > /dev/null || exit 1

timeit() {
    local best=""
    for _ in $(seq "$REPEATS"); do
        local s e d
        s=$(date +%s%N)
        "$1" > /dev/null
        e=$(date +%s%N)
        d=$(( (e - s) / 1000000 ))
        if [ -z "$best" ] || [ "$d" -lt "$best" ]; then best="$d"; fi
    done
    echo "$best"
}

serial_ms=$(timeit "$tmp/serial.bin")
parallel_ms=$(timeit "$tmp/parallel.bin")
speedup=$(awk -v s="$serial_ms" -v p="$parallel_ms" 'BEGIN { printf "%.2f", s / p }')
echo "matvec serial: ${serial_ms} ms, parallel(4): ${parallel_ms} ms"
echo "matvec speedup: ${speedup}x (target >= 2.50x, gate >= 1.50x)"

ok=$(awk -v x="$speedup" 'BEGIN { print (x >= 1.50) ? 1 : 0 }')
if [ "$ok" -ne 1 ]; then
    echo "threads: FAIL (speedup below gate)"
    fail=1
fi

if [ "$fail" -eq 0 ]; then
    echo "threads example: OK"
fi
exit $fail
