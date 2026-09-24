// spec: diagnostics — Array slice assignment requires a matching `Array<T>`
func main(): unit {
    var a = [1, 2, 3];
    a[0..2] = 5;
}
// diag: array slice assignment expects a matching `Array<T>`
