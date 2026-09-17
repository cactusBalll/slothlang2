// spec: optionals — narrowing through parameters, live boxes, nil chains
class Node {
    var v: int = 0;
    var next: Node? = nil;
}
func first(n: Node?): int {
    if n is not nil { return n.v; }
    return 0 - 1;
}
func main(): unit {
    var n = Node();
    n.v = 5;
    print(first(n));            // expect: 5
    print(first(nil));          // expect: -1
    print(n.next is nil);       // expect: true
    var m = Node();
    m.v = 9;
    n.next = m;
    if n.next is not nil {
        let t = n.next;
        if t is not nil { print(t.v); }   // expect: 9
    }
    var i: int? = nil;
    print(i is nil);            // expect: true
    i = 0;
    print(i is nil);            // expect: false
    if i is not nil { print(i); } // expect: 0
    var b: bool? = false;
    print(b is nil);            // expect: false
    var f: float? = nil;
    print(f is nil);            // expect: true
    print(m.next is nil);       // expect: true
}
