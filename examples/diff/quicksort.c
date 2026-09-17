#include <stdio.h>
#include <stdint.h>

static int64_t seed = 55555555;
static int64_t rnd(void) {
    seed = (seed * 1103515245 + 12345) % 2147483648LL;
    return seed;
}

static int partition(int *a, int lo, int hi) {
    int pivot = a[hi];
    int i = lo - 1;
    for (int j = lo; j < hi; j++) {
        if (a[j] <= pivot) {
            i++;
            int t = a[i]; a[i] = a[j]; a[j] = t;
        }
    }
    int t2 = a[i + 1]; a[i + 1] = a[hi]; a[hi] = t2;
    return i + 1;
}

static void qsort_i(int *a, int lo, int hi) {
    if (lo >= hi) return;
    int p = partition(a, lo, hi);
    qsort_i(a, lo, p - 1);
    qsort_i(a, p + 1, hi);
}

int main(void) {
    for (int c = 0; c < 60; c++) {
        int n = (int)(rnd() % 61);
        int a[64];
        for (int i = 0; i < n; i++) a[i] = (int)(rnd() % 201) - 100;
        qsort_i(a, 0, n - 1);
        printf("%d\n", n);
        for (int i = 0; i < n; i++) printf("%d\n", a[i]);
    }
    return 0;
}
