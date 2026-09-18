// TH-P3: data-parallel matvec (design §8.2 / Appendix A). Rows of `W(d,n)`
// are split into P contiguous blocks, one OS thread each, then joined. Each
// thread writes a disjoint row range of `out` — no write conflicts; `w`/`x`
// are read-only shared. This is the "legal without synchronization" shape of
// the shared-memory model (§6).

func matvec_parallel(
    w: Tensor<float, 2>,
    x: Tensor<float, 1>,
    d: int,
    n: int,
    p: int
): Tensor<float, 1> {
    var out: Tensor<float, 1> = tensor.zeros([d]);
    let rows = d / p;
    var handles: Array<JoinHandle<int>> = [];
    for (var t: 0..p) {
        let lo = t * rows;
        var hi = lo + rows;
        if (t == p - 1) {
            hi = d;
        }
        handles.push(thread.spawn(|unused: int| -> int {
            for (var i: lo..hi) {
                let row: Tensor<float, 1> = w[i];
                var acc = 0.0;
                for (var j: 0..n) {
                    acc = acc + row[j] * x[j];
                }
                out[i] = acc;
            }
            return 0;
        }, 0));
    }
    for (var h: handles) {
        h.join();
    }
    return out;
}

func main(): unit {
    // correctness: W(2,3) @ x(3) = [14, 32], split across two threads
    let w: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], [2, 3]);
    let x: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
    let check: Tensor<float, 1> = matvec_parallel(w, x, 2, 3, 2);
    print(check[0]);
    print(check[1]);

    // benchmark: y = W(512,512) @ x(512), repeated; timed by run.sh
    let dim = 512;
    let iters = 300;
    var big_w: Tensor<float, 2> = tensor.zeros([dim, dim]);
    var big_x: Tensor<float, 1> = tensor.zeros([dim]);
    var i = 0;
    while (i < dim) {
        big_x[i] = 1.0;
        var j = 0;
        while (j < dim) {
            big_w[i][j] = 1.0;
            j = j + 1;
        }
        i = i + 1;
    }
    var it = 0;
    while (it < iters) {
        let y: Tensor<float, 1> = matvec_parallel(big_w, big_x, dim, dim, 4);
        if (it == iters - 1) {
            print(y[0]);
        }
        it = it + 1;
    }
    print("parallel matvec OK");
}
