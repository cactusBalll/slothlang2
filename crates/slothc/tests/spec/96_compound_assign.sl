// spec: compound assignment `+=` / `-=` (design §3.3 AddAssign/SubAssign)
var calls: int = 0;

func next_index(): int {
    calls += 1;
    return 0;
}

class Acc {
    var v: int;
    func __init__(a: int) { this.v = a; }
}

func main(): unit {
    var a = 5;
    a += 3;
    print(a);              // expect: 8
    a -= 1;
    print(a);              // expect: 7

    var f = 1.5;
    f += 2.0;
    print(f);              // expect: 3.5
    f -= 0.5;
    print(f);              // expect: 3

    var arr = [1, 2, 3];
    arr[1] += 10;
    print(arr[1]);         // expect: 12
    arr[0] -= 1;
    print(arr[0]);         // expect: 0

    var m = @("k": 1);
    m["k"] += 5;
    print(m["k"]);         // expect: 6
    m["k"] -= 2;
    print(m["k"]);         // expect: 4

    var p = Acc(10);
    p.v += 4;
    print(p.v);            // expect: 14
    p.v -= 3;
    print(p.v);            // expect: 11

    // the index expression is evaluated exactly once per compound assign
    var data = [10];
    data[next_index()] += 5;
    print(data[0]);        // expect: 15
    print(calls);          // expect: 1
    data[next_index()] -= 1;
    print(data[0]);        // expect: 14
    print(calls);          // expect: 2
}
