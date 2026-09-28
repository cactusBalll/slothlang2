// seed: diagnostic — indexing a class without __index__ is an error (book ch20)
class A { var x: int; }
func main(): unit {
    let a = A();
    print(a[0]);
}
// diag: class `A` requires an `__index__` overload for indexing
