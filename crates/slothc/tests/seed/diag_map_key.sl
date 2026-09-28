// seed: map index assignment checks the key surface — containers Bug5
func main(): unit {
    var m: Map<int, int> = @();
    m["x"] = 20;
    print(len(m));
}
// diag: type mismatch in map key assignment
