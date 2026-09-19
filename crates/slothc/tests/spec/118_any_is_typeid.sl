// spec: `any` runtime tests — `is` narrows, `typeid`/`type_name`, nil, and a
// churn loop that returns to the rc baseline
class Animal { var n: int; func __init__(n: int) { this.n = n; } }
class Cat: Animal { func __init__(n: int) { super.__init__(n); } }
class Dog: Animal { func __init__(n: int) { super.__init__(n); } }

func classify(v: any): str {
    if v is Cat { return "cat"; }
    if v is Dog { return "dog"; }
    if v is Animal { return "animal"; }
    if v is int { return "int"; }
    if v is float { return "float"; }
    if v is bool { return "bool"; }
    if v is str { return "str"; }
    if v is nil { return "nil"; }
    return "other";
}

func churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var v: any = Cat(i);
        if v is Cat { acc = acc + v.n; }
        var s: any = "x${i}";
        if s is str { acc = acc + s.len(); }
        var arr: Array<any> = [i, "y"];
        acc = acc + arr.len();
        i = i + 1;
    }
    return acc;
}

func main(): unit {
    print(classify(Cat(1)));        // expect: cat
    print(classify(Dog(1)));        // expect: dog
    print(classify(5));             // expect: int
    print(classify(2.5));           // expect: float
    print(classify(true));          // expect: bool
    print(classify("hi"));          // expect: str
    print(classify(nil));           // expect: nil
    var c: any = Cat(1);
    print(type_name(c));            // expect: Cat
    print(typeid(c) == typeid(Cat(9))); // expect: true
    var e: any = nil;
    print(type_name(e));            // expect: nil
    let base = sloth_rc_live();
    let r = churn(200);
    print(r > 0);                   // expect: true
    print(sloth_rc_live() == base); // expect: true
}
