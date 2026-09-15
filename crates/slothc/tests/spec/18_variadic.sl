// spec: variadic funcs — pack, float promotion, empty pack (§3.3)
func sum(xs...: Array<int>): int {
    var t = 0;
    for x in xs {
        t = t + x;
    }
    return t;
}
func fsum(xs...: Array<float>): float {
    var t = 0.0;
    for x in xs {
        t = t + x;
    }
    return t;
}
func mix(base: int, xs...: Array<int>): int {
    var t = base;
    for x in xs {
        t = t + x;
    }
    return t;
}
func main(): unit {
    print(sum(1, 2, 3));         // expect: 6
    print(sum());                // expect: 0
    print(fsum(2.5, 0.5));       // expect: 3
    print(fsum(1.5, 1.5, 1.0));  // expect: 4
    print(mix(1, 2, 3));         // expect: 6
    print(mix(10));              // expect: 10
}

