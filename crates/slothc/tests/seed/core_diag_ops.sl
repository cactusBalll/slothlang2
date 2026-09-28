// seed diag: int-only bitwise/shift, and no implicit widening between two
// distinct fixed widths (book §5.6, §7.1)
func main(): unit {
    print(1.5 & 1.0);       // diag: bitwise operators require integer operands
    print(~1.5);            // diag: `~` requires an integer operand
    var a: int8 = int8(1);
    var b: int16 = int16(1);
    print(a + b);           // diag: type mismatch in arithmetic operand
    print(a < b);           // diag: type mismatch in comparison operand
    print(uint(1) < int(1)); // diag: type mismatch in comparison operand
}
