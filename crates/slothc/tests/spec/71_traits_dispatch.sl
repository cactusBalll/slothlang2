// spec: trait dispatch — dyn arrays, inherited impls, default bodies
trait Named {
    func name(): str;
}
trait Greeter {
    func greet(): str { return "hi"; }
    func name(): str;
}
class A impl Named {
    func name(): str { return "a"; }
}
class B: A {
    func name(): str { return "b"; }
}
class C impl Named {
    func name(): str { return "c"; }
}
class D impl Greeter {
    func name(): str { return "d"; }
}
func main(): unit {
    var list: Array<dyn Named> = [A(), B(), C()];
    for x in list { print(x.name()); }   // expect: a
    // expect: b
    // expect: c
    let aa = A();
    print(aa.name());         // expect: a
    let bb = B();
    print(bb.name());         // expect: b
    let dd = D();
    print(dd.greet() + dd.name());  // expect: hid
}
