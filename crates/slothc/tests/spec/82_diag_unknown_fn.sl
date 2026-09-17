// spec: diagnostics — unknown callee is reported
func main(): unit {
    foo(1);
}
// diag: call to unknown `foo`
