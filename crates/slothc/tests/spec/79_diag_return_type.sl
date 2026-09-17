// spec: diagnostics — declared return surface is enforced
func f(): int {
    return "x";
}
func main(): unit {
    print(f());
}
// diag: type mismatch in return value
