// spec: diagnostics — array element assignment surface is enforced
func main(): unit {
    var a = [1, 2];
    a[0] = "x";
}
// diag: type mismatch in array element assignment
