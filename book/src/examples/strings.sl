func main() {
    let a = "hello";
    print(a + ", world");        // 拼接
    print(len(a));               // 5   字节长度
    print("héllo".len());        // 6   é 占 2 字节
    print(a == "hel" + "lo");    // true 内容相等（intern）
    print("a" != "b");           // true

    var n = 0;
    for c in "aé中" {            // 迭代按 Unicode 字符
        n = n + 1;
    }
    print(n);                    // 3

    var t = "x";
    t = t + "y" + "z";
    print(t);                    // xyz

    let f = 1.5;
    print("n=${n} f=${f} b=${true}");
}
