var g = 10;              // 模块级可变全局（推断为 int）
let fixed = 3;           // 模块级不可变全局

func main() {
    var a = 1;           // 可变，类型由初值推断为 int
    let b = "hi";        // 不可变绑定
    a = a + 1;
    print(a);            // 2
    print(b);            // hi
    {
        let a = 100;     // 块级遮蔽
        print(a);        // 100
    }
    print(a);            // 2
    print(g + fixed);    // 13
}
