// 运算符重载：实现"魔术方法"，编译期解析，缺重载即编译错误
class Vec2 {
    var x: int;
    var y: int;
    func __init__(x: int, y: int) { this.x = x; this.y = y; }
    func __add__(o: Vec2): Vec2 { return Vec2(this.x + o.x, this.y + o.y); }
    func __sub__(o: Vec2): Vec2 { return Vec2(this.x - o.x, this.y - o.y); }
    func __neg__(): Vec2 { return Vec2(0 - this.x, 0 - this.y); }
    func __eq__(o: Vec2): bool { return this.x == o.x and this.y == o.y; }
    func __lt__(o: Vec2): bool { return this.x < o.x; }
    func show(): str { return "(${this.x}, ${this.y})"; }
}

class Bag {
    var items: Array<int>;
    func __init__() { this.items = [0, 0, 0]; }
    func __index__(i: int): int { return this.items[i]; }
    func __assign__(i: int, v: int) { this.items[i] = v; }
}

func main() {
    let a = Vec2(1, 2);
    let b = Vec2(3, 4);
    print((a + b).show());       // (4, 6)
    print((b - a).show());       // (2, 2)
    print((-a).show());          // (-1, -2)
    print(a == Vec2(1, 2));      // true
    print(a < b);                // true

    let bag = Bag();
    bag[0] = 7;                  // __assign__
    bag[1] = 8;
    print(bag[0] + bag[1]);      // 15  __index__
}
