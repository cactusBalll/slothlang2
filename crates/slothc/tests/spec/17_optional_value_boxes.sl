// spec: value-optional boxes (patch 42) — int?/float?/bool? carry their
// payload in an rc box; 0/0.0/false inside a box never confuse with nil,
// and nil prints "nil" (§2.1/, §2.6 明确的 nil 语义)
class C {
    var n: int? = nil;
    var f: float? = nil;
}
func picki(o: int?): int {
    return o ?: 7;
}
func pickf(o: float?): float {
    return o ?: 1.5;
}
func main(): unit {
    // boxed 0 vs nil
    var n: int? = 0;
    print(n is nil);            // expect: false
    var m: int? = nil;
    print(m is nil);            // expect: true
    if n is not nil {
        print(n);               // expect: 0
    }
    // float optionals ride the box (previously rejected)
    var f: float? = nil;
    print(f is nil);            // expect: true
    f = 0.0;
    print(f is nil);            // expect: false
    if f is not nil {
        print(f);               // expect: 0
    }
    // bool optional: false is a live box, not nil
    var b: bool? = false;
    print(b is nil);            // expect: false
    if b is not nil {
        print(b);               // expect: false
    }
    // elvis + narrowing + conversion
    print(picki(3));            // expect: 3
    print(picki(nil));          // expect: 7
    print(pickf(0.0) == 0.0);   // expect: true
    print(pickf(nil) == 1.5);   // expect: true
    var i: int? = 9;
    print(int(i));              // expect: 9
    // nil-aware print/interpolation
    print(m);                   // expect: nil
    print("v=${f}");            // expect: v=0
    print("n=${m}");            // expect: n=nil
    // class fields and containers
    var c = C();
    print(c.n is nil);          // expect: true
    c.n = 5;
    c.f = 0.0;
    if c.n is not nil {
        print(c.n + 1);         // expect: 6
    }
    if c.f is not nil {
        print(c.f);             // expect: 0
    }
    var arr = [n, m, i];
    print(arr[0]);              // expect: 0
    print(arr[1]);              // expect: nil
    print(arr[2]);              // expect: 9
}
