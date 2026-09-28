// seed: reading an undeclared field is a diagnostic, not OOB memory
// (core Bug3 / oop Bug7)
class C { var n: int = 7; }
func main(): unit { var c = C(); print(c.bogus); }
// diag: unknown field `bogus`
