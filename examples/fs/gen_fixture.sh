#!/bin/bash
# Write a minimal llama2.c-style checkpoint: 7 little-endian i32 config words
# (dim=4, hidden=8, n_layers=1, n_heads=1, n_kv_heads=1, vocab=32, seq=16)
# followed by four f32 weights [1, 2, 3, 4].
set -eu
out="${1:-fixture.bin}"
printf '\x04\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x20\x00\x00\x00\x10\x00\x00\x00' > "$out"
printf '\x00\x00\x80\x3f\x00\x00\x00\x40\x00\x00\x40\x40\x00\x00\x80\x40' >> "$out"
