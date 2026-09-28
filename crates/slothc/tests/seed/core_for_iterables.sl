// seed: core for-iteration protocol — str by char, Array by element, Map by
// Entry<K,V> with live key/val fields (book §8.2, §3.5)
func main(): unit {
    for c in "abc" { print(c); }     // expect: a
                                     // expect: b
                                     // expect: c
    let arr = [10, 20, 30];
    var s = 0;
    for v in arr { s = s + v; }
    print(s);                        // expect: 60
    let m = @("a": 1, "b": 2, "c": 3);
    var total = 0;
    var n = 0;
    for (var e: m) {
        total = total + e.val;
        n = n + 1;
    }
    print(total);                    // expect: 6
    print(n);                        // expect: 3
    // early return out of a char loop (owned per-iteration elements)
    print(first_big("abCd"));        // expect: C
    // break / continue inside a string loop
    var kept = "";
    for c in "abcdef" {
        if c == "c" { continue; }
        if c == "e" { break; }
        kept = kept + c;
    }
    print(kept);                     // expect: abd
}
func first_big(s: str): str {
    for c in s {
        if c == "C" { return c; }
    }
    return "?";
}
