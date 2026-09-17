// ARC stress: strong borrows obtained from Weak<T>.upgrade(). The returned
// T? is an owned +1 (producer) that must be released wherever it is consumed
// — local, container element, map value, field, returned from a function, or
// captured by a closure. Dead-target upgrades yield nil and must be inert.
class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

class Holder {
    var b: Box? = nil;
    var ws: Array<Box> = [];
}

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

// upgrade into a local, read, drop (target alive across the borrow)
func borrow_local(n: int): int {
    var acc = 0;
    var t = Box(n);
    var w: Weak<Box> = t;
    var i = 0;
    while i < n {
        var u = w.upgrade();
        if u is nil {
            acc = acc - 1;
        } else {
            acc = acc + u.v;
        }
        i = i + 1;
    }
    return acc;
}

// dead target: upgrade is nil each round
func borrow_dead(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var w: Weak<Box> = nil;
        {
            let t = Box(i);
            w = t;
        }
        var u = w.upgrade();
        if u is nil { acc = acc + 1; } else { acc = acc + u.v; }
        i = i + 1;
    }
    return acc;
}

// borrow stored into an array/map, then the containers drop
func borrow_containers(n: int): int {
    var acc = 0;
    var t = Box(n);
    var w: Weak<Box> = t;
    var i = 0;
    while i < n {
        var a: Array<Box> = [];
        var m: Map<str, Box> = @();
        var u = w.upgrade();
        if u is not nil {
            a.push(u);
            m["k"] = u;
            acc = acc + a[0].v + m["k"].v;
        }
        i = i + 1;
    }
    return acc;
}

// borrow stored into a field, overwritten each round (overwrite-out release)
func borrow_field(n: int): int {
    var acc = 0;
    var t = Box(n);
    var w: Weak<Box> = t;
    var h = Holder();
    var i = 0;
    while i < n {
        var u = w.upgrade();
        h.b = u;                       // field holds strong ref
        var cur: Box? = h.b;
        if cur is not nil { acc = acc + cur.v; }
        i = i + 1;
    }
    return acc;
}

// borrow returned from a helper (owned on the return face)
func grab(w: Weak<Box>): Box? {
    return w.upgrade();
}
func borrow_return(n: int): int {
    var acc = 0;
    var t = Box(n);
    var w: Weak<Box> = t;
    var i = 0;
    while i < n {
        var u = grab(w);
        if u is nil { acc = acc - 1; } else { acc = acc + u.v; }
        i = i + 1;
    }
    return acc;
}

// closure capturing a weak box, escaping its defining scope
func make_probe(w: Weak<Box>): () -> int {
    return || {
        var u = w.upgrade();
        if u is nil {
            return -1;
        } else {
            return u.v;
        }
    };
}
func borrow_closure(n: int): int {
    var acc = 0;
    var t = Box(n);
    var w: Weak<Box> = t;
    var i = 0;
    while i < n {
        let f = make_probe(w);
        acc = acc + f();
        i = i + 1;
    }
    return acc;
}

// closure capturing a weak box whose target dies before the closure is called
func probe_of(b: Box): () -> int {
    let w: Weak<Box> = b;
    return || {
        var u = w.upgrade();
        if u is nil {
            return -1;
        } else {
            return u.v;
        }
    };
}
func borrow_closure_dead(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var f = probe_of(Box(i));   // the argument temp dies at stmt end
        acc = acc + f();            // upgrade must read nil
        i = i + 1;
    }
    return acc;
}

// upgrade churn where target dies mid-loop
func borrow_mixed(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var w: Weak<Box> = nil;
        var u: Box? = nil;
        {
            let t = Box(i);
            w = t;
            u = w.upgrade();
        }
        if u is nil { acc = acc - 1; } else { acc = acc + 1; }
        var dead = w.upgrade();
        if dead is nil { acc = acc + 1; }
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    let _w = borrow_local(20) + borrow_dead(20) + borrow_containers(20)
        + borrow_field(20) + borrow_return(20) + borrow_closure(20)
        + borrow_closure_dead(20) + borrow_mixed(20);
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    borrow_local(1000);
    now = sloth_rc_live();
    rep("borrow_local", now - before);
    before = now;

    borrow_dead(1000);
    now = sloth_rc_live();
    rep("borrow_dead", now - before);
    before = now;

    borrow_containers(1000);
    now = sloth_rc_live();
    rep("borrow_containers", now - before);
    before = now;

    borrow_field(1000);
    now = sloth_rc_live();
    rep("borrow_field", now - before);
    before = now;

    borrow_return(1000);
    now = sloth_rc_live();
    rep("borrow_return", now - before);
    before = now;

    borrow_closure(1000);
    now = sloth_rc_live();
    rep("borrow_closure", now - before);
    before = now;

    borrow_closure_dead(1000);
    now = sloth_rc_live();
    rep("borrow_closure_dead", now - before);
    before = now;

    borrow_mixed(1000);
    now = sloth_rc_live();
    rep("borrow_mixed", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
