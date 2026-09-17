// spec: literals + arithmetic (design §2.1 builtins, unboxed word kinds)
func main(): unit {
    print(1 + 2);           // expect: 3
    print(7 - 9);           // expect: -2
    print(6 * 7);           // expect: 42
    print(7 / 2);           // expect: 3
    print(7 % 3);           // expect: 1
    print(2 + 3 * 4);       // expect: 14
    print((2 + 3) * 4);     // expect: 20
    print(1.5 + 2.0);       // expect: 3.5
    print(2.0 * 3.5);       // expect: 7
    print(float(1) + 2.5);  // expect: 3.5
    print(10 - 3 - 2);      // expect: 5
    print(-4 + 10);         // expect: 6
    print(2 * 3 + 1 == 7);  // expect: true
    print(1.5 % 1.0);       // expect: 0.5
    var z = 0;
    print(z);               // expect: 0
    print(2.5 == 2.5);      // expect: true
    print(1 < 2 and 3 != 4);// expect: true
    print(2.5 > 2.0);       // expect: true
    print(1.5 <= 1.5);      // expect: true
    print(2.5 != 3.5);      // expect: true
}

