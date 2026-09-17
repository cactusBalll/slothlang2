// ARC stress: dyn/trait dispatch, Result payloads, optional (value + ref)
// surfaces, generic containers, keys()/values(), first-class ranges, and
// string producers — all reference-heavy edges that must settle to baseline.
trait Speaker {
    func noise(): str;
}

class Cat impl Speaker {
    var tag: str;
    func __init__(tag: str) { this.tag = tag; }
    func noise(): str { return "meow:" + this.tag; }
}

class Dog impl Speaker {
    var tag: str;
    func __init__(tag: str) { this.tag = tag; }
    func noise(): str { return "woof:" + this.tag; }
}

class Node {
    var v: int;
    var next: Node? = nil;
    func __init__(v: int) { this.v = v; }
}

class NodeBox<T> {
    var v: T;
    func __init__(v: T) { this.v = v; }
}

func rep(name: str, d: int): unit {
    if d == 0 {
        print("OK   " + name);
    } else {
        print("LEAK " + name + " ${d}");
    }
}

// dynamic dispatch through a trait surface; results are fresh owned strings
func dyn_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var xs: Array<dyn Speaker> = [];
        xs.push(Cat("a"));
        xs.push(Dog("b"));
        acc = acc + xs[0].noise().len() + xs[1].noise().len();
        i = i + 1;
    }
    return acc;
}

func dyn_from(i: int): dyn Speaker {
    if i % 2 == 0 { return Cat("e"); } else { return Dog("o"); }
}
func dyn_ret_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let s: dyn Speaker = dyn_from(i);
        acc = acc + s.noise().len();
        i = i + 1;
    }
    return acc;
}

func dyn_map_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var m: Map<str, dyn Speaker> = @();
        m["c"] = Cat("m");
        m["d"] = Dog("n");
        acc = acc + m["c"].noise().len() + m["d"].noise().len();
        i = i + 1;
    }
    return acc;
}

// Result with ref payloads (str and Array<int>) churn
func res_str(i: int): Result<str, str> {
    if i % 2 == 0 { return ok("good${i}"); } else { return err("bad${i}"); }
}
func result_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let r = res_str(i);
        if r.is_ok() {
            acc = acc + r.unwrap().len();
        } else {
            acc = acc + r.err().len();
        }
        i = i + 1;
    }
    return acc;
}

// value-optional boxes: int?/float?/bool? through containers and returns
func maybe(i: int): int? {
    if i % 2 == 0 { return i; } else { return nil; }
}
func opt_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a: Array<int?> = [];
        a.push(maybe(i));
        a.push(maybe(i + 1));
        var x = a[0];
        var y = a[1];
        if x is not nil { acc = acc + x; }
        if y is not nil { acc = acc + y; }
        i = i + 1;
    }
    return acc;
}

func opt_field_churn(n: int): int {
    var acc = 0;
    var c = Node(0);
    var i = 0;
    while i < n {
        c.next = Node(i);            // optional ref field overwrite-out
        var w: Node? = c.next;
        if w is not nil { acc = acc + w.v; }
        i = i + 1;
    }
    return acc;
}

// generic class holding a ref field
func generic_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let b = NodeBox<Node>(Node(i));
        var inner = b.v;
        acc = acc + inner.v;
        i = i + 1;
    }
    return acc;
}

// keys()/values() produce fresh arrays (+1 each element for ref values)
func keys_values_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var m: Map<str, Node> = @("a": Node(i), "b": Node(i + 1), "c": Node(i + 2));
        var ks = keys(m);
        var vs = values(m);
        acc = acc + ks.len() + vs.len() + vs[0].v;
        i = i + 1;
    }
    return acc;
}

// first-class range boxes
func range_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let r = 0..5;
        for x in r { acc = acc + x; }
        i = i + 1;
    }
    return acc;
}

// string producers: interpolation, concat, char iteration
func string_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        let s = "v${i}";
        let t = s + "!" + s;
        for c in t { acc = acc + c.len(); }
        i = i + 1;
    }
    return acc;
}

pub func main(): unit {
    let _w = dyn_churn(20) + dyn_ret_churn(20) + dyn_map_churn(20)
        + result_churn(20) + opt_churn(20) + opt_field_churn(20)
        + generic_churn(20) + keys_values_churn(20) + range_churn(20)
        + string_churn(20);
    var init = sloth_rc_live();
    var before = init;
    var now = 0;

    dyn_churn(500);
    now = sloth_rc_live();
    rep("dyn_array", now - before);
    before = now;

    dyn_ret_churn(500);
    now = sloth_rc_live();
    rep("dyn_return", now - before);
    before = now;

    dyn_map_churn(500);
    now = sloth_rc_live();
    rep("dyn_map", now - before);
    before = now;

    result_churn(1000);
    now = sloth_rc_live();
    rep("result_ref", now - before);
    before = now;

    opt_churn(1000);
    now = sloth_rc_live();
    rep("optional_value", now - before);
    before = now;

    opt_field_churn(1000);
    now = sloth_rc_live();
    rep("optional_field", now - before);
    before = now;

    generic_churn(1000);
    now = sloth_rc_live();
    rep("generic_ref", now - before);
    before = now;

    keys_values_churn(1000);
    now = sloth_rc_live();
    rep("keys_values", now - before);
    before = now;

    range_churn(1000);
    now = sloth_rc_live();
    rep("range", now - before);
    before = now;

    string_churn(500);
    now = sloth_rc_live();
    rep("string_producers", now - before);
    before = now;

    let final = sloth_rc_live();
    rep("overall", final - init);
    print("sink=${final}");
}
