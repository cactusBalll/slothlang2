// seed: Map Entry iteration — `for (var e: m)` yields Entry<K,V> with key/val
// fields (book ch13 §13.2, ch16)
func main(): unit {
    let m = @(1: 10, 2: 20, 3: 30);
    var ksum = 0;
    var vsum = 0;
    for (var e: m) {
        ksum = ksum + e.key;
        vsum = vsum + e.val;
    }
    print(ksum);                        // expect: 6
    print(vsum);                        // expect: 60

    var gm: Map<str, int> = @("a": 1, "b": 2, "c": 4);
    var vs = 0;
    var n = 0;
    for (var e: gm) {
        n = n + 1;
        vs = vs + e.val;
    }
    print(n);                           // expect: 3
    print(vs);                          // expect: 7

    // shorthand form yields the same Entry
    var c = 0;
    for e in m { c = c + 1; }
    print(c);                           // expect: 3
}
