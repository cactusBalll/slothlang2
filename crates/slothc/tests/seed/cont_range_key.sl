// seed: `range` is a first-class Hashable/Equatable key (cross C2/C3/C4, E1/E2)
// plus boolean/fixed-width key inference (C3) and mixed-family rejection
func main(): unit {
    print(type_name(@((0..3): 100)));   // expect: Map<range, int>
    var m2: Map<range, int> = @();
    m2[0..3] = 100;
    print(len(m2));                     // expect: 1
    print(m2[0..3]);                    // expect: 100

    print((0..3) == (0..3));            // expect: true
    var a = 0..3; var b = 0..3;
    print(a == b);                      // expect: true
    print(a != b);                      // expect: false

    print(type_name(@(true: 1)));       // expect: Map<bool, int>
    print(type_name(@((int8(1)): 1)));  // expect: Map<int8, int>
    print(type_name(@(1.5: 1)));        // expect: Map<float, int>
}
