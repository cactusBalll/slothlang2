// spec: generic shape inference — Array<T>, Map<K,V>, T? shapes unify (§2.3)
func sum_array<T>(xs: Array<T>): int {
    var n = 0;
    for x in xs {
        n = n + 1;
    }
    return n;
}
func get<K, V>(m: Map<K, V>, k: K): V {
    return m[k];   // word route returns V word
}
func main(): unit {
    var a = [1, 2, 3];
    print(sum_array(a));            // expect: 3
    var b = ["x", "y"];
    print(sum_array(b));            // expect: 2
    print(get(@(1: 5), 1));         // expect: 5
}

