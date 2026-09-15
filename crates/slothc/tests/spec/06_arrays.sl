// spec: arrays — literal, index r/w, push/pop, len, for-in, float elements (§5.3)
func main(): unit {
    var a = [1, 2, 3];
    print(a[0]);              // expect: 1
    print(a[2]);              // expect: 3
    print(a.len());           // expect: 3
    a.push(4);
    print(a.len());           // expect: 4
    print(a.pop());           // expect: 4
    print(len(a));            // expect: 3
    var f = [1.5, 2.5];
    print(f[0]);              // expect: 1.5
    print(f[0] + f[1]);       // expect: 4
    var total = 0;
    for x in a {
        total = total + x;
    }
    print(total);             // expect: 6
    a[0] = 11;
    print(a[0]);              // expect: 11
    var empty = [];
    print(empty.len());       // expect: 0
    empty.push(9);
    print(empty.pop());       // expect: 9
    var m = [1, 2, 3];
    m[2] = 10;
    print(m[2]);              // expect: 10
    f[1] = 5;
    print(f[1]);              // expect: 5
    print(m[1]);              // expect: 2
}

