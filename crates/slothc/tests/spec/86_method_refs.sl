// spec: method references `obj.method` (design §6 迁移表) — bound `this`
class Counter {
    var n: int = 0;
    func __init__(k: int) { this.n = k; }
    func bump(): int { this.n = this.n + 1; return this.n; }
    func get(): int { return this.n; }
}
func run(f: () -> int): int { return f(); }
func main(): unit {
    var c = Counter(10);
    let bump = c.bump;
    print(bump());          // expect: 11
    print(bump());          // expect: 12
    print(c.get());         // expect: 12
    let get = c.get;
    print(run(get));        // expect: 12
    print(run(c.get));      // expect: 12
}
