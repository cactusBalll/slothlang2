// seed: declarations of reserved `__sloth_*` names are prelude-only.
func __sloth_user_fn(): int {
    return 1;
}

func main(): unit {
    print(1);
    // diag: may only be declared in the prelude
}
