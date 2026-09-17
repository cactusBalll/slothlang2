// Float matrix multiply: results printed as scaled ints (unified format).
var seed = 13572468;
func rnd(): int {
    seed = (seed * 1103515245 + 12345) % 2147483648;
    return seed;
}
func mm(a: Array<Array<float>>, b: Array<Array<float>>): Array<Array<float>> {
    let n = a.len();
    let m = b[0].len();
    let p = b.len();
    var c: Array<Array<float>> = [];
    var i = 0;
    while i < n {
        var row: Array<float> = [];
        var j = 0;
        while j < m {
            var acc = 0.0;
            var k = 0;
            while k < p { acc = acc + a[i][k] * b[k][j]; k = k + 1; }
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
    while c < 4 {
        let n = 8 + rnd() % 28;
        var a: Array<Array<float>> = [];
        var b: Array<Array<float>> = [];
        var i = 0;
        while i < n {
            var ra: Array<float> = [];
            var rb: Array<float> = [];
            var j = 0;
            while j < n {
                ra.push(float(rnd() % 1000000) / 1000000.0 - 0.5);
                rb.push(float(rnd() % 1000000) / 1000000.0 - 0.5);
                j = j + 1;
            }
            a.push(ra);
            b.push(rb);
            i = i + 1;
        }
        let r = mm(a, b);
        print(n);
        i = 0;
        while i < n {
            var j = 0;
            while j < n { print(int(r[i][j] * 1000000000000000.0)); j = j + 1; }
            i = i + 1;
        }
        c = c + 1;
    }
}
