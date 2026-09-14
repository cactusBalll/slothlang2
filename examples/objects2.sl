// sloth-lang 2.0 §9.2 example, adapted to the implemented subset
trait Speaker { func say(): unit; }

class Mammal impl Speaker {
    let kind: str;
    func __init__() {
        this.kind = "Mammal";
    }
    func say(): unit {
        print("Mammal kind is: ${this.kind}\n");
    }
}

class Cat: Mammal {
    func __init__() {
        super.__init__();
        this.kind = "Cat";
    }
    func say(): unit {
        print("meow\n");
        super.say();
    }
}

class Fish impl Speaker {
    let kind: str = "Fish";
    func say(): unit {
        print("Fish kind is: ${this.kind}\n");
    }
}

pub func main(): unit {
    let l: Array<dyn Speaker> = [Cat(), Mammal(), Fish()];
    for (var m: l) { m.say(); }
}
