// KNOWN GAP: a closure nested inside another closure cannot capture an
// enclosing lambda's parameter/capture. `walk_ids_expr` does not descend into
// nested Lambda nodes, so the outer frame never records `b` as a capture and
// the inner lambda reports "lambda captures unknown `b`". Curried return types
// (`(int) -> (int) -> int`) additionally fail the return type check.
//
// Actual: `slothc check` fails with:
//   lambda captures unknown `b`
//   type mismatch in return value (expected fn, got fn)
class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

func make_nested(v: int): (int) -> (int) -> int {
    let b = Box(v);
    return |x: int| {
        return |y: int| { return x + y + b.v; };
    };
}

func main(): unit {
    let outer = make_nested(100);
    let inner = outer(1);
    print(inner(2));
}
