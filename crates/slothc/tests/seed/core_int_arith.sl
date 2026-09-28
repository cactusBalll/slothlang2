// seed: core integer arithmetic — division/modulo signs, associativity, and
// full 64-bit wrapping (book §4.2, §5.2, §7.1)
func main(): unit {
    print(7 / 2);                       // expect: 3
    print(0 - 7 / 2);                   // expect: -3
    print(7 % 3);                       // expect: 1
    print(0 - 7 % 3);                   // expect: -1
    print(7 % (0 - 3));                 // expect: 1
    print(0 - 7 % (0 - 3));             // expect: -1
    print(2 - 3 - 4);                   // expect: -5
    print(2 * 3 % 4);                   // expect: 2
    print(10 - 2 + 3);                  // expect: 11
    print(100 / 10 / 2);                // expect: 5
    print(17 % 5 + 17 / 5);             // expect: 5
    print(0 - (0 - 8));                 // expect: 8
    print(2 * 0 - 3);                   // expect: -3
    // full-range literals and two's-complement wrapping
    print(9223372036854775807);         // expect: 9223372036854775807
    print(9223372036854775807 + 1);     // expect: -9223372036854775808
    print(-9223372036854775807 - 1);    // expect: -9223372036854775808
    print(9223372036854775807 * 2);     // expect: -2
}
