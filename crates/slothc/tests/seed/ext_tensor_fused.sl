// ext: fused tensor kernels and math elementwise
// (exp/sqrt/silu/silu_mul_into/softmax/rmsnorm/add_scaled_into/div_scalar_into).

func main(): unit {
    // math elementwise
    var v: Tensor<float, 1> = tensor.from_array([0.0, 1.0, 4.0], [3]);
    var e: Tensor<float, 1> = tensor.exp(v);
    print(e[0] == 1.0);
    print(e[1] > 2.71 and e[1] < 2.72);
    var sq: Tensor<float, 1> = tensor.sqrt(v);
    print(sq[2] == 2.0);

    // silu
    var x: Tensor<float, 1> = tensor.from_array([0.0, 1.0, -1.0], [3]);
    var si: Tensor<float, 1> = tensor.silu(x);
    print(si[0] == 0.0);
    print(si[1] > 0.73 and si[1] < 0.74);

    // SwiGLU in place: a = silu(a) * b
    var a: Tensor<float, 1> = tensor.from_array([1.0, 0.0], [2]);
    var b: Tensor<float, 1> = tensor.from_array([2.0, 2.0], [2]);
    tensor.silu_mul_into(a, b);
    print(a[0] > 1.46 and a[0] < 1.47);
    print(a[1] == 0.0);

    // rmsnorm
    var rn: Tensor<float, 1> = tensor.from_array([3.0, 4.0], [2]);
    var w: Tensor<float, 1> = tensor.from_array([1.0, 1.0], [2]);
    var r: Tensor<float, 1> = tensor.rmsnorm(rn, w);
    print(r[0] > 0.84 and r[0] < 0.85);
    print(r[1] > 1.13 and r[1] < 1.14);

    // softmax fresh + in place
    var sm: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    var p: Tensor<float, 1> = tensor.softmax(sm);
    print(p[0] > 0.09 and p[0] < 0.10);
    print(p[2] > 0.66 and p[2] < 0.67);
    tensor.softmax_into(sm);
    print(sm[0] > 0.09 and sm[0] < 0.10);

    // attention-style accumulation + scalar division
    var dst: Tensor<float, 1> = tensor.from_array([1.0, 1.0], [2]);
    var src: Tensor<float, 1> = tensor.from_array([2.0, 3.0], [2]);
    tensor.add_scaled_into(dst, src, 0.5);
    print(dst[0] == 2.0);
    print(dst[1] == 2.5);
    tensor.div_scalar_into(dst, 2.0);
    print(dst[0] == 1.0);

    // scalar math faces
    print(float_sqrt(4.0) == 2.0);
    print(float_exp(0.0) == 1.0);
    print(float_pow(2.0, 10.0) == 1024.0);

    // softmax_into on a dynamic sub-slice leaves the tail untouched
    var d: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [4]);
    var pos = 2;
    var sub: Tensor<float, 1> = d[0..=pos];
    tensor.softmax_into(sub);
    print(tensor.sum(sub) > 0.99 and tensor.sum(sub) < 1.01);
    print(d[3] == 4.0);

    // expects
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
    // expect: true
}
