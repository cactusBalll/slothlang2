// KMP differential test: random binary text/pattern, report all match positions.
var seed = 12345;
func rnd(): int {
    seed = (seed * 1103515245 + 12345) % 2147483648;
    return seed;
}

func build_lps(pat: Array<int>): Array<int> {
    let m = pat.len();
    var lps: Array<int> = [];
    var i = 0;
    while i < m { lps.push(0); i = i + 1; }
    var len = 0;
    var k = 1;
    while k < m {
        if pat[k] == pat[len] {
            len = len + 1;
            lps[k] = len;
            k = k + 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[k] = 0;
                k = k + 1;
            }
        }
    }
    return lps;
}

func kmp_all(text: Array<int>, pat: Array<int>): Array<int> {
    var res: Array<int> = [];
    let m = pat.len();
    if m == 0 { return res; }
    let lps = build_lps(pat);
    var i = 0;
    var j = 0;
    let n = text.len();
    while i < n {
        if text[i] == pat[j] {
            i = i + 1;
            j = j + 1;
        }
        if j == m {
            res.push(i - j);
            j = lps[j - 1];
        } else {
            if i < n and text[i] != pat[j] {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i = i + 1;
                }
            }
        }
    }
    return res;
}

func main(): unit {
    var c = 0;
    while c < 60 {
        let tlen = rnd() % 41;
        let plen = 1 + rnd() % 6;
        var text: Array<int> = [];
        var i = 0;
        while i < tlen { text.push(rnd() % 2); i = i + 1; }
        var pat: Array<int> = [];
        i = 0;
        while i < plen { pat.push(rnd() % 2); i = i + 1; }
        let pos = kmp_all(text, pat);
        print(pos.len());
        for p in pos { print(p); }
        c = c + 1;
    }
}
