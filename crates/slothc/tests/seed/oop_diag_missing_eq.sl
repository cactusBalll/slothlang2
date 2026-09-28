// seed: diagnostic — comparing classes without a __eq__ overload is an error
// (book ch20)
class Plain {
    var v: int;
    func __init__(v: int) { this.v = v; }
}
func main(): unit {
    let a = Plain(1);
    let b = Plain(2);
    if a == b {
        print(1);
    }
}
// diag: comparison `EqEq` on classes `Plain` and `Plain` requires a `__eq__` overload
