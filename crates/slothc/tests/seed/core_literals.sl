// seed: core literal forms — decimal, uint literals, `u`/`U` suffix,
// scientific notation, booleans (book §4.2)
func main(): unit {
    print(42);                       // expect: 42
    print(0);                        // expect: 0
    print(5u);                       // expect: 5
    print(5U);                       // expect: 5
    print(18446744073709551615);     // expect: 18446744073709551615
    print(9223372036854775807);      // expect: 9223372036854775807
    print(3.5);                      // expect: 3.5
    print(1e3);                      // expect: 1000
    print(2.5e-1);                   // expect: 0.25
    print(2E+4);                     // expect: 20000
    print(1e0);                      // expect: 1
    print(1.5e-3);                   // expect: 0.0015
    print(true);                     // expect: true
    print(false);                    // expect: false
    print(-9223372036854775807 - 1); // expect: -9223372036854775808
}
