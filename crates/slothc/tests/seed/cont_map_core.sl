// seed: Map core — literal, index read/write, len, keys()/values(), key
// families, nested maps, str content-key equality, class keys (book ch13)
trait Hashable { func __hash__(): int; }
trait Equatable { func __eq__(other: K): bool; }
class K impl Equatable, Hashable {
    var x: int;
    func __init__(x: int) { this.x = x; }
    func __hash__(): int { return this.x; }
    func __eq__(other: K): bool { return this.x == other.x; }
}
func main(): unit {
    let m = @(1: 10, 2: 20, 3: 30);
    print(len(m));                              // expect: 3
    print(m[1]);                                // expect: 10
    m[4] = 40;
    print(m[4]);                                // expect: 40
    print(len(m));                              // expect: 4
    print(m.len());                             // expect: 4

    var g: Map<str, int> = @("x": 1, "y": 2);
    g["z"] = 3;
    print(g["x"] + g["y"] + g["z"]);            // expect: 6
    print(len(g));                              // expect: 3

    var b = @(true: 1, false: 2);
    print(b[true] + b[false]);                  // expect: 3
    let fl = @(1.5: 10, 2.5: 20);
    print(fl[1.5] + fl[2.5]);                   // expect: 30

    let ks = keys(g);
    print(len(ks));                             // expect: 3
    let vs = values(g);
    var s = 0;
    for v in vs { s = s + v; }
    print(s);                                   // expect: 6

    var nested: Map<int, Map<str, int>> = @();
    nested[1] = @("a": 5);
    print(nested[1]["a"]);                      // expect: 5

    // str keys compare by content, so a fresh equal string hits the same slot
    var cm: Map<str, int> = @();
    let base = "hello world";
    cm[base[0..5]] = 7;
    print(cm["hel" + "lo"]);                    // expect: 7

    // class keys use content hash/eq
    var km = @(K(1): 10, K(2): 20);
    km[K(3)] = 30;
    print(len(km));                             // expect: 3
    print(km[K(3)]);                            // expect: 30
    var total = 0;
    for (var e: km) { total = total + e.val; }
    print(total);                               // expect: 60
}
