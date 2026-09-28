// seed: a module using its own generic class works (module M1a / F1)
import "mod_generic_chain_user.slt";
func main(): unit {
    print(use());       // expect: 5
}
