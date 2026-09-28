// spec: runtime `typeid` for all reference types (design §2.6/§4.3.2) —
// monomorphic concrete ids; classes/dyn resolve their most-derived runtime
// class id through the object header
trait Animal {
    func sound(): str;
}
class Dog impl Animal {
    func sound(): str { return "woof"; }
}
class Cat impl Animal {
    func sound(): str { return "meow"; }
}
class Box<T> {
    var v: T;
    func __init__(x: T) {
        this.v = x;
    }
}
func main(): unit {
    // dyn upcast: runtime class id, not the static trait surface
    let a: dyn Animal = Dog();
    print(typeid(a) == typeid(Dog()));          // expect: true
    print(typeid(a) == typeid(Cat()));          // expect: false
    // most-derived through a `dyn` reference
    let b: dyn Animal = Dog();
    print(typeid(b) == typeid(Dog()));          // expect: true
    print(typeid(b) == typeid(Cat()));          // expect: false
    // monomorphized container/class instances are distinct types
    print(typeid([1, 2]) == typeid([3, 4]));    // expect: true
    print(typeid([1]) == typeid(["s"]));        // expect: false
    print(typeid(Box<int>(1)) == typeid(Box<int>(2)));  // expect: true
    print(typeid(Box<int>(1)) == typeid(Box<str>("x"))); // expect: false
    // distinct families never collide
    print(typeid("s") == typeid(1..2));         // expect: false
    print(typeid(@("k": 1)) == typeid([1]));    // expect: false
    // optional reference: nil -> 0, live -> inner concrete id
    let d: Dog? = nil;
    print(typeid(d));                           // expect: 0
    let d2: Dog? = Dog();
    print(typeid(d2) == typeid(Dog()));         // expect: true
}
