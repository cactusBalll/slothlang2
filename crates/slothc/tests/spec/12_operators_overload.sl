// spec: operator overloads — arith, comparison family, neg, indexable (§3.4)
class Vec {
    var v: int;
    func __init__(v: int) {
        this.v = v;
    }
    func __add__(o: Vec): Vec {
        return Vec(this.v + o.v);
    }
    func __sub__(o: Vec): Vec {
        return Vec(this.v - o.v);
    }
    func __neg__(): Vec {
        return Vec(0 - this.v);
    }
    func get(): int {
        return this.v;
    }
    func __lt__(o: Vec): bool {
        return this.v < o.v;
    }
    func __gt__(o: Vec): bool {
        return this.v > o.v;
    }
    func __eq__(o: Vec): bool {
        return this.v == o.v;
    }
    func __ne__(o: Vec): bool {
        return this.v != o.v;
    }
}
class Bag {
    var items: Array<int>;
    func __init__() {
        this.items = [0, 0];
    }
    func __index__(i: int): int {
        return this.items[i];
    }
    func __assign__(i: int, v: int) {
        var arr: Array<int> = this.items;
        arr[i] = v;
    }
}
func main(): unit {
    let a = Vec(2);
    let b = Vec(3);
    print((a + b).get());       // expect: 5
    print((b - a).get());       // expect: 1
    print((-a).get());          // expect: -2
    print(a < b);               // expect: true
    print(a > b);               // expect: false
    print((a + b) == Vec(5));   // expect: true
    print(a != b);              // expect: true
    let bag = Bag();
    bag[0] = 7;
    bag[1] = 8;
    print(bag[0]);              // expect: 7
    print(bag[1]);              // expect: 8
    print(bag[0] + bag[1]);     // expect: 15
}

