// spec: inheritance — three levels, inherited fields, super calls, is
class Animal {
    var tag: int;
    func __init__(t: int) { this.tag = t; }
    func kind(): str { return "animal"; }
}
class Dog: Animal {
    var loud: int;
    func __init__(l: int) { super.__init__(7); this.loud = l; }
    func kind(): str { return "dog"; }
    func bark(): int { return this.loud; }
}
class Puppy: Dog {
    func __init__() { super.__init__(2); }
}
func main(): unit {
    let a = Animal(1);
    print(a.kind());          // expect: animal
    let d = Dog(3);
    print(d.tag);             // expect: 7
    print(d.kind());          // expect: dog
    print(d.bark());          // expect: 3
    let p = Puppy();
    print(p.kind());          // expect: dog
    print(p.bark());          // expect: 2
    if p is Dog { print(1); } else { print(0); }        // expect: 1
    if a is Dog { print(0); } else { print(2); }        // expect: 2
    let arr = [a, d, p];      // LUB upcast: Array<Animal>
    print(arr.len());         // expect: 3
    print(arr[1].kind());     // expect: dog
    print(arr[1].tag);        // expect: 7
}
