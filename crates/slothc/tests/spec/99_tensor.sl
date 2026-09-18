// spec: tensor extension TE-P1 — create/index/slice/view write (design §3.1/§3.2)
func main(): unit {
    var t: Tensor<float, 2> = tensor.zeros([2, 3]);
    t[0][1] = 5.0;
    print(t[0][1]);        // expect: 5
    t[1][2] = 7.5;
    print(t[1][2]);        // expect: 7.5
    print(t[0][2]);        // expect: 0

    // row view shares storage: writing the row writes the tensor
    var row: Tensor<float, 1> = t[1];
    row[0] = 9.0;
    print(t[1][0]);        // expect: 9

    // rank-1 slice keeps the rank and shares storage
    var col: Tensor<float, 1> = t[0][0..2];
    col[1] = 4.0;
    print(t[0][1]);        // expect: 4

    // from_array builds a tensor; copy_into assigns a whole view
    var src: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    t[0] = src;
    print(t[0][0]);        // expect: 1
    print(t[0][1]);        // expect: 2
    print(t[0][2]);        // expect: 3

    // rank-3 indexing chain
    var cube: Tensor<float, 3> = tensor.zeros([2, 2, 2]);
    cube[1][1][1] = 8.0;
    print(cube[1][1][1]);  // expect: 8

    // 1D tensor
    var v: Tensor<float, 1> = tensor.zeros([4]);
    v[3] = 2.5;
    print(v[3]);           // expect: 2.5

    // int tensors: from_array + element read/write
    var it: Tensor<int, 1> = tensor.from_array([10, 20, 30], [3]);
    print(it[1]);          // expect: 20
    it[2] = 99;
    print(it[2]);          // expect: 99

    // element kind is part of the surface: int element indexes are scalars
    var m: Tensor<float, 2> = tensor.zeros([2, 2]);
    m[0][0] = 1.0;
    m[1][1] = 2.0;
    print(m[0][0] + m[1][1]);  // expect: 3

    // slice targets assign through a keep-rank view
    var rowv: Tensor<float, 1> = tensor.from_array([7.0, 8.0, 9.0], [3]);
    var d: Tensor<float, 2> = tensor.zeros([2, 3]);
    d[1][0..3] = rowv;
    print(d[1][2]);            // expect: 9
    var rows: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], [2, 3]);
    d[0..2] = rows;
    print(d[0][0]);            // expect: 1
    print(d[1][2]);            // expect: 6
}
