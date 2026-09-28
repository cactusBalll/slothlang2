// seed: `ok()`/`err()` resolve against a declared container element/value
// surface inside a literal (OPT5 / E4)
func main(): unit {
    var arr: Array<Result<int, str>> = [ok(1), err("e")];
    print(arr.len());                       // expect: 2
    var m: Map<str, Result<int, str>> = @("a": ok(2));
    print(len(m));                          // expect: 1
}
