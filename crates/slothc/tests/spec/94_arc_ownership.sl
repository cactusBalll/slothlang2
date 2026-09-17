// spec: ARC ownership protocol (§5.1.1) — every reference path is settled to
// the exact rc baseline: ref-return temporaries (unbound call results), class
// ref-field death cascades, loop-body locals abandoned by `continue`, and the
// iterator protocol (iter()/next() owned temps). sloth_rc_live is predeclared
// by the SLOTH_STATS runtime surface.

class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

class Holder {
    var a: Array<int>;
    var s: str;
    func __init__() { this.a = mkarr(); this.s = fstr(); }
}

func fstr(): str { return "hello"; }
func mkarr(): Array<int> { return [1, 2, 3]; }

func temp_calls(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        // unbound call results also cover the array-typed call + ctor temps
        acc = acc + len(fstr()) + mkarr().len() + Box(i).v;
        i = i + 1;
    }
    return acc;
}

func fields(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        // the instance owns two ref fields; death must release both
        let h = Holder();
        acc = acc + h.a.len() + h.s.len();
        i = i + 1;
    }
    return acc;
}

func continues(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let s = "abc";
        let a = [i, i + 1];
        if i % 2 == 0 {
            acc = acc + s.len();
            i = i + 1;
            continue;
        }
        acc = acc + a.len();
        i = i + 1;
    }
    return acc;
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

func iters(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let c = Counter(8);
        for x in c { acc = acc + x; }
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    // warm up the one-time interned/pooled entries
    let _ = temp_calls(50) + fields(50) + continues(50) + iters(50);
    let base = sloth_rc_live();
    let r = temp_calls(500) + fields(500) + continues(500) + iters(500);
    print(r > 0);                   // expect: true
    print(sloth_rc_live() == base); // expect: true
}
