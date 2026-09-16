// spec: value-optional box churn (patch 42, ARC D1) — transferred call
// results keep exact counts: dead boxes return to the rc baseline, and only
// the two still-owned results stay live at the end
// (sloth_rc_live/sloth_rc_drops are predeclared by the SLOTH_STATS surface)
func one(v: int): int? {
    return v;
}
func churn(n: int): int {
    var a: int? = nil;
    var i = 0;
    while i < n {
        a = one(i);
        i = i + 1;
    }
    if a is not nil {
        return a;
    }
    return -1;
}
pub func main(): unit {
    var base = sloth_rc_live();
    let r = churn(5000);
    print(r);                              // expect: 4999
    print(sloth_rc_live() == base + 1);    // expect: true
    let d0 = sloth_rc_drops();
    let r2 = churn(2000);
    print(sloth_rc_drops() > d0);          // expect: true
    print(sloth_rc_live() == base + 2);    // expect: true
}
