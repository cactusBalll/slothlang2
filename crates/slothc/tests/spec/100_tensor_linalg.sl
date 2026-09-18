// spec: tensor extension TE-P2 — channel-B linalg operators (design §4.2)
func main(): unit {
    // matvec: W(2,3) @ x(3)
    var w: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], [2, 3]);
    var x: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    var y: Tensor<float, 1> = tensor.matvec(w, x);
    print(y[0]);  // expect: 14
    print(y[1]);  // expect: 32

    // matvec works on a rank-3 layer view (weights are (layer, dim, dim))
    var w3: Tensor<float, 3> = tensor.from_array(
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0], [2, 2, 3]);
    var layer: Tensor<float, 2> = w3[1];
    var ones: Tensor<float, 1> = tensor.from_array([1.0, 1.0, 1.0], [3]);
    var yl: Tensor<float, 1> = tensor.matvec(layer, ones);
    print(yl[0]);  // expect: 24
    print(yl[1]);  // expect: 33

    // matmul: identity leaves the matrix unchanged
    var m1: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [2, 2]);
    var id: Tensor<float, 2> = tensor.from_array([1.0, 0.0, 0.0, 1.0], [2, 2]);
    var m3: Tensor<float, 2> = tensor.matmul(m1, id);
    print(m3[0][1]);  // expect: 2
    print(m3[1][0]);  // expect: 3

    // elementwise family
    var a: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    var b: Tensor<float, 1> = tensor.from_array([10.0, 20.0, 30.0], [3]);
    var c: Tensor<float, 1> = tensor.add(a, b);
    print(c[2]);  // expect: 33
    var s: Tensor<float, 1> = tensor.sub(b, a);
    print(s[0]);  // expect: 9
    var p: Tensor<float, 1> = tensor.mul(a, b);
    print(p[1]);  // expect: 40
    var q: Tensor<float, 1> = tensor.div(b, a);
    print(q[2]);  // expect: 10

    // reductions
    print(tensor.dot(a, b));  // expect: 140
    print(tensor.sum(a));     // expect: 6

    // in-place accumulation (residual `x += xb`)
    tensor.add_into(a, b);
    print(a[0]);  // expect: 11
    print(a[2]);  // expect: 33

    // elementwise on a rank-2 tensor
    var u: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [2, 2]);
    var v: Tensor<float, 2> = tensor.add(u, u);
    print(v[1][1]);  // expect: 8
}
