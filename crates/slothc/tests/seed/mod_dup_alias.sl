// seed: a duplicate import with a second alias keeps both aliases (module M7 / F7)
import "mod_dup_alias_lib.slt" as L1;
import "mod_dup_alias_lib.slt" as L2;
func main(): unit {
    print(L1.f());   // expect: 7
    print(L2.f());   // expect: 7
}
