// seed: a module carrying `import "__sloth";` can use reserved container
// symbols (module M6 / F6)
import "mod_imported_reserved_lib.slt";
func main(): unit {
    print(slen("hello"));   // expect: 5
}
