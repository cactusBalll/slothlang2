// seed: Map compile-time diagnostics — mixed key families and value type
// mismatch (book ch13)
func f(): unit {
    let m = @("a": 1, 2: 3);           // diag: mixed map key types
}
func g(): unit {
    var m: Map<int, str> = @();
    m[1] = 99;                         // diag: type mismatch in map value assignment
}
func main(): unit {
    f();
    g();
}
