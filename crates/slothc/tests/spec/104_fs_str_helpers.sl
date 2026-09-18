// spec: TE-P4 low-level string faces used by the tokenizer (sloth/fs.slt)
import "sloth/fs.slt";

func main(): unit {
    print(cmp_str("abc", "abd") < 0);  // expect: true
    print(cmp_str("abc", "abc"));      // expect: 0
    print(cmp_str("b", "a") > 0);      // expect: true
    print(byte_at("abc", 0));          // expect: 97
    print(str_slice("abcdef", 2, 3));  // expect: cde
    write(byte_str(65));
    write(byte_str(66));
    print("");                         // expect: AB
    print(len(byte_str(0)));           // expect: 1
}
