// spec: first-class functions (design §2.3/§2.6) — function-typed params,
// named-function values, returned closures, arbitrary callees
func apply(f: (int) -> int, x: int): int { return f(x); }
func inc(x: int): int { return x + 1; }
func dbl(x: int): int { return x * 2; }
func compose(f: (int) -> int, g: (int) -> int, x: int): int { return f(g(x)); }
func make_add(n: int): (int) -> int {
    return |x: int| { return x + n; };
}
func main(): unit {
    print(apply(|y: int| { return y * y; }, 5));   // expect: 25
    let g = inc;
    print(g(4));                                    // expect: 5
    print(apply(inc, 10));                          // expect: 11
    print(compose(inc, dbl, 3));                    // expect: 7
    let add5 = make_add(5);
    print(add5(10));                                // expect: 15
    print((|x: int| { return x - 1; })(9));         // expect: 8
}
