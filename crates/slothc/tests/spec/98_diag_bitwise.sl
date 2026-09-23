// spec: bitwise/shift/`~` require integer operands (design §3.3 int-only)
func main(): unit {
    print(1.5 & 1.0);        // diag: bitwise operators require integer operands
    print("a" | "b");        // diag: bitwise operators require integer operands
    print(~1.5);             // diag: `~` requires an integer operand
}
