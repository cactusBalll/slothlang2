// seed: builtins — len/int/float/typeid/type_name, `is` runtime tests on
// `any` (incl. width-precise ints), range values, container/class rendering
// through print and interpolation (book ch24, ch5).
trait Display {
    func to_str(): str;
}
class Pt impl Display {
    var x: int;
    var y: int;
    func __init__(x: int, y: int) { this.x = x; this.y = y; }
    func to_str(): str { return "Pt(${this.x}, ${this.y})"; }
}
class Plain {
    var n: int;
    func __init__(n: int) { this.n = n; }
}
func classify(v: any): str {
    if v is int8 { return "int8"; }
    if v is int { return "int"; }
    if v is float { return "float"; }
    if v is bool { return "bool"; }
    if v is str { return "str"; }
    if v is nil { return "nil"; }
    return "other";
}
func main(): unit {
    print(42);                       // expect: 42
    print(2.5);                      // expect: 2.5
    print(true);                     // expect: true
    print("s");                      // expect: s

    print(len("abc"));               // expect: 3
    print(len([1, 2, 3]));           // expect: 3
    print(len(@("a": 1)));           // expect: 1

    print(int(3.9));                 // expect: 3
    print(int(0.0 - 3.9));           // expect: -3
    print(float(4));                 // expect: 4
    var io: int? = nil;
    print(int(io));                  // expect: 0

    // runtime type identity for reference types
    print(type_name("x"));           // expect: str
    print(type_name(1..3));          // expect: range
    print(type_name([1, 2]));        // expect: Array<int>
    print(type_name(@("a": 1)));     // expect: Map<str, int>
    let d: Pt? = nil;
    print(type_name(d));             // expect: nil
    print(typeid(d));                // expect: 0

    // `is` on any
    var v: any = int8(5);
    print(v is int8);                // expect: true
    print(v is int);                 // expect: false
    print(type_name(v));             // expect: int8
    print(classify(2.5));            // expect: float
    print(classify(true));           // expect: bool
    print(classify("hi"));           // expect: str
    print(classify(nil));            // expect: nil

    // rendering
    print(Pt(1, 2));                 // expect: Pt(1, 2)
    print(Plain(9));                 // expect: Plain
    print([1, 2, 3]);                // expect: [1, 2, 3]
    print(@("a": 1, "b": 2));        // expect: {a: 1, b: 2}
    print("i=${42} p=${Pt(3, 0)}");  // expect: i=42 p=Pt(3, 0)

    // width wrapping
    print(int8(300));                // expect: 44
    print(uint8(300));               // expect: 44

    // range values
    let r = 2..=5;
    var sum = 0;
    for x in r { sum = sum + x; }
    print(sum);                      // expect: 14
}
