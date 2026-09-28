// seed: a non-`pub` class of an imported module cannot be constructed via its
// module qualifier (the class surface is not exported).
import "mod_visibility_priv.slt" as P;

func main(): unit {
    let h = P.Hidden();
    // diag: `P.Hidden` is private to its module (not `pub`)
    print(1);
}
