// seed: core pipes and elvis — left-assoc chaining, last-arg append, and
// elvis over value and reference optionals (book §7.1, §7.2)
func dbl(x: int): int { return x * 2; }
func add(a: int, b: int): int { return a + b; }
func g3(a: int, b: int, c: int): int { return a * 100 + b * 10 + c; }
class C { var n: int = 7; }
func main(): unit {
    print(3 |> dbl);                 // expect: 6
    print(3 |> add(4));              // expect: 7
    print(1 |> dbl |> dbl);          // expect: 4
    print(1 |> add(2) |> g3(3, 4));  // expect: 343
    print(2 |> add(3) |> add(4));    // expect: 9

    // value optional
    var a: int? = nil;
    print(a ?: 5);                   // expect: 5
    a = 3;
    print(a ?: 5);                   // expect: 3
    var f: float? = nil;
    print(f ?: 1.5);                 // expect: 1.5
    var b: bool? = nil;
    print(b ?: true);                // expect: true
    b = false;
    print(b ?: true);                // expect: false

    // reference optional
    var o: C? = nil;
    var d = C();
    print((o ?: d).n);               // expect: 7
    o = C();
    print((o ?: d).n);               // expect: 7
    var s: str? = nil;
    print(s ?: "def");               // expect: def
}
