// spec: generics — parameter-swapped instances, generic class methods,
// monomorphized inference over strings/floats
class Pair<A, B> {
    var a: A;
    var b: B;
    func __init__(a: A, b: B) { this.a = a; this.b = b; }
    func swap(): Pair<B, A> { return Pair<B, A>(this.b, this.a); }
    func first(): A { return this.a; }
}
class Box<T> {
    var v: T;
    func set(x: T) { this.v = x; }
    func get(): T { return this.v; }
}
func id<T>(x: T): T { return x; }
func twice<T>(x: T): Array<T> { return [x, x]; }
func main(): unit {
    let p = Pair<int, str>(1, "a");
    let q = p.swap();
    print(q.first());         // expect: a
    print(p.first());         // expect: 1
    print(id("s"));           // expect: s
    let bs = twice("z");
    print(bs.len());          // expect: 2
    print(bs[0] + bs[1]);     // expect: zz
    let bx = Box<float>();
    bx.set(2.5);
    print(bx.get());          // expect: 2.5
    let bi: Box<int> = Box<int>();
    bi.set(7);
    print(bi.get());          // expect: 7
    print(id(id(id(9))));     // expect: 9
}
