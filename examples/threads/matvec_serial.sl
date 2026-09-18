// TH-P3 baseline: single-threaded matvec with the same row-major triple loop
// as `matvec_parallel.sl`, so run.sh can measure the parallel speedup.

func matvec_serial(
    w: Tensor<float, 2>,
    x: Tensor<float, 1>,
    d: int,
    n: int
): Tensor<float, 1> {
    var out: Tensor<float, 1> = tensor.zeros([d]);
    for (var i: 0..d) {
        let row: Tensor<float, 1> = w[i];
        var acc = 0.0;
        for (var j: 0..n) {
            acc = acc + row[j] * x[j];
        }
        out[i] = acc;
    }
    return out;
}

func main(): unit {
    let dim = 512;
    let iters = 300;
    var w: Tensor<float, 2> = tensor.zeros([dim, dim]);
    var x: Tensor<float, 1> = tensor.zeros([dim]);
    var i = 0;
    while (i < dim) {
        x[i] = 1.0;
        var j = 0;
        while (j < dim) {
            w[i][j] = 1.0;
            j = j + 1;
        }
        i = i + 1;
    }
    var it = 0;
    while (it < iters) {
        let y: Tensor<float, 1> = matvec_serial(w, x, dim, dim);
        if (it == iters - 1) {
            print(y[0]);
        }
        it = it + 1;
    }
    print("serial matvec OK");
}
