// spec: functions — typed params, recursion, unit return, expression call rsp (§3.3)
func add(a: int, b: int): int {
    return a + b;
}
func fact(n: int): int {
    if n <= 1 {
        return 1;
    }
    return n * fact(n - 1);
}
func fib(n: int): int {
    if n < 2 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
func show(): unit {
    print(100);              // expect: 100
    return;
}
func strjoin(a: str, b: str): str {
    return a + b;
}
func main(): unit {
    print(add(2, 3));           // expect: 5
    print(add(add(1, 2), 3));   // expect: 6
    print(fact(5));             // expect: 120
    print(fib(10));             // expect: 55
    show();
    var r = add(1, 1) * add(2, 2);
    print(r);                   // expect: 8
    print(add(10, -3));         // expect: 7
    print(strjoin("hi", "!"));  // expect: hi!
}

