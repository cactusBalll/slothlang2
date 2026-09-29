// seed: nested optional container literals. A nested array literal under an
// `Array<Array<int>?>` annotation must not adopt the outer `Array` hint as its
// own element type (that mis-tagged an int element as a reference and retained
// a raw word — rc subtract overflow; probe p_optmatrix). ARC must balance.
func churn(n: int): int {
    var acc = 0;
    var i = 0;
    while i < n {
        var a: Array<Array<int>?> = [[i], [i, i + 1]];
        acc = acc + a.len();
        i = i + 1;
    }
    return acc;
}
func main(): unit {
    let _ = churn(20);
    let base = sloth_rc_live();
    print(churn(2000) > 0);          // expect: true
    print(sloth_rc_live() == base);  // expect: true
    var a1: Array<int?> = [1];
    print(type_name(a1));            // expect: Array<int?>
    var a2: Array<str?> = ["x"];
    print(type_name(a2));            // expect: Array<str?>
    var a3: Array<float?> = [1.5];
    print(type_name(a3));            // expect: Array<float?>
    var a4: Array<bool?> = [true];
    print(type_name(a4));            // expect: Array<bool?>
    var a5: Array<C?> = [C()];
    print(type_name(a5));            // expect: Array<C?>
    var a6: Array<Array<int>?> = [[1]];
    print(type_name(a6));            // expect: Array<Array<int>?>
    print(a6.len());                 // expect: 1
}
class C {
    var n: int = 5;
}
