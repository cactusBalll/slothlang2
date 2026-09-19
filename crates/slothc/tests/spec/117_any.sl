// spec: `any` top type — boxing, runtime `write` rendering, containers,
// Display vs class-name fallback, nil, and RC balance
trait Display { func to_str(): str; }
class Pt impl Display {
    var x: int;
    var y: int;
    func __init__(x: int, y: int) { this.x = x; this.y = y; }
    func to_str(): str { return "Pt(${this.x}, ${this.y})"; }
}
class Plain { var n: int; func __init__(n: int) { this.n = n; } }

func main(): unit {
    var i: any = 42;
    print(i);                       // expect: 42
    var f: any = 3.5;
    print(f);                       // expect: 3.5
    var s: any = "hi";
    print(s);                       // expect: hi
    var b: any = true;
    print(b);                       // expect: true
    var arr: Array<int> = [1, 2, 3];
    var ea: any = arr;
    print(ea);                      // expect: [1, 2, 3]
    var nest: Array<Array<int>> = [[1, 2], [3]];
    var en: any = nest;
    print(en);                      // expect: [[1, 2], [3]]
    var m: Map<str, int> = @("a": 1, "b": 2);
    var em: any = m;
    print(em);                      // expect: {a: 1, b: 2}
    var p: any = Pt(1, 2);
    print(p);                       // expect: Pt(1, 2)
    var q: any = Plain(9);
    print(q);                       // expect: Plain
    var n: any = nil;
    print(n);                       // expect: nil
    print("interp ${ea} ${p} ${q}"); // expect: interp [1, 2, 3] Pt(1, 2) Plain
    var mixed: Array<any> = [1, "a", Pt(2, 0)];
    print(mixed);                   // expect: [1, a, Pt(2, 0)]
    var anymap: Map<str, any> = @("k": 9, "p": Pt(3, 0));
    print(anymap);                  // expect: {k: 9, p: Pt(3, 0)}
}
