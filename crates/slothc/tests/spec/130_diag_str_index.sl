// spec: diagnostics — str subscript requires an int index / chars() needs str
func main(): unit {
    var s = "ab";
    print(s["x"]);
}
// diag: str index must be `int`
