// spec: diagnostics — var word-class reassignment (patch #22)
func main(): unit {
    var x = 1;
    x = "s";
}
// diag: type mismatch in assignment to `x`
