// spec: cross-module inherited override virtual dispatch + imported-module
// virtual call before root slots exist (VD-P1 vt_cap timing)
import "m128.mod.sl" as M;

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
    print(a.kind());              // expect: animal
    print(M.describe_animal());   // expect: animal
    let d = Dog();
    print(report(d));             // expect: dog
    print(d.describe());          // expect: dog
}
