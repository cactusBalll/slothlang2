// seed: a generic-class instance without Display prints its friendly name,
// matching `type_name` (OPT6 / H4)
class Box<T> {
    var v: T;
    func __init__(x: T) { this.v = x; }
}
func main(): unit {
    print(Box<int>(1));         // expect: Box<int>
    print("${Box<int>(2)}");    // expect: Box<int>
}
