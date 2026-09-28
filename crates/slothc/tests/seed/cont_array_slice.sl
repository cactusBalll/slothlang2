// seed: Array range slicing — a[lo..hi] / a[lo..=hi] fresh copies, chained
// reads, and slice assignment (book ch12 §12.1)
func main(): unit {
    var a = [10, 20, 30, 40, 50];
    print(a[1..3]);           // expect: [20, 30]
    print(a[1..=3]);          // expect: [20, 30, 40]
    print(a[0..0]);           // expect: []
    print(a[5..5]);           // expect: []
    print(a[2..5]);           // expect: [30, 40, 50]

    // a slice is a fresh copy: writing it does not affect the source
    var s = a[1..3];
    s[0] = 99;
    print(s);                 // expect: [99, 30]
    print(a);                 // expect: [10, 20, 30, 40, 50]

    // chained slice reads
    print(a[1..4][0]);        // expect: 20
    print(a[1..4][0..2]);     // expect: [20, 30]

    // slice assignment
    a[1..3] = [9, 8];
    print(a);                 // expect: [10, 9, 8, 40, 50]
    a[0..=1] = [7, 6];
    print(a);                 // expect: [7, 6, 8, 40, 50]
    var src = [100, 200, 300];
    a[2..5] = src;
    print(a);                 // expect: [7, 6, 100, 200, 300]

    // nested slice assignment
    var g = [[1, 2, 3], [4, 5, 6]];
    g[0][1..2] = [9];
    print(g);                 // expect: [[1, 9, 3], [4, 5, 6]]
    g[1][0..=1] = [7, 8];
    print(g);                 // expect: [[1, 9, 3], [7, 8, 6]]
}
