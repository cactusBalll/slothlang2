// spec: first-class range values — bind, pass, return, iterate (§2.1/§3.5)
func mk(a: int, b: int): range {
    return a..b;
}
func sum(r: range): int {
    var s = 0;
    for x in r { s = s + x; }
    return s;
}
func main(): unit {
    var r = 0..5;
    var s = 0;
    for x in r { s = s + x; }
    print(s);                 // expect: 10
    let inc = 2..=4;
    var t = 0;
    for x in inc { t = t + x; }
    print(t);                 // expect: 9
    var n = 3;
    let rn = 0..n;
    var u = 0;
    for x in rn { u = u + x; }
    print(u);                 // expect: 3
    print(sum(mk(1, 4)));     // expect: 6
    print(sum(0..=3));        // expect: 6
    var empty = 5..5;
    var c = 0;
    for x in empty { c = c + 1; }
    print(c);                 // expect: 0
    print(r is range);        // expect: true
    let last = 10..=10;
    var q = 0;
    for x in last { q = q + x; }
    print(q);                 // expect: 10
}
