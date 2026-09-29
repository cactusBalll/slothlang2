// seed: binding a `unit` expression has no value — diagnose instead of
// emitting `memref.store , …` (MLIR "expected SSA operand"; probe x7)
func main(): unit {
    let x = print("hi");
    print(x);
}
// diag: a `unit` expression has no value
