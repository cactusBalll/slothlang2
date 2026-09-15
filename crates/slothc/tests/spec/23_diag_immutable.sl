// spec: diagnostics — expected compile-surface failures (run via slothc check)
// each `// diag:` marker is a case: check must exit nonzero with that message
func main(): unit {
    let imm = 1;
    imm = 2;
}
// diag: cannot assign to immutable `imm` (declared with `let`)
