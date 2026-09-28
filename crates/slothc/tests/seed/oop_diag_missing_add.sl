// seed: diagnostic — using `+` on a class without __add__ is an error (book ch20)
class A { var x: int; }
func main(): unit {
    let a = A();
    print(a + a);
}
// diag: operator `Add` on class `A` requires a `__add__` overload
