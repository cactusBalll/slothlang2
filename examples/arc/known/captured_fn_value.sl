// KNOWN GAP: capturing a first-class function value in a closure loses its
// type. `syn_ty_of` (lambda.rs) has no `Ty::Fn` arm, so the synthesized capture
// parameter is typed `unit`; the captured name then resolves to a non-Fn slot
// and the call reports "call to unknown `g`".
//
// Actual: `slothc check` fails with `call to unknown `g``.
func main(): unit {
    var g = |x: int| { return x + 1; };
    var h = |x: int| { return g(x) * 2; };
    print(h(3));
}
