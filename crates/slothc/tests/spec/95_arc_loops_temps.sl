// spec: ARC stress fixes — map/str/iterator loop producers settle on break
// too, an owned producer used as a for-in iterable is not freed mid-loop,
// values() is a producer like keys(), and a bare strong target stored into a
// Weak element (push / list literal / map literal) is wrapped in a weak box.
class Node {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

class Counter {
    var n: int = 0;
    var limit: int = 0;
    func __init__(limit: int) { this.limit = limit; }
    func iter(): Counter { return this; }
    func next(): Node? {
        if this.n >= this.limit { return nil; }
        this.n = this.n + 1;
        return Node(this.n);
    }
}

func mkarr(i: int): Array<Node> { return [Node(i), Node(i + 1)]; }
func mkmap(i: int): Map<str, Node> { return @("a": Node(i), "b": Node(i + 1)); }
func mkcounter(i: int): Counter { return Counter(i); }

func loops(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        // producer iterables: must stay alive for the whole loop
        for x in mkarr(i) { acc = acc + x.v; }
        for (var e: mkmap(i)) { acc = acc + e.val.v; }
        for b in mkcounter(3) { acc = acc + b.v; }
        // abrupt exits from producer loops
        for x in mkarr(i) { acc = acc + x.v; break; }
        for (var e: mkmap(i)) { acc = acc + e.val.v; break; }
        for b in mkcounter(3) { acc = acc + b.v; break; }
        for c in "abc${i}" { acc = acc + c.len(); break; }
        i = i + 1;
    }
    return acc;
}

func values_settle(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var m: Map<str, Node> = @("a": Node(i), "b": Node(i + 1));
        var vs = values(m);
        var ks = keys(m);
        acc = acc + vs.len() + ks.len() + vs[0].v;
        i = i + 1;
    }
    return acc;
}

func weak_store(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var pushed: Array<Weak<Node>> = [];
        var listed: Array<Weak<Node>> = [];
        var mapped: Map<str, Weak<Node>> = @();
        {
            let t = Node(i);
            pushed.push(t);
            listed = [t];
            mapped["k"] = t;
            var u = pushed[0].upgrade();
            if u is not nil { acc = acc + u.v; }
        }
        // target dead: every box must read as nil
        if pushed[0].upgrade() is nil { acc = acc + 1; }
        if listed[0].upgrade() is nil { acc = acc + 1; }
        if mapped["k"].upgrade() is nil { acc = acc + 1; }
        i = i + 1;
    }
    return acc;
}

// value-optional literal under a declared element face boxes bare scalars so
// 0/false never read back as nil
func opt_literal(): int {
    var a: Array<int?> = [0, nil, 2];
    var acc = 0;
    if a[0] is nil { acc = acc - 1; } else { acc = acc + a[0]; }
    if a[1] is nil { acc = acc + 10; }
    if a[2] is not nil { acc = acc + a[2]; }
    var m: Map<str, int?> = @("z": 0, "n": nil);
    if m["z"] is nil { acc = acc - 1; } else { acc = acc + m["z"]; }
    if m["n"] is nil { acc = acc + 1; }
    return acc;
}

pub func main(): unit {
    // warm up + settle the interned/literal baseline
    let _ = loops(20) + values_settle(20) + weak_store(20);
    print(opt_literal());           // expect: 13
    let base = sloth_rc_live();
    let r = loops(500) + values_settle(500) + weak_store(500);
    print(r > 0);                   // expect: true
    print(sloth_rc_live() == base); // expect: true
}
