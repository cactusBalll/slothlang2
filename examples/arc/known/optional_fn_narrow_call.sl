// KNOWN GAP: an optional function-typed value cannot be called after a nil
// check. The narrowed local is not recognized as callable, so `ff(3)` reports
// "call to unknown `ff`". (A non-optional fn field/local does call correctly.)
//
// Actual: `slothc check` fails with `call to unknown `ff``.
class Field {
    var f: (int) -> int?;
    func __init__(g: (int) -> int) { this.f = g; }
}

func main(): unit {
    var s = Field(|x: int| { return x + 5; });
    var ff = s.f;
    if ff is not nil {
        print(ff(3));
    }
}
