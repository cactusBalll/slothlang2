// spec: fixed-width integer operands must share their surface (no implicit
// widening between distinct widths — use an explicit conversion)
func main(): unit {
    var a: int8 = int8(1);
    var b: int16 = int16(1);
    print(a + b);                 // diag: type mismatch in arithmetic operand
    print(a < b);                 // diag: type mismatch in comparison operand
    var u: uint = uint(1);
    print(u == int8(1));          // diag: type mismatch in comparison operand
}
