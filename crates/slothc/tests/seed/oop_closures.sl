// seed: closure capture semantics (frozen) — scalar snapshot at construction,
// str/Array/Map/class captures are shared references (book ch10.3)
class C { var n: int; func __init__() { this.n = 0; } }
func main(): unit {
    // scalar capture is a construction-time value snapshot
    var base = 100;
    let snap = |x: int| { return x + base; };
    base = 0;
    print(snap(1));            // expect: 101
    // Array capture shares the object: external push is visible
    var arr = [1, 2];
    let lenf = || { return arr.len(); };
    arr.push(3);
    print(lenf());             // expect: 3
    // Array capture: rebinding the outside name is NOT visible
    arr = [1, 2, 3, 4];
    print(lenf());             // expect: 3
    // class capture shares the instance: field mutation is visible
    var c = C();
    let getn = || { return c.n; };
    c.n = 42;
    print(getn());             // expect: 42
    // Map capture is shared
    var m: Map<str, int> = @();
    m["x"] = 1;
    let getx = || { return m["x"]; };
    m["x"] = 9;
    print(getx());             // expect: 9
    // nested lambda captures the outer lambda's captured var
    let outer = |a: int| -> (int) -> int {
        return |b: int| -> int { return a + b; };
    };
    let f = outer(10);
    print(f(5));               // expect: 15
}
