// seed: constructing a non-`pub` imported class is rejected (module M4 / F4)
import "diag_private_class_lib.slt";
func main(): unit {
    let h = Hidden();
    print(h.get());
}
// diag: is private to its module
