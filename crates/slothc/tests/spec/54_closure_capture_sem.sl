// spec: closure capture semantics (frozen, patch 44): scalar captures are
// VALUE SNAPSHOTS at lambda construction (later scalar rebinds invisible);
// str/Array/Map/class/box captures are SHARED REFERENCES (retain at fill —
// external content mutations visible inside the closure and vice versa)
class Counter {
    var n: int = 0;
}
pub func main(): unit {
    // scalar: value snapshot at construction (base rebinds stay invisible)
    var base = 10;
    var inc = |x: int| { return x + base; };
    print(inc(1));            // expect: 11
    base = 99;                // rebinding the OUTSIDE scalar is not seen
    print(inc(1));            // expect: 11
    // ref: shared reference — external mutation visible through the capture
    var c = Counter();
    var get = || { return c.n; };
    c.n = 5;
    print(get());             // expect: 5
    get();                    // borrow still alive: rc sees the shared handle
    // ref list capture: appended elements flow through the shared array
    var arr = [1, 2];
    var call = || { return arr.len(); };
    arr.push(3);
    print(call());            // expect: 3
    let d0 = sloth_rc_drops();
    var jam = 0;
    var i = 0;
    while i < 2000 {
        let lam2 = |x: int| { return x + i; };
        jam = jam + lam2(1);
        i = i + 1;
    }
    print(sloth_rc_drops() > d0);   // capture closures collect (frames die)
}
