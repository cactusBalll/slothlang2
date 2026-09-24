// spec: sloth/array.slt — common Array<T> operations.
// Clamping helpers never panic; ordering helpers use `<`/`>` for
// int/float/Comparable, dedicated `*_str` variants for byte-lexicographic
// string order, and `*_by` with a three-way comparator for full control.
import "sloth/array.slt";

func is_even(x: int): bool {
    return x % 2 == 0;
}

func main(): unit {
    // ---- queries ----
    let a = [10, 20, 30, 20];
    print(array_is_empty(a));                          // expect: false
    var empty: Array<int> = [];
    print(array_is_empty(empty));                      // expect: true
    print(array_index_of(a, 20));                      // expect: 1
    print(array_last_index_of(a, 20));                 // expect: 3
    print(array_index_of(a, 99));                      // expect: -1
    print(array_contains(a, 30));                      // expect: true
    print(array_contains(a, 40));                      // expect: false
    print(array_count(a, 20));                         // expect: 2
    print(array_count_if(a, is_even));                 // expect: 4
    print(array_any(a, |x: int| -> bool { return x > 25; })); // expect: true
    print(array_all(a, |x: int| -> bool { return x > 5; }));  // expect: true
    print(array_get_or(a, 1, -1));                     // expect: 20
    print(array_get_or(a, 9, -1));                     // expect: -1
    let f = array_first(a);
    if f is not nil {
        print(f);                                      // expect: 10
    }
    let l = array_last(a);
    if l is not nil {
        print(l);                                      // expect: 20
    }
    print(array_first(empty) is nil);                  // expect: true

    // ---- copies / slices (clamping) ----
    print(array_copy(a));                              // expect: [10, 20, 30, 20]
    print(array_take(a, 2));                           // expect: [10, 20]
    print(array_take(a, 99));                          // expect: [10, 20, 30, 20]
    print(array_drop(a, 2));                           // expect: [30, 20]
    print(array_drop(a, -5));                          // expect: [10, 20, 30, 20]
    print(array_slice(a, 1, 2));                       // expect: [20, 30]
    print(array_slice(a, 3, 99));                      // expect: [20]
    print(array_slice(a, 9, 2).len());                 // expect: 0

    // ---- combine ----
    print(array_concat([1, 2], [3, 4]));               // expect: [1, 2, 3, 4]
    var ext = [1, 2];
    array_extend(ext, [3, 4]);
    print(ext);                                        // expect: [1, 2, 3, 4]
    print(array_repeat([1, 2], 3));                    // expect: [1, 2, 1, 2, 1, 2]
    print(array_reverse([1, 2, 3]));                   // expect: [3, 2, 1]
    print(array_flatten([[1], [2, 3], [4]]));          // expect: [1, 2, 3, 4]

    // ---- transform ----
    print(array_map([1, 2, 3], |x: int| -> int { return x * 2; })); // expect: [2, 4, 6]
    print(array_map(["a", "b"], |s: str| -> str { return s + "!"; })); // expect: [a!, b!]
    print(array_map_indexed([10, 20], |i: int, x: int| -> int { return i + x; })); // expect: [10, 21]
    print(array_filter([1, 2, 3, 4], is_even));        // expect: [2, 4]
    print(array_filter_map([1, 2, 3, 4], |x: int| -> int? {
        if x % 2 == 0 { return x * 10; }
        return nil;
    }));                                               // expect: [20, 40]
    print(array_filter_map(["a", "bb", "ccc"], |s: str| -> str? {
        if len(s) > 1 { return s + "!"; }
        return nil;
    }));                                               // expect: [bb!, ccc!]
    print(array_flat_map([1, 3], |x: int| -> Array<int> { return [x, x + 1]; })); // expect: [1, 2, 3, 4]
    print(array_unique([1, 2, 2, 3, 1, 4]));           // expect: [1, 2, 3, 4]
    print(array_unique(["x", "y", "x"]));              // expect: [x, y]

    // ---- aggregation ----
    print(array_fold([1, 2, 3, 4], 0, |acc: int, x: int| -> int { return acc + x; })); // expect: 10
    print(array_fold(["a", "b", "c"], "", |acc: str, s: str| -> str { return acc + s; })); // expect: abc
    print(array_sum_int([1, 2, 3]));                   // expect: 6
    print(array_sum_float([1.5, 2.0, 3.0]));           // expect: 6.5
    print(array_sum_int(empty));                       // expect: 0
    let mn = array_min([3, 1, 2]);
    if mn is not nil {
        print(mn);                                     // expect: 1
    }
    let mx = array_max([3, 1, 2]);
    if mx is not nil {
        print(mx);                                     // expect: 3
    }
    let mxd = array_min_by([3, 1, 2], |x: int, y: int| -> int { return y - x; });
    if mxd is not nil {
        print(mxd);                                    // expect: 3
    }
    let ms = array_min_str(["banana", "apple", "cherry"]);
    if ms is not nil {
        print(ms);                                     // expect: apple
    }
    let xs = array_max_str(["banana", "apple", "cherry"]);
    if xs is not nil {
        print(xs);                                     // expect: cherry
    }
    print(array_min(empty) is nil);                    // expect: true

    // ---- mutation ----
    var ins = [1, 3];
    array_insert(ins, 1, 2);
    print(ins);                                        // expect: [1, 2, 3]
    array_insert(ins, 99, 4);
    print(ins);                                        // expect: [1, 2, 3, 4]
    var rem = [1, 2, 3];
    let removed = array_remove_at(rem, 1);
    print(removed);                                    // expect: 2
    print(rem);                                        // expect: [1, 3]
    var rb = [1, 2, 2, 3];
    print(array_remove(rb, 2));                        // expect: true
    print(rb);                                         // expect: [1, 2, 3]
    print(array_remove(rb, 9));                        // expect: false
    print(array_remove_all(rb, 2));                    // expect: 1
    var ra = [1, 2, 2, 3, 2];
    print(array_remove_all(ra, 2));                    // expect: 3
    print(ra);                                         // expect: [1, 3]
    var cl = [1, 2, 3];
    array_clear(cl);
    print(cl.len());                                   // expect: 0
    var sw = [1, 2, 3];
    array_swap(sw, 0, 2);
    print(sw);                                         // expect: [3, 2, 1]
    array_reverse_in_place(sw);
    print(sw);                                         // expect: [1, 2, 3]

    // ---- ordering ----
    var s1 = [3, 1, 4, 1, 5, 9, 2, 6];
    array_sort(s1);
    print(s1);                                         // expect: [1, 1, 2, 3, 4, 5, 6, 9]
    var s2 = [3, 1, 2];
    array_sort_desc(s2);
    print(s2);                                         // expect: [3, 2, 1]
    var s3 = [3, 1, 2];
    array_sort_by(s3, |x: int, y: int| -> int { return y - x; });
    print(s3);                                         // expect: [3, 2, 1]
    var s4 = ["banana", "apple", "cherry"];
    array_sort_str(s4);
    print(s4);                                         // expect: [apple, banana, cherry]
    var s5 = [2.5, 1.5, 3.5];
    array_sort(s5);
    print(s5);                                         // expect: [1.5, 2.5, 3.5]
    print(array_is_sorted([1, 2, 2, 3]));              // expect: true
    print(array_is_sorted([1, 3, 2]));                 // expect: false
    print(array_binary_search([1, 2, 3, 4, 5], 3));    // expect: 2
    print(array_binary_search([1, 2, 3, 4, 5], 9));    // expect: -1
    print(array_binary_search_by([1, 2, 3, 4, 5], 4, |x: int, y: int| -> int { return x - y; })); // expect: 3
    print(array_binary_search_str(["apple", "banana", "cherry"], "banana")); // expect: 1
    print(array_binary_search_str(["apple", "banana", "cherry"], "zzz"));    // expect: -1

    // ---- equality / generation ----
    print(array_equals([1, 2, 3], [1, 2, 3]));         // expect: true
    print(array_equals([1, 2], [1, 2, 3]));            // expect: false
    print(array_equals(["a", "b"], ["a", "b"]));       // expect: true
    print(array_iota(3));                              // expect: [0, 1, 2]
    print(array_range(2, 5));                          // expect: [2, 3, 4]
    print(array_range(5, 2).len());                    // expect: 0
    print(array_range_step(0, 6, 2));                  // expect: [0, 2, 4]
    print(array_range_step(5, 0, -2));                 // expect: [5, 3, 1]
    print(array_range_step(0, 5, 0).len());            // expect: 0

    // ---- ARC: a container-heavy loop must return to the live baseline ----
    let base = sloth_rc_live();
    var i = 0;
    while i < 2000 {
        var xs2 = [3, 1, 2];
        array_sort(xs2);
        let m = array_map(xs2, |x: int| -> int { return x + 1; });
        let fl = array_filter(m, is_even);
        let un = array_unique([1, 1, 2]);
        let rv = array_reverse(fl);
        let cc = array_concat(rv, un);
        let rm = array_remove_at(cc, 0);
        let ct = array_slice(cc, 1, 2);
        let fm = array_filter_map(cc, |x: int| -> int? {
            if x > 1 { return x; }
            return nil;
        });
        let ss2 = ["c", "a", "b"];
        array_sort_str(ss2);
        let mn2 = array_min(xs2);
        // reference-element mutation must balance ARC too
        var names = ["c", "a", "b", "a"];
        array_sort_str(names);
        array_insert(names, 1, "x");
        let r1 = array_remove(names, "a");
        let r2 = array_remove_all(names, "x");
        let u2 = array_unique(names);
        let c2 = array_concat(names, u2);
        let rv2 = array_reverse(c2);
        i = i + 1;
    }
    print(sloth_rc_live() - base);                     // expect: 0
}
