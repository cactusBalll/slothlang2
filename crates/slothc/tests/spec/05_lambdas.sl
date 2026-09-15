// spec: lambda values — plain lambda, capture snapshot, lambda-in-lambda (§3.3)
func main(): unit {
    var add = |a: int, b: int| { return a + b; };
    print(add(2, 3));          // expect: 5
    var base = 10;
    var inc = |x: int| { return x + base; };
    print(inc(5));             // expect: 15
    var id = |x: int| { return x; };
    print(id(id(7)));          // expect: 7
}

