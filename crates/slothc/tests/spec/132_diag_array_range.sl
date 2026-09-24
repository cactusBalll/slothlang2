// spec: diagnostics — Array has no range slicing (only str supports s[a..b])
func main(): unit {
    var a = [1, 2, 3, 4];
    print(a[1..3]);
}
// diag: Array does not support range slicing
