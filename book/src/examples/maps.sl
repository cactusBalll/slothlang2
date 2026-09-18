func main() {
    var m = @("a": 1, "b": 2);
    print(m["a"]);           // 1
    m["c"] = 3;              // 写入（不存在则插入）
    print(len(m));           // 3
    print(m.len());          // 3

    // 遍历产生 Entry<K,V> 记录：字段 key / val
    for (var e: m) {
        print("${e.key}=${e.val}");
    }

    var ks = keys(m);
    print(ks.len());         // 3
    var vs = values(m);
    print(vs.len());         // 3

    // 显式标注驱动空 Map 的键值类型
    var empty: Map<str, int> = @();
    empty["x"] = 1;
    print(empty["x"]);       // 1

    // 嵌套 Map
    var mm: Map<str, Map<str, int>> = @();
    mm["a"] = @();
    mm["a"]["b"] = 7;
    print(mm["a"]["b"]);     // 7
}
