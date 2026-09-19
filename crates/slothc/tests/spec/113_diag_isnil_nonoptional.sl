// spec: `is nil` is rejected on a non-optional value/ref surface
func main(): unit {
    var n: int = 0;
    print(n is nil);
}
// diag: is nil` on non-optional type `int`
