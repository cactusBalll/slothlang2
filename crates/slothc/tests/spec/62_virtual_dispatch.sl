// spec: virtual dispatch — trait methods invoked from a base-class body
// honor the runtime override (design §2.4)
trait Shape {
    func area(): int;
    func label(): str { return "A"; }
    func describe(): str { return "${this.label()}:${this.area()}"; }
}
class Base impl Shape {
    func area(): int { return 1; }
    func label(): str { return "base"; }
}
class Sub: Base {
    func area(): int { return 2; }
    func label(): str { return "sub"; }
}
func report(s: dyn Shape): str { return s.describe(); }
func main(): unit {
    let b: Base = Base();
    print(b.describe());      // expect: base:1
    let s = Sub();
    print(s.describe());      // expect: sub:2
    print(report(Sub()));     // expect: sub:2
    print(report(Base()));    // expect: base:1
}
