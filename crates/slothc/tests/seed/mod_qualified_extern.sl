// seed: qualified call to an imported `extern func` uses the C ABI
// (module M5 / F5)
import "mod_qualified_extern_lib.slt" as F;
func main(): unit {
    print(F.sloth_extern_floor(2.7));   // expect: 2
}
