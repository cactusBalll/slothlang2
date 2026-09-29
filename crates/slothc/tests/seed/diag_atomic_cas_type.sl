// seed: `atomic.cas` operands are checked as `int` (extension G5)
func main(): unit {
    let a = atomic.new(0);
    print(a.cas("x", 1));
}
// diag: atomic.cas requires `int` operands
