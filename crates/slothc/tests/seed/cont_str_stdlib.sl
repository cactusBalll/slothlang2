// seed: sloth/str.slt — length/search/slice/trim/case/split/join/replace/
// pad/parse surface (book ch14 §14.4)
import "sloth/str.slt";
func main(): unit {
    print(str_len("aé中"));                 // expect: 6
    print(str_clen("aé中"));                // expect: 3
    print(str_is_empty(""));                // expect: true
    print(str_eq("a", "a"));                // expect: true
    print(str_cmp("a", "b") < 0);           // expect: true
    print(str_byte("ABC", 1));              // expect: 66
    print(str_of_byte(65));                 // expect: A
    print(str_slice("hello", 1, 3));        // expect: ell
    print(str_find("hello", "l", 3));       // expect: 3
    print(str_starts_with("hello", "he"));  // expect: true
    print(str_ends_with("hello", "lo"));    // expect: true
    print(str_contains("hello", "ell"));    // expect: true
    print(str_count("banana", "an"));       // expect: 2
    print(str_index_of("aé中é", "é"));      // expect: 1
    print(str_last_index_of("aé中é", "é")); // expect: 3
    print(str_char_at("aé中", 2));          // expect: 中
    print(str_substr("aé中", 1, 2));        // expect: é中
    print(str_take("aé中", 2));             // expect: aé
    print(str_drop("aé中", 2));             // expect: 中
    print(str_trim("  hi  "));              // expect: hi
    print(str_to_upper("héllo"));           // expect: HéLLO
    print(str_to_lower("HÉLLO"));           // expect: hÉllo
    print(str_is_alpha("a"));               // expect: true
    print(str_is_digit("5"));               // expect: true
    print(str_is_alnum("5"));               // expect: true
    print(str_is_upper("A"));               // expect: true
    print(str_is_lower("a"));               // expect: true
    print(str_is_hex("F"));                 // expect: true
    print(str_is_digits("123"));            // expect: true
    print(str_split("a,b,,c", ","));        // expect: [a, b, , c]
    print(str_split_lines("a\r\nb\nc"));    // expect: [a, b, c]
    print(str_join(["a", "b", "c"], "-"));  // expect: a-b-c
    print(str_replace("aaa", "a", "b"));    // expect: bbb
    print(str_replace_first("aaa", "a", "b")); // expect: baa
    print(str_repeat("ab", 3));             // expect: ababab
    print(str_reverse("aé中"));             // expect: 中éa
    print(str_pad_start("7", 3, "0"));      // expect: 007
    print(str_pad_end("7", 3, "0"));        // expect: 700
    print(str_to_int("-42x"));              // expect: -42
    print(str_to_int_or("nope", -1));       // expect: -1
    print(str_to_float("3.25abc"));         // expect: 3.25
}
