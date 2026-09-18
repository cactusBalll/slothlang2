// TE-P3 fusion benchmark (fused): one reduction + one fused x*inv*w generic,
// a single output allocation per iteration (design §8.2).
func main(): unit {
    let n = 4096;
    let iters = 4000;
    var x: Tensor<float, 1> = tensor.zeros([n]);
    var w: Tensor<float, 1> = tensor.zeros([n]);
    var acc: float = 0.0;
    var i = 0;
    while i < iters {
        var y: Tensor<float, 1> = tensor.rmsnorm(x, w);
        acc += y[0];
        i = i + 1;
    }
    print(acc);
}
