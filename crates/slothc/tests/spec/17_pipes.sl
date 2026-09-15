// spec: pipes — single/two-arg append, chained pipes (§3.8)
func double(x: int): int {
    return x * 2;
}
func add_d(a: int, b: int): int {
    return a + b;
}
func inc(x: int): int {
    return x + 1;
}
func main(): unit {
    print(3 |> double);              // expect: 6
    print(3 |> add_d(10));           // expect: 13
    print(1 |> inc |> inc |> double); // expect: 6
    var base = 5;
    print(base |> double);           // expect: 10
    print((1 |> add_d(2)) |> inc);   // expect: 4
}

