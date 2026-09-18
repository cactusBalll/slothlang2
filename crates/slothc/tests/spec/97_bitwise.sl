// spec: int bitwise/shift operators `& | ^ << >> ~` (design §3.3)
func main(): unit {
    print(6 & 3);            // expect: 2
    print(6 | 3);            // expect: 7
    print(6 ^ 3);            // expect: 5
    print(1 << 4);           // expect: 16
    print(256 >> 4);         // expect: 16
    print(~5);               // expect: -6
    print(~0);               // expect: -1

    // precedence: `<<`/`>>` looser than `+`/`-`, tighter than `&`, which is
    // tighter than `^`, which is tighter than `|`; all tighter than `==`
    print(1 + 2 << 3);       // expect: 24
    print(1 << 2 + 1);       // expect: 8
    print(5 & 3 | 8);        // expect: 9
    print(1 | 2 ^ 3);        // expect: 1
    print(1024 >> 2 >> 3);   // expect: 32
    print(6 & 3 == 3);       // expect: false
    print(1 & 1 == 1);       // expect: true
    print(1 == 1 & 1);       // expect: true

    var x = 12;
    print(x & 10);           // expect: 8
    print(x | 1);            // expect: 13
    print(x ^ 3);            // expect: 15
    print(x << 2);           // expect: 48
    print(x >> 1);           // expect: 6
    print(~x);               // expect: -13
    print(~(~7));            // expect: 7
    print(20 & ~3);          // expect: 20

    // `<<`/`>>` must not disturb generic closers (`Map<int,Array<int>>`)
    var g: Array<Array<int>> = [[1, 2], [3, 4]];
    print(g[1][1] >> 1);     // expect: 2
}
