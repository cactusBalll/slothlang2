// TE-P2 matvec micro-benchmark: y = W @ x, W is (DIM x DIM) float tensor,
// repeated ITERS times. Built AOT (`slothc build`) and compared against the
// `matvec.c` reference compiled with `gcc -O3` (design §8.2).
func main(): unit {
    let dim = 512;
    let iters = 300;
    var w: Tensor<float, 2> = tensor.zeros([dim, dim]);
    var x: Tensor<float, 1> = tensor.zeros([dim]);
    var y: Tensor<float, 1> = tensor.zeros([dim]);
    var it = 0;
    while it < iters {
        y = tensor.matvec(w, x);
        it = it + 1;
    }
    print(y[0]);
}
