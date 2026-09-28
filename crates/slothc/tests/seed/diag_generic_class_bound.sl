// seed: generic-class type args must satisfy their trait bounds (oop Bug12/B12)
trait Named { func name(): str; }
class Holder<T: Named> {
    var v: T;
    func __init__(x: T) { this.v = x; }
}
class Q { var z: int; }
func main(): unit {
    let h = Holder<Q>(Q());
    print(h.v);
}
// diag: does not satisfy trait bound
