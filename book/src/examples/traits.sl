trait Speaker {
    func name(): str;
    // 默认方法体：可调用 this 上的其它（虚）方法
    func say(): str { return "I am ${this.name()}"; }
}

trait Display {
    func to_str(): str;
}

class Cat impl Speaker {
    func name(): str { return "cat"; }
}

class Dog impl Speaker, Display {
    func name(): str { return "dog"; }
    func to_str(): str { return "Dog()"; }
}

// dyn Trait：运行时多态
func announce(s: dyn Speaker): str {
    return s.say();
}

func main() {
    var xs: Array<dyn Speaker> = [Cat(), Dog()];
    for x in xs {
        print(x.say());          // 默认体在子类上下文里虚分派
    }
    print(announce(Dog()));      // I am dog
    print("${Dog()}");           // Dog()  插值走 Display.to_str
}
