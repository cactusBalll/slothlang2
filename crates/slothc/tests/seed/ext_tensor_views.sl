// ext: tensor views — a view keeps its owner's storage alive past the base
// handle, strided sub-views feed linalg, slice extents are exact, and write
// through a view is observed by the parent.
import "sloth/tensor.slt";

func make_view(): Tensor<float, 1> {
    var d: Tensor<float, 3> = tensor.zeros([2, 2, 2]);
    d[1][1][0] = 42.0;
    var v: Tensor<float, 1> = d[1][1];
    return v;
}

func main(): unit {
    // view keeps storage after the base handle dies
    let v = make_view();
    print(v[0]);
    v[0] = 7.0;
    print(v[0]);

    // strided sub-views feed matvec correctly
    var d: Tensor<float, 2> = tensor.from_array(
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0], [4, 3]);
    var sub: Tensor<float, 2> = d[1..3];
    var ones: Tensor<float, 1> = tensor.from_array([1.0, 1.0, 1.0], [3]);
    var y: Tensor<float, 1> = tensor.matvec(sub, ones);
    print(y[0]);
    print(y[1]);

    // slice extents via reductions, including an inclusive range and an empty one
    var a: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [4]);
    var s02: Tensor<float, 1> = a[0..2];
    print(tensor.sum(s02));
    var inc: Tensor<float, 1> = a[1..=2];
    print(tensor.sum(inc));
    var tail: Tensor<float, 1> = a[3..4];
    print(tensor.sum(tail));
    var empty: Tensor<float, 1> = a[2..2];
    print(tensor.sum(empty));

    // ../ parent writes through a sub-view
    var m: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [2, 2]);
    var r0: Tensor<float, 1> = m[0];
    r0[1] = 9.0;
    print(m[0][1]);

    // expects
    // expect: 42
    // expect: 7
    // expect: 15
    // expect: 24
    // expect: 3
    // expect: 5
    // expect: 4
    // expect: 0
    // expect: 9
}
