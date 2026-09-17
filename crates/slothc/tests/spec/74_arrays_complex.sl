// spec: arrays — growth, push/pop, float promotion, aliasing, strings
func main(): unit {
    var a = [];
    var i = 0;
    while i < 100 { a.push(i); i = i + 1; }
    print(a.len());           // expect: 100
    print(a[0] + a[99]);      // expect: 99
    print(a.pop());           // expect: 99
    print(a.len());           // expect: 99
    var f = [1.5, 2.5];
    f.push(3.0);
    print(f[2]);              // expect: 3
    print(f[0] + f[1]);       // expect: 4
    var s = ["x", "y", "z"];
    print(s[1] + s[2]);       // expect: yz
    a[0] = 100;
    print(a[0]);              // expect: 100
    var e: Array<int> = [];
    print(e.len());           // expect: 0
    var copy = a;
    copy[1] = 7;
    print(a[1]);              // expect: 7
}
