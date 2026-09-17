// Quicksort differential test: random int arrays incl. duplicates/negatives.
var seed = 55555555;
func rnd(): int {
    seed = (seed * 1103515245 + 12345) % 2147483648;
    return seed;
}

func partition(a: Array<int>, lo: int, hi: int): int {
    let pivot = a[hi];
    var i = lo - 1;
    var j = lo;
    while j < hi {
        if a[j] <= pivot {
            i = i + 1;
            let t = a[i];
            a[i] = a[j];
            a[j] = t;
        }
        j = j + 1;
    }
    let t2 = a[i + 1];
    a[i + 1] = a[hi];
    a[hi] = t2;
    return i + 1;
}

func qsort(a: Array<int>, lo: int, hi: int): unit {
    if lo >= hi { return; }
    let p = partition(a, lo, hi);
    qsort(a, lo, p - 1);
    qsort(a, p + 1, hi);
}

func main(): unit {
    var c = 0;
    while c < 60 {
        let n = rnd() % 61;
        var a: Array<int> = [];
        var i = 0;
        while i < n { a.push(rnd() % 201 - 100); i = i + 1; }
        qsort(a, 0, n - 1);
        print(n);
        i = 0;
        while i < n { print(a[i]); i = i + 1; }
        c = c + 1;
    }
}
