// seed diag: condition must be bool, no implicit numeric conversion, and a
// declared return type is mandatory (book §5.4, §7.1, §8.1, §9.1)
func bad(): int { return "x"; }     // diag: type mismatch in return value
func main(): unit {
    if 1 { print(1); }              // diag: type mismatch in condition
    print(1 + 2.5);                 // diag: type mismatch in arithmetic operand
    print(bad());
}
