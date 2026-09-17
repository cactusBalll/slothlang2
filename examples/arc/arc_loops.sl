// ARC stress: loop protocols and abrupt exits. Every per-iteration owned
// producer (map Entry + keys snapshot, iterator next() element, pooled str
// char) must settle on fallthrough, continue AND break, including nested
// loops where only the inner one is abandoned.
class Box {
    var v: int;
    func __init__(v: int) { this.v = v; }
}

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

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

func str_full(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let s = "abcdefghij";
        for c in s { acc = acc + c.len(); }
        i = i + 1;
    }
    return acc;
}

func str_break(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let s = "abcdefghij";
        for c in s {
            acc = acc + c.len();
            if c == "c" { break; }
        }
        i = i + 1;
    }
    return acc;
}

func str_continue(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let s = "abcdefghij";
        for c in s {
            if c == "c" { continue; }
            acc = acc + c.len();
        }
        i = i + 1;
    }
    return acc;
}

func iter_full(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let c = Counter(8);
        for b in c { acc = acc + b.v; }
        i = i + 1;
    }
    return acc;
}

func iter_break(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let c = Counter(8);
        for b in c {
            acc = acc + b.v;
            if b.v >= 3 { break; }
        }
        i = i + 1;
    }
    return acc;
}

func iter_continue(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let c = Counter(8);
        for b in c {
            if b.v % 2 == 0 { continue; }
            acc = acc + b.v;
        }
        i = i + 1;
    }
    return acc;
}

func map_full(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let m: Map<str, Box> = @("a": Box(i), "b": Box(i + 1), "c": Box(i + 2));
        for (var e: m) { acc = acc + e.val.v; }
        i = i + 1;
    }
    return acc;
}

func map_break(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let m: Map<str, Box> = @("a": Box(i), "b": Box(i + 1), "c": Box(i + 2));
        var k = 0;
        for (var e: m) {
            acc = acc + e.val.v;
            k = k + 1;
            if k >= 2 { break; }
        }
        i = i + 1;
    }
    return acc;
}

func map_continue(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let m: Map<str, Box> = @("a": Box(i), "b": Box(i + 1), "c": Box(i + 2));
        for (var e: m) {
            if e.val.v < 0 { continue; }
            acc = acc + e.val.v;
        }
        i = i + 1;
    }
    return acc;
}

// nested loops: inner break/continue must not disturb the outer protocol
func nested_loops(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let outer: Map<str, Box> = @("x": Box(i), "y": Box(i + 1));
        for (var e: outer) {
            let inner = Counter(4);
            for b in inner {
                acc = acc + b.v;
                if b.v == 2 { break; }
            }
            acc = acc + e.val.v;
        }
        i = i + 1;
    }
    return acc;
}

// range loops are bound-driven: no per-iteration handles, but cover exits
func range_loops(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        for j in 0..10 {
            acc = acc + j;
            if j == 4 { break; }
        }
        for k in 0..10 {
            if k % 2 == 0 { continue; }
            acc = acc + k;
        }
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    let _w = str_full(20) + str_break(20) + str_continue(20) + iter_full(20)
        + iter_break(20) + iter_continue(20) + map_full(20) + map_break(20)
        + map_continue(20) + nested_loops(20) + range_loops(20);
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    str_full(500);
    now = sloth_rc_live();
    rep("str_full", now - before);
    before = now;

    str_break(500);
    now = sloth_rc_live();
    rep("str_break", now - before);
    before = now;

    str_continue(500);
    now = sloth_rc_live();
    rep("str_continue", now - before);
    before = now;

    iter_full(1000);
    now = sloth_rc_live();
    rep("iter_full", now - before);
    before = now;

    iter_break(1000);
    now = sloth_rc_live();
    rep("iter_break", now - before);
    before = now;

    iter_continue(1000);
    now = sloth_rc_live();
    rep("iter_continue", now - before);
    before = now;

    map_full(1000);
    now = sloth_rc_live();
    rep("map_full", now - before);
    before = now;

    map_break(1000);
    now = sloth_rc_live();
    rep("map_break", now - before);
    before = now;

    map_continue(1000);
    now = sloth_rc_live();
    rep("map_continue", now - before);
    before = now;

    nested_loops(500);
    now = sloth_rc_live();
    rep("nested_loops", now - before);
    before = now;

    range_loops(1000);
    now = sloth_rc_live();
    rep("range_loops", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
