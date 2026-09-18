// spec: TE-P3 stdlib `.slt` import via the D5 search path
import "sloth/random.slt";
import "sloth/tensor.slt";

func main(): unit {
    var rng = XorShift(12345);
    var a = rng.next_u32();
    var b = rng.next_u32();
    print(a != b);   // expect: true
    print(a >= 0);   // expect: true

    var rng2 = XorShift(12345);
    print(rng2.next_u32() == a);  // expect: true

    var x: Tensor<float, 1> = tensor.from_array([3.0, 4.0], [2]);
    var w: Tensor<float, 1> = tensor.from_array([1.0, 1.0], [2]);
    var r: Tensor<float, 1> = tensor.normalize(x, w);
    print(r[0] > 0.84);           // expect: true
    print(tensor.l2(x) == 5.0);   // expect: true
}
