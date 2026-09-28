// seed: cross-module class construction, inheritance from an imported base,
// virtual dispatch and method calls across the module edge.
import "mod_crossmod_classes_lib.slt" as M;

class Dog: Animal {
    func __init__() {
        super.__init__();
        return;
    }
    func kind(): str {
        return "dog";
    }
}

func report(a: Animal): str {
    return a.kind();
}

func main(): unit {
    let a = M.newborn();
    print(a.kind());            // expect: animal
    print(M.Animal().kind());   // expect: animal
    let d = Dog();
    print(report(d));           // expect: dog
    print(d.describe());        // expect: dog
}
