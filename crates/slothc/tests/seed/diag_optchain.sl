// seed: `?.` is deliberately unsupported; the parser says so instead of the
// generic `expected ')'` (cross C6 / H2)
class C { var x: int; }
func main(): unit {
    var c: C? = nil;
    print(c?.x);
}
// diag: optional chaining
