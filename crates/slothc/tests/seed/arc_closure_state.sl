// seed: a stateful closure keeps its captured scalar across calls (cross C0b /
// A4); the outer binding still sees the snapshot value (book §10.3)
func make_counter(): () -> int {
    var n = 0;
    return || { n = n + 1; return n; };
}
func main(): unit {
    let c = make_counter();
    print(c());  // expect: 1
    print(c());  // expect: 2
    print(c());  // expect: 3
    var n = 0;
    let g = || { n = n + 1; };
    g();
    print(n);    // expect: 0
}
