// seed: diagnostic — a type argument must satisfy a trait bound (book ch11.1)
trait Named { func name(): str; }
class Point {
    var x: int;
    func name(): str { return "p"; }
}
func label<T: Named>(x: T): str { return x.name(); }
func main(): unit {
    print(label(Point()));
}
// diag: type argument `Point` does not satisfy trait bound `Named`
