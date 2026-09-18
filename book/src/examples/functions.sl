func add(a: int, b: int): int {
    return a + b;
}

func fact(n: int): int {
    if n <= 1 {
        return 1;
    }
    return n * fact(n - 1);
}

func greet(name: str): unit {
    print("hi ${name}");
    return;
}

// 类型化可变参数：调用处逐参传入，编译器收集为 Array<int>
func sum(xs...: Array<int>): int {
    var t = 0;
    for x in xs { t = t + x; }
    return t;
}

func main() {
    print(add(2, 3));       // 5
    print(add(add(1, 2), 3)); // 6
    print(fact(5));         // 120
    greet("sloth");
    print(sum(1, 2, 3, 4)); // 10
    print(sum());           // 0
}
