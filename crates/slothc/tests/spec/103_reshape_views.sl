// spec: TE-P4 shared-storage reshape views (llama2.c checkpoint weights)
import "sloth/tensor.slt";

func main(): unit {
    var d: Tensor<float, 1> = tensor.from_array(
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0], [12]);
    var m: Tensor<float, 2> = matrix_view(d, 0, 3, 4);
    print(m[1][2]);  // expect: 7
    print(m[2][3]);  // expect: 12
    m[0][0] = 99.0;
    print(d[0]);     // expect: 99

    var c: Tensor<float, 3> = cube_view(d, 0, 2, 2, 3);
    print(c[1][1][2]);  // expect: 12
    c[1][1][2] = 42.0;
    print(d[11]);       // expect: 42

    var f: Tensor<float, 1> = flatten_view(d, 3, 3);
    print(f[0]);  // expect: 4

    var w: Tensor<float, 2> = matrix_view(d, 0, 2, 3);
    var x: Tensor<float, 1> = tensor.from_array([1.0, 1.0, 1.0], [3]);
    var y: Tensor<float, 1> = tensor.matvec(w, x);
    print(y[0]);  // expect: 104
    print(y[1]);  // expect: 15
}
