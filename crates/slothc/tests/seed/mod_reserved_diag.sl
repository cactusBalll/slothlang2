// seed: the reserved `__sloth_*` ABI needs the `import "__sloth"` pseudo-import.
func main(): unit {
    print(__sloth_str_len("x"));
    // diag: may only be called from a module that imports "__sloth"
}
