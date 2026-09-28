// seed diag: assignment surface checks — immutable `let` and word-family
// mismatch (book §6.1)
func main(): unit {
    let z = 1;
    z = 2;                  // diag: cannot assign to immutable
    var x = 1;
    x = "s";                // diag: type mismatch in assignment to
}
