// seed: nested optional is rejected at parse time (book ch15.1)
// diag: nested optional T?? is not allowed
func f(x: int??): unit {
    return;
}
func main(): unit {
    f(nil);
}
