// spec: diagnostics — object field assignment surface is enforced
class C {
    var n: int;
    func __init__() { this.n = 1; }
}
func main(): unit {
    let c = C();
    c.n = "s";
}
// diag: type mismatch in field assignment `n`
