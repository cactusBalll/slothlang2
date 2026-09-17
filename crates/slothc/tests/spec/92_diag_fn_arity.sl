// spec: diagnostics — first-class function call arity is checked
// diag: function call arity
func apply(f: (int) -> int): int {
    return f(1, 2);
}
func main(): unit {
    print(apply(|x: int| { return x; }));
}
