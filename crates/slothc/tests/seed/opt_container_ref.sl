// seed: a declared `Array<T?>` keeps the reference-optional element surface
// (optionals Bug4 / B9)
class C { var n: int = 5; }
func main(): unit {
    var a: Array<C?> = [C()];
    print(a[0] is nil);        // expect: false
    var f: Array<float?> = [1.5, nil];
    print(f[1] is nil);        // expect: true
    print(f[0] is nil);        // expect: false
}
