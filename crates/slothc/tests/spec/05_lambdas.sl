// spec: lambda values — plain lambda, capture snapshot, lambda-in-lambda (§3.3)
// patch #39 extension: annotated params/ret, free-fn calls inside lambdas
func pick<T>(v: T): T {
    return v;
}
func main(): unit {
    var add = |a: int, b: int| { return a + b; };
    print(add(2, 3));          // expect: 5
    var base = 10;
    var inc = |x: int| { return x + base; };
    print(inc(5));             // expect: 15
    var id = |x: int| { return x; };
    print(id(id(7)));          // expect: 7
    let e: str = "m";
    var gen = |x: int| -> Result<int, str> { return err(e); };
    print(gen(1).err());       // expect: m
    var withT = |x: int| -> int { return pick(x); };
    print(withT(4));           // expect: 4
    var neg = |x: int, y: str| -> bool { return y == "x"; };
    print(neg(1, "x"));        // expect: true
    print(neg(2, "z"));        // expect: false
}
