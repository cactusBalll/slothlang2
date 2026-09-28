// seed: range bounds must be `int` (a float bound would hang the loop) — core Bug8
func main(): unit { for x in 0.0..2.0 { print(x); } }
// diag: range start must be `int`
