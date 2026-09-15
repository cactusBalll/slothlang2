// spec: iterators & protocols — string chars, class iter()/next() protocol (§3.5)
class Queue {
    var data: Array<int> = [];
    var pos: int = 0;
    func push(v: int): unit {
        this.data.push(v);
        return;
    }
    func iter(): Queue {
        return this;
    }
    func next(): int? {
        if this.pos >= this.data.len() {
            return nil;
        }
        var v = this.data[this.pos];
        this.pos = this.pos + 1;
        return v;
    }
}
func main(): unit {
    let q = Queue();
    q.push(10);
    q.push(20);
    q.push(30);
    var s: int = 0;
    for (var x: q) {
        print(x);               // expect: 10
        // expect: 20
        // expect: 30
        s = s + x;
    }
    print(s);                   // expect: 60
    let s2 = "ab";
    var n = 0;
    for (var c: s2) {
        n = n + 1;
        print(c);               // expect: a
        // expect: b
    }
    print(n);                   // expect: 2
}

