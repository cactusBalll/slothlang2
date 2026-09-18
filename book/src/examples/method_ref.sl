// 方法引用 obj.method 生成 method bridge（env = 接收者，绑定 this）
class Counter {
    var n: int = 0;
    func __init__(k: int) { this.n = k; }
    func bump(): int {
        this.n = this.n + 1;
        return this.n;
    }
}

func run(f: () -> int): int {
    return f();
}

func main() {
    var c = Counter(10);
    let bump = c.bump;
    print(bump());          // 11
    print(bump());          // 12
    print(run(c.bump));     // 13
}
