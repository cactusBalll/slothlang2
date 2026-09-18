// spec: TE-P3 fused kernels + math elementwise (design §4.2/§5.3)
func main(): unit {
    // math elementwise
    var v: Tensor<float, 1> = tensor.from_array([0.0, 1.0, 4.0], [3]);
    var e: Tensor<float, 1> = tensor.exp(v);
    print(e[0] == 1.0);              // expect: true
    print(e[1] > 2.71 and e[1] < 2.72); // expect: true
    print(e[2] > 54.5 and e[2] < 54.6); // expect: true
    var sq: Tensor<float, 1> = tensor.sqrt(v);
    print(sq[2] == 2.0);             // expect: true

    // silu fused
    var x: Tensor<float, 1> = tensor.from_array([0.0, 1.0, -1.0], [3]);
    var si: Tensor<float, 1> = tensor.silu(x);
    print(si[0] == 0.0);             // expect: true
    print(si[1] > 0.73 and si[1] < 0.74);  // expect: true
    print(si[2] > -0.27 and si[2] < -0.26);// expect: true

    // SwiGLU fused: a = silu(a) * b
    var a: Tensor<float, 1> = tensor.from_array([1.0, 0.0], [2]);
    var b: Tensor<float, 1> = tensor.from_array([2.0, 2.0], [2]);
    tensor.silu_mul_into(a, b);
    print(a[0] > 1.46 and a[0] < 1.47);  // expect: true
    print(a[1] == 0.0);              // expect: true

    // rmsnorm: one reduction + fused x*inv*w
    var rn: Tensor<float, 1> = tensor.from_array([3.0, 4.0], [2]);
    var w: Tensor<float, 1> = tensor.from_array([1.0, 1.0], [2]);
    var r: Tensor<float, 1> = tensor.rmsnorm(rn, w);
    print(r[0] > 0.84 and r[0] < 0.85);  // expect: true
    print(r[1] > 1.13 and r[1] < 1.14);  // expect: true

    // softmax (fresh and in place)
    var sm: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    var p: Tensor<float, 1> = tensor.softmax(sm);
    print(p[0] > 0.09 and p[0] < 0.10);  // expect: true
    print(p[2] > 0.66 and p[2] < 0.67);  // expect: true
    tensor.softmax_into(sm);
    print(sm[0] > 0.09 and sm[0] < 0.10); // expect: true

    // attention-style accumulation
    var dst: Tensor<float, 1> = tensor.from_array([1.0, 1.0], [2]);
    var src: Tensor<float, 1> = tensor.from_array([2.0, 3.0], [2]);
    tensor.add_scaled_into(dst, src, 0.5);
    print(dst[0] == 2.0);            // expect: true
    print(dst[1] == 2.5);            // expect: true

    var dv: Tensor<float, 1> = tensor.from_array([2.0, 4.0], [2]);
    tensor.div_scalar_into(dv, 2.0);
    print(dv[0] == 1.0);             // expect: true
    print(dv[1] == 2.0);             // expect: true

    // scalar math faces
    print(float_sqrt(4.0) == 2.0);   // expect: true
    print(float_exp(0.0) == 1.0);    // expect: true
    print(float_pow(2.0, 10.0) == 1024.0); // expect: true
}
