// spec: maps — literal, index r/w, len, keys/values (§2.1/§5.3, word route)
func main(): unit {
    let m = @(1: 10, 2: 20);
    print(len(m));             // expect: 2
    print(m[1]);               // expect: 10
    var g: Map<str, int> = @("x": 1);
    print(g["x"]);             // expect: 1
    g["y"] = 2;
    print(g["y"]);             // expect: 2
    print(g.len());            // expect: 2
    print(len(g));             // expect: 2
    let fm = @(1: 2.5);
    print(fm[1]);              // expect: 2.5
    fm[2] = 3;
    print(fm[2]);              // expect: 3
    var ks = keys(g);
    print(len(ks));            // expect: 2
    var vs = values(g);
    print(vs[0] + vs[1]);      // expect: 3
    var read = f7read();
    print(read);               // expect: 77
    var incr = @(4: 1);
    incr[4] = incr[4] + 1;
    print(incr[4]);            // expect: 2
}
func f7read(): int {
    let mm = @(5: 77);
    return mm[5];
}

