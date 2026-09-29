// seed: an assignment target's member segment is not a closure capture
// (`b.v = 7` captured `b`, not `v`); regression for the target-path walker
class Box {
    var v: int = 0;
}
func main(): unit {
    var b = Box();
    let f = || -> int {
        b = Box();
        b.v = 7;
        return b.v;
    };
    print(f()); // expect: 7
    print(f()); // expect: 7
}
