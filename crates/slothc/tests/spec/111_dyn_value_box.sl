// spec: builtin value types auto-box onto dyn surfaces (int/float/bool)
trait Any {}

trait Display {
    func to_str(): str;
}

func idyn(x: dyn Any): int {
    if x is int {
        return x;
    }
    return -1;
}

func main(): unit {
    var a: dyn Any = 42;
    if a is int {
        var n: int = a;
        print("unbox ${n + 1}"); // expect: unbox 43
    }
    print(idyn(5)); // expect: 5
    var d: dyn Display = 7;
    print(d.to_str()); // expect: 7
    print("i=${d}"); // expect: i=7
    var f: dyn Display = 2.5;
    print(f.to_str()); // expect: 2.5
    var b: dyn Any = true;
    if b is bool {
        print("bool"); // expect: bool
    }
    var arr: Array<dyn Any> = [1, 2, 3];
    var sum = 0;
    for x in arr {
        if x is int {
            var v: int = x;
            sum = sum + v;
        }
    }
    print("sum ${sum}"); // expect: sum 6
}
