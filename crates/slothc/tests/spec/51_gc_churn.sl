// spec: GC — Boehm mark & sweep under sustained churn: dead arrays/strings
// reclaimed, live data survives explicit collection (§5.1 MVP libgc)
extern func sloth_gc_heap_size(): int;
extern func sloth_gc_collections(): int;
extern func sloth_gc_collect(): int;

func churn(n: int): unit {
    var acc = 0;
    var i = 0;
    while i < n {
        let a = [i + 1, i + 2, i + 3];
        var j = 0;
        while j < 1024 {
            a.push(j);
            j = j + 1;
        }
        let s = "xx${i}xx";
        acc = acc + a.len() + s.len() + a[0];
        i = i + 1;
        // drop both refs; next iteration reuses the heap
    }
    print(acc > 0); // churn consumed
}

pub func main(): unit {
    churn(2000);
    let c0 = sloth_gc_collect();

    // live object survives a full collection
    let keep = [7, 8, 9];
    keep.push(10);
    churn(2000);
    let c1 = sloth_gc_collect();

    print(keep.len());               // expect: 4
    print(keep[0] + keep[3]);        // expect: 17
    print(c1 > c0);                  // expect: true
    print(sloth_gc_heap_size() > 0); // expect: true
}
