// ARC stress: patterns that commonly leak under refcounting — pop-returned
// owners, nested container death cascades, overwrite-out on map/array/field,
// abandoned loop-body locals on break/continue, iterator temporaries, deep
// chains, recursive ref returns, and array growth (relocate) with weak boxes.
class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

class Node {
    var v: int;
    var next: Node? = nil;
    var weak_next: Weak<Node> = nil;
    func __init__(v: int) { this.v = v; }
}

class Holder {
    var a: Array<Box> = [];
    var m: Map<str, Box> = @();
    var s: str = "";
    func __init__(v: int) {
        this.a = [Box(v)];
        this.m = @("k": Box(v + 1));
        this.s = "s${v}";
    }
}

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

// pop hands the caller an owned element; every popped value must be released
func pop_churn(n: int): int {
    var acc = 0;
    var a: Array<Box> = [];
    var i = 0;
    while i < n {
        a.push(Box(i));
        var b = a.pop();
        acc = acc + b.v;
        i = i + 1;
    }
    return acc;
}

// pop ignored (unbound temporary) must still release
func pop_ignore(n: int): int {
    var a: Array<Box> = [];
    var i = 0;
    while i < n {
        a.push(Box(i));
        a.pop();
        i = i + 1;
    }
    return a.len();
}

// nested container: array of arrays of boxes, all dropped at scope exit
func nested_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var g: Array<Array<Box>> = [];
        g.push([Box(i), Box(i + 1)]);
        g.push([Box(i + 2)]);
        acc = acc + g[0][0].v + g[1][0].v;
        i = i + 1;
    }
    return acc;
}

// map of arrays of boxes, overwritten each round (evicts old nested graph)
func map_nested_churn(n: int): int {
    var acc = 0;
    var m: Map<str, Array<Box>> = @();
    var i = 0;
    while i < n {
        m["k"] = [Box(i), Box(i + 1)];
        acc = acc + m["k"][0].v + m["k"][1].v;
        i = i + 1;
    }
    return acc;
}

// holder owns multiple ref fields; instance death cascades all of them
func holder_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let h = Holder(i);
        acc = acc + h.a[0].v + h.m["k"].v + h.s.len();
        i = i + 1;
    }
    return acc;
}

// continue abandons ref locals mid-loop; break abandons them too
func break_continue(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let b = Box(i);
        let s = "x${i}";
        if i % 3 == 0 {
            acc = acc + b.v;
            i = i + 1;
            continue;
        }
        if i % 7 == 0 {
            acc = acc + s.len();
            break;
        }
        acc = acc + b.v + s.len();
        i = i + 1;
    }
    return acc;
}

// iterator protocol: iter() receiver + per-iteration next() owned temporaries
class Counter {
    var n: int = 0;
    var limit: int = 0;
    func __init__(limit: int) { this.limit = limit; }
    func iter(): Counter { return this; }
    func next(): Box? {
        if this.n >= this.limit { return nil; }
        this.n = this.n + 1;
        return Box(this.n);
    }
}
func iter_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let c = Counter(6);
        for b in c {
            acc = acc + b.v;
        }
        i = i + 1;
    }
    return acc;
}

// for-in over an array of refs: the loop var is a borrow, the container owns
func forin_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let a: Array<Box> = [Box(i), Box(i + 1), Box(i + 2)];
        for b in a {
            acc = acc + b.v;
        }
        i = i + 1;
    }
    return acc;
}

// map iteration produces Entry temporaries that must settle
func map_forin_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let m: Map<str, Box> = @("a": Box(i), "b": Box(i + 1));
        for (var e: m) {
            acc = acc + e.val.v;
        }
        i = i + 1;
    }
    return acc;
}

// deep strong chain: dropping the head must cascade the whole tail
func chain(n: int): Node {
    var head = Node(0);
    var cur = head;
    var i = 1;
    while i < n {
        var nx = Node(i);
        cur.next = nx;
        cur = nx;
        i = i + 1;
    }
    return head;
}
func chain_churn(n: int, rounds: int): int {
    var acc = 0;
    var i = 0;
    while i < rounds {
        let h = chain(n);
        acc = acc + h.v;
        i = i + 1;
    }
    return acc;
}

// recursive function returning a fresh ref at every level
func rec_build(depth: int): Array<Box> {
    if depth <= 0 {
        return [Box(depth)];
    } else {
        var inner = rec_build(depth - 1);
        inner.push(Box(depth));
        return inner;
    }
}
func rec_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let a = rec_build(10);
        acc = acc + a.len();
        i = i + 1;
    }
    return acc;
}

// array growth past the initial capacity exercises rt relocate; weak boxes
// chained to a target that is never relocated must keep resolving
func growth_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a: Array<Box> = [];
        var w: Weak<Box> = nil;
        {
            let t = Box(i);
            w = t;
            var j = 0;
            while j < 300 {
                a.push(Box(j));
                j = j + 1;
            }
            var u = w.upgrade();
            if u is nil {
                acc = acc - 1;
            } else {
                acc = acc + u.v;
            }
        }
        var u2 = w.upgrade();
        if u2 is nil {
            acc = acc + 1;
        }
        acc = acc + a.len();
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    let _w = pop_churn(20) + pop_ignore(20) + nested_churn(20)
        + map_nested_churn(20) + holder_churn(20) + break_continue(20)
        + iter_churn(20) + forin_churn(20) + map_forin_churn(20)
        + chain_churn(20, 5) + rec_churn(20) + growth_churn(20);
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    pop_churn(2000);
    now = sloth_rc_live();
    rep("pop_bound", now - before);
    before = now;

    pop_ignore(2000);
    now = sloth_rc_live();
    rep("pop_temporary", now - before);
    before = now;

    nested_churn(1000);
    now = sloth_rc_live();
    rep("nested_container", now - before);
    before = now;

    map_nested_churn(1000);
    now = sloth_rc_live();
    rep("map_nested", now - before);
    before = now;

    holder_churn(1000);
    now = sloth_rc_live();
    rep("holder_fields", now - before);
    before = now;

    break_continue(2000);
    now = sloth_rc_live();
    rep("break_continue", now - before);
    before = now;

    iter_churn(1000);
    now = sloth_rc_live();
    rep("iterator", now - before);
    before = now;

    forin_churn(1000);
    now = sloth_rc_live();
    rep("forin_array", now - before);
    before = now;

    map_forin_churn(1000);
    now = sloth_rc_live();
    rep("forin_map", now - before);
    before = now;

    chain_churn(200, 20);
    now = sloth_rc_live();
    rep("deep_chain", now - before);
    before = now;

    rec_churn(1000);
    now = sloth_rc_live();
    rep("recursive", now - before);
    before = now;

    growth_churn(200);
    now = sloth_rc_live();
    rep("growth_weak", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
