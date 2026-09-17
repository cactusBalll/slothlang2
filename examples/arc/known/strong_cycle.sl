// KNOWN LIMITATION (by design): plain ARC cannot reclaim strong reference
// cycles. A ring of `Node.next: Node` fields leaks its members when the last
// external handle drops. The language answers this with `Weak<T>` back-edges
// (see examples/arc/arc_weak.sl `weak_ring`, which settles to the baseline).
//
// Expected output: a positive leak equal to 2 handles per ring (both nodes).
// This is documented, not a crash: the leak is bounded by the cycle size.
class Node {
    var next: Node? = nil;
}

func ring(n: int): int {
    var i = 0;
    while i < n {
        var a = Node();
        var b = Node();
        a.next = b;
        b.next = a;      // strong back-edge: cycle
        i = i + 1;
    }
    return n;
}

func main(): unit {
    ring(20);
    var base = sloth_rc_live();
    ring(1000);
    print(sloth_rc_live() - base);   // 2000: cycle leak (expected)
}
