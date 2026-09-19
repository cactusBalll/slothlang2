// spec: integer semantics — division/modulo signs, precedence, full 64-bit range
func main(): unit {
    print(7 / 2);                 // expect: 3
    print(0 - 7 / 2);             // expect: -3
    print(7 % 3);                 // expect: 1
    print(0 - 7 % 3);             // expect: -1
    print(7 % (0 - 3));           // expect: 1
    print(4611686018427387903);   // expect: 4611686018427387903
    print(9223372036854775807);   // expect: 9223372036854775807
    print(2 * 3 + 4 * 5);         // expect: 26
    print((2 + 3) * (4 + 5));     // expect: 45
    print(10 - 3 - 2);            // expect: 5
    print(100 / 10 / 2);          // expect: 5
    print(2 * 0 - 3);             // expect: -3
    print(0 - (0 - 8));           // expect: 8
    print(2 + 3 * 4 - 1);         // expect: 13
    print(17 % 5 + 17 / 5);       // expect: 5
}
