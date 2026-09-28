// seed: a field write through an optional receiver is diagnosed, not dropped
// (optionals Bug2 / B8)
class C { var n: int = 3; }
func main(): unit {
    var c: C? = C();
    c.n = 9;
    if c is not nil { print(c.n); }
}
// diag: optional receiver
