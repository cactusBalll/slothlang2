// spec: nested containers — arrays of arrays, maps of maps, dynamic keys
func main(): unit {
    var grid = [[1, 2], [3, 4]];
    print(grid[0][1]);        // expect: 2
    print(grid[1][0]);        // expect: 3
    var row = grid[0];
    row[0] = 9;
    print(grid[0][0]);        // expect: 9
    var mm = @("a": @("x": 1));
    print(mm["a"]["x"]);      // expect: 1
    var names = ["a", "b"];
    var lookup: Map<str, int> = @();
    lookup[names[0]] = 10;
    print(lookup["a"]);       // expect: 10
    var arrs: Array<Array<int>> = [[7], [8, 9]];
    print(arrs[1][1]);        // expect: 9
    var fm = [[1.5], [2.5, 3.5]];
    print(fm[1][1]);          // expect: 3.5
    print(grid[0].len() + grid[1].len()); // expect: 4
}
