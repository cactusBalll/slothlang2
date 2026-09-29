// seed: a `unit` expression stored as an array element, a map key/value or an
// assignment value is diagnosed, not turned into a missing SSA operand
// (probe containers/x7)
func main(): unit {
    var a: Array<int> = [print("hi")];
    var m: Map<int, int> = @(1: print("hi"));
    a[0] = print("bye");
    print(a.len() + m.len());
}
// diag: a `unit` expression has no value
