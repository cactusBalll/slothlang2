// spec: str byte subscript / byte ranges + lazy chars() code-point iterator
// `s[i]` = raw byte (int); `s[a..b]` / `s[a..=b]` = byte slice (fresh str);
// `s.chars()` = lazy iterator yielding each UTF-8 code point as an int.
// NOTE: Array has no range slice — only `str` supports `s[a..b]`.
func main(): unit {
    var s = "héllo";
    print(s[0]);            // expect: 104
    print(s[1]);            // expect: 195
    print(s[2]);            // expect: 169
    print(len(s));          // expect: 6
    print(s[1..3]);         // expect: é
    print(s[1..=3]);        // expect: él
    print(s[1..1] == "");   // expect: true

    var sum = 0;
    for c in "aé中".chars() { sum = sum + c; }
    print(sum);             // expect: 20343
    for c in "aé中".chars() { print(c); }  // expect: 97
    // expect: 233
    // expect: 20013

    var it = "ab".chars();
    print(it.next());       // expect: 97
    print(it.next());       // expect: 98
    print(it.next() is nil); // expect: true
}
