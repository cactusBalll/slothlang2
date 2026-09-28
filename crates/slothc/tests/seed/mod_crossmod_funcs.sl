// seed: cross-module functions (aliased + unqualified), transitive import,
// pub var read and foreign-body write routed back to the defining module.
import "mod_crossmod_funcs_lib.slt" as L;

func main(): unit {
    print(double(21));        // expect: 42
    print(L.via_dep());       // expect: 11
    print(L.counter);         // expect: 5
    L.bump();
    print(L.counter);         // expect: 6
}
