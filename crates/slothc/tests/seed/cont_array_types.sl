// seed: Array element type inference — class literals upcast to the nearest
// common ancestor; empty literals adopt their context type; float/bool/nested
// literals (book ch12 §12.1)
class A { var x: int = 1; }
class B: A { var y: int = 2; }
func main(): unit {
    let arr = [A(), B()];
    print(arr.len());             // expect: 2

    let e: Array<str> = [];
    e.push("x");
    print(e);                     // expect: [x]

    let f: Array<float> = [];
    f.push(1.5);
    f.push(2.5);
    print(f);                     // expect: [1.5, 2.5]

    let nested: Array<Array<int>> = [];
    nested.push([1, 2]);
    print(nested[0][1]);          // expect: 2

    var b = [true, false];
    print(b[0]);                  // expect: true

    var fl = [1.5, 2.5];
    print(fl[0] + fl[1]);         // expect: 4
}
