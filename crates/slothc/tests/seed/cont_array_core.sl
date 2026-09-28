// seed: Array core — literal, index read/write, push/pop, len, empty literal
// context type, stable handles after growth, nested arrays (book ch12 §12.1/12.2)
func grow(a: Array<int>): unit {
    a.push(42);
}
func main(): unit {
    var a = [1, 2, 3];
    print(a[0]);              // expect: 1
    print(a[2]);              // expect: 3
    print(a.len());           // expect: 3
    print(len(a));            // expect: 3
    a[0] = 10;
    print(a[0]);              // expect: 10
    print(a.pop());           // expect: 3
    print(a.len());           // expect: 2
    a.push(7);
    print(a);                 // expect: [10, 2, 7]

    // stable handle: an alias sees growth
    var b = a;
    a.push(8);
    a.push(9);
    print(b.len());           // expect: 5
    print(b[4]);              // expect: 9
    print(a.len());           // expect: 5

    // stable handle across a call boundary
    grow(a);
    print(a[5]);              // expect: 42

    // nested array element mutation
    let g = [[1], [2, 3]];
    g[0].push(7);
    print(g[0]);              // expect: [1, 7]
    print(g);                 // expect: [[1, 7], [2, 3]]

    // empty literal adopts its context type
    let e: Array<float> = [];
    e.push(1.5);
    print(e);                 // expect: [1.5]
    var d = [];
    d.push(5);
    print(d);                 // expect: [5]
}
