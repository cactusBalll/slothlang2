// spec: exported module surface (sibling for 31_modules_main.sl; not run directly)
pub func twofold(a: int) -> int {
    return a + a;
}
pub func add(a: int, b: int): int {
    return a + b;
}
pub var g: int = 99;
pub class Counter {
    var n: int;
    func __init__(a: int) {
        this.n = a;
    }
    func bump(d: int) -> int {
        this.n = this.n + d;
        return this.n;
    }
}
