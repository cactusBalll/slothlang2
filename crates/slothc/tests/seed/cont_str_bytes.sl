// seed: str — byte length/indexing/slicing (UTF-8 raw bytes), content `==`,
// Unicode iteration vs chars() codepoints, escapes and interpolation
// (book ch14 §14.1/14.2/14.3)
func main(): unit {
    let s = "héllo";
    print(s.len());              // expect: 6
    print(s[0]);                 // expect: 104
    print(s[1]);                 // expect: 195
    print(s[2]);                 // expect: 169
    print(s[1..3]);              // expect: é
    print(s[1..=3]);             // expect: él
    print(s[1..1].len());        // expect: 0

    // content equality despite no interning / fresh allocation
    let a = "ab";
    let b = "a" + "b";
    print(a == b);               // expect: true
    print(a == "abc");           // expect: false

    // `for c in s` yields single-character str values
    let t = "aé中";
    for c in t { print(len(c)); } // expect: 1
                                  // expect: 2
                                  // expect: 3

    // `chars()` yields lazy codepoints (int)
    let it = t.chars();
    print(it.next());            // expect: 97
    print(it.next());            // expect: 233
    print(it.next());            // expect: 20013
    print(it.next() is nil);     // expect: true

    // escapes and interpolation
    let e = "a\tb\nc";
    print(e.len());              // expect: 5
    print("x=${1 + 2}");         // expect: x=3
    print([1, 2]);               // expect: [1, 2]
    print(@("k": 7));            // expect: {k: 7}
}
