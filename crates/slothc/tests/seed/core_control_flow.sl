// seed: core control flow — if/while with and without parens, for forms A/B,
// nested break/continue, early return (book §8)
func find(a: Array<int>, t: int): int {
    var i = 0;
    while i < a.len() {
        if a[i] == t { return i; }
        i = i + 1;
    }
    return 0 - 1;
}
func main(): unit {
    var x = 2;
    if x == 1 { print("one"); } else if x == 2 { print("two"); } else { print("other"); } // expect: two
    if (x == 2) { print("paren-if"); }   // expect: paren-if
    var i = 0;
    while (i < 3) { i = i + 1; }
    print(i);                            // expect: 3
    var j = 0;
    while j < 3 { j = j + 1; }
    print(j);                            // expect: 3
    // form A and form B are equivalent
    for k in 0..3 { print(k); }          // expect: 0
                                         // expect: 1
                                         // expect: 2
    for (var k: 0..3) { print(k); }      // expect: 0
                                         // expect: 1
                                         // expect: 2
    // nested break/continue act on the nearest loop
    var s = 0;
    for a in 0..4 {
        for b in 0..4 {
            if b == 2 { continue; }
            if a == 3 { break; }
            s = s + 1;
        }
    }
    print(s);                            // expect: 9
    let arr = [4, 5, 6];
    print(find(arr, 6));                 // expect: 2
    print(find(arr, 9));                 // expect: -1
}
