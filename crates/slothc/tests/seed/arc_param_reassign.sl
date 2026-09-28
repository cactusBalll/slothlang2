// seed: assigning to a borrowed reference parameter must not release the
// caller's count (cross C0 / A1) — previously double free
func f(s: str): str {
    s = "z";
    return s;
}
func main(): unit { print(f("x")); }  // expect: z
