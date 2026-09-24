// spec: Array range slicing — `a[lo..hi]` / `a[lo..=hi]` = fresh copy;
// `a[lo..hi] = src` writes element-wise (design §3, book ch12).
func main(): unit {
    var a = [1, 2, 3, 4, 5];
    print(a[1..3]);        // expect: [2, 3]
    print(a[1..=3]);       // expect: [2, 3, 4]
    print(a[0..0]);        // expect: []
    print(a[2..5]);        // expect: [3, 4, 5]
    print(a[2..5][0]);     // expect: 3

    // the slice is a fresh copy: writing it does not touch the source
    var sub = a[1..3];
    sub[0] = 20;
    print(sub);            // expect: [20, 3]
    print(a[1]);           // expect: 2

    // str-element slices retain their elements
    var s = ["x", "y", "z"];
    print(s[1..3]);        // expect: [y, z]

    // slice assignment writes through the target
    var b = [10, 20, 30, 40];
    b[1..3] = [200, 300];
    print(b);              // expect: [10, 200, 300, 40]
    b[0..2] = a[2..4];
    print(b);              // expect: [3, 4, 300, 40]
    b[3..=3] = [99];
    print(b);              // expect: [3, 4, 300, 99]

    // ref-typed slice assignment releases the evicted elements
    var t = ["a", "b", "c", "d"];
    t[1..3] = ["X", "Y"];
    print(t);              // expect: [a, X, Y, d]

    // nested containers: slice an inner array, write through an inner slice
    var g = [[1, 2, 3], [4, 5, 6]];
    print(g[0][0..2]);     // expect: [1, 2]
    g[1][0..2] = [40, 50];
    print(g[1]);           // expect: [40, 50, 6]
}
