// spec: Result<T,E> — typed ctors, is_ok/unwrap/err, return position (§5.5)
func make_ok(): Result<int, str> {
    return ok(5);
}
func make_err(): Result<int, str> {
    return err("boom");
}
func main(): unit {
    let a: Result<int, str> = ok(5);
    print(a.is_ok());            // expect: true
    print(a.unwrap());           // expect: 5
    let b: Result<int, str> = err("boom");
    print(b.is_ok());            // expect: false
    print(b.err());              // expect: boom
    let f: Result<float, int> = ok(2.5);
    print(f.unwrap());           // expect: 2.5
    let z: Result<float, int> = err(9);
    print(z.err());              // expect: 9
    let ro = make_ok();
    print(ro.unwrap());          // expect: 5
    let re = make_err();
    print(re.is_ok());           // expect: false
}

