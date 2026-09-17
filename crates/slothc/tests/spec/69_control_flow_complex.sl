// spec: control flow — nested break/continue, early return, while guards
func find(a: Array<int>, t: int): int {
    var i = 0;
    while i < a.len() {
        if a[i] == t { return i; }
        i = i + 1;
    }
    return 0 - 1;
}
func main(): unit {
    var s = 0;
    for i in 0..3 {
        for j in 0..3 {
            if j == 1 { continue; }
            if i == 2 { break; }
            s = s + i * 10 + j;
        }
    }
    print(s);                       // expect: 24
    var k = 0;
    while true {
        k = k + 1;
        if k >= 4 { break; }
    }
    print(k);                       // expect: 4
    let a = [4, 5, 6];
    print(find(a, 6));              // expect: 2
    print(find(a, 9));              // expect: -1
    var odd = 0;
    for x in 0..10 {
        if x % 2 == 0 { continue; }
        odd = odd + x;
    }
    print(odd);                     // expect: 25
}
