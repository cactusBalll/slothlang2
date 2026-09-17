// spec: strings — concat, interpolation, escapes, content equality
func main(): unit {
    print("ab" + "cd");       // expect: abcd
    print(len("hello"));      // expect: 5
    print(len(""));           // expect: 0
    print("a" == "a");        // expect: true
    print("a" != "b");        // expect: true
    print("ab" == "a" + "b"); // expect: true
    var x = 7;
    print("x=${x}");          // expect: x=7
    print("sum=${x + 1}");    // expect: sum=8
    print("bool=${x > 3}");   // expect: bool=true
    print("float=${1.5}");    // expect: float=1.5
    var s = "";
    print("e${s}y");          // expect: ey
    print("q\"q");            // expect: q"q
    var t = "a";
    t = t + "b" + "c";
    print(t);                 // expect: abc
}
