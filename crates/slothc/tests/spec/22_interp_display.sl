// spec: interpolation & Display plumbing — user class to_str in interpolation (§3.8, patch #21)
trait Display {
    func to_str(): str;
}
class Box impl Display {
    var v: int;
    func __init__(v: int) {
        this.v = v;
    }
    func to_str(): str {
        return "Box{ v=${this.v} }";
    }
}
func main(): unit {
    let b = Box(42);
    print("${b}");                        // expect: Box{ v=42 }
    print("x=${b} n=${42} f=${2.5}");     // expect: x=Box{ v=42 } n=42 f=2.5
    print(to_str_call(Box(7)));           // expect: got Box{ v=7 }
}
func to_str_call(x: Box): str {
    return "got ${x}";
}

