#include <stdio.h>

static void build_lps(const int *pat, int m, int *lps) {
    for (int i = 0; i < m; i++) lps[i] = 0;
    int len = 0, k = 1;
    while (k < m) {
        if (pat[k] == pat[len]) { len++; lps[k] = len; k++; }
        else if (len != 0) len = lps[len - 1];
        else { lps[k] = 0; k++; }
    }
}
static int kmp_all(const int *text, int n, const int *pat, int m, int *res) {
    int cnt = 0;
    if (m == 0) return 0;
    int lps[64];
    build_lps(pat, m, lps);
    int i = 0, j = 0;
    while (i < n) {
        if (text[i] == pat[j]) { i++; j++; }
        if (j == m) { res[cnt++] = i - j; j = lps[j - 1]; }
        else if (i < n && text[i] != pat[j]) { if (j != 0) j = lps[j - 1]; else i++; }
    }
    return cnt;
}
static void mm(int a[4][4], int ar, int ac, int b[4][4], int br, int bc, int c[4][4]) {
    for (int i = 0; i < ar; i++)
        for (int j = 0; j < bc; j++) {
            int acc = 0;
            for (int k = 0; k < ac; k++) acc += a[i][k] * b[k][j];
            c[i][j] = acc;
        }
}
#define INF 1000000000
static void dijkstra(int adj[4][4], int n, int src, int *dist) {
    int done[4];
    for (int i = 0; i < n; i++) { dist[i] = INF; done[i] = 0; }
    dist[src] = 0;
    for (int it = 0; it < n; it++) {
        int u = -1, best = INF;
        for (int v = 0; v < n; v++) if (!done[v] && dist[v] < best) { best = dist[v]; u = v; }
        if (u < 0) break;
        done[u] = 1;
        for (int v = 0; v < n; v++)
            if (adj[u][v] >= 0 && dist[u] + adj[u][v] < dist[v]) dist[v] = dist[u] + adj[u][v];
    }
}
static int partition(int *a, int lo, int hi) {
    int pivot = a[hi], i = lo - 1;
    for (int j = lo; j < hi; j++) if (a[j] <= pivot) { i++; int t = a[i]; a[i] = a[j]; a[j] = t; }
    int t2 = a[i + 1]; a[i + 1] = a[hi]; a[hi] = t2;
    return i + 1;
}
static void qsort_i(int *a, int lo, int hi) {
    if (lo >= hi) return;
    int p = partition(a, lo, hi);
    qsort_i(a, lo, p - 1);
    qsort_i(a, p + 1, hi);
}
static int nq(int *col, int row, int n) {
    if (row == n) return 1;
    int cnt = 0;
    for (int c = 0; c < n; c++) {
        int ok = 1;
        for (int i = 0; i < row; i++) {
            if (col[i] == c) ok = 0;
            int d = row - i;
            if (col[i] - c == d || c - col[i] == d) ok = 0;
        }
        if (ok) { col[row] = c; cnt += nq(col, row + 1, n); }
    }
    return cnt;
}
static int count(int n) { int col[16]; for (int i = 0; i < n; i++) col[i] = 0; return nq(col, 0, n); }

int main(void) {
    int text[10]; for (int i = 0; i < 10; i++) text[i] = 0;
    int p3[3] = {0, 0, 0}, res[64];
    int cnt = kmp_all(text, 10, p3, 3, res);
    printf("%d\n", cnt);
    for (int i = 0; i < cnt; i++) printf("%d\n", res[i]);
    printf("%d\n", kmp_all(text, 10, p3, 0, res));
    int big[11]; for (int i = 0; i < 11; i++) big[i] = 0;
    printf("%d\n", kmp_all(text, 10, big, 11, res));

    int a2[4][4] = {{1, 2, 3, 0}, {4, 5, 6, 0}};
    int b2[4][4] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}};
    int c2[4][4];
    mm(a2, 2, 3, b2, 3, 4, c2);
    printf("%d\n", 2);
    printf("%d\n", 4);
    for (int i = 0; i < 2; i++)
        for (int j = 0; j < 4; j++) printf("%d\n", c2[i][j]);

    int g[4][4];
    for (int i = 0; i < 4; i++) for (int j = 0; j < 4; j++) g[i][j] = -1;
    g[0][1] = 0; g[1][0] = 0; g[1][2] = 0; g[2][1] = 0;
    g[2][3] = 5; g[3][2] = 5; g[0][3] = 10; g[3][0] = 10;
    int d[4];
    dijkstra(g, 4, 0, d);
    for (int i = 0; i < 4; i++) printf("%d\n", d[i]);

    printf("%d\n", count(0));
    printf("%d\n", count(12));

    int eq[200]; for (int i = 0; i < 200; i++) eq[i] = 5;
    qsort_i(eq, 0, 199);
    printf("%d\n", eq[0]); printf("%d\n", eq[199]);
    int inc[200]; for (int i = 0; i < 200; i++) inc[i] = i;
    qsort_i(inc, 0, 199);
    printf("%d\n", inc[0]); printf("%d\n", inc[199]);
    int dec[200]; for (int i = 0; i < 200; i++) dec[i] = 199 - i;
    qsort_i(dec, 0, 199);
    printf("%d\n", dec[0]); printf("%d\n", dec[199]);
    return 0;
}
