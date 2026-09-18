// TE-P3 fusion benchmark (naive): the same math decomposed into several
// operators with intermediate allocations (design §8.2 baseline).
func main(): unit {
    let n = 4096;
    let iters = 4000;
    var x: Tensor<float, 1> = tensor.zeros([n]);
    var w: Tensor<float, 1> = tensor.zeros([n]);
    var acc: float = 0.0;
    var i = 0;
    while i < iters {
        let sq: Tensor<float, 1> = tensor.mul(x, x);
        let ss: float = tensor.sum(sq);
        let mn: float = ss / float(n);
        let root: float = float_sqrt(mn + 0.00001);
        let y: Tensor<float, 1> = tensor.mul(x, w);
        tensor.div_scalar_into(y, root);
        acc += y[0];
        i = i + 1;
    }
    print(acc);
}
