// seed: optionals — value-typed boxes (int?/float?/bool?) vs reference
// optionals, `is nil` / `is not nil` narrowing on identifiers and in `else`,
// Elvis `?:`, nil-aware print/interpolation, and `int()`/`float()` unboxing
// (book ch15, ch24).
class C {
    var n: int = 3;
    var next: C? = nil;
}
func picki(o: int?): int {
    return o ?: 7;
}
func pickf(o: float?): float {
    return o ?: 1.5;
}
func first(o: C?): int {
    if o is not nil {
        return o.n;
    }
    return 0 - 1;
}
func main(): unit {
    // value-typed box: 0 is a live payload, not nil
    var i: int? = nil;
    print(i is nil);                 // expect: true
    i = 0;
    print(i is nil);                 // expect: false
    if i is not nil {
        print(i);                    // expect: 0
    }
    print(picki(nil));               // expect: 7
    print(picki(3));                 // expect: 3

    var f: float? = nil;
    print(f is nil);                 // expect: true
    f = 0.0;
    print(f is nil);                 // expect: false
    if f is not nil {
        print(f);                    // expect: 0
    }
    print(pickf(0.0) == 0.0);        // expect: true
    print(pickf(nil) == 1.5);        // expect: true

    var b: bool? = false;
    print(b is nil);                 // expect: false
    if b is not nil {
        print(b);                    // expect: false
    }

    // reference optionals: nil is a null handle
    var s: str? = nil;
    print(s is nil);                 // expect: true
    s = "hi";
    if s is not nil {
        print(s.len());              // expect: 2
    }
    var c: C? = nil;
    print(first(c));                 // expect: -1
    c = C();
    if c is nil {
        print(0);
    } else {
        print(c.n);                  // expect: 3
    }

    // elvis on a reference optional
    var d: str? = nil;
    let e = d ?: "fallback";
    print(e);                        // expect: fallback

    // nil-aware print / interpolation
    var m: int? = nil;
    print(m);                        // expect: nil
    print("v=${f} n=${m}");          // expect: v=0 n=nil

    // unboxing conversions
    var x: int? = 9;
    print(int(x));                   // expect: 9
    var y: float? = nil;
    print(int(y));                   // expect: 0
    print(float(x));                 // expect: 9
}
