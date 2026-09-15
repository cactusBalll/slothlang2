// spec: dyn-call return surface — str-valued vtable call keeps identity (§2.5)
trait Named {
    func name(): str;
}
class Alpha impl Named {
    func name(): str {
        return "alpha";
    }
}
class Beta impl Named {
    func name(): str {
        return "beta";
    }
}
func tag(n: Array<dyn Named>): unit {
    for x in n {
        print(x.name());            // expect: alpha
        // expect: beta
    }
}
func main(): unit {
    let l: Array<dyn Named> = [Alpha(), Beta()];
    tag(l);
    print("tagged ${Alpha().name()}");   // expect: tagged alpha
}

