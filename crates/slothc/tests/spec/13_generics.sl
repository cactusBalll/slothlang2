// spec: generics — function monomorphization, explicit targs, generic class (§2.3)
func id<T>(x: T): T {
    return x;
}
func first<T>(xs: Array<T>): T {
    return xs[0];
}
func count<T>(xs: Array<T>): int {
    var n = 0;
    for x in xs {
        n = n + 1;
    }
    return n;
}
class Box<T> {
    var v: T;
    func unwrap(): T {
        return this.v;
    }
}
func main(): unit {
    print(id(42));              // expect: 42
    print(id(id(7)));           // expect: 7
    print(id(2.5));             // expect: 2.5
    print(id("s"));             // expect: s
    var a = [10, 20];
    print(first(a));            // expect: 10
    var b = ["u", "v"];
    print(first(b));            // expect: u
    print(count(a));            // expect: 2
    print(first<int>(a));       // expect: 10
    print(id<bool>(true));      // expect: true
    let bi = Box<int>();
    bi.v = 12;
    print(bi.unwrap());         // expect: 12
    let bf = Box<float>();
    bf.v = 1.5;
    print(bf.unwrap());         // expect: 1.5
}

