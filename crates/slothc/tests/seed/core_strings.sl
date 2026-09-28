// seed: core strings — escapes, interpolation of all builtin surfaces
// (book §4.3)
func main(): unit {
    // escape decoding proven via length (avoids raw control chars in output)
    print("a\nb".len());                // expect: 3
    print("t\tx".len());                // expect: 3
    print("q\"q");                       // expect: q"q
    print("b\\b");                       // expect: b\b
    print("cost: $5");                   // expect: cost: $5
    print("x${1 + 2}y");                 // expect: x3y
    print("i=${42} f=${3.5} b=${true}");  // expect: i=42 f=3.5 b=true
    print("s=${"hi"}");                  // expect: s=hi
    print("r=${0..3}");                  // expect: r=0..3
    print("a=${[1, 2, 3]}");             // expect: a=[1, 2, 3]
    print("n=${nil}");                   // expect: n=nil
    var x: int? = nil;
    print("opt=${x}");                   // expect: opt=nil
    x = 5;
    print("opt=${x}");                   // expect: opt=5
    print("nested ${"a${1}b"}");         // expect: nested a1b
}
