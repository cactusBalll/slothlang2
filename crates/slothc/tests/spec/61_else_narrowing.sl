// spec: flow typing on the else branch — `if x is nil {} else {}` narrows x
// to its non-nil inner type (design §3.6)
class C {
    var n: int = 3;
}
func p1(o: C?): int {
    if o is nil { return 0 - 1; } else { return o.n; }
}
func p2(n: int?): int {
    if n is nil { return 0 - 1; } else { return n + 1; }
}
func main(): unit {
    print(p1(C()));           // expect: 3
    print(p1(nil));           // expect: -1
    print(p2(5));             // expect: 6
    print(p2(nil));           // expect: -1
    var o: C? = nil;
    if o is nil { print(1); } else { print(o.n); }   // expect: 1
    o = C();
    if o is nil { print(0); } else { print(o.n); }   // expect: 3
    var b: bool? = false;
    if b is nil { print(9); } else { print(b); }     // expect: false
}
