// spec: Weak<T> reference boxes (patch 43) — weak fields hold a weak box
// instead of retaining the target, upgrade() yields T? (0 = dead), and a
// self-reference ring through weak fields is fully collected on death
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
    // inner scope is gone: the strong reference died with it
    var s = w.upgrade();
    if s is nil {
        return 1;
    }
    return 0;
}
func weak_ring_churn(): unit {
    var a = Node();
    var b = Node();
    a.next = b;
    b.next = a;               // weak fields hold no strong count: ring dies
}
pub func main(): unit {
    var a = Node();
    var b = Node();
    var w: Weak<Node> = nil;
    print(w is nil);          // expect: true
    w = a;                    // weak box wraps the target (no retain)
    print(w is nil);          // expect: false
    var s = w.upgrade();      // strong borrow; T? surface
    print(s is nil);          // expect: false
    if s is not nil {
        print(s.name);        // expect: n
    }
    var sm: Weak<Node> = b;
    var sb = sm.upgrade();
    if sb is not nil {
        print(sb.name);       // expect: n
    }
    print(dead_check());      // expect: 1
    // rc observability: the weak ring self-reference is collected
    let d0 = sloth_rc_drops();
    weak_ring_churn();
    print(sloth_rc_drops() > d0);   // expect: true
}
