// spec: interpolation of a class without Display falls back to the class name
class Plain {
    var v: int;
    func __init__(v: int) {
        this.v = v;
    }
}
func main(): unit {
    let p = Plain(1);
    print("oops ${p}");
    // expect: oops Plain
}
