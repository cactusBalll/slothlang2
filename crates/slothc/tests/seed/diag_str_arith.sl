// seed: arithmetic requires numeric operands (`str - str` is not pointer math;
// only `str + str` is builtin) — core Bug5 / cross C1
func main(): unit { print("a" - "b"); }
// diag: type mismatch in arithmetic operand
