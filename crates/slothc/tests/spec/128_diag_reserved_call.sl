// spec: reserved `__sloth_*` symbols may only be called with `import "__sloth";`
func main(): unit {
    let h = __sloth_rc_new(16, 0, 0);
    print(h);
}
// diag: may only be called from a module that imports "__sloth"
