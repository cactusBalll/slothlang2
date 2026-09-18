func main() {
    var a = [1, 2, 3];
    print(a.len());          // 3
    a.push(4);
    print(a.len());          // 4
    print(a.pop());          // 4
    a[0] = 10;
    print(a[0]);             // 10

    var s = 0;
    for x in a { s = s + x; }
    print(s);                // 10 + 2 + 3 = 15

    // 稳定句柄：增长只换缓冲，别名看到同一容器
    var b = a;
    b.push(99);
    print(a.len());          // 4
    print(a[3]);             // 99

    // 嵌套数组 + 链式索引赋值
    var grid = [[1, 2], [3, 4]];
    grid[0][1] = 9;
    print(grid[0][1]);       // 9

    // 显式标注驱动空字面量类型
    var empty: Array<float> = [];
    empty.push(1.5);
    print(empty[0]);         // 1.5
}
