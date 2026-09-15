// spec: control flow — if/else, while, debug-for, break/continue, logic (§3.x)
func main(): unit {
    if 1 < 2 {
        print(1);             // expect: 1
    } else {
        print(0);
    }
    if 2 < 1 {
        print(0);
    } else {
        print(2);             // expect: 2
    }
    var i = 0;
    while i < 3 {
        print(i);             // expect: 0
        // expect: 1
        // expect: 2
        i = i + 1;
    }
    var acc = 0;
    for i in 0..4 {
        acc = acc + i;
    }
    print(acc);               // expect: 6
    var acc2 = 0;
    for i in 0..=4 {
        acc2 = acc2 + i;
    }
    print(acc2);              // expect: 10
    var acc3 = 0;
    for i in 0..5 {
        if i == 2 {
            continue;
        }
        if i == 4 {
            break;
        }
        acc3 = acc3 + i;
    }
    print(acc3);              // expect: 4
    var j = 5;
    if j > 3 {
        if j == 5 {
            print(5);         // expect: 5
        } else {
            print(50);
        }
    }
    if 2 > 1 and 3 > 2 {
        print(21);            // expect: 21
    }
    if 1 > 2 or 3 > 2 {
        print(32);            // expect: 32
    }
    print(not (1 > 2));       // expect: true
    var wflag = true;
    while wflag {
        wflag = false;
        print(70);            // expect: 70
    }
    var d = [5, 6, 7];
    var dt: int = 0;
    for x in d {
        if x > 5 {
            continue;
        }
        dt = dt + x;
    }
    print(dt);                // expect: 5
    var big = 100;
    if big >= 100 and big <= 100 {
        print(100);            // expect: 100
    }
    if big != 100 {
        print(1);
    } else {
        print(2);              // expect: 2
    }
}

