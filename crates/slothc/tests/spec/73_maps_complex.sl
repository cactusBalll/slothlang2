// spec: maps — overwrite, negation keys, dynamic content keys, iteration
func main(): unit {
    var m = @(1: 10, 2: 20, 3: 30);
    m[2] = 99;
    print(m[2]);              // expect: 99
    print(len(m));            // expect: 3
    print(m[1] + m[3]);       // expect: 40
    var s: Map<str, int> = @("a": 1);
    var k = "b";
    s[k] = 2;
    print(s["b"]);            // expect: 2
    print(len(s));            // expect: 2
    var neg = @(0 - 1: 5, 0 - 2: 6);
    print(neg[0 - 1]);        // expect: 5
    print(neg[0 - 2]);        // expect: 6
    var total = 0;
    for e in m { total = total + e.val; }
    print(total);             // expect: 139
    var ks = keys(s);
    print(ks.len());          // expect: 2
}
