// seed: Weak<T> — weak boxes do not raise the target count, upgrade() yields
// T? (dead target -> nil), and a weak back-edge breaks a strong ring so the
// whole ring is collected (book ch15.4, ch23.4).
class Node {
    var name: str = "n";
    var next: Weak<Node> = nil;
}
func dead_check(): int {
    var w: Weak<Node> = nil;
    {
        var tmp = Node();
        w = tmp;
    }
    let s = w.upgrade();
    if s is nil {
        return 1;
    }
    return 0;
}
func main(): unit {
    var a = Node();
    a.name = "alive";
    var w: Weak<Node> = nil;
    print(w is nil);                 // expect: true
    w = a;                           // weak box wraps the target (no retain)
    print(w is nil);                 // expect: false
    let u = w.upgrade();
    if u is not nil {
        print(u.name);               // expect: alive
    }
    print(dead_check());             // expect: 1

    // weak back-edges break a strong ring: both nodes die together
    let d0 = sloth_rc_drops();
    {
        var x = Node();
        var y = Node();
        x.next = y;
        y.next = x;
    }
    print(sloth_rc_drops() > d0);    // expect: true
}
