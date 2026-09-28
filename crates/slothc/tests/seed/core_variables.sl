// seed: core variables — var/let, nested-block shadowing, module globals
// (book §6)
var g = 10;
var h: int = 3;
let fixed = 7;
func bump(): unit { g = g + 1; }
func main(): unit {
    var a = 1;
    {
        var a = 2;
        print(a);              // expect: 2
        {
            var a = 3;
            print(a);          // expect: 3
        }
        print(a);              // expect: 2
        let a = 4;
        print(a);              // expect: 4
    }
    print(a);                  // expect: 1
    var f = 2.5;
    print(f);                  // expect: 2.5
    var s = "hi";
    print(s);                  // expect: hi
    print(g);                  // expect: 10
    print(h);                  // expect: 3
    print(fixed);              // expect: 7
    bump();
    bump();
    print(g);                  // expect: 12
    g = 100;
    print(g);                  // expect: 100
    var i = 0;
    for i in 0..2 { print(i); } // expect: 0
                                // expect: 1
    print(i);                  // expect: 0
}
