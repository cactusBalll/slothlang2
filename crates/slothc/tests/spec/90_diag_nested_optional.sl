// spec: diagnostics — nested optional `int??` rejected (design §2.6)
// diag: nested optional T?? is not allowed
func f(x: int??): unit {
    return;
}
func main(): unit {
    f(nil);
}
