#!/bin/bash
# IO layer example runner: fiber TCP/UDP/HTTP servers plus the blocking
# thread-per-connection model, across every available event backend.
#
# Usage:  examples/net/run.sh
# Env:    SLOTHC=/path/to/slothc   (defaults to target/debug/slothc)
set -u
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
SLOTHC="${SLOTHC:-$root/target/debug/slothc}"
if [ ! -x "$SLOTHC" ]; then
    echo "slothc not found at $SLOTHC (run 'cargo build' first)"
    exit 2
fi

fail=0
expect() {
    local file="$1"; shift
    local out
    out="$("$SLOTHC" run "$here/$file" 2>&1)"
    local rc=$?
    if [ "$rc" -ne 0 ]; then
        echo "$file: FAIL (rc=$rc)"
        printf '%s\n' "$out" | tail -15
        fail=1
        return
    fi
    for line in "$@"; do
        if ! printf '%s\n' "$out" | grep -qxF "$line"; then
            echo "$file: FAIL (missing '$line')"
            printf '%s\n' "$out" | tail -15
            fail=1
        fi
    done
}

expect "event_backends.sl"  "event backends OK"
expect "tcp_echo_fiber.sl"  "tcp echo (fiber) OK"
expect "udp_echo.sl"        "udp echo: ping" "udp echo OK"
expect "http_server.sl"     "http server OK"
expect "tcp_echo_threads.sl" "tcp echo (threads) OK"

if [ "$fail" -eq 0 ]; then
    echo "net examples: OK"
fi
exit $fail
