// ARC stress: variable escape paths (§5.1.1 rule 6 transfer).
// Every scenario must return sloth_rc_live() to the exact baseline.
class Box {
    var v: int;
    var tag: str;
    func __init__(v: int) { this.v = v; this.tag = "b${v}"; }
}

class Holder {
    var a: Array<Box>;
    var m: Map<str, Box>;
    var b: Box;
    func __init__(v: int) {
        this.a = [Box(v)];
        this.m = @("k": Box(v + 1));
        this.b = Box(v + 2);
    }
}

var gbox: Box? = nil;
var garr: Array<Box> = [];

func make_box(v: int): Box { return Box(v); }
func make_holder(v: int): Holder { return Holder(v); }
func make_arr(v: int): Array<Box> { return [Box(v), Box(v + 1)]; }
func pass_box(b: Box): Box { return b; }
func pass_arr(a: Array<Box>): Array<Box> { return a; }

// NOTE: rc_live must be sampled in the caller. Measuring inside a helper
// includes the owned +1 of the name argument's literal temporary, which
// would read as a spurious one-count "leak" during the call.
func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

// returned local owned by caller; unbound temporary released at stmt end
func esc_return(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let b = make_box(i);          // bound: slot owner
        acc = acc + b.v + make_box(i).v; // temp: released
        i = i + 1;
    }
    return acc;
}

// local escaping into a freshly built container that is returned
func esc_into_container(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let h = make_holder(i);
        acc = acc + h.a[0].v + h.m["k"].v + h.b.v;
        i = i + 1;
    }
    return acc;
}

// alias: two slots share one object; dropping one must not free the other
func esc_alias(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let b = Box(i);
        let c = b;                    // alias: retain into c
        acc = acc + c.v;
        i = i + 1;
    }
    return acc;
}

// escape via a param that is returned back (borrowed in, owned out)
func esc_param_roundtrip(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let b = Box(i);
        let c = pass_box(b);
        acc = acc + c.v;
        i = i + 1;
    }
    return acc;
}

// escape through globals; overwriting the global must release the old value
func esc_global(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        gbox = Box(i);
        var gb: Box? = gbox;
        if gb is not nil {
            acc = acc + gb.v;
        }
        i = i + 1;
    }
    return acc;
}

// escape into a global array pushed in a loop, then dropped
func esc_global_arr(n: int): int {
    var la = garr;
    var i = 0;
    while i < n {
        la.push(Box(i));
        i = i + 1;
    }
    let L = la.len();
    while la.len() > 0 {
        let b = la.pop();
        i = i + b.v;
    }
    return L + i;
}

// nested containers returned through two frames
func esc_nested(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let aa: Array<Array<Box>> = [make_arr(i), make_arr(i + 1)];
        acc = acc + aa[0][0].v + aa[1][1].v;
        i = i + 1;
    }
    return acc;
}

// field overwrite: instance owns a ref field, rebound each iteration
class Slot {
    var b: Box? = nil;
}
func esc_field_overwrite(n: int): int {
    var acc = 0;
    var s = Slot();
    var i = 0;
    while i < n {
        s.b = Box(i);                 // overwrite-out must drop old
        var sb: Box? = s.b;
        if sb is not nil { acc = acc + sb.v; }
        i = i + 1;
    }
    return acc;
}

// array element overwrite of ref elements
func esc_elem_overwrite(n: int): int {
    var acc = 0;
    var a: Array<Box> = [Box(0), Box(1), Box(2)];
    var i = 0;
    while i < n {
        a[i % 3] = Box(i);            // release old elem, retain new
        acc = acc + a[i % 3].v;
        i = i + 1;
    }
    return acc;
}

// map value overwrite of ref values under a stable key
func esc_map_overwrite(n: int): int {
    var acc = 0;
    var m: Map<str, Box> = @();
    var i = 0;
    while i < n {
        m["k"] = Box(i);              // evicted old pair must be released
        acc = acc + m["k"].v;
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    // warm up every path (also interns the literals used below)
    let _w = esc_return(50) + esc_into_container(50) + esc_alias(50)
        + esc_param_roundtrip(50) + esc_nested(50) + esc_field_overwrite(50)
        + esc_elem_overwrite(50) + esc_map_overwrite(50) + esc_global(50)
        + esc_global_arr(5);
    gbox = nil;
    while garr.len() > 0 { let _d = garr.pop(); }
    let init = sloth_rc_live();

    var before = init;
    esc_return(2000);
    var now = sloth_rc_live();
    rep("return", now - before);
    before = now;

    esc_into_container(2000);
    now = sloth_rc_live();
    rep("into_container", now - before);
    before = now;

    esc_alias(2000);
    now = sloth_rc_live();
    rep("alias", now - before);
    before = now;

    esc_param_roundtrip(2000);
    now = sloth_rc_live();
    rep("param_roundtrip", now - before);
    before = now;

    esc_nested(2000);
    now = sloth_rc_live();
    rep("nested", now - before);
    before = now;

    esc_field_overwrite(2000);
    now = sloth_rc_live();
    rep("field_overwrite", now - before);
    before = now;

    esc_elem_overwrite(2000);
    now = sloth_rc_live();
    rep("elem_overwrite", now - before);
    before = now;

    esc_map_overwrite(2000);
    now = sloth_rc_live();
    rep("map_overwrite", now - before);
    before = now;

    // a live Box owns its `tag` str: the expected steady state is +2 handles
    esc_global(2000);
    now = sloth_rc_live();
    rep("global_assign(+2 live)", now - before - 2);
    gbox = nil;
    now = sloth_rc_live();
    rep("global_clear", now - before);
    before = now;

    esc_global_arr(2000);
    now = sloth_rc_live();
    rep("global_arr", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
