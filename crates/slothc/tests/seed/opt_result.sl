// seed: Result<T,E> — typed ok()/err() constructors from let/var/return and
// assignment targets, is_ok()/unwrap()/err(), reference payloads, and the
// unwrap-on-err panic channel (book ch17).
class C {
    var n: int;
    func __init__(n: int) { this.n = n; }
}
func parse(x: int): Result<int, str> {
    if x < 0 {
        return err("neg");
    }
    if x == 0 {
        return err("zero");
    }
    return ok(x * 2);
}
func mkc(): Result<C, str> {
    return ok(C(7));
}
func mks(): Result<str, str> {
    return ok("hello");
}
func main(): unit {
    let a = parse(5);
    print(a.is_ok());                // expect: true
    print(a.unwrap());               // expect: 10
    let b = parse(-1);
    print(b.is_ok());                // expect: false
    print(b.err());                  // expect: neg
    print(parse(0).err());           // expect: zero

    // explicit Result targets select the instance frame
    let t: Result<float, int> = ok(2.5);
    print(t.unwrap());               // expect: 2.5
    var q: Result<int, str> = err("seed");
    q = ok(8);                       // assign-face ctor
    print(q.unwrap());               // expect: 8
    q = err("end");
    print(q.err());                  // expect: end

    // reference payloads: unwrap delivers a usable value
    let c = mkc();
    let o = c.unwrap();
    print(o.n);                      // expect: 7
    let s = mks();
    print(s.unwrap().len());         // expect: 5
}
