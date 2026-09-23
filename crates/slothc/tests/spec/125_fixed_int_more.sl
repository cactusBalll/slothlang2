// spec: fixed-width integer supplements — `is` tests, unsigned literals, and
// unsigned float promotion
func classify(v: any): str {
    if v is int8 { return "int8"; }
    if v is uint8 { return "uint8"; }
    if v is uint { return "uint"; }
    if v is int { return "int"; }
    return "other";
}
func main(): unit {
    // a decimal beyond i64::MAX is an unsigned literal; `u` forces unsigned
    print(18446744073709551615);     // expect: 18446744073709551615
    print(5u);                       // expect: 5
    print(9223372036854775807);      // expect: 9223372036854775807

    // `is` on concrete and `any` surfaces is width-precise
    var x: int8 = int8(5);
    print(x is int8);                // expect: true
    print(x is int);                 // expect: false
    print(classify(x));              // expect: int8
    print(classify(7u));             // expect: uint
    print(classify(7));              // expect: int
    var u8: uint8 = uint8(3);
    print(classify(u8));             // expect: uint8

    // unsigned promotion to float keeps the magnitude
    print(float(uint(-1)) > 1.0e18); // expect: true
    var f: float = uint(-1);
    print(f > 1.0e18);               // expect: true
    print(float(int8(-5)));          // expect: -5
}
