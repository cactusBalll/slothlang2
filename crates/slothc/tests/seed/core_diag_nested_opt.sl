// seed diag: nested optionals `T??` are rejected by the parser (design §3.9,
// book §5.1)
func main(): unit {
    var n: int?? = nil;     // diag: nested optional T?? is not allowed
    print(n);
}
