// spec: module-level globals — initializers, reads/writes from functions,
// inferred and annotated surfaces (design §3.2/§3.7)
var g = 10;
var h: int = 3;
var name = "world";
let fixed = 7;
func bump(): unit { g = g + 1; }
func readg(): int { return g; }
func greet(): str { return "hi " + name; }
func main(): unit {
    print(g);              // expect: 10
    print(h);              // expect: 3
    print(fixed);          // expect: 7
    bump();
    bump();
    print(g);              // expect: 12
    print(readg());        // expect: 12
    g = 100;
    print(g);              // expect: 100
    print(name);           // expect: world
    name = "there";
    print(greet());        // expect: hi there
    h = h * 2;
    print(h);              // expect: 6
    print(name);           // expect: there
}
