// seed: a death cascade releasing the last Weak<T> box targeting the object
// must not double-free (optionals Bug1 / A2)
class Node { var w: Weak<Node> = nil; }
func main(): unit {
    var a = Node();
    a.w = a;
    print("done");  // expect: done
}
