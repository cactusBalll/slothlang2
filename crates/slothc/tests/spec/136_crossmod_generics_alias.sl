// spec: aliased cross-module calls to imported generic functions — the
// module's own name is registered alongside the alias so private generic
// helpers (e.g. the recursive quicksort) resolve during monomorphization.
import "m135.mod.sl" as M;

func main(): unit {
    print(M.wrapped(7));                       // expect: [7]
    print(M.sorted([3, 1, 2]));                // expect: [1, 2, 3]
    print(M.sorted_by([3, 1, 2], |x: int, y: int| -> int { return y - x; })); // expect: [3, 2, 1]
    let h = M.head([9, 8]);
    if h is not nil {
        print(h);                              // expect: 9
    }
    print(sloth_rc_live() >= 0);               // expect: true
}
