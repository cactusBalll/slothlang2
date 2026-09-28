// seed: sloth/array.slt — query/copy/combine/transform/aggregate/mutate/sort
// surface (book ch12 §12.4)
import "sloth/array.slt";
func main(): unit {
    let a = [3, 1, 4, 1, 5, 9, 2, 6];
    print(array_is_empty(a));                                        // expect: false
    print(array_index_of(a, 1));                                     // expect: 1
    print(array_last_index_of(a, 1));                                // expect: 3
    print(array_contains(a, 9));                                     // expect: true
    print(array_count(a, 1));                                        // expect: 2
    print(array_count_if(a, |x: int| -> bool { return x > 3; }));    // expect: 4
    print(array_any(a, |x: int| -> bool { return x == 5; }));        // expect: true
    print(array_all(a, |x: int| -> bool { return x > 0; }));         // expect: true
    let f = array_first(a);
    if f is not nil { print(f); }                                    // expect: 3
    let l = array_last(a);
    if l is not nil { print(l); }                                    // expect: 6
    print(array_get_or(a, 0, -1));                                   // expect: 3
    print(array_get_or(a, 100, -1));                                 // expect: -1
    print(array_copy(a));                                            // expect: [3, 1, 4, 1, 5, 9, 2, 6]
    print(array_take(a, 2));                                         // expect: [3, 1]
    print(array_drop(a, 6));                                         // expect: [2, 6]
    print(array_slice(a, 2, 3));                                     // expect: [4, 1, 5]
    print(array_concat([1, 2], [3]));                                // expect: [1, 2, 3]
    print(array_repeat([1, 2], 2));                                  // expect: [1, 2, 1, 2]
    print(array_reverse(a));                                         // expect: [6, 2, 9, 5, 1, 4, 1, 3]
    print(array_flatten([[1], [2, 3]]));                             // expect: [1, 2, 3]
    print(array_map([1, 2, 3], |x: int| -> int { return x * x; }));   // expect: [1, 4, 9]
    print(array_filter([1, 2, 3, 4], |x: int| -> bool { return x % 2 == 0; })); // expect: [2, 4]
    print(array_unique([1, 1, 2, 3, 3]));                            // expect: [1, 2, 3]
    print(array_fold([1, 2, 3], 0, |s: int, x: int| -> int { return s + x; })); // expect: 6
    print(array_sum_int([1, 2, 3]));                                 // expect: 6
    print(array_sum_float([1.5, 2.5]));                              // expect: 4
    let mn = array_min(a);
    if mn is not nil { print(mn); }                                  // expect: 1
    let mx = array_max(a);
    if mx is not nil { print(mx); }                                  // expect: 9
    var b = [3, 1, 2];
    array_sort(b);
    print(b);                                                        // expect: [1, 2, 3]
    print(array_binary_search(b, 2));                                // expect: 1
    print(array_binary_search(b, 7));                                // expect: -1
    print(array_is_sorted(b));                                       // expect: true
    var names = ["banana", "apple"];
    array_sort_str(names);
    print(names);                                                    // expect: [apple, banana]
    print(array_binary_search_str(names, "banana"));                 // expect: 1
    print(array_iota(3));                                            // expect: [0, 1, 2]
    print(array_range(2, 5));                                        // expect: [2, 3, 4]
    print(array_range_step(0, 6, 2));                                // expect: [0, 2, 4]
    print(array_equals([1, 2], [1, 2]));                             // expect: true
    var m = [1, 2, 3, 4, 5];
    array_insert(m, 1, 9);
    print(m);                                                        // expect: [1, 9, 2, 3, 4, 5]
    print(array_remove_at(m, 0));                                    // expect: 1
    print(m);                                                        // expect: [9, 2, 3, 4, 5]
    print(array_remove(m, 2));                                       // expect: true
    print(m);                                                        // expect: [9, 3, 4, 5]
    var r = [1, 2, 1, 3, 1];
    print(array_remove_all(r, 1));                                   // expect: 3
    print(r);                                                        // expect: [2, 3]
    array_reverse_in_place(m);
    print(m);                                                        // expect: [5, 4, 3, 9]
    array_swap(m, 0, 1);
    print(m);                                                        // expect: [4, 5, 3, 9]
}
