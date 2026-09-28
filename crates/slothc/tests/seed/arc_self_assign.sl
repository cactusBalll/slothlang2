// seed: `a[i] = a[i]` on a reference-element array is a no-op, not a UAF
// (containers Bug1 / A3)
class Res {
    var name: str;
    func __init__(n: str) { this.name = n; }
    func __dispose__() { print("close " + this.name); }
}
func main(): unit {
    var a: Array<Res> = [Res("x")];
    a[0] = a[0];
    print("after");     // expect: after
    print(a[0].name);   // expect: x
}
