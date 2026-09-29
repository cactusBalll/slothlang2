// seed: reference captures persist across calls with correct ARC (bug B1);
// the outer binding still sees the construction-time snapshot handle (book §10.3)
func make(): () -> Array<int> {
    var a: Array<int> = [1];
    return || -> Array<int> { a = [a[0] + 1]; return a; };
}
func main(): unit {
    let f = make();
    print(f()[0]); // expect: 2
    print(f()[0]); // expect: 3
    print(f()[0]); // expect: 4
}
