// seed diag: break/continue outside a loop, unknown call, field access on a
// non-class receiver, and a non-iterable for source (book §8.1, §9.1)
func main(): unit {
    break;                  // diag: break outside loop
    continue;               // diag: continue outside loop
    print(nope(1));         // diag: call to unknown
    var x = 5;
    print(x.y);             // diag: on unknown type
    for e in 1.5 { print(e); }  // diag: for-iteration requires
}
