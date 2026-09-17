// spec: empty literal context typing — declared element types drive the
// empty Array/Map constructors (patch: annotation-aware empty literals)
func main(): unit {
    var a: Array<float> = [];
    a.push(1.5);
    print(a[0]);              // expect: 1.5
    print(a.len());           // expect: 1
    var b: Array<str> = [];
    b.push("x");
    print(b[0]);              // expect: x
    var m: Map<str, int> = @();
    m["a"] = 1;
    print(m["a"]);            // expect: 1
    print(len(m));            // expect: 1
    var mf: Map<float, int> = @();
    mf[0.5] = 7;
    print(mf[0.5]);           // expect: 7
    var e: Array<int> = [];
    print(e.len());           // expect: 0
    var em: Map<int, float> = @();
    print(len(em));           // expect: 0
}
