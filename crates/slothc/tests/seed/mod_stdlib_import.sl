// seed: stdlib `.slt` import via the D5 search path (array + str helpers).
import "sloth/array.slt";
import "sloth/str.slt";

func main(): unit {
    var a = [1, 2, 3];
    print(array_sum_int(a));            // expect: 6
    print(array_contains(a, 2));        // expect: true
    print(array_reverse(a));            // expect: [3, 2, 1]
    print(str_to_upper("abc"));         // expect: ABC
    print(str_contains("hello", "ell"));// expect: true
    print(str_trim("  hi  "));          // expect: hi
}
