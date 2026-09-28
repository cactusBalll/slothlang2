// seed: first-class functions — named fn value, IIFE, compose, returned
// closures, fn-typed params and fields, method refs with zero args
// (book ch10.1/ch10.5)
func apply(f: (int) -> int, x: int): int { return f(x); }
func inc(x: int): int { return x + 1; }
func dbl(x: int): int { return x * 2; }
func compose(f: (int) -> int, g: (int) -> int, x: int): int { return f(g(x)); }
func make_add(n: int): (int) -> int { return |x: int| { return x + n; }; }
class Counter {
    var n: int = 0;
    func __init__(k: int) { this.n = k; }
    func bump(): int { this.n = this.n + 1; return this.n; }
}
class Holder {
    var op: (int) -> int;
    var k: int;
    func __init__(f: (int) -> int, k: int) { this.op = f; this.k = k; }
    func run(x: int): int { return this.op(x) * this.k; }
}
func main(): unit {
    print(apply(|y: int| { return y * y; }, 5));  // expect: 25
    let g = inc;
    print(g(4));                                  // expect: 5
    print(compose(inc, dbl, 3));                  // expect: 7
    let add5 = make_add(5);
    print(add5(10));                              // expect: 15
    print((|x: int| { return x - 1; })(9));       // expect: 8
    var c = Counter(10);
    let bump = c.bump;
    print(bump());                                // expect: 11
    print(c.n);                                   // expect: 11
    let h = Holder(inc, 3);
    print(h.run(4));                              // expect: 15
    h.op = dbl;
    print(h.run(4));                              // expect: 24
}
