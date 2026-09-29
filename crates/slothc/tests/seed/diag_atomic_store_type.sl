// seed: `atomic.store` argument is checked as `int` (extension G5)
func main(): unit {
    let a = atomic.new(0);
    a.store("x");
    print(a.load());
}
// diag: atomic.store requires an `int` argument
