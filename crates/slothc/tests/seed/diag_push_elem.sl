// seed: `a.push(v)` checks v against the element type — containers Bug2
func main(): unit {
    var a: Array<str> = ["a", "b"];
    a.push(9);
    print(a);
}
// diag: type mismatch in push element
