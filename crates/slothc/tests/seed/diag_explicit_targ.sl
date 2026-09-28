// seed: explicit generic type args are validated against the arguments
// (oop Bug4 / C4)
func first<T>(xs: Array<T>): T { return xs[0]; }
func main(): unit {
    let a = [1, 2];
    print(first<str>(a));
}
// diag: type mismatch in function argument
