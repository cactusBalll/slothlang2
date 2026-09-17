// spec: chained index/field assignment (design §3 EBNF assignable)
class Grid {
    var xs: Array<Array<int>> = [[1, 2], [3, 4]];
}
func main(): unit {
    var a = [[1, 2], [3, 4]];
    a[0][1] = 9;
    print(a[0][1]);           // expect: 9
    a[1][0] = a[0][0] + 100;
    print(a[1][0]);           // expect: 101
    var g = Grid();
    g.xs[1][1] = 42;
    print(g.xs[1][1]);        // expect: 42
    var m: Map<str, Map<str, int>> = @();
    m["a"] = @();
    m["a"]["b"] = 7;
    print(m["a"]["b"]);       // expect: 7
}
