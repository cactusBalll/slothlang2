// spec: cross-module globals — pub var reads and foreign-body writes (§3.7)
import "m56.mod.sl" as M;

func main(): unit {
    print(M.counter);        // expect: 5
    M.bump();
    print(M.counter);        // expect: 6
    print(M.get());          // expect: 6
    print(M.label);          // expect: mod
}
