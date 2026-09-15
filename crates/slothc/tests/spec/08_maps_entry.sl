// spec: Entry<K,V> map-iteration — live key/val pairs (§3.5, patch #26)
func main(): unit {
    let m = @("a": 1, "b": 2, "c": 3);
    var total = 0;
    var n = 0;
    for (var e: m) {
        total = total + e.val;
        n = n + 1;
    }
    print(total);              // expect: 6
    print(n);                  // expect: 3
    let f = @(1: 2.5, 2: 0.5);
    var fs = 0.0;
    for (var e: f) {
        fs = fs + e.val;
    }
    print(fs);                 // expect: 3
    let hm = @(1: 10, 2: 20);
    var s = 0;
    for (var e: hm) {
        s = s + e.key + e.val;
    }
    print(s);                  // expect: 33
}

