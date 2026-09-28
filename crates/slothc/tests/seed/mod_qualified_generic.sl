// seed: qualified construction of an imported generic class (module M1b / F1)
import "mod_qualified_generic_lib.slt" as G;
func main(): unit {
    let b = G.Box<int>(3);
    print(b.get());     // expect: 3
}
