// seed: traits — required + default methods, default body virtual this-call,
// inherited impl satisfying a bound, multiple traits (book ch19.1)
trait Named { func name(): str; }
trait Greeter {
    func greet(): str { return "hello ${this.name()}"; }
}
class Base {
    func name(): str { return "base"; }
}
class Child: Base impl Named, Greeter {
    func name(): str { return "child"; }
}
class Other impl Named {
    func name(): str { return "other"; }
}
func label<T: Named>(x: T): str { return "[" + x.name() + "]"; }
func main(): unit {
    let c = Child();
    print(c.name());           // expect: child
    print(c.greet());          // expect: hello child
    print(label(c));           // expect: [child]
    print(label(Other()));     // expect: [other]
    let n: dyn Named = c;
    print(n.name());           // expect: child
    let g: dyn Greeter = c;
    print(g.greet());          // expect: hello child
    var xs: Array<dyn Named> = [c, Other()];
    for x in xs {
        print(x.name());       // expect: child
        print(x.name());       // expect: other
    }
}
