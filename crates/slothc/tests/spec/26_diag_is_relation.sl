// spec: diagnostics — unrelated class `is` comparability (patch #25)
class A {
    func __init__() {
        return;
    }
}
class B {
    func __init__() {
        return;
    }
}
func main(): unit {
    let a = A();
    if a is B {
        print(1);
    }
}
// diag: types `A` and `B` have no class relation for `is`
