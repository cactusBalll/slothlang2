// spec: diagnostics — indexing without __index__/__assign__ overloads (patch #20)
class Plain {
    var v: int;
    func __init__(v: int) {
        this.v = v;
    }
}
func main(): unit {
    let a = Plain(1);
    print(a[0]);
}
// diag: requires an `__index__` overload
