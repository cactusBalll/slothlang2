// seed: ARC ownership for optionals/Result — value-optional payload boxes,
// reference-optional temporaries, elvis and Result ref payloads all settle
// back to the exact rc baseline across churn loops (book ch23).
class Node {
    var s: str = "hello";
    var next: Node? = nil;
}
func opt_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var o: Node? = nil;
        o = Node();
        if o is not nil { acc = acc + o.s.len(); }
        var si: int? = i;
        if si is not nil { acc = acc + si; }
        var b: int? = i;
        b = i + 1;                   // overwrite releases the old box
        b = nil;
        i = i + 1;
    }
    return acc;
}
func elvis_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var s: str? = nil;
        if i % 2 == 0 { s = "abc"; }
        let t = s ?: "z";
        acc = acc + t.len();
        i = i + 1;
    }
    return acc;
}
func res_churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var r: Result<int, str> = ok(i);
        if r.is_ok() { acc = acc + r.unwrap(); }
        var rs: Result<str, int> = err(0);
        if rs.is_ok() { acc = acc + rs.unwrap().len(); }
        i = i + 1;
    }
    return acc;
}
func main(): unit {
    let _ = opt_churn(50) + elvis_churn(50) + res_churn(50);
    let base = sloth_rc_live();
    print(opt_churn(2000) > 0);          // expect: true
    print(sloth_rc_live() == base);      // expect: true
    print(elvis_churn(2000) > 0);        // expect: true
    print(sloth_rc_live() == base);      // expect: true
    print(res_churn(2000) > 0);          // expect: true
    print(sloth_rc_live() == base);      // expect: true
}
