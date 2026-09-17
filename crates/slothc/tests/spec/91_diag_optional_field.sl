// spec: diagnostics — field access on an optional receiver asks for
// narrowing (`?.` is deferred per design §3.6)
// diag: field `v` on an optional receiver
class N {
    var v: int = 1;
    var next: N? = nil;
}
func main(): unit {
    var n = N();
    print(n.next.v);
}
