// seed: circular imports are detected during module assembly.
import "mod_circular_a.slt";

func main(): unit {
    print(a_fn());
    // diag: circular import
}
