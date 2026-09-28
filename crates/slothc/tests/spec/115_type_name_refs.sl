// spec: runtime `type_name` for all reference types (design §2.6/§4.3.2) —
// readable monomorphic names; classes/dyn report the most-derived class
trait Display {
    func to_str(): str;
}
trait Animal {
    func sound(): str;
}
class Dog impl Animal {
    func sound(): str { return "woof"; }
}
class Box<T> {
    var v: T;
    func __init__(x: T) {
        this.v = x;
    }
}
func mk(): int { return 1; }
func main(): unit {
    // dynamic surfaces report the concrete runtime class
    let d: dyn Animal = Dog();
    print(type_name(d));             // expect: Dog
    let b: dyn Animal = Dog();
    print(type_name(b));             // expect: Dog
    // monomorphic reference types report their full surface
    print(type_name("x"));           // expect: str
    print(type_name(1..3));          // expect: range
    print(type_name([1, 2]));        // expect: Array<int>
    print(type_name(["a"]));         // expect: Array<str>
    print(type_name(@("a": 1)));     // expect: Map<str, int>
    print(type_name(Box<int>(1)));   // expect: Box<int>
    print(type_name(mk));            // expect: () -> int
    // dyn builtin value boxes report the concrete value type
    let x: dyn Display = 7;
    print(type_name(x));             // expect: int
    // nullable reference: nil names as nil
    let d2: Dog? = nil;
    print(type_name(d2));            // expect: nil
    let d3: Dog? = Dog();
    print(type_name(d3));            // expect: Dog
    // value-optional box is itself a reference type
    let n: int? = 5;
    print(type_name(n));             // expect: int?
}
