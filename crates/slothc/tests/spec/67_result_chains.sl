// spec: Result<T,E> chains — unwrap/err/is_ok, reassign, branching (§5.5)
func parse(x: int): Result<int, str> {
    if x < 0 { return err("neg"); }
    if x == 0 { return err("zero"); }
    return ok(x * 2);
}
func main(): unit {
    print(parse(3).unwrap());   // expect: 6
    print(parse(-1).is_ok());   // expect: false
    print(parse(-1).err());     // expect: neg
    print(parse(0).err());      // expect: zero
    let r = parse(5);
    if r.is_ok() { print(r.unwrap()); } else { print(0); }  // expect: 10
    var a: Result<int, str> = ok(1);
    a = ok(2);
    print(a.unwrap());          // expect: 2
    a = err("x");
    print(a.err());             // expect: x
    let z: Result<float, int> = err(7);
    print(z.err());             // expect: 7
}
