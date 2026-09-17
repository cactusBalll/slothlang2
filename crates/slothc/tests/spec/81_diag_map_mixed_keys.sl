// spec: diagnostics — mixed map key families are rejected
func main(): unit {
    var m = @("a": 1, 2: 3);
    print(len(m));
}
// diag: mixed map key types
