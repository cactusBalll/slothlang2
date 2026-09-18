// 用户类型实现 Iterable/Iterator 协议：iter() 返回迭代器，next() 返回 T?
class Range3 {
    var i: int = 0;
    func iter(): Range3 { return this; }
    func next(): int? {
        if this.i >= 3 {
            return nil;
        }
        this.i = this.i + 1;
        return this.i - 1;
    }
}

func main() {
    let r = Range3();
    for x in r {
        print(x);                // 0 1 2
    }

    // 内建 Iterable：range / str / Array / Map
    var s = 0;
    for x in 0..=3 { s = s + x; }
    print(s);                    // 6

    for e in @(1: 10) {
        print(e.key + e.val);    // 11
    }
}
