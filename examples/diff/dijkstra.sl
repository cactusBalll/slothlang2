// Dijkstra differential test: random undirected weighted graphs, all-pairs.
var seed = 987654321;
func rnd(): int {
    seed = (seed * 1103515245 + 12345) % 2147483648;
    return seed;
}

func dijkstra(adj: Array<Array<int>>, src: int): Array<int> {
    let n = adj.len();
    let INF = 1000000000;
    var dist: Array<int> = [];
    var done: Array<int> = [];
    var i = 0;
    while i < n { dist.push(INF); done.push(0); i = i + 1; }
    dist[src] = 0;
    var iter = 0;
    while iter < n {
        var u = -1;
        var best = INF;
        var v = 0;
        while v < n {
            if done[v] == 0 and dist[v] < best {
                best = dist[v];
                u = v;
            }
            v = v + 1;
        }
        if u < 0 { break; }
        done[u] = 1;
        v = 0;
        while v < n {
            if adj[u][v] > 0 {
                let nd = dist[u] + adj[u][v];
                if nd < dist[v] { dist[v] = nd; }
            }
            v = v + 1;
        }
        iter = iter + 1;
    }
    return dist;
}

func main(): unit {
    let INF = 1000000000;
    var c = 0;
    while c < 40 {
        let n = 2 + rnd() % 6;
        var adj: Array<Array<int>> = [];
        var i = 0;
        while i < n {
            var row: Array<int> = [];
            var j = 0;
            while j < n { row.push(0); j = j + 1; }
            adj.push(row);
            i = i + 1;
        }
        i = 0;
        while i < n {
            var j = i + 1;
            while j < n {
                if rnd() % 2 == 0 {
                    let w = 1 + rnd() % 9;
                    adj[i][j] = w;
                    adj[j][i] = w;
                }
                j = j + 1;
            }
            i = i + 1;
        }
        print(n);
        var s = 0;
        while s < n {
            let d = dijkstra(adj, s);
            var k = 0;
            while k < n {
                if d[k] >= INF { print(0 - 1); } else { print(d[k]); }
                k = k + 1;
            }
            s = s + 1;
        }
        c = c + 1;
    }
}
