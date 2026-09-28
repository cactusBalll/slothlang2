// seed: cross-module traits — imported trait implemented by a root class and
// vice versa, `dyn Trait` dispatch across the module edge.
import "mod_crossmod_traits_lib.slt";

trait Greeter {
    func greet(): str;
}

class Human impl Greeter {
    func greet(): str {
        return "hi";
    }
}

class Android impl Speaker {
    func speak(): str {
        return "buzz";
    }
}

func announce(s: dyn Speaker): str {
    return s.speak();
}

func greet_all(g: dyn Greeter): str {
    return g.greet();
}

func main(): unit {
    print(announce(Robot()));    // expect: beep
    print(announce(Android()));  // expect: buzz
    print(announce(mk()));       // expect: beep
    print(greet_all(Human()));   // expect: hi
    let r: dyn Speaker = Robot();
    print(r.speak());            // expect: beep
}
