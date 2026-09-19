#!/bin/bash
# TE-P4 llama2.c inference verification.
#
#  1. tiny differential: generate two deterministic checkpoints (shared /
#     unshared, GQA) + a synthetic 32000-token tokenizer, run greedy 40 steps
#     and compare byte-for-byte against golden output captured from `run.c`.
#  2. real model: when `examples/res/stories42M.bin` exists, generate 40 greedy
#     steps (and compare against `run.c` when $LLAMA2C is available).
#
# Usage:  examples/llama/run.sh
# Env:    SLOTHC=/path/to/slothc      (defaults to target/debug/slothc)
#         LLAMA2C=/path/to/llama2.c   (optional C reference for the differential)
set -eu
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"
SLOTHC="${SLOTHC:-$root/target/debug/slothc}"
LLAMA2C="${LLAMA2C:-/home/undatus63/llama2.c}"
if [ ! -x "$SLOTHC" ]; then
    echo "slothc not found at $SLOTHC (run 'cargo build' first)"
    exit 2
fi
command -v python3 >/dev/null || { echo "python3 required"; exit 2; }

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cd "$here"

python3 gen_tiny.py "$tmp" >/dev/null
"$SLOTHC" build main_tiny.sl "$tmp/llama.bin" >/dev/null || exit 1

# ---- 1. tiny differential (shared + unshared/GQA) ----
for m in shared unshared; do
    cp "$tmp/tiny_${m}.bin" "$tmp/model.bin"
    cp "$tmp/tiny_tok.bin" "$tmp/tokenizer.bin"
    got="$(cd "$tmp" && ./llama.bin 2>/dev/null)"
    want="$(cat "expected_tiny_${m}.txt")"
    if [ "$got" != "$want" ]; then
        echo "tiny $m: MISMATCH"
        echo "--- got ---"; echo "$got"
        echo "--- want ---"; echo "$want"
        exit 1
    fi
    echo "tiny $m (greedy 40): OK"
done

# ---- 2. real stories42M model ----
res_model="$root/examples/res/stories42M.bin"
tokenizer=""
for cand in "$root/examples/res/tokenizer.bin" "$LLAMA2C/tokenizer.bin"; do
    if [ -f "$cand" ]; then tokenizer="$cand"; break; fi
done
if [ -f "$res_model" ] && [ -n "$tokenizer" ]; then
    cp "$res_model" "$tmp/model.bin"
    cp "$tokenizer" "$tmp/tokenizer.bin"
    s=$(date +%s%N)
    got="$(cd "$tmp" && ./llama.bin 2>/dev/null)"
    e=$(date +%s%N)
    ms=$(( (e - s) / 1000000 ))
    echo "stories42M (greedy 40): ${ms} ms"
    echo "$got"
    if [ -f "$LLAMA2C/run.c" ]; then
        gcc -O3 -o "$tmp/run_c" "$LLAMA2C/run.c" -lm || exit 1
        want="$(cd "$tmp" && ./run_c model.bin -t 0 -n 40 -i "Once upon a time" -z tokenizer.bin 2>/dev/null)"
        if [ "$got" != "$want" ]; then
            echo "stories42M: MISMATCH vs run.c"
            exit 1
        fi
        echo "stories42M vs run.c (greedy 40): IDENTICAL"
    fi
else
    echo "stories42M.bin / tokenizer.bin not present — skipping real-model run"
fi
