// seed: wrong call arity is a user diagnostic, not an MLIR ICE (core Bug4)
func f(a: int): int { return a; }
func main(): unit { print(f(1, 2)); }
// diag: wrong number of arguments
