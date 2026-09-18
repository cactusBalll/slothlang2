func pick(o: int?): int {
    return o ?: 7;
}

func main() {
    var i: int? = nil;
    print(i is nil);             // true

    // 值型 optional 用 rc 跟踪的 payload 盒；0 是合法值而非 nil
    i = 0;
    print(i is nil);             // false
    if i is not nil {
        print(i);                // 0  分支内收窄为 int
    }

    // else 分支同样收窄
    if i is nil {
        print("none");
    } else {
        print(i);                // 0
    }

    print(pick(nil));            // 7
    print(pick(3));              // 3

    // 引用型 optional：nil 即空指针
    var c: str? = nil;
    c = "hi";
    if c is not nil {
        print(c.len());          // 2
    }
}
