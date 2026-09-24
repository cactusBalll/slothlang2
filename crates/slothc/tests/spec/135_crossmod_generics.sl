// spec: unqualified cross-module calls to imported generic functions —
// `Array<T>`/`T?` returns and private-helper delegation must monomorphize in
// the defining module (not call a type-param template).
import "m135.mod.sl";

func main(): unit {
    print(wrapped(7));                         // expect: [7]
    print(wrapped("hi"));                      // expect: [hi]
    print(sorted([3, 1, 2]));                  // expect: [1, 2, 3]
    print(sorted([2.5, 1.5, 3.5]));            // expect: [1.5, 2.5, 3.5]
    print(sorted_by([3, 1, 2], |x: int, y: int| -> int { return y - x; })); // expect: [3, 2, 1]
    let h = head([9, 8]);
    if h is not nil {
        print(h);                              // expect: 9
    }
    var e: Array<int> = [];
    print(head(e) is nil);                     // expect: true

    // ARC: repeated cross-module generic calls must balance the live count
    let base = sloth_rc_live();
    var i = 0;
    while i < 2000 {
        let a = wrapped("x");
        let b = sorted([2, 1]);
        let c = sorted_by([2, 1], |x: int, y: int| -> int { return x - y; });
        let d = head(a);
        i = i + 1;
    }
    print(sloth_rc_live() - base);             // expect: 0
}
