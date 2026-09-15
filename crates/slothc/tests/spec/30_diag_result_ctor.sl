// spec: diagnostics — ok()/err() require a Result-typed target (patch #23)
func main(): unit {
    let x = ok(5);
}
// diag: ctor `ok` requires a declared Result target
