// spec: FFI extern func — word route (floor/pow), f64 route; extern type passes (patch #17/#24)
extern type Tok;
extern func sloth_extern_tok_new(): Tok;
extern func sloth_extern_tok_val(t: Tok): int;
extern func sloth_extern_floor(x: float): float;
extern func sloth_extern_powf(x: float, m: float): float;
func main(): unit {
    let f = sloth_extern_floor(2.7);
    print(f);                        // expect: 2
    print(sloth_extern_floor(-1.5)); // expect: -2
    let p = sloth_extern_powf(2.0, 3.0);
    print(p);                        // expect: 8
    print(sloth_extern_powf(1.5, 0.0)); // expect: 1
    var t = sloth_extern_tok_new();
    print(sloth_extern_tok_val(t));  // expect: 99
    t = sloth_extern_tok_new();
    print(sloth_extern_tok_val(t));  // expect: 99
}

