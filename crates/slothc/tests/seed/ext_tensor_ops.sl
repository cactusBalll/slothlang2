// ext: tensor construction / indexing / slicing / views / reshape / linalg /
// elementwise ops / reductions / in-place accumulation.
import "sloth/tensor.slt";

func main(): unit {
    // zeros + index write/read (float and int kinds)
    var t: Tensor<float, 2> = tensor.zeros([2, 3]);
    t[0][1] = 5.0;
    t[1][2] = 7.5;
    print(t[0][1]);
    print(t[1][2]);
    print(t[0][2]);

    // row view shares storage
    var row: Tensor<float, 1> = t[1];
    row[0] = 9.0;
    print(t[1][0]);

    // rank-1 slice keeps the rank and shares storage
    var col: Tensor<float, 1> = t[0][0..2];
    col[1] = 4.0;
    print(t[0][1]);

    // from_array + whole-view assign
    var d: Tensor<float, 2> = tensor.zeros([2, 3]);
    var s: Tensor<float, 1> = tensor.from_array([7.0, 8.0, 9.0], [3]);
    d[0][0..3] = s;
    print(d[0][0]);
    print(d[0][2]);
    var rows: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], [2, 3]);
    d[0..2] = rows;
    print(d[1][2]);

    // fill_zero then check via sum
    tensor.fill_zero(d);
    print(tensor.sum(d[0]));

    // reshape views share storage
    var flat: Tensor<float, 1> = tensor.from_array(
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0], [12]);
    var m: Tensor<float, 2> = matrix_view(flat, 0, 3, 4);
    print(m[1][2]);
    print(m[2][3]);
    m[0][0] = 99.0;
    print(flat[0]);
    var c: Tensor<float, 3> = cube_view(flat, 0, 2, 2, 3);
    print(c[1][1][2]);
    var f1: Tensor<float, 1> = flatten_view(flat, 3, 3);
    print(f1[0]);

    // linalg
    var w: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], [2, 3]);
    var x: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    var y: Tensor<float, 1> = tensor.matvec(w, x);
    print(y[0]);
    print(y[1]);
    var mm: Tensor<float, 2> = tensor.matmul(w, tensor.from_array(
        [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], [3, 2]));
    print(mm[0][1]);

    // elementwise + reductions
    var a: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    var b: Tensor<float, 1> = tensor.from_array([10.0, 20.0, 30.0], [3]);
    print(tensor.sum(tensor.add(a, b)));
    print(tensor.sum(tensor.sub(b, a)));
    print(tensor.sum(tensor.mul(a, b)));
    print(tensor.sum(tensor.div(b, a)));
    print(tensor.dot(a, b));
    print(tensor.sum(a));
    tensor.add_into(a, b);
    print(a[0]);
    print(a[2]);

    // int tensors
    var it: Tensor<int, 1> = tensor.from_array([10, 20, 30], [3]);
    print(it[1]);
    it[2] = 99;
    print(it[2]);
    print(tensor.sum(it));
    print(tensor.dot(it, it));

    // expected output
    // expect: 5
    // expect: 7.5
    // expect: 0
    // expect: 9
    // expect: 4
    // expect: 7
    // expect: 9
    // expect: 6
    // expect: 0
    // expect: 7
    // expect: 12
    // expect: 99
    // expect: 12
    // expect: 4
    // expect: 14
    // expect: 32
    // expect: 2
    // expect: 66
    // expect: 54
    // expect: 140
    // expect: 30
    // expect: 140
    // expect: 6
    // expect: 11
    // expect: 33
    // expect: 20
    // expect: 99
    // expect: 129
    // expect: 10301
}
