// 字面量与字符串插值（ch04）
func main() {
    print(42);                    // int
    print(3.5);                   // float
    print(true);                  // bool
    print("text");                // str
    print(1e3);                   // 科学计数法 float
    print(2.5e-1);                // 0.25
    print("tab\there");           // 转义
    print("quote \" inside");
    print("1 + 2 = ${1 + 2}");    // 插值：表达式在编译期展开
    let who = "sloth";
    print("hi ${who}, len=${len(who)}");
}
