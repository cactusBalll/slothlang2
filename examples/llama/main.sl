// TE-P4 llama2.c inference demo (sloth2). Expects `model.bin` and
// `tokenizer.bin` in the current directory (run.sh stages them); greedy
// generation is deterministic and matches llama2.c `run.c` token for token.
import "sloth/llama.slt";
import "sloth/tokenizer.slt";
import "sloth/random.slt";

func main(): unit {
    let cfg = load_config("model.bin");
    let shared = config_shared("model.bin");
    let w = load_weights("model.bin", cfg, shared);
    let tok = load_tokenizer("tokenizer.bin", cfg.vocab_size);
    let s = RunState(cfg);
    let rng = XorShift(42);
    generate(cfg, w, s, tok, rng, "Once upon a time", 40, 0.0, 0.9);
}
