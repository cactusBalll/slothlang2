// spec: deterministic allocator — churned containers are freed by the rc
// core when their counts reach zero (ARC replaced Boehm); live data survives
// (sloth_rc_drops is predeclared by the SLOTH_STATS runtime surface)

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
        // both refs die each iteration: rc frees the chunks deterministically
    }
    print(acc > 0); // churn consumed
}

pub func main(): unit {
    let d0 = sloth_rc_drops();
    churn(2000);
    churn(2000);
    let keep = [7, 8, 9];
    keep.push(10);
    print(keep.len());         // expect: 4
    print(keep[0] + keep[3]);  // expect: 17
    print(sloth_rc_drops() > d0); // expect: true
}
