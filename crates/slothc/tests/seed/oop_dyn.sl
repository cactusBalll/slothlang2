// seed: dyn Trait — heterogeneous arrays, Display interpolation/print,
// class-name fallback, value-type auto-box for int/float/bool (book ch19.2–19.4)
trait Speaker {
    func name(): str;
    func say(): str { return "I am ${this.name()}"; }
}
trait Display { func to_str(): str; }
class Cat impl Speaker {
    func name(): str { return "cat"; }
}
class Dog impl Speaker, Display {
    func name(): str { return "dog"; }
    func to_str(): str { return "Dog()"; }
}
class Plain { var x: int; func __init__(v: int) { this.x = v; } }
func announce(s: dyn Speaker): str { return s.say(); }
func main(): unit {
    var xs: Array<dyn Speaker> = [Cat(), Dog()];
    for x in xs {
        print(x.say());        // expect: I am cat
        print(x.say());        // expect: I am dog
    }
    print(announce(Dog()));    // expect: I am dog
    print("${Dog()}");         // expect: Dog()
    let p = Plain(7);
    print(p);                  // expect: Plain
    print("p=${p}");           // expect: p=Plain
    var xs2: Array<dyn Display> = [Dog(), 7, true];
    for x in xs2 { print(x.to_str()); }   // expect: Dog()
    for x in xs2 { print(x.to_str()); }   // expect: 7
    for x in xs2 { print(x.to_str()); }   // expect: true
}
