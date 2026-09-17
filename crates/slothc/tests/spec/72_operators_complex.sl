// spec: operator overloads — full arithmetic/comparison family + neg (§3.4)
class V {
    var v: int;
    func __init__(v: int) { this.v = v; }
    func __add__(o: V): V { return V(this.v + o.v); }
    func __sub__(o: V): V { return V(this.v - o.v); }
    func __mul__(o: V): V { return V(this.v * o.v); }
    func __div__(o: V): V { return V(this.v / o.v); }
    func __mod__(o: V): V { return V(this.v % o.v); }
    func __neg__(): V { return V(0 - this.v); }
    func __eq__(o: V): bool { return this.v == o.v; }
    func __ne__(o: V): bool { return this.v != o.v; }
    func __lt__(o: V): bool { return this.v < o.v; }
    func __le__(o: V): bool { return this.v <= o.v; }
    func __gt__(o: V): bool { return this.v > o.v; }
    func __ge__(o: V): bool { return this.v >= o.v; }
    func get(): int { return this.v; }
}
func main(): unit {
    let a = V(6);
    let b = V(3);
    print((a + b).get());     // expect: 9
    print((a - b).get());     // expect: 3
    print((a * b).get());     // expect: 18
    print((a / b).get());     // expect: 2
    print((a % b).get());     // expect: 0
    print((-a).get());        // expect: -6
    print(a == V(6));         // expect: true
    print(a != b);            // expect: true
    print(b < a);             // expect: true
    print(b <= a);            // expect: true
    print(a > b);             // expect: true
    print(a >= V(6));         // expect: true
}
