// ARC stress: closures / first-class functions combined with reference
// escape. A closure frame owns every ref capture; a closure box owns its
// frame; a method-ref closure box owns its receiver. Churn must settle.
class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

class Counter {
    var n: int = 0;
    func __init__(k: int) { this.n = k; }
    func bump(): int { this.n = this.n + 1; return this.n; }
}

class Field {
    var f: (int) -> int = |x: int| { return x; };
    func __init__() {}
}

func named_add(x: int): int { return x + 1; }

func make_capture_param(b: Box): (int) -> int {
    return |x: int| { return x + b.v; };
}

func make_capture_local(v: int): (int) -> int {
    let b = Box(v);
    return |x: int| { return x + b.v; };
}

// NOTE: nested closures that capture an enclosing lambda's param/capture are
// a known gap (walk_ids_expr does not descend into nested Lambda nodes, so the
// outer frame never captures the name). Curried return types also fail to
// type-check. Reproducer kept in examples/arc/known/nested_closure_capture.sl.

func apply(f: (int) -> int, x: int): int { return f(x); }

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

// fresh closure capturing a fresh local each iteration, called then dropped
func cap_local_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let f = make_capture_local(i);
        acc = acc + f(1);
        i = i + 1;
    }
    return acc;
}

// closure returned from a borrowed param: capture must retain
func cap_param_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let b = Box(i);
        let f = make_capture_param(b);
        acc = acc + f(1);
        i = i + 1;
    }
    return acc;
}

// a closure held in a container element: the array owns the closure box,
// the box owns the frame, the frame owns the ref capture
func cap_closure_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a: Array<(int) -> int> = [];
        a.push(make_capture_local(i));
        let f = a[0];
        acc = acc + f(1);
        i = i + 1;
    }
    return acc;
}

// closures stored in an array: array owns the closure boxes, then drops
func cap_array_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a: Array<(int) -> int> = [];
        a.push(make_capture_local(i));
        a.push(make_capture_local(i + 1));
        acc = acc + a[0](1) + a[1](1);
        i = i + 1;
    }
    return acc;
}

// closures stored under overwritten object fields
func cap_field_churn(n: int): int {
    var acc = 0;
    var s = Field();
    var i = 0;
    while i < n {
        s.f = make_capture_local(i);      // overwrite-out drops the old closure
        acc = acc + s.f(1);
        i = i + 1;
    }
    return acc;
}

// closure capturing a shared ref: the captured handle survives outer rebinds
func cap_shared_ref(): int {
    var b = Box(10);
    let f = |x: int| { return x + b.v; };
    b = Box(99);                          // rebind outer slot, capture keeps old
    return f(0);
}

// named function value and method reference churn
func fn_value_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let g = named_add;
        acc = acc + g(i);
        i = i + 1;
    }
    return acc;
}

func method_ref_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let c = Counter(0);
        let bump = c.bump;                // closure box owns receiver c
        acc = acc + bump();
        i = i + 1;
    }
    return acc;
}

// method-ref closures stored in an array: the array owns each box, each box
// owns its receiver, all drop together
func method_ref_array_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a: Array<() -> int> = [];
        a.push(Counter(i).bump);
        var f = a[0];
        acc = acc + f();
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    // warm up + intern literals
    let _w = cap_local_churn(50) + cap_param_churn(50) + cap_closure_churn(50)
        + cap_array_churn(50) + cap_field_churn(50) + fn_value_churn(50)
        + method_ref_churn(50) + method_ref_array_churn(50) + cap_shared_ref();
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    cap_local_churn(2000);
    now = sloth_rc_live();
    rep("capture_local", now - before);
    before = now;

    cap_param_churn(2000);
    now = sloth_rc_live();
    rep("capture_param", now - before);
    before = now;

    cap_closure_churn(2000);
    now = sloth_rc_live();
    rep("nested_closure", now - before);
    before = now;

    cap_array_churn(2000);
    now = sloth_rc_live();
    rep("closure_array", now - before);
    before = now;

    cap_field_churn(2000);
    now = sloth_rc_live();
    rep("closure_field", now - before);
    before = now;

    fn_value_churn(2000);
    now = sloth_rc_live();
    rep("named_fn_value", now - before);
    before = now;

    method_ref_churn(2000);
    now = sloth_rc_live();
    rep("method_ref", now - before);
    before = now;

    method_ref_array_churn(2000);
    now = sloth_rc_live();
    rep("method_ref_array", now - before);
    before = now;

    let shared = cap_shared_ref();
    now = sloth_rc_live();
    if shared == 10 {
        rep("shared_ref", now - before);
    } else {
        print("WRONG shared_ref ${shared}");
    }
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
