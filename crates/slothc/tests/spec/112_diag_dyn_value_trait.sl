// spec: a value type cannot inhabit a dyn trait whose methods it lacks
trait Speaker {
    func noise(): str;
}

func main(): unit {
    var s: dyn Speaker = 42;
    print(s.noise());
}
// diag: type mismatch in initializer
