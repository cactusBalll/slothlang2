#include <stdio.h>
#include <stdint.h>

static int64_t seed = 24681357;
static int64_t rnd(void) {
    seed = (seed * 1103515245 + 12345) % 2147483648LL;
    return seed;
}

#define NMAX 16

static void matmul(int a[NMAX][NMAX], int b[NMAX][NMAX], int c[NMAX][NMAX], int n) {
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++) {
            int acc = 0;
            for (int k = 0; k < n; k++) acc += a[i][k] * b[k][j];
            c[i][j] = acc;
        }
}

int main(void) {
    for (int c = 0; c < 40; c++) {
        int n = 1 + (int)(rnd() % 12);
        int a[NMAX][NMAX], b[NMAX][NMAX], r[NMAX][NMAX];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++) {
                a[i][j] = (int)(rnd() % 19) - 9;
                b[i][j] = (int)(rnd() % 19) - 9;
            }
        matmul(a, b, r, n);
        printf("%d\n", n);
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                printf("%d\n", r[i][j]);
    }
    return 0;
}
