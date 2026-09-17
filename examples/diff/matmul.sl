// Matrix multiplication differential test: random square int matrices.
var seed = 24681357;
func rnd(): int {
    seed = (seed * 1103515245 + 12345) % 2147483648;
    return seed;
}

func matmul(a: Array<Array<int>>, b: Array<Array<int>>): Array<Array<int>> {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();
    var c: Array<Array<int>> = [];
    var i = 0;
    while i < n {
        var row: Array<int> = [];
        var j = 0;
        while j < m {
            var acc = 0;
            var k = 0;
            while k < p {
                acc = acc + a[i][k] * b[k][j];
                k = k + 1;
            }
            row.push(acc);
            j = j + 1;
        }
        c.push(row);
        i = i + 1;
    }
    return c;
}

func main(): unit {
    var c = 0;
    while c < 40 {
        let n = 1 + rnd() % 12;
        var a: Array<Array<int>> = [];
        var b: Array<Array<int>> = [];
        var i = 0;
        while i < n {
            var ra: Array<int> = [];
            var rb: Array<int> = [];
            var j = 0;
            while j < n {
                ra.push(rnd() % 19 - 9);
                rb.push(rnd() % 19 - 9);
                j = j + 1;
            }
            a.push(ra);
            b.push(rb);
            i = i + 1;
        }
        let r = matmul(a, b);
        print(n);
        i = 0;
        while i < n {
            var j = 0;
            while j < n {
                print(r[i][j]);
                j = j + 1;
            }
            i = i + 1;
        }
        c = c + 1;
    }
}
