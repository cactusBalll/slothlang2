// seed: `?:` joins two operands of the same surface; a reference optional
// against a scalar is a diagnostic, not a runtime rc underflow
// (test_workspace/core/bug2_elvis_ref_value.sl)
class C {
    var n: int = 7;
}
func main(): unit {
    var b: C? = nil;
    print(b ?: 5);
}
// diag: type mismatch in elvis operand
