// seed: `is`/`is not` type tests and `any` narrowing (book §5.1, §5.6, §15)
func classify(v: any): str {
    if v is int { return "int"; }
    if v is float { return "float"; }
    if v is bool { return "bool"; }
    if v is str { return "str"; }
    return "other";
}
func addone(v: any): int {
    if v is int {
        return v + 1;
    }
    return 0 - 1;
}
func main(): unit {
    print(classify(1));          // expect: int
    print(classify(1.5));        // expect: float
    print(classify(true));       // expect: bool
    print(classify("s"));        // expect: str
    print(addone(4));            // expect: 5
    print(addone("x"));          // expect: -1
    var x: any = 5;
    print(x is int);             // expect: true
    print(x is str);             // expect: false
    print(x is not str);         // expect: true
    var i8: int8 = int8(5);
    print(i8 is int8);           // expect: true
    print(i8 is int);            // expect: false
    var r = 1..2;
    print(r is range);           // expect: true
    print(nil is nil);           // expect: true
}
