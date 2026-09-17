#include <stdio.h>
#include <stdint.h>

static int64_t seed = 12345;
static int64_t rnd(void) {
    seed = (seed * 1103515245 + 12345) % 2147483648LL;
    return seed;
}

static void build_lps(const int *pat, int m, int *lps) {
    for (int i = 0; i < m; i++) lps[i] = 0;
    int len = 0, k = 1;
    while (k < m) {
        if (pat[k] == pat[len]) {
            len++;
            lps[k] = len;
            k++;
        } else {
            if (len != 0) {
                len = lps[len - 1];
            } else {
                lps[k] = 0;
                k++;
            }
        }
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
        if (j == m) {
            res[cnt++] = i - j;
            j = lps[j - 1];
        } else if (i < n && text[i] != pat[j]) {
            if (j != 0) j = lps[j - 1];
            else i++;
        }
    }
    return cnt;
}

int main(void) {
    for (int c = 0; c < 60; c++) {
        int tlen = (int)(rnd() % 41);
        int plen = 1 + (int)(rnd() % 6);
        int text[64], pat[64], res[64];
        for (int i = 0; i < tlen; i++) text[i] = (int)(rnd() % 2);
        for (int i = 0; i < plen; i++) pat[i] = (int)(rnd() % 2);
        int cnt = kmp_all(text, tlen, pat, plen, res);
        printf("%d\n", cnt);
        for (int i = 0; i < cnt; i++) printf("%d\n", res[i]);
    }
    return 0;
}
