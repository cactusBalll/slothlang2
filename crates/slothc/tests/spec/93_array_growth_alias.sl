// spec: array growth keeps the handle stable — aliases, callee params,
// closure captures and nested container elements all observe the growth
func fill(a: Array<int>, n: int) {
    for i in 0..n { a.push(i); }
}
func main(): unit {
    var a: Array<int> = [];
    var b = a;
    for i in 0..20 { a.push(i); }
    print(a.len());              // expect: 20
    print(b.len());              // expect: 20
    print(b[19]);                // expect: 19

    var c: Array<int> = [];
    fill(c, 100);
    print(c.len());              // expect: 100
    var s = 0;
    for i in 0..c.len() { s = s + c[i]; }
    print(s);                    // expect: 4950

    var d: Array<int> = [];
    var pushd = |x: int| { d.push(x); return 0; };
    for i in 0..20 { pushd(i); }
    print(d.len());              // expect: 20

    var g: Array<Array<int>> = [];
    g.push([]);
    for j in 0..20 { g[0].push(j); }
    print(g[0].len());           // expect: 20
    print(g[0][19]);             // expect: 19
}
