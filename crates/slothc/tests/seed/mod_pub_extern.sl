// seed: `pub extern func` parses (appendix C / module M2 / F2)
pub extern func sloth_extern_floor(x: float): float;
func main(): unit {
    print(sloth_extern_floor(2.7));     // expect: 2
}
