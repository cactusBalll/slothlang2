// seed: an `extern type` is opaque — field access is a compile error.
extern type Tok;
extern func tok_new(): Tok;

func main(): unit {
    let t = tok_new();
    print(t.x);
    // diag: extern type `Tok` is opaque
}
