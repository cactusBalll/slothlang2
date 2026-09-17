// ARC stress: unbound reference temporaries (§5.1.1 rule 5/7). Every call
// result consumed inline — field chains, indexing, len, arguments, virtual
// dispatch, elvis, short-circuit, condition/iterable positions — must be
// released at statement end even across mid-expression CFG splits.
trait Speaker {
    func noise(): str;
}
class Cat impl Speaker {
    var tag: str;
    func __init__(tag: str) { this.tag = tag; }
    func noise(): str { return "m:" + this.tag; }
}

class Box {
    var v: int;
    var s: str;
    var inner: Box? = nil;
    func __init__(v: int) { this.v = v; this.s = "b${v}"; }
}

class Holder {
    var b: Box;
    var a: Array<int>;
    var m: Map<str, int>;
    func __init__(v: int) {
        this.b = Box(v);
        this.a = [v, v + 1];
        this.m = @("k": v);
    }
}

class Counter {
    var n: int = 0;
    var limit: int = 0;
    func __init__(limit: int) { this.limit = limit; }
    func iter(): Counter { return this; }
    func next(): int? {
        if this.n >= this.limit { return nil; }
        this.n = this.n + 1;
        return this.n;
    }
}

class Chain {
    var v: int;
    var s: str;
    func __init__(v: int) { this.v = v; this.s = "c${v}"; }
    func clone(): Chain { return this; }
    func name(): str { return this.s; }
    func bump(): int { this.v = this.v + 1; return this.v; }
}

func mkstr(i: int): str { return "s${i}"; }
func mkarr(i: int): Array<int> { return [i, i + 1, i + 2]; }
func mkmap(i: int): Map<str, int> { return @("k": i); }
func mkbox(i: int): Box { return Box(i); }
func mkholder(i: int): Holder { return Holder(i); }
func mkdyn(i: int): dyn Speaker { return Cat("${i}"); }
func mkopt(i: int): Box? { if i < 0 { return nil; } else { return Box(i); } }
func mkcounter(i: int): Counter { return Counter(i); }
func consume_str(s: str): int { return s.len(); }
func consume_box(b: Box): int { return b.v; }
func consume_arr(a: Array<int>): int { return a.len(); }

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

func temp_basic(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        acc = acc + mkbox(i).v + mkstr(i).len() + mkarr(i).len()
            + mkmap(i)["k"] + mkbox(i).s.len();
        i = i + 1;
    }
    return acc;
}

func temp_field_chain(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        acc = acc + mkholder(i).b.v + mkholder(i).a[0] + mkholder(i).m["k"];
        i = i + 1;
    }
    return acc;
}

func temp_args(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        acc = acc + consume_str(mkstr(i)) + consume_box(mkbox(i))
            + consume_arr(mkarr(i));
        i = i + 1;
    }
    return acc;
}

func temp_dyn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        acc = acc + mkdyn(i).noise().len();
        i = i + 1;
    }
    return acc;
}

func temp_elvis(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var o = mkopt(i);
        if o is nil {
            acc = acc - 1;
        } else {
            acc = acc + o.v;
        }
        i = i + 1;
    }
    return acc;
}

// short-circuit: the rhs ref temporary must settle on both taken/skipped paths
func temp_short_circuit(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        if i > 0 and mkstr(i).len() > 1 {
            acc = acc + 1;
        }
        if i < 0 or mkbox(i).v >= 0 {
            acc = acc + 1;
        }
        i = i + 1;
    }
    return acc;
}

// temporaries in condition position (including loops)
func temp_conditions(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        if mkopt(i) is not nil { acc = acc + 1; }
        i = i + 1;
    }
    return acc;
}

// ref producers as the iterable of a for-in
func temp_iterable(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        for x in mkarr(i) { acc = acc + x; }
        for c in mkstr(i) { acc = acc + c.len(); }
        for (var e: mkmap(i)) { acc = acc + e.val; }
        for b in mkcounter(5) { acc = acc + b; }
        i = i + 1;
    }
    return acc;
}

// nested temporaries: a temporary whose field is itself a call result
func temp_nested(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        acc = acc + mkholder(i).b.v + Box(mkbox(i).v).v;
        i = i + 1;
    }
    return acc;
}

func mkchain(i: int): Chain { return Chain(i); }

// method calls on a temporary receiver: `this`-returning methods and ref
// results must transfer correctly through the receiver temp
func temp_method(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        acc = acc + mkchain(i).clone().v;
        acc = acc + mkchain(i).name().len();
        acc = acc + mkchain(i).bump();
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    let _w = temp_basic(20) + temp_field_chain(20) + temp_args(20)
        + temp_dyn(20) + temp_elvis(20) + temp_short_circuit(20)
        + temp_conditions(20) + temp_iterable(20) + temp_nested(20)
        + temp_method(20);
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    temp_basic(1000);
    now = sloth_rc_live();
    rep("temp_basic", now - before);
    before = now;

    temp_field_chain(1000);
    now = sloth_rc_live();
    rep("temp_field_chain", now - before);
    before = now;

    temp_args(1000);
    now = sloth_rc_live();
    rep("temp_args", now - before);
    before = now;

    temp_dyn(1000);
    now = sloth_rc_live();
    rep("temp_dyn", now - before);
    before = now;

    temp_elvis(1000);
    now = sloth_rc_live();
    rep("temp_elvis", now - before);
    before = now;

    temp_short_circuit(1000);
    now = sloth_rc_live();
    rep("temp_short_circuit", now - before);
    before = now;

    temp_conditions(1000);
    now = sloth_rc_live();
    rep("temp_conditions", now - before);
    before = now;

    temp_iterable(500);
    now = sloth_rc_live();
    rep("temp_iterable", now - before);
    before = now;

    temp_nested(1000);
    now = sloth_rc_live();
    rep("temp_nested", now - before);
    before = now;

    temp_method(1000);
    now = sloth_rc_live();
    rep("temp_method", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
