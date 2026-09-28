// seed: inheritance — ctor chain, base-first fields, super.method, is narrowing
// (book ch18.2/ch18.3)
class Animal {
    var tag: int;
    func __init__(t: int) { this.tag = t; }
    func who(): str { return "animal"; }
}
class Dog: Animal {
    var loud: int;
    func __init__(l: int) {
        super.__init__(7);
        this.loud = l;
    }
    func who(): str { return "dog"; }
    func loud_who(): str { return super.who() + "/" + this.who(); }
}
class Puppy: Dog {
    func __init__() { super.__init__(2); }
}
func main(): unit {
    let d = Dog(2);
    print(d.tag);              // expect: 7
    print(d.loud);             // expect: 2
    print(d.who());            // expect: dog
    print(d.loud_who());       // expect: animal/dog
    let p = Puppy();
    print(p.tag);              // expect: 7
    print(p.loud);             // expect: 2
    if p is Dog {
        print(p.loud);         // expect: 2
    }
    if d is Puppy {
        print(0);
    } else {
        print(11);             // expect: 11
    }
}
