// spec: plain-class virtual dispatch through base-typed references (design §2.4)
class Shape {
    var tag: int = 1;
    func area(): int { return 1; }
    func describe(): str { return "shape"; }
    func scaled(k: int): int { return this.area() * k; }
}
class Square: Shape {
    var side: int = 0;
    func area(): int { return this.side * this.side; }
    func describe(): str { return "square"; }
}
class Cube: Square {
    func area(): int { return 6 * super.area(); }
}
func total(xs: Array<Shape>): int {
    var s = 0;
    var i = 0;
    while i < xs.len() { s = s + xs[i].area(); i = i + 1; }
    return s;
}
func main(): unit {
    var s: Shape = Shape();
    print(s.area());          // expect: 1
    print(s.describe());      // expect: shape
    var sq = Square();
    sq.side = 3;
    var q: Shape = sq;        // upcast: runtime Square
    print(q.area());          // expect: 9
    print(q.describe());      // expect: square
    print(q.scaled(2));       // expect: 18
    var c = Cube();
    c.side = 2;               // Square field inherited by Cube
    print(c.area());          // expect: 24
    var xs: Array<Shape> = [Shape(), sq, c];
    print(total(xs));         // expect: 34
}
