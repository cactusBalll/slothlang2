// spec: diagnostics — bool condition discipline (patch #25)
func main(): unit {
    var n = 3;
    while n {
        n = n - 1;
    }
}
// diag: condition must be `bool` (no implicit truthy conversion from `int`)
