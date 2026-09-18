#!/usr/bin/env python3
"""Generate the tiny deterministic checkpoints + tokenizer for the TE-P4
differential test (`run.sh`). Layout matches llama2.c `run.c`: a 7-i32 header
followed by the f32 weights in memory_map_weights order (including the skipped
freq_cis region), and a tokenizer.bin with the 3 specials + 256 byte tokens +
a handful of BPE merges. Stdlib only (deterministic via `random.Random`).
"""
import random
import struct
import sys

OUT = sys.argv[1] if len(sys.argv) > 1 else "."
DIM, HIDDEN, NL, NH, NKV, VOCAB, SEQ = 32, 64, 2, 4, 2, 32000, 64


def weights(seed, unshared):
    rng = random.Random(seed)
    hs = DIM // NH
    kv = DIM * NKV // NH
    floats = []
    # embedding rows are one-hot-dominant so the trained-like logit gap is
    # huge: greedy argmax is then stable under the f32 (run.c) vs f64 (sloth)
    # rounding differences, while every other weight stays a real random matrix
    for t in range(VOCAB):
        row = [rng.gauss(0.0, 0.01) for _ in range(DIM)]
        row[t % DIM] = 10.0
        floats.extend(row)
    counts = [
        NL * DIM,           # rms_att_weight
        NL * DIM * DIM,     # wq
        NL * DIM * kv,      # wk
        NL * DIM * kv,      # wv
        NL * DIM * DIM,     # wo
        NL * DIM,           # rms_ffn_weight
        NL * DIM * HIDDEN,  # w1
        NL * HIDDEN * DIM,  # w2
        NL * DIM * HIDDEN,  # w3
        DIM,                # rms_final_weight
        SEQ * hs // 2,      # freq_cis_real (skipped by the loader)
        SEQ * hs // 2,      # freq_cis_imag (skipped by the loader)
    ]
    if unshared:
        counts.append(VOCAB * DIM)  # wcls
    for n in counts:
        floats.extend(rng.gauss(0.0, 0.05) for _ in range(n))
    vsign = -VOCAB if unshared else VOCAB
    name = f"{OUT}/tiny_{'un' if unshared else ''}shared.bin"
    with open(name, "wb") as f:
        f.write(struct.pack("<7i", DIM, HIDDEN, NL, NH, NKV, vsign, SEQ))
        for x in floats:
            f.write(struct.pack("<f", x))


def tokenizer():
    entries = [("<unk>", 0.0), ("<s>", 0.0), ("</s>", 0.0)]
    for b in range(256):
        entries.append((f"<0x{b:02X}>", 0.0))
    merges = [
        " ", " t", "th", "the", "he", "e ", "Once", " upon", "upon", "n ",
        "a", "an", " time", "time", "im", "me", " o", "on", "once", "ce",
        "up", "p ", "li", "it", "tt", "tl", "le", " g", "ir", "rl", "l ",
        "s", "h", "i", "n", "o", "u", "p", "m", "w", "d", "r", "y", "b",
        "c", "f", "g", "k", "v", "x", "z", "l", "e", "t", "a", "d",
        "st", "or", "and", "ing", "ed", "er", "ar", "ou", "ll", "ee",
    ]
    for i, s in enumerate(merges):
        entries.append((s, 1.0 + i * 0.5))
    # run.c reads exactly vocab_size entries: pad with never-matching strings
    while len(entries) < VOCAB:
        entries.append((f"#pad{len(entries)}", 0.0))
    maxlen = max(len(s.encode()) for s, _ in entries)
    with open(f"{OUT}/tiny_tok.bin", "wb") as f:
        f.write(struct.pack("<i", maxlen))
        for s, score in entries:
            b = s.encode()
            f.write(struct.pack("<f", score))
            f.write(struct.pack("<i", len(b)))
            f.write(b)


if __name__ == "__main__":
    weights(1234, unshared=False)
    weights(4321, unshared=True)
    tokenizer()
    print(f"wrote tiny_shared.bin, tiny_unshared.bin, tiny_tok.bin to {OUT}")
