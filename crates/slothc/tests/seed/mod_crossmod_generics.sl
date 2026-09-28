// seed: cross-module generic functions — unqualified and aliased calls must
// monomorphize in the defining module (private generic helper delegation).
import "mod_crossmod_generics_lib.slt" as G;

func main(): unit {
    print(wrapped(7));                    // expect: [7]
    print(G.wrapped("hi"));               // expect: [hi]
    print(sorted_by([3, 1, 2], |x: int, y: int| -> int { return y - x; }));
    // expect: [3, 2, 1]
    let h = head([9, 8]);
    if h is not nil {
        print(h);                         // expect: 9
    }
    var e: Array<int> = [];
    print(head(e) is nil);                // expect: true
}
