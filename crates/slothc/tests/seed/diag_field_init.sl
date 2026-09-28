// seed: field initializer surface conflicts are diagnosed — core Bug6
class C { var n: int = "hello"; }
func main(): unit { var c = C(); print(c.n); }
// diag: type mismatch in field initializer
