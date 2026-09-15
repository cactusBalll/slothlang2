// spec: strings — concat, interpolation, len (§3.8/§5.2)
func main(): unit {
    print("abc" + "def");             // expect: abcdef
    let s = "x";
    print(s + "yz");                  // expect: xyz
    print(len("hello") == 5);         // expect: true
    var n = 42;
    print("n=${n}");                  // expect: n=42
    print("f=${2.5}");                // expect: f=2.5
    print("b=${true}");               // expect: b=true
    print("s=${"q"}");                // expect: s=q
    let x = 4;
    let y = 5;
    print("${x}+${y}=${x + y}");      // expect: 4+5=9
    var t = "a";
    t = t + "b";
    print(t);                         // expect: ab
    print(len(s + s));                // expect: 2
    print(s + t + "c");               // expect: xabc
    let empty = "e";
    print(empty.len());               // expect: 1
}

