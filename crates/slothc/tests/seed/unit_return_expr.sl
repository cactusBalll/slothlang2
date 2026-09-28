// seed: `return expr;` in a unit function is accepted and the value discarded
// (core Bug1 / book §9.1)
func g(): unit { return 5; }
func main(): unit { g(); print("ok"); }  // expect: ok
