#include <stdio.h>

static int safe(const int *col, int row, int c) {
    for (int i = 0; i < row; i++) {
        if (col[i] == c) return 0;
        int d = row - i;
        if (col[i] - c == d || c - col[i] == d) return 0;
    }
    return 1;
}

static int solve(int *col, int row, int n) {
    if (row == n) return 1;
    int cnt = 0;
    for (int c = 0; c < n; c++) {
        if (safe(col, row, c)) {
            col[row] = c;
            cnt += solve(col, row + 1, n);
        }
    }
    return cnt;
}

static int count(int n) {
    int col[16];
    for (int i = 0; i < n; i++) col[i] = 0;
    return solve(col, 0, n);
}

int main(void) {
    for (int n = 1; n <= 11; n++) printf("%d\n", count(n));
    return 0;
}
