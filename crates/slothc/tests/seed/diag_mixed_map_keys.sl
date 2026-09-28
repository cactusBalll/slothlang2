// seed: `int` and `bool` are distinct Hashable families; a literal mixing
// them is rejected (containers Bug7 / E3)
func main(): unit {
    let m = @(1: 10, true: 20);
    print(len(m));
}
// diag: mixed map key types
