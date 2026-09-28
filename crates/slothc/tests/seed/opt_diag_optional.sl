// seed: optional receiver/`is nil` surface diagnostics (book ch15.1-15.2)
// diag: field `v` on an optional receiver
// diag: is nil` on non-optional type `int`
class N {
    var v: int = 1;
    var next: N? = nil;
}
func main(): unit {
    var n = N();
    print(n.next.v);
    var i: int = 0;
    print(i is nil);
}
