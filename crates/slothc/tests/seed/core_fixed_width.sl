// seed: fixed-width & unsigned integers — truncation, wrapping, signed vs
// unsigned division/comparison (book §5.6, §7.1)
func main(): unit {
    // conversion truncates to target width
    print(int8(300));        // expect: 44
    print(int8(200));        // expect: -56
    print(uint8(300));       // expect: 44
    print(int32(5000000000)); // expect: 705032704
    print(uint(-1));         // expect: 18446744073709551615

    // wrapping at the declared width
    print(int8(127) + 1);    // expect: -128
    print(int8(-128) - 1);   // expect: 127
    print(uint8(255) + 1);   // expect: 0
    print(uint8(250) + 10);  // expect: 4
    print(uint(0) - 1);      // expect: 18446744073709551615

    // signed vs unsigned interpretation of the same bit pattern
    print(int8(-1) < int8(1));    // expect: true
    print(uint8(200) > uint8(100)); // expect: true
    print(uint(-1) > uint(1));    // expect: true
    print(int8(-1) / int8(2));    // expect: 0
    print(uint8(200) / uint8(3)); // expect: 66
    print(int8(-1) >> int8(1));   // expect: -1
    print(uint8(200) >> uint8(1)); // expect: 100
    print(uint(1) << uint(63));   // expect: 9223372036854775808

    // an `int` literal adopts the other operand's width
    var u: uint = 5;
    print(u + 3);            // expect: 8
    print(u == 5);           // expect: true

    // unsigned promotion to float keeps the magnitude
    print(float(uint(-1)) > 1.0e18); // expect: true
}
