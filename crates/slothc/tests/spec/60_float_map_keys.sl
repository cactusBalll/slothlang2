// spec: float map keys ride the word route (design §2.5: float is Hashable)
func main(): unit {
    var m = @(1.5: 10, 2.5: 20);
    print(m[1.5]);            // expect: 10
    print(m[2.5]);            // expect: 20
    print(len(m));            // expect: 2
    var m2: Map<float, int> = @();
    m2[0.25] = 4;
    m2[0.25] = 5;
    print(m2[0.25]);          // expect: 5
    print(len(m2));           // expect: 1
    var mb = @(true: 1, false: 2);
    print(mb[true]);          // expect: 1
    print(mb[false]);         // expect: 2
    var mn = @(0.5: 1.5, 1.5: 2.5);
    print(mn[0.5]);           // expect: 1.5
    print(mn[1.5]);           // expect: 2.5
}
