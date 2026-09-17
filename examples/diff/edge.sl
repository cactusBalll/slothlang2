// Edge-case differential: array growth aliasing, param mutation, closures,
// nested containers, pop, and identity semantics.
func pushn(a: Array<int>, n: int): unit {
    var i = 0;
    while i < n { a.push(i * 3); i = i + 1; }
}
func sum(a: Array<int>): int {
    var s = 0;
    var i = 0;
    while i < a.len() { s = s + a[i]; i = i + 1; }
    return s;
}
func main(): unit {
    var a: Array<int> = [];
    pushn(a, 10);
    print(a.len());
    print(sum(a));
    var b = a;
    b.push(99);
    print(a.len());
    print(a[10]);
    print(sum(b));
    var grid: Array<Array<int>> = [];
    var i = 0;
    while i < 5 {
        var row: Array<int> = [i];
        grid.push(row);
        i = i + 1;
    }
    grid[2].push(77);
    grid[2].push(78);
    print(grid[2].len());
    print(grid[2][0] + grid[2][1] + grid[2][2]);
    print(grid[0].len());
    grid[4].push(1000);
    print(grid[4][1]);
    var acc: Array<int> = [];
    let add = || { acc.push(1); };
    add();
    add();
    add();
    print(acc.len());
    print(sum(acc));
    var s = ["ab", "cd"];
    s.push("ef");
    print(s[2]);
    print(s.len());
    var popped = a.pop();
    print(popped);
    print(a.len());
    var deep: Array<Array<Array<int>>> = [];
    var x: Array<Array<int>> = [];
    var y: Array<int> = [7];
    x.push(y);
    deep.push(x);
    deep[0][0].push(8);
    print(deep[0][0].len());
    print(deep[0][0][1]);
}
