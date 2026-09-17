// spec: UTF-8 strings — iteration yields characters, len stays byte length
// (§3.5: str iterates by character; string interning keeps content equality)
func main(): unit {
    var s = "aé中";
    var n = 0;
    for c in s { n = n + 1; }
    print(n);                     // expect: 3
    for c in s { print(c); }      // expect: a
    // expect: é
    // expect: 中
    print(len("héllo"));          // expect: 6
    print("héllo" == "hé" + "llo"); // expect: true
    print(len("中"));             // expect: 3
}
