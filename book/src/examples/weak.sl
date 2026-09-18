// Weak<T>：弱持有不增加目标计数，用于打破强引用环
class Node {
    var name: str = "n";
    var next: Weak<Node> = nil;
}

func main() {
    var w: Weak<Node> = nil;
    {
        var tmp = Node();
        w = tmp;                 // 存入弱盒（不 retain 目标）
    }                            // tmp 死亡，目标归零
    var dead = w.upgrade();      // 死亡目标 -> nil
    print(dead is nil);          // true

    var a = Node();
    a.name = "alive";
    var w2: Weak<Node> = a;      // 目标仍被 a 强持有
    var u = w2.upgrade();        // 强借用 -> Node?
    if u is not nil {
        print(u.name);           // alive
    }
}
