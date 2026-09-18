func apply(f: (int) -> int, x: int): int {
    return f(x);
}

func make_adder(n: int): (int) -> int {
    return |x: int| { return x + n; };
}

func main() {
    let sq = |x: int| { return x * x; };
    print(sq(6));            // 36
    print(apply(sq, 5));     // 25

    let add10 = make_adder(10);
    print(add10(4));         // 14

    // 标量捕获是构造时的值快照
    var base = 100;
    let snap = |x: int| { return x + base; };
    base = 0;
    print(snap(1));          // 101

    // 具名函数作为值
    let g = sq;
    print(g(3));             // 9
}
