// spec: inheritance — ctor chain, base fields, virtual dispatch, is narrowing (§2.4)
class Animal {
    var tag: int;
    func __init__(t: int) {
        this.tag = t;
    }
    func who(): str {
        return "animal";
    }
}
class Dog: Animal {
    var loud: int;
    func __init__(l: int) {
        super.__init__(7);
        this.loud = l;
    }
    func who(): str {
        return "dog";
    }
    func bark(): int {
        return this.loud;
    }
}
class Puppy: Dog {
    func __init__() {
        super.__init__(2);
    }
}
func main(): unit {
    let d = Dog(2);
    print(d.tag);              // expect: 7
    print(d.loud);             // expect: 2
    print(d.who());            // expect: dog
    let p = Puppy();
    print(p.who());            // expect: dog
    print(p.tag);              // expect: 7
    print(p.bark());           // expect: 2
    if p is Dog {
        print(p.tag);          // expect: 7
    }
    if p is Animal {
        print(9);              // expect: 9
    } else {
        print(0);
    }
    if d is Puppy {
        print(0);
    } else {
        print(11);             // expect: 11
    }
}

