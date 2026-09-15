// spec: traits & dyn — impl dispatch, default method bodies, dyn arrays (§2.5)
trait Speaker {
    func noise(): str;
}
class Cat impl Speaker {
    func noise(): str {
        return "meow";
    }
}
class Bird impl Speaker {
    func noise(): str {
        return "tweet";
    }
}
trait Display {
    func to_str(): str;
}
class Pt impl Display {
    var x: int;
    var y: int;
    func __init__(x: int, y: int) {
        this.x = x;
        this.y = y;
    }
    func to_str(): str {
        return "Pt(${this.x}, ${this.y})";
    }
}
trait Greeter {
    func greet(): str {
        return "hello";
    }
    func name(): str;
}
class Formal impl Greeter {
    func name(): str {
        return "world";
    }
}
func main(): unit {
    let c = Cat();
    print(c.noise());              // expect: meow
    var sp: Array<dyn Speaker> = [Cat(), Bird()];
    for x in sp {
        print(x.noise());          // expect: meow
        // expect: tweet
    }
    print(Pt(1, 2));               // expect: Pt(1, 2)
    print("p is ${Pt(4, 5)}");     // expect: p is Pt(4, 5)
    let g = Formal();
    print(g.greet() + (" " + g.name()));  // expect: hello world
}

