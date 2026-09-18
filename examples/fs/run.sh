#!/bin/bash
# TE-P3 checkpoint loading end-to-end: generate the fixture, then load the
# config + widen the f32 weights through `sloth/fs.slt`.
#
# Usage:  examples/fs/run.sh
# Env:    SLOTHC=/path/to/slothc   (defaults to target/debug/slothc)
set -eu
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
SLOTHC="${SLOTHC:-$root/target/debug/slothc}"
if [ ! -x "$SLOTHC" ]; then
    echo "slothc not found at $SLOTHC (run 'cargo build' first)"
    exit 2
fi
cd "$here"
bash gen_fixture.sh fixture.bin

got="$("$SLOTHC" run load.sl | grep -v '^invokePacked')"
want="4
8
32
44
1
4
10"
if [ "$got" = "$want" ]; then
    echo "checkpoint load: OK"
else
    echo "checkpoint load: MISMATCH"
    echo "--- got ---"; echo "$got"
    echo "--- want ---"; echo "$want"
    exit 1
fi
