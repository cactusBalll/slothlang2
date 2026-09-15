// spec: classes — fields, ctor, methods, default field init (§2.4)
class Counter {
    var n: int;
    var step: int = 2;
    func __init__(a: int) {
        this.n = a;
    }
    func bump(d: int) -> int {
        this.n = this.n + d;
        return this.n;
    }
    func double_up() {
        this.n = this.n + this.step;
    }
}
class Point {
    var x: int;
    var y: int;
    func __init__(x: int, y: int) {
        this.x = x;
        this.y = y;
    }
    func sum(): int {
        return this.x + this.y;
    }
}
class Empty {
    var x: int;
}
func main(): unit {
    var c = Counter(1);
    print(c.bump(4));           // expect: 5
    print(c.step);              // expect: 2
    c.double_up();
    print(c.n);                 // expect: 7
    let p = Point(3, 4);
    print(p.sum());             // expect: 7
    print(p.x);                 // expect: 3
    var e = Empty();
    print(e.x);                 // expect: 0
}

