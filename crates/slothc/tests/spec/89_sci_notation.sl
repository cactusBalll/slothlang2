// spec: scientific-notation float literals (design §3.9 FLOAT terminal)
func main(): unit {
    print(1e0);        // expect: 1
    print(1.5e-3);     // expect: 0.0015
    print(2E+4);       // expect: 20000
    print(1e2 + 0.0);  // expect: 100
    var x = 5e-1;
    print(x);          // expect: 0.5
}
