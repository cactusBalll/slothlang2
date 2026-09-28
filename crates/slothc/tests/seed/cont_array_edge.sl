// seed: sloth/array.slt clamping family never panics; empty-array conventions
// return nil / 0 / -1 (book ch12 §12.4)
import "sloth/array.slt";
func main(): unit {
    let a = [1, 2, 3];
    print(array_take(a, -1));        // expect: []
    print(array_take(a, 100));       // expect: [1, 2, 3]
    print(array_drop(a, -5));        // expect: [1, 2, 3]
    print(array_drop(a, 100));       // expect: []
    print(array_slice(a, -3, 2));    // expect: [1, 2]
    print(array_slice(a, 2, 100));   // expect: [3]
    print(array_get_or(a, -1, 9));   // expect: 9
    print(array_get_or(a, 3, 9));    // expect: 9
    print(array_index_of(a, 42));    // expect: -1

    let e: Array<int> = [];
    print(array_is_empty(e));        // expect: true
    print(array_sum_int(e));         // expect: 0
    print(array_reverse(e));         // expect: []
    let mn = array_min(e);
    print(mn is nil);                // expect: true
    print(array_binary_search(e, 1)); // expect: -1

    var b = [1, 2, 1, 3, 1];
    print(array_remove_all(b, 1));   // expect: 3
    print(b);                        // expect: [2, 3]
    array_clear(b);
    print(b.len());                  // expect: 0
}
