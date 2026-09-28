// seed: vtable virtual dispatch through base refs and heterogeneous arrays,
// 3-level chain, override in middle and leaf (PLAN §17 / book ch18.3)
class A {
    var n: int;
    func __init__() { this.n = 1; }
    func make(): str { return "A"; }
    func id(): int { return 1; }
}
class B: A {
    func __init__() { super.__init__(); }
    func make(): str { return "B"; }
}
class C: B {
    func __init__() { super.__init__(); }
    func id(): int { return 3; }
}
class D: C {
    func __init__() { super.__init__(); }
    func make(): str { return "D"; }
}
func show(a: A): str { return a.make(); }
func main(): unit {
    var xs = [A(), B(), C(), D()];
    for x in xs {
        print(show(x));        // expect: A
        print(show(x));        // expect: B
        print(show(x));        // expect: B
        print(show(x));        // expect: D
    }
    let c: A = C();
    print(c.id());             // expect: 3
    print(c.make());           // expect: B
    // rc balance: base-ref dispatch of a ref-returning method does not leak
    let base = sloth_rc_live();
    var i = 0;
    while i < 2000 {
        let d: A = D();
        let s = d.make();
        i = i + 1;
    }
    print(sloth_rc_live() - base); // expect: 0
}
