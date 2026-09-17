// spec: diagnostics — no implicit int/float conversion (design §2.1)
func main(): unit {
    print(1 + 2.5);    // diag: type mismatch in arithmetic operand
    print(3.0 == 3);   // diag: type mismatch in comparison operand
    print(2.5 > 2);    // diag: type mismatch in comparison operand
    var a = [1, 2.5];  // diag: type mismatch in array literal element
    var f = 1.0;
    f = f + 1;         // diag: type mismatch in arithmetic operand
}
