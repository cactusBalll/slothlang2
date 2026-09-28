// seed: generics — monomorphization, inference and explicit type args,
// generic classes, generic class implementing a trait, trait bounds
// (book ch11)
func first<T>(xs: Array<T>): T { return xs[0]; }
func twice<T>(x: T): Array<T> { return [x, x]; }
func maxi<T: Comparable>(a: T, b: T): T { if a < b { return b; } return a; }
class Box<T> {
    var v: T;
    func __init__(x: T) { this.v = x; }
    func set(x: T) { this.v = x; }
    func get(): T { return this.v; }
}
trait Show { func show(): str; }
class Wrapper<T> impl Show {
    var v: T;
    func __init__(x: T) { this.v = x; }
    func show(): str { return "W(${this.v})"; }
}
func main(): unit {
    var a = [10, 20];
    print(first(a));           // expect: 10
    print(first<int>(a));      // expect: 10
    var s = ["x", "y"];
    print(first(s));           // expect: x
    let b = twice("z");
    print(b[0] + b[1]);        // expect: zz
    print(maxi(3, 7));         // expect: 7
    print(maxi(2.5, 1.5));     // expect: 2.5
    let bi = Box<int>(5);
    print(bi.get());           // expect: 5
    let bf = Box<float>(1.5);
    bf.set(2.5);
    print(bf.get());           // expect: 2.5
    let nested = Box<Box<int>>(Box<int>(9));
    print(nested.get().get()); // expect: 9
    let w = Wrapper<int>(4);
    print(w.show());           // expect: W(4)
    let d: dyn Show = Wrapper<str>("q");
    print(d.show());           // expect: W(q)
}
