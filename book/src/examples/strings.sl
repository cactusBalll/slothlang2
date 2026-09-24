func main() {
    let a = "hello";
    print(a + ", world");        // 拼接
    print(len(a));               // 5   字节长度
    print("héllo".len());        // 6   é 占 2 字节
    print(a == "hel" + "lo");    // true 内容相等（不做 interning）
    print("a" != "b");           // true

    var n = 0;
    for c in "aé中" {            // 迭代按 Unicode 字符
        n = n + 1;
    }
    print(n);                    // 3

    // 下标按字节：s[i] 取单字节，s[a..b] / s[a..=b] 取字节切片
    let s = "héllo";
    print(s[0]);                 // 104
    print(s[1..3]);              // é
    print(s[1..=3]);             // él

    // chars()：惰性码点迭代器，每个元素是 int 码点
    var sum = 0;
    for c in "aé中".chars() {    // 97 / 233 / 20013
        sum = sum + c;
    }
    print(sum);                  // 20343

    var t = "x";
    t = t + "y" + "z";
    print(t);                    // xyz

    let f = 1.5;
    print("n=${n} f=${f} b=${true}");
}
