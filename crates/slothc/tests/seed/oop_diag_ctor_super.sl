// seed: diagnostic — a subclass declaring its own ctor must call
// super.__init__ (book ch18.2)
class A {
    var x: int;
    func __init__(v: int) { this.x = v; }
}
class B: A {
    var y: int;
    func __init__(v: int) { this.y = v; }
}
func main(): unit {
    let b = B(1);
    print(b.y);
}
// diag: constructor of `B` must call super.__init__
