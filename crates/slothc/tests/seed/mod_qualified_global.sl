// seed: qualified cross-module global write (module M3 / F3)
import "mod_qualified_global_lib.slt" as D;
func main(): unit {
    print(D.counter);       // expect: 3
    D.counter = 42;
    print(D.counter);       // expect: 42
}
