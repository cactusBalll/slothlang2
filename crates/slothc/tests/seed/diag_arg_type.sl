// seed: argument surfaces are checked against declared parameter surfaces
// (core Bug7 / oop Bug8)
func take_str(s: str): int { return s.len(); }
func main(): unit { print(take_str(5)); }
// diag: type mismatch in function argument
