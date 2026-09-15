// spec: assignments — let immutability, var reassign checks (patch #22/#25)
func main(): unit {
    var x = 1;
    x = 2;
    print(x);                // expect: 2
    var f = 0.0;
    f = f + 5;
    print(f);                // expect: 5
    let l = 10;
    print(l);                // expect: 10
    var s = "a";
    s = s + "b";
    print(s);                // expect: ab
    var arr: Array<int> = [1];
    arr = [2];
    print(arr.len());        // expect: 1
    let b: bool = true;
    print(b);                // expect: true
    var y = 5;
    y = y - 1;
    print(y);                // expect: 4
    let note = "hi";
    print(note);             // expect: hi
}

