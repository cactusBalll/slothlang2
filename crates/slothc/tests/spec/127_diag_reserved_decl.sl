// spec: reserved `__sloth_*` symbols may only be declared in the prelude
extern func __sloth_my_reserved(x: int): int;
func main(): unit {
    print(__sloth_my_reserved(1));
}
// diag: may only be declared in the prelude
