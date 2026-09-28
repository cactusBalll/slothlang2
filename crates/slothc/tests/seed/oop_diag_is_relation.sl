// seed: diagnostic — `is` between unrelated classes is a compile error
// (book ch18.3)
class A { var x: int; }
class B { var y: int; }
func main(): unit {
    let a = A();
    if a is B {
        print(1);
    }
}
// diag: types `A` and `B` have no class relation for `is`
