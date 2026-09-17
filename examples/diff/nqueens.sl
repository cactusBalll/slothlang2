// N-Queens differential test: count solutions for n = 1..11.
func safe(col: Array<int>, row: int, c: int): bool {
    var i = 0;
    while i < row {
        if col[i] == c { return false; }
        let d = row - i;
        if col[i] - c == d or c - col[i] == d { return false; }
        i = i + 1;
    }
    return true;
}

func solve(col: Array<int>, row: int, n: int): int {
    if row == n { return 1; }
    var cnt = 0;
    var c = 0;
    while c < n {
        if safe(col, row, c) {
            col[row] = c;
            cnt = cnt + solve(col, row + 1, n);
        }
        c = c + 1;
    }
    return cnt;
}

func count(n: int): int {
    var col: Array<int> = [];
    var i = 0;
    while i < n { col.push(0); i = i + 1; }
    return solve(col, 0, n);
}

func main(): unit {
    var n = 1;
    while n <= 11 {
        print(count(n));
        n = n + 1;
    }
}
