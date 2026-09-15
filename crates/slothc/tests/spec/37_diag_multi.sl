// spec: diagnostics — multi-error batch reporting (patch #41, B base)
func main(): unit {
    var x = 1;
    x = "s";
    var y = 2;
    y = 1.5;
    if 3 {
        print(0);
    }
}
// diag: type mismatch in assignment to `x`
// diag: type mismatch in assignment to `y`
// diag: type mismatch in condition
