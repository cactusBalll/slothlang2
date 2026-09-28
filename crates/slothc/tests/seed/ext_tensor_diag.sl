// ext: tensor surface diagnostics — rank bounds (1..=3), float-only linalg,
// and rank-1-only softmax are all rejected by `check`.
func rank4(): unit {
    var t: Tensor<float, 4> = tensor.zeros([1, 2, 3, 4]);
    print(t);
    // diag: tensor rank 4 unsupported
}

func rank0(): unit {
    var t: Tensor<float, 0> = tensor.zeros([]);
    print(t);
    // diag: tensor rank 0 unsupported
}

func int_matvec(): unit {
    var m: Tensor<int, 2> = tensor.zeros([2, 2]);
    var x: Tensor<int, 1> = tensor.zeros([2]);
    var y: Tensor<int, 1> = tensor.matvec(m, x);
    print(y[0]);
    // diag: tensor.matvec` supports `float` elements only
}

func rank2_softmax(): unit {
    var m: Tensor<float, 2> = tensor.zeros([2, 2]);
    var s: Tensor<float, 2> = tensor.softmax(m);
    print(s[0][0]);
    // diag: tensor.softmax` requires a rank-1 `float` tensor
}

func main(): unit {
    rank4();
    rank0();
    int_matvec();
    rank2_softmax();
}
