#include <stdio.h>
#include <stdint.h>

static int64_t seed = 13572468;
static int64_t rnd(void) {
    seed = (seed * 1103515245 + 12345) % 2147483648LL;
    return seed;
}

#define NMAX 40
static double a[NMAX][NMAX], b[NMAX][NMAX], r[NMAX][NMAX];

int main(void) {
    for (int c = 0; c < 4; c++) {
        int n = 8 + (int)(rnd() % 28);
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++) {
                a[i][j] = (double)(rnd() % 1000000) / 1000000.0 - 0.5;
                b[i][j] = (double)(rnd() % 1000000) / 1000000.0 - 0.5;
            }
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++) {
                double acc = 0.0;
                for (int k = 0; k < n; k++) acc += a[i][k] * b[k][j];
                r[i][j] = acc;
            }
        printf("%d\n", n);
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                printf("%lld\n", (long long)(r[i][j] * 1000000000000000.0));
    }
    return 0;
}
