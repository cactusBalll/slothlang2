// ARC stress: Weak<T> boxes (§5.1 D2). A weak box does not retain its target;
// target death invalidates every box chained to it; box churn must settle.
class Node {
    var v: int;
    var next: Weak<Node> = nil;
    func __init__(v: int) { this.v = v; }
}

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

// a weak box must go dead once the last strong ref leaves the scope
func weak_dies(): int {
    var w: Weak<Node> = nil;
    {
        let tmp = Node(7);
        w = tmp;
    }
    var s = w.upgrade();
    if s is nil {
        return 1;
    }
    return 0;
}

// a strong borrow returned by upgrade keeps the target alive independently
func weak_upgrade_keeps_alive(): int {
    var strong = Node(5);
    var w: Weak<Node> = strong;
    var s = w.upgrade();
    strong = Node(6);                 // rebind: the upgraded borrow still owns
    if s is nil {
        return -1;
    } else {
        return s.v;
    }
}

// many weak boxes targeting one object; target dies, all upgrades nil
func many_weak(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var ws: Array<Weak<Node>> = [];
        {
            let t = Node(i);
            var j = 0;
            while j < 4 {
                ws.push(t);
                j = j + 1;
            }
        }
        var k = 0;
        while k < ws.len() {
            var u = ws[k].upgrade();
            if u is nil {
                acc = acc + 1;
            }
            k = k + 1;
        }
        i = i + 1;
    }
    return acc;
}

// weak ring: strong cycle is broken by weak back-edges, so both die
func weak_ring(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a = Node(i);
        var b = Node(i + 1);
        a.next = b;
        b.next = a;
        acc = acc + a.v;
        i = i + 1;
    }
    return acc;
}

// weak boxes stored in a field, overwritten each round (old box released)
class Link {
    var w: Weak<Node> = nil;
}
func weak_field_churn(n: int): int {
    var acc = 0;
    var l = Link();
    var i = 0;
    while i < n {
        let t = Node(i);
        l.w = t;
        var u = l.w.upgrade();
        if u is not nil {
            acc = acc + u.v;
        }
        i = i + 1;
    }
    return acc;
}

// weak box captured by a closure; the box dies with the closure frame
func weak_in_closure(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let t = Node(i);
        let w: Weak<Node> = t;
        let f = || {
            var u = w.upgrade();
            if u is nil {
                return 0;
            } else {
                return u.v;
            }
        };
        acc = acc + f();
        i = i + 1;
    }
    return acc;
}

// Weak<int> value target: the scalar rides a box that dies with the source
func weak_value(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var w: Weak<int> = nil;
        {
            let x = i;
            w = x;
        }
        var u = w.upgrade();
        acc = acc + x_add(u);
        i = i + 1;
    }
    return acc;
}

// helper: int? -> int (nil -> -1), keeps the Weak<int> surface checked
func x_add(u: int?): int {
    if u is not nil {
        return u;
    } else {
        return -1;
    }
}

pub func main(): unit {
    // correctness first
    print(weak_dies());              // 1
    print(weak_upgrade_keeps_alive()); // 5

    let _w = many_weak(20) + weak_ring(20) + weak_field_churn(20)
        + weak_in_closure(20) + weak_value(20);
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    many_weak(2000);
    now = sloth_rc_live();
    rep("many_weak", now - before);
    before = now;

    weak_ring(2000);
    now = sloth_rc_live();
    rep("weak_ring", now - before);
    before = now;

    weak_field_churn(2000);
    now = sloth_rc_live();
    rep("weak_field", now - before);
    before = now;

    weak_in_closure(2000);
    now = sloth_rc_live();
    rep("weak_closure", now - before);
    before = now;

    weak_value(2000);
    now = sloth_rc_live();
    rep("weak_value", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
