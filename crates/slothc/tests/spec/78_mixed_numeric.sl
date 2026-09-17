// spec: explicit numeric conversion — int()/float() casts, no implicit
// int/float promotion (design §2.1 "无隐式数值转换")
func main(): unit {
    print(float(3) + 0.5);     // expect: 3.5
    print(2.0 * float(3));     // expect: 6
    print(6.0 / float(2));     // expect: 3
    print(int(2.9));           // expect: 2
    print(int(0.0 - 2.9));     // expect: -2
    print(int(3.9) + int(2.0));// expect: 5
    print(float(1) < 2.5);     // expect: true
    print(3.0 == float(3));    // expect: true
    var a = [1, 2, 3];
    print(a[1]);               // expect: 2
    var fs = [1.0, 2.5, 3.0];
    print(fs[1]);              // expect: 2.5
}
