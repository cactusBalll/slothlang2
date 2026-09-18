/* Reference matvec kernel for the TE-P2 benchmark (design §8.2). */
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    const int DIM = 512;
    const int ITERS = 300;
    double *w = calloc((size_t)DIM * DIM, sizeof(double));
    double *x = calloc(DIM, sizeof(double));
    double *y = calloc(DIM, sizeof(double));
    for (int it = 0; it < ITERS; it++) {
        for (int i = 0; i < DIM; i++) {
            double acc = 0.0;
            for (int k = 0; k < DIM; k++) {
                acc += w[i * DIM + k] * x[k];
            }
            y[i] = acc;
        }
    }
    printf("%f\n", y[0]);
    return 0;
}
