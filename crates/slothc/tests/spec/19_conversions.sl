// spec: conversions & bool discipline — int()/float() casts; only bool tests conds (§3.4/§2.1)
func main(): unit {
    print(int(2.7));             // expect: 2
    print(int(-2.9));            // expect: -2
    print(float(3));             // expect: 3
    print(float(3) + 0.5);       // expect: 3.5
    print(int(float(7)));        // expect: 7
    var flag = false;
    while not flag {
        flag = true;
        print(1);                // expect: 1
    }
    if 2 == 2 {
        print(2);                // expect: 2
    }
    if 0 > 1 {
        print(3);
    } else {
        print(4);                // expect: 4
    }
}

