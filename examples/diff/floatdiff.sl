// Float precision differential: native f64 (de-tag); print format differs from C %.17g.
func main(): unit {
    print(1.0 / 3.0 * 3.0);            // exact 1.0 in IEEE f64
    print(1.0 / 7.0 * 7.0);            // exact 1.0 in IEEE f64
    var s = 0.0;
    var i = 1;
    while i <= 1000 { s = s + 1.0 / float(i); i = i + 1; }
    print(s);
    var p = 1.0;
    i = 1;
    while i <= 50 { p = p * 1.0001; i = i + 1; }
    print(p);
}
