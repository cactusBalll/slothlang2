// spec: class value interactions — methods returning objects, reference
// aliasing, object-valued fields
class P {
    var x: int;
    var y: int;
    func __init__(x: int, y: int) {
        this.x = x;
        this.y = y;
    }
    func sum(): int { return this.x + this.y; }
    func with(o: P): P { return P(this.x + o.x, this.y + o.y); }
}
class Box {
    var p: P;
    func __init__(p: P) { this.p = p; }
    func get(): P { return this.p; }
}
func main(): unit {
    let a = P(1, 2);
    let b = P(3, 4);
    print(a.sum());           // expect: 3
    print(a.with(b).sum());   // expect: 10
    let bx = Box(a);
    print(bx.get().x);        // expect: 1
    bx.p = P(5, 6);
    print(bx.p.sum());        // expect: 11
    let c = a;
    c.x = 100;
    print(a.x);               // expect: 100
    print(a.with(a).sum());   // expect: 204
    print(bx.get().sum());    // expect: 11
}
