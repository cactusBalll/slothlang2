// seed: diagnostic — an override must keep the ancestor's arity (PLAN §17)
class A {
    func f(x: int): int { return x; }
}
class B: A {
    func f(): int { return 0; }
}
func main(): unit {
    let b = B();
    print(b.f());
}
// diag: override `B.f` of `A.f`: arity want 1, got 0
