// seed: core operators — &&/|| synonyms, short-circuit side effects, and
// OOB-guard safety (book §7.1, §7.3)
var calls = 0;
func yes(): bool { calls = calls + 1; return true; }
func no(): bool { calls = calls + 1; return false; }
func main(): unit {
    print(true && true);        // expect: true
    print(true || false);       // expect: true
    print(not false);           // expect: true
    print(not true or true);    // expect: true
    print(true or false and false); // expect: true
    calls = 0;
    if no() and yes() { print("bad"); } else { print("short-and"); } // expect: short-and
    print(calls);               // expect: 1
    if yes() or no() { print("short-or"); } else { print("bad"); }   // expect: short-or
    print(calls);               // expect: 2
    // guard idiom: the right side is not evaluated when the left is false
    let a = [7, 8, 9];
    var i = 99;
    if i < a.len() and a[i] == 7 { print("bad"); } else { print("guard"); } // expect: guard
    i = 1;
    if i < a.len() and a[i] == 8 { print("hit"); } else { print("bad"); }   // expect: hit
}
