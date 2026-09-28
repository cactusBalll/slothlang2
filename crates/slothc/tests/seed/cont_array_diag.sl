// seed: Array compile-time diagnostics — element assignment, mixed int/float
// literal, slice-assignment RHS shape, initializer element type (book ch12)
func f(): unit {
    var a = [1, 2, 3];
    a[0] = "x";                    // diag: type mismatch in array element assignment
    let b = [1, 2.5];              // diag: type mismatch in array literal element
    var c: Array<str> = [1, 2];    // diag: type mismatch in initializer
    var d = [4, 5, 6];
    d[0..1] = "hi";                // diag: array slice assignment expects a matching `Array<T>`
}
func main(): unit {
    f();
}
