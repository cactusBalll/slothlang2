// seed: `atomic.add`/`sub` argument is checked as `int` (extension G5)
func main(): unit {
    let a = atomic.new(0);
    a.add(1.5);
    print(a.load());
}
// diag: atomic.add requires an `int` argument
