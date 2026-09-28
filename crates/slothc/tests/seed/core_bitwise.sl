// seed: core bitwise/shift operators and their precedence (book §7.1, §7.2)
func main(): unit {
    print(6 & 3);            // expect: 2
    print(6 | 3);            // expect: 7
    print(6 ^ 3);            // expect: 5
    print(1 << 4);           // expect: 16
    print(256 >> 4);         // expect: 16
    print(~5);               // expect: -6
    print(~0);               // expect: -1
    print(~(~7));            // expect: 7
    // `<<`/`>>` are looser than `+`/`-`, tighter than `&`/`^`/`|`
    print(1 + 2 << 3);       // expect: 24
    print(1 << 2 + 1);       // expect: 8
    print(5 & 3 | 8);        // expect: 9
    print(1 | 2 ^ 3);        // expect: 1
    print(1024 >> 2 >> 3);   // expect: 32
    // `|`/`^`/`&` bind tighter than comparisons
    print(6 & 3 == 3);       // expect: false
    print(1 & 1 == 1);       // expect: true
    print(1 ^ 3 == 2);       // expect: true
    // unary binds tightest
    print(~ 0 + 1);          // expect: 0
    print(~-1);              // expect: 0
}
