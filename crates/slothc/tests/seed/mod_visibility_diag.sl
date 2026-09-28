// seed: non-`pub` top-level symbols of an imported module are inaccessible.
import "mod_visibility_priv.slt" as P;

func main(): unit {
    print(P.hidden_fn());
    // diag: is private to its module
    print(P.visible());
}
