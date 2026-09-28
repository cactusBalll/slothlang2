// seed: diagnostic — a class impl'ing a trait must implement every required
// method (book ch19.1)
trait Speaker {
    func name(): str;
}
class Cat impl Speaker {
    var x: int;
}
func main(): unit {
    let c = Cat();
    print(c.x);
}
// diag: trait `Speaker` method `name` not implemented by `Cat`
