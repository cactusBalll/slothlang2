// seed: core functions — forward references, recursion, variadics with
// zero/many args, and unit-return functions (book §9)
func fib(n: int): int {
    if n < 2 { return n; }
    return fib(n - 1) + fib(n - 2);
}
func even(n: int): bool {
    if n == 0 { return true; }
    return odd(n - 1);
}
func odd(n: int): bool {
    if n == 0 { return false; }
    return even(n - 1);
}
func sum(xs...: Array<int>): int {
    var t = 0;
    for x in xs { t = t + x; }
    return t;
}
func mix(base: int, xs...: Array<int>): int {
    var t = base;
    for x in xs { t = t + x; }
    return t;
}
func greet(): unit {
    print("hi");
    return;
}
func bump(v: int): int { return v + 1; }
func main(): unit {
    print(fib(10));          // expect: 55
    print(even(10));         // expect: true
    print(odd(10));          // expect: false
    print(sum());            // expect: 0
    print(sum(5));           // expect: 5
    print(sum(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)); // expect: 55
    print(mix(100));         // expect: 100
    print(mix(100, 1, 2));   // expect: 103
    greet();                 // expect: hi
    print(bump(41));         // expect: 42
}
