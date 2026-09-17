#include <stdio.h>
int main(void) {
    printf("%.17g\n", 1.0 / 3.0 * 3.0);
    printf("%.17g\n", 1.0 / 7.0 * 7.0);
    double s = 0.0;
    for (int i = 1; i <= 1000; i++) s += 1.0 / (double)i;
    printf("%.17g\n", s);
    double p = 1.0;
    for (int i = 1; i <= 50; i++) p *= 1.0001;
    printf("%.17g\n", p);
    return 0;
}
