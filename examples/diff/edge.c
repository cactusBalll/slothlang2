#include <stdio.h>
#include <stdlib.h>

typedef struct { int *d; int len, cap; } Vec;
static void push(Vec *v, int x) {
    if (v->len == v->cap) {
        v->cap = v->cap ? v->cap * 2 : 4;
        int *nd = (int *)malloc(sizeof(int) * v->cap);
        for (int i = 0; i < v->len; i++) nd[i] = v->d[i];
        free(v->d);
        v->d = nd;
    }
    v->d[v->len++] = x;
}
static void pushn(Vec *a, int n) { for (int i = 0; i < n; i++) push(a, i * 3); }
static int sum(Vec *a) { int s = 0; for (int i = 0; i < a->len; i++) s += a->d[i]; return s; }

int main(void) {
    Vec a = {0};
    pushn(&a, 10);
    printf("%d\n", a.len);
    printf("%d\n", sum(&a));
    Vec *b = &a;
    push(b, 99);
    printf("%d\n", a.len);
    printf("%d\n", a.d[10]);
    printf("%d\n", sum(b));

    Vec grid[5];
    for (int i = 0; i < 5; i++) { grid[i].d = 0; grid[i].len = grid[i].cap = 0; push(&grid[i], i); }
    push(&grid[2], 77);
    push(&grid[2], 78);
    printf("%d\n", grid[2].len);
    printf("%d\n", grid[2].d[0] + grid[2].d[1] + grid[2].d[2]);
    printf("%d\n", grid[0].len);
    push(&grid[4], 1000);
    printf("%d\n", grid[4].d[1]);

    Vec acc = {0};
    for (int k = 0; k < 3; k++) push(&acc, 1);
    printf("%d\n", acc.len);
    printf("%d\n", sum(&acc));

    char *s[4] = {"ab", "cd"};
    s[2] = "ef";
    printf("%s\n", s[2]);
    printf("%d\n", 3);

    int popped = a.d[--a.len];
    printf("%d\n", popped);
    printf("%d\n", a.len);

    Vec y = {0}; push(&y, 7);
    push(&y, 8);
    printf("%d\n", y.len);
    printf("%d\n", y.d[1]);
    return 0;
}
