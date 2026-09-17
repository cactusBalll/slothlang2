#include <stdio.h>
#include <stdint.h>

static int64_t seed = 987654321;
static int64_t rnd(void) {
    seed = (seed * 1103515245 + 12345) % 2147483648LL;
    return seed;
}

#define NMAX 16
#define INF 1000000000

static void dijkstra(int adj[NMAX][NMAX], int n, int src, int *dist) {
    int done[NMAX];
    for (int i = 0; i < n; i++) { dist[i] = INF; done[i] = 0; }
    dist[src] = 0;
    for (int iter = 0; iter < n; iter++) {
        int u = -1, best = INF;
        for (int v = 0; v < n; v++)
            if (!done[v] && dist[v] < best) { best = dist[v]; u = v; }
        if (u < 0) break;
        done[u] = 1;
        for (int v = 0; v < n; v++)
            if (adj[u][v] > 0 && dist[u] + adj[u][v] < dist[v])
                dist[v] = dist[u] + adj[u][v];
    }
}

int main(void) {
    for (int c = 0; c < 40; c++) {
        int n = 2 + (int)(rnd() % 6);
        int adj[NMAX][NMAX] = {{0}};
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++)
                if (rnd() % 2 == 0) {
                    int w = 1 + (int)(rnd() % 9);
                    adj[i][j] = w;
                    adj[j][i] = w;
                }
        printf("%d\n", n);
        for (int s = 0; s < n; s++) {
            int dist[NMAX];
            dijkstra(adj, n, s, dist);
            for (int k = 0; k < n; k++) printf("%d\n", dist[k] >= INF ? -1 : dist[k]);
        }
    }
    return 0;
}
