func main() {
    let n = 3;
    if n > 2 {
        print("big");
    } else {
        print("small");
    }
    // 括号形式与无括号形式均可
    if (n == 3) { print("three"); }

    var i = 0;
    while i < 3 {
        print(i);
        i = i + 1;
    }

    var sum = 0;
    for x in 0..5 {          // 半开区间 0,1,2,3,4
        if x == 1 { continue; }
        if x == 4 { break; }
        sum = sum + x;
    }
    print(sum);              // 0 + 2 + 3 = 5

    for (var c: "ab") {      // 括号形式：遍历字符串字符
        print(c);
    }
}
