// seed: an opaque extern type created and consumed inside an imported module,
// returned to the root only as a plain int word.
import "mod_ffi_module_lib.slt" as F;

func main(): unit {
    print(F.tok_val());    // expect: 99
}
