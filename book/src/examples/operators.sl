func double(x: int): int {
    return x * 2;
}

func main() {
    print(7 / 2);             // 3   整数除法向零截断
    print(7 % 3);             // 1
    print(2 + 3 * 4);         // 14  优先级
    print((2 + 3) * 4);       // 20
    print(not false);         // true
    print(1 < 2 and 2 < 3);   // true  短路与
    print(1 > 2 or 2 > 1);    // true  短路或
    print(5 |> double);       // 10  管道：double(5)
    print(2 |> double |> double); // 8  管道左结合链
    var n: int? = nil;
    print(n ?: 7);            // 7   Elvis 空合并
    print(3 is int);          // true
    print("x" is not str);    // false
    var i = 0;
    for x in 0..3 { i = i + x; } // 范围 0..3（半开）
    print(i);                 // 3
}
