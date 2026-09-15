// spec: diagnostics — Display requirement for interpolated classes (patch #21)
class Plain {
    var v: int;
    func __init__(v: int) {
        this.v = v;
    }
}
func main(): unit {
    let p = Plain(1);
    print("oops ${p}");
}
// diag: requires trait bound `Display`
