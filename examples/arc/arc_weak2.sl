// ARC stress: weak-box chain interleaving. Weak boxes are intrusive nodes in
// the target's header chain; dropping boxes before/after the target, dropping
// the target with live/dead boxes, and mixed weak targets (class/array/str)
// must all leave the rc baseline unchanged with no dangling chain.
class Node {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

// many boxes to one target; drop a prefix of boxes, then the target dies,
// then the surviving boxes are chained off a freed target (must be inert)
func chain_interleave(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var ws: Array<Weak<Node>> = [];
        {
            let t = Node(i);
            var j = 0;
            while j < 8 {
                ws.push(t);
                j = j + 1;
            }
        }
        // target dead here; drop half the boxes (detach from its chain)
        var d = 0;
        while d < 4 {
            var _drop = ws.pop();
            d = d + 1;
        }
        var k = 0;
        while k < ws.len() {
            var u = ws[k].upgrade();
            if u is nil { acc = acc + 1; }
            k = k + 1;
        }
        i = i + 1;
    }
    return acc;
}

// boxes dropped only after the target: dead-target upgrades then detach
func boxes_outlive_target(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var w1: Weak<Node> = nil;
        var w2: Weak<Node> = nil;
        {
            let t = Node(i);
            w1 = t;
            w2 = t;
        }
        var a = w1.upgrade();
        var b = w2.upgrade();
        if a is nil { acc = acc + 1; }
        if b is nil { acc = acc + 1; }
        i = i + 1;
    }
    return acc;
}

// mixed weak targets: class, array, string, and a value box
func mixed_targets(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var wn: Weak<Node> = nil;
        var wa: Weak<Array<int>> = nil;
        var wsv: Weak<str> = nil;
        var wi: Weak<int> = nil;
        {
            let t = Node(i);
            let a = [i, i + 1];
            let s = "s${i}";
            let x = i;
            wn = t;
            wa = a;
            wsv = s;
            wi = x;
        }
        var un = wn.upgrade();
        var ua = wa.upgrade();
        var us = wsv.upgrade();
        var ui = wi.upgrade();
        if un is nil { acc = acc + 1; }
        if ua is nil { acc = acc + 1; }
        if us is nil { acc = acc + 1; }
        if ui is nil { acc = acc + 1; }
        i = i + 1;
    }
    return acc;
}

// store-face coercion: a bare strong target pushed / listed / mapped into a
// Weak element must be wrapped in a weak box (not written as a raw handle)
func weak_store_faces(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var pushed: Array<Weak<Node>> = [];
        var listed: Array<Weak<Node>> = [];
        var mapped: Map<str, Weak<Node>> = @();
        {
            let t = Node(i);
            pushed.push(t);
            listed = [t, t];
            mapped["k"] = t;
            var u1 = pushed[0].upgrade();
            var u2 = listed[0].upgrade();
            var u3 = mapped["k"].upgrade();
            if u1 is nil { acc = acc - 1; } else { acc = acc + u1.v; }
            if u2 is nil { acc = acc - 1; } else { acc = acc + u2.v; }
            if u3 is nil { acc = acc - 1; } else { acc = acc + u3.v; }
        }
        // target dead: every box must read as nil
        var d1 = pushed[0].upgrade();
        var d2 = listed[0].upgrade();
        var d3 = mapped["k"].upgrade();
        if d1 is nil { acc = acc + 1; }
        if d2 is nil { acc = acc + 1; }
        if d3 is nil { acc = acc + 1; }
        i = i + 1;
    }
    return acc;
}

// weak boxes forming a ring reference (weak back-edges break the cycle)
func weak_field_ring(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a = Node(i);
        var b = Node(i + 1);
        var wa: Weak<Node> = b;
        var wb: Weak<Node> = a;
        var ua = wa.upgrade();
        var ub = wb.upgrade();
        if ua is nil { acc = acc + 1; } else { acc = acc + ua.v; }
        if ub is nil { acc = acc + 1; } else { acc = acc + ub.v; }
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    let _w = chain_interleave(20) + boxes_outlive_target(20)
        + mixed_targets(20) + weak_store_faces(20) + weak_field_ring(5);
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    chain_interleave(1000);
    now = sloth_rc_live();
    rep("chain_interleave", now - before);
    before = now;

    boxes_outlive_target(1000);
    now = sloth_rc_live();
    rep("boxes_outlive_target", now - before);
    before = now;

    mixed_targets(1000);
    now = sloth_rc_live();
    rep("mixed_targets", now - before);
    before = now;

    weak_store_faces(1000);
    now = sloth_rc_live();
    rep("weak_store_faces", now - before);
    before = now;

    weak_field_ring(1000);
    now = sloth_rc_live();
    rep("weak_field_ring", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
