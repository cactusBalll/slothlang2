// seed: a parameterized method reference bridges receiver + all args
// (oop Bug1)
class Box2 {
    var v: int;
    func __init__(x: int) { this.v = x; }
    func add(n: int): int { return this.v + n; }
}
func apply1(f: (int) -> int, a: int): int { return f(a); }
func main(): unit {
    let b = Box2(10);
    let add = b.add;
    print(add(5));        // expect: 15
    print(apply1(b.add, 3)); // expect: 13
}
