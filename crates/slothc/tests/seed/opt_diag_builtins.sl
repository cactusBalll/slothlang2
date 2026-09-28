// seed: Result ctor needs a declared target; builtin argument-type diagnostics
// diag: ctor `ok` requires a declared Result target
// diag: int() of str unsupported (MVP)
// diag: float() of str unsupported (MVP)
// diag: chars() expects a `str`, got `int`
func main(): unit {
    let x = ok(5);
    print(int("5"));
    print(float("5"));
    print(chars(5).len());
}
