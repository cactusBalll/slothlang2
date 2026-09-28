// seed: documented limit — generic functions cannot be used as values;
// must produce a diagnostic (book ch10.6)
func id<T>(x: T): T { return x; }
func main(): unit {
    let f = id;
    print(f(1));
}
// diag: `id` cannot be used as a value (generic/variadic/extern function)
