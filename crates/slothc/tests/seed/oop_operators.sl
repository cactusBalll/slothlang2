// seed: operator overloads — arithmetic family, unary neg, comparison family,
// Indexable __index__/__assign__, inherited overloads (book ch20)
class V {
    var x: int;
    var y: int;
    func __init__(x: int, y: int) { this.x = x; this.y = y; }
    func __add__(o: V): V { return V(this.x + o.x, this.y + o.y); }
    func __sub__(o: V): V { return V(this.x - o.x, this.y - o.y); }
    func __mul__(o: V): V { return V(this.x * o.x, this.y * o.y); }
    func __neg__(): V { return V(0 - this.x, 0 - this.y); }
    func __eq__(o: V): bool { return this.x == o.x and this.y == o.y; }
    func __lt__(o: V): bool { return this.x < o.x; }
    func __ge__(o: V): bool { return this.x >= o.x; }
    func s(): str { return "(${this.x},${this.y})"; }
}
class Bag {
    var items: Array<int>;
    func __init__() { this.items = [0, 0, 0]; }
    func __index__(i: int): int { return this.items[i]; }
    func __assign__(i: int, v: int) { this.items[i] = v; }
}
class Derived: V {
    func __init__(x: int, y: int) { super.__init__(x, y); }
}
func main(): unit {
    let a = V(4, 6);
    let b = V(2, 3);
    print((a + b).s());        // expect: (6,9)
    print((a - b).s());        // expect: (2,3)
    print((a * b).s());        // expect: (8,18)
    print((-a).s());           // expect: (-4,-6)
    print(a == V(4, 6));       // expect: true
    print(b < a);              // expect: true
    print(a >= V(4, 6));       // expect: true
    let bag = Bag();
    bag[0] = 7;
    bag[1] = 8;
    print(bag[0] + bag[1]);    // expect: 15
    let d = Derived(5, 6);
    print((d + b).s());        // expect: (7,9)
}
