// spec: sloth/str.slt — common string processing helpers.
// Byte-oriented primitives follow `len`/`s[i]`/`s[a..b]`; character-oriented
// helpers (`str_clen`, `str_char_at`, `str_substr`, `str_index_of`) count
// Unicode scalar values. Each result is a fresh allocation (no interning).
import "sloth/str.slt";

func main(): unit {
    // ---- length / primitives ----
    print(str_len("héllo"));                 // expect: 6
    print(str_clen("héllo"));                // expect: 5
    print(str_is_empty(""));                 // expect: true
    print(str_is_empty("x"));                // expect: false
    print(str_eq("ab", "a" + "b"));          // expect: true
    print(str_cmp("a", "b"));                // expect: -1
    print(str_byte("A", 0));                 // expect: 65
    print(str_of_byte(66));                  // expect: B
    print(str_slice("abcdef", 2, 3));        // expect: cde
    print(str_find("banana", "na", 0));      // expect: 2

    // ---- search predicates ----
    print(str_starts_with("hello", "he"));   // expect: true
    print(str_ends_with("hello", "lo"));     // expect: true
    print(str_contains("hello", "ell"));     // expect: true
    print(str_count("banana", "an"));        // expect: 2
    print(str_index_of("banana", "na"));     // expect: 2
    print(str_last_index_of("banana", "na")); // expect: 4
    print(str_index_of("abc", "z"));         // expect: -1

    // ---- char slices ----
    print(str_char_at("aé中", 1));           // expect: é
    print("[" + str_char_at("aé中", 9) + "]"); // expect: []
    print(str_substr("abcdef", 2, 3));       // expect: cde
    print(str_substr("aé中", 1, 2));         // expect: é中
    print(str_take("abcdef", 3));            // expect: abc
    print(str_drop("abcdef", 3));            // expect: def

    // ---- trimming / case ----
    print("[" + str_trim("  hi  ") + "]");   // expect: [hi]
    print("[" + str_trim_start("  hi  ") + "]"); // expect: [hi  ]
    print("[" + str_trim_end("  hi  ") + "]");   // expect: [  hi]
    print(str_to_upper("aBc1中"));           // expect: ABC1中
    print(str_to_lower("AbC1中"));           // expect: abc1中

    // ---- split / join ----
    print(str_split("a,b,c", ",").len());    // expect: 3
    print(str_split("a,b,c", ",")[1]);       // expect: b
    print(str_join(str_split("a,b,c", ","), "-")); // expect: a-b-c
    print(str_split("aé", "").len());        // expect: 2
    print(str_split("aé", "")[1]);           // expect: é
    let lines = str_split_lines("a\r\nb\nc");
    print(lines.len());                      // expect: 3
    print(lines[0]);                         // expect: a

    // ---- replace / repeat / reverse ----
    print(str_replace("a-b-c", "-", "+"));   // expect: a+b+c
    print(str_replace("aaa", "aa", "b"));    // expect: ba
    print(str_replace_first("aaa", "a", "b")); // expect: baa
    print(str_repeat("ab", 3));              // expect: ababab
    print(str_reverse("abc中"));             // expect: 中cba

    // ---- padding ----
    print(str_pad_start("7", 3, "0"));       // expect: 007
    print(str_pad_end("7", 3, "0"));         // expect: 700

    // ---- numeric parsing ----
    print(str_to_int("-42"));                // expect: -42
    print(str_to_int("12x"));                // expect: 12
    print(str_to_int_or("abc", 99));         // expect: 99
    print(str_to_float("3.14"));             // expect: 3.14
    print(str_to_float("-2.5"));             // expect: -2.5

    // ---- character classes ----
    print(str_is_digit("5"));                // expect: true
    print(str_is_alpha("a"));                // expect: true
    print(str_is_alnum("7"));                // expect: true
    print(str_is_digits("1234"));            // expect: true
    print(str_is_hex("F"));                  // expect: true

    // ---- ARC: a string-heavy loop must return to the live baseline ----
    // (exercises owned extern string returns, cross-module ref-return
    // transfer, and loop elements abandoned by `return`)
    var base = sloth_rc_live();
    var i = 0;
    while i < 2000 {
        let parts = str_split("a,b,c", ",");
        let j = str_join(parts, "-");
        let t = str_trim("  " + j + "  ");
        let u = str_to_upper(t);
        let v = str_replace(u, "-", ":");
        let w = str_repeat(v, 2);
        let x = str_reverse(w);
        let y = str_substr(x, 1, 3);
        let z = str_pad_start(y, 6, "0");
        let q = str_char_at(z, 2);
        i = i + 1;
    }
    print(sloth_rc_live() - base);           // expect: 0
}
