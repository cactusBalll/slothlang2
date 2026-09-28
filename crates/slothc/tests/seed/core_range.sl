// seed: core ranges — for-form lowering, first-class range values, empty and
// inclusive ranges (book §5.1, §7.1, §8.2)
func mk(a: int, b: int): range {
    return a..b;
}
func sum(r: range): int {
    var s = 0;
    for x in r { s = s + x; }
    return s;
}
func main(): unit {
    for x in 0..3 { print(x); }      // expect: 0
                                     // expect: 1
                                     // expect: 2
    for x in 0..=2 { print(x); }     // expect: 0
                                     // expect: 1
                                     // expect: 2
    print(sum(mk(1, 4)));            // expect: 6
    print(sum(0..=3));               // expect: 6
    var n = 3;
    var r = 0..n;
    var t = 0;
    for x in r { t = t + x; }
    print(t);                        // expect: 3
    var empty = 5..5;
    var c = 0;
    for x in empty { c = c + 1; }
    print(c);                        // expect: 0
    // a range whose low > high is empty, not an infinite loop
    var d = 0;
    for x in 3..0 { d = d + 1; }
    print(d);                        // expect: 0
    print(r is range);               // expect: true
    var neg_lo = 0 - 3;
    var s = 0;
    for x in neg_lo..0 { s = s + x; }
    print(s);                        // expect: -6
}
