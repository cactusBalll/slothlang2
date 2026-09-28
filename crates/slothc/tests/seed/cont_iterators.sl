// seed: iteration protocol — user Iterable (structural iter()/next()), Array,
// str, range, chars() (book ch16, ch14 §14.3)
class Range3 {
    var i: int = 0;
    func iter(): Range3 { return this; }
    func next(): int? {
        if this.i >= 3 { return nil; }
        this.i = this.i + 1;
        return this.i - 1;
    }
}
func mkarr(): Array<int> { return [10, 20, 30]; }
func main(): unit {
    for x in Range3() { print(x); }        // expect: 0
                                           // expect: 1
                                           // expect: 2
    for x in mkarr() { print(x); }         // expect: 10
                                           // expect: 20
                                           // expect: 30
    for i in 5..8 { print(i); }            // expect: 5
                                           // expect: 6
                                           // expect: 7
    for i in 0..=2 { print(i); }           // expect: 0
                                           // expect: 1
                                           // expect: 2
    for c in "aé中" { print(c); }          // expect: a
                                           // expect: é
                                           // expect: 中
    let it = "ab".chars();
    print(it.next());                      // expect: 97
    print(it.next());                      // expect: 98
    print(it.next() is nil);               // expect: true

    for x in [1, 2, 3, 4, 5] {
        if x == 3 { continue; }
        if x == 5 { break; }
        print(x);                          // expect: 1
                                           // expect: 2
                                           // expect: 4
    }
}
