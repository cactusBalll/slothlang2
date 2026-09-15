// spec: cross-module — plain import funcs; alias module for classes (proven routes)
import "m26.mod.sl" as M26;

func main(): unit {
    let a = twofold(6);
    print(a);                        // expect: 12
    let b = M26.add(2, 3);
    print(b);                        // expect: 5
    let c = Counter(2);
    print(c.bump(3));                // expect: 5
}

