// spec: boolean logic — and/or/not, short-circuit side effects, comparisons
var calls = 0;
func yes(): bool { calls = calls + 1; return true; }
func no(): bool { calls = calls + 1; return false; }
func main(): unit {
    calls = 0;
    if no() and yes() { print(1); } else { print(2); }   // expect: 2
    print(calls);              // expect: 1
    if yes() or no() { print(3); } else { print(4); }    // expect: 3
    print(calls);              // expect: 2
    print(true and true);      // expect: true
    print(true and false);     // expect: false
    print(false or true);      // expect: true
    print(false or false);     // expect: false
    print(not false);          // expect: true
    print(not (1 > 2));        // expect: true
    print(1 < 2 and 2 < 3);    // expect: true
    print(1 > 2 or 2 > 1);     // expect: true
    print(1 <= 1 and 2 >= 2);  // expect: true
}
