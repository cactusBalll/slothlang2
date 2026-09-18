func parse(x: int): Result<int, str> {
    if x < 0 {
        return err("neg");
    }
    return ok(x * 2);
}

func main() {
    let r = parse(5);
    print(r.is_ok());            // true
    print(r.unwrap());           // 10

    let e = parse(-1);
    print(e.is_ok());            // false
    print(e.err());              // neg

    // 显式标注的 Result 目标：ok()/err() 据此解析实例
    let t: Result<float, int> = ok(2.5);
    print(t.unwrap());           // 2.5

    var q: Result<int, str> = err("seed");
    q = ok(8);                   // 赋值面构造器
    print(q.unwrap());           // 8
}
