// Corner-case differential: non-square matmul, zero-weight Dijkstra,
// empty KMP pattern, n=0 queens, pathological quicksort inputs.
func build_lps(pat: Array<int>): Array<int> {
    let m = pat.len();
    var lps: Array<int> = [];
    var i = 0;
    while i < m { lps.push(0); i = i + 1; }
    var len = 0;
    var k = 1;
    while k < m {
        if pat[k] == pat[len] { len = len + 1; lps[k] = len; k = k + 1; }
        else { if len != 0 { len = lps[len - 1]; } else { lps[k] = 0; k = k + 1; } }
    }
    return lps;
}
func kmp_all(text: Array<int>, pat: Array<int>): Array<int> {
    var res: Array<int> = [];
    let m = pat.len();
    if m == 0 { return res; }
    let lps = build_lps(pat);
    var i = 0;
    var j = 0;
    let n = text.len();
    while i < n {
        if text[i] == pat[j] { i = i + 1; j = j + 1; }
        if j == m { res.push(i - j); j = lps[j - 1]; }
        else { if i < n and text[i] != pat[j] { if j != 0 { j = lps[j - 1]; } else { i = i + 1; } } }
    }
    return res;
}
func mm(a: Array<Array<int>>, b: Array<Array<int>>): Array<Array<int>> {
    let n = a.len();
    let p = b.len();
    let m = b[0].len();
    var c: Array<Array<int>> = [];
    var i = 0;
    while i < n {
        var row: Array<int> = [];
        var j = 0;
        while j < m {
            var acc = 0;
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
func dijkstra(adj: Array<Array<int>>, src: int): Array<int> {
    let n = adj.len();
    let INF = 1000000000;
    var dist: Array<int> = [];
    var done: Array<int> = [];
    var i = 0;
    while i < n { dist.push(INF); done.push(0); i = i + 1; }
    dist[src] = 0;
    var iter = 0;
    while iter < n {
        var u = -1;
        var best = INF;
        var v = 0;
        while v < n { if done[v] == 0 and dist[v] < best { best = dist[v]; u = v; } v = v + 1; }
        if u < 0 { break; }
        done[u] = 1;
        v = 0;
        while v < n {
            if adj[u][v] >= 0 { let nd = dist[u] + adj[u][v]; if nd < dist[v] { dist[v] = nd; } }
            v = v + 1;
        }
        iter = iter + 1;
    }
    return dist;
}
func partition(a: Array<int>, lo: int, hi: int): int {
    let pivot = a[hi];
    var i = lo - 1;
    var j = lo;
    while j < hi {
        if a[j] <= pivot { i = i + 1; let t = a[i]; a[i] = a[j]; a[j] = t; }
        j = j + 1;
    }
    let t2 = a[i + 1]; a[i + 1] = a[hi]; a[hi] = t2;
    return i + 1;
}
func qsort(a: Array<int>, lo: int, hi: int): unit {
    if lo >= hi { return; }
    let p = partition(a, lo, hi);
    qsort(a, lo, p - 1);
    qsort(a, p + 1, hi);
}
func nq(col: Array<int>, row: int, n: int, safe_cnt: Array<int>): int {
    if row == n { return 1; }
    var cnt = 0;
    var c = 0;
    while c < n {
        var ok = true;
        var i = 0;
        while i < row {
            if col[i] == c { ok = false; }
            let d = row - i;
            if col[i] - c == d or c - col[i] == d { ok = false; }
            i = i + 1;
        }
        if ok { col[row] = c; cnt = cnt + nq(col, row + 1, n, safe_cnt); }
        c = c + 1;
    }
    return cnt;
}
func count(n: int): int {
    var col: Array<int> = [];
    var i = 0;
    while i < n { col.push(0); i = i + 1; }
    var sc: Array<int> = [];
    return nq(col, 0, n, sc);
}
func main(): unit {
    // KMP: all-same text/pattern, empty pattern, pattern longer than text
    var text: Array<int> = [];
    var i = 0;
    while i < 10 { text.push(0); i = i + 1; }
    let p3: Array<int> = [0, 0, 0];
    let r1 = kmp_all(text, p3);
    print(r1.len());
    for x in r1 { print(x); }
    var empty: Array<int> = [];
    print(kmp_all(text, empty).len());
    let big: Array<int> = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    print(kmp_all(text, big).len());
    // non-square matmul 2x3 * 3x4
    let a2: Array<Array<int>> = [[1, 2, 3], [4, 5, 6]];
    let b2: Array<Array<int>> = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    let c2 = mm(a2, b2);
    print(c2.len());
    print(c2[0].len());
    i = 0;
    while i < c2.len() {
        var j = 0;
        while j < c2[0].len() { print(c2[i][j]); j = j + 1; }
        i = i + 1;
    }
    // Dijkstra with zero-weight edges, -1 = no edge
    var g: Array<Array<int>> = [];
    i = 0;
    while i < 4 {
        var row: Array<int> = [];
        var j = 0;
        while j < 4 { row.push(0 - 1); j = j + 1; }
        g.push(row);
        i = i + 1;
    }
    g[0][1] = 0; g[1][0] = 0;
    g[1][2] = 0; g[2][1] = 0;
    g[2][3] = 5; g[3][2] = 5;
    g[0][3] = 10; g[3][0] = 10;
    let d = dijkstra(g, 0);
    i = 0;
    while i < 4 { print(d[i]); i = i + 1; }
    // queens n=0 and n=12
    print(count(0));
    print(count(12));
    // pathological quicksort: all equal / increasing / decreasing
    var eq: Array<int> = [];
    i = 0;
    while i < 200 { eq.push(5); i = i + 1; }
    qsort(eq, 0, eq.len() - 1);
    print(eq[0]);
    print(eq[199]);
    var inc: Array<int> = [];
    i = 0;
    while i < 200 { inc.push(i); i = i + 1; }
    qsort(inc, 0, inc.len() - 1);
    print(inc[0]);
    print(inc[199]);
    var dec: Array<int> = [];
    i = 0;
    while i < 200 { dec.push(199 - i); i = i + 1; }
    qsort(dec, 0, dec.len() - 1);
    print(dec[0]);
    print(dec[199]);
}
