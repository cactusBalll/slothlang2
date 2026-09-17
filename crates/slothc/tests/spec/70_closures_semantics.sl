// spec: closure capture semantics (frozen patch 44) — scalar snapshots,
// shared reference captures, loop-local captures
class Counter {
    var n: int = 0;
    func inc() { this.n = this.n + 1; }
}
func main(): unit {
    var base = 100;
    var f = |x: int| { return x + base; };
    print(f(1));              // expect: 101
    base = 0;
    print(f(1));              // expect: 101
    var c = Counter();
    var g = || { c.inc(); return c.n; };
    print(g());               // expect: 1
    print(g());               // expect: 2
    var arr = [1, 2];
    var lenf = || { return arr.len(); };
    arr.push(3);
    print(lenf());            // expect: 3
    var jam = 0;
    var i = 0;
    while i < 100 {
        let k = i;
        let h = |x: int| { return x + k; };
        jam = jam + h(0);
        i = i + 1;
    }
    print(jam);               // expect: 4950
}
