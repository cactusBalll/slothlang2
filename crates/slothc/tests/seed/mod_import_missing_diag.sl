// seed: an unresolvable import path is a compile error.
import "mod_this_file_does_not_exist.sl";

func main(): unit {
    print(1);
    // diag: cannot resolve import
}
