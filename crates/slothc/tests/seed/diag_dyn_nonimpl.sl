// seed: `dyn T` is nominal — a class without `impl T` cannot be used
// (oop Bug3 / D2)
trait Named { func name(): str; }
class A {
    func name(): str { return "A"; }
}
func main(): unit {
    var n: dyn Named = A();
    print(n.name());
}
// diag: dyn Named
