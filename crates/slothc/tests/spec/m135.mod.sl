// spec: helper module for 135 — exported generic functions that return
// `Array<T>` / `T?` and delegate to a private generic helper. Prior to the
// cross-module generic fix, an unqualified call to these from the root called
// a template emitted with unresolved type params, corrupting ref-element ARC.
import "__sloth";

// private generic helper (recursive), like a stdlib quicksort
func qsort_priv<T>(a: Array<T>, lo: int, hi: int, cmp: (T, T) -> int): unit {
    if lo >= hi {
        return;
    }
    let pivot = a[(lo + hi) / 2];
    var i = lo;
    var j = hi;
    while i <= j {
        while cmp(a[i], pivot) < 0 {
            i = i + 1;
        }
        while cmp(a[j], pivot) > 0 {
            j = j - 1;
        }
        if i <= j {
            let t = a[i];
            a[i] = a[j];
            a[j] = t;
            i = i + 1;
            j = j - 1;
        }
    }
    if lo < j {
        qsort_priv(a, lo, j, cmp);
    }
    if i < hi {
        qsort_priv(a, i, hi, cmp);
    }
}

/// fresh single-element array (cross-module `Array<T>` return)
pub func wrapped<T>(x: T): Array<T> {
    return [x];
}

/// sorted copy of an int/float/Comparable array
pub func sorted<T: Comparable>(a: Array<T>): Array<T> {
    var c = a[0..a.len()];
    var i = 1;
    while i < c.len() {
        let key = c[i];
        var j = i - 1;
        while j >= 0 and c[j] > key {
            c[j + 1] = c[j];
            j = j - 1;
        }
        c[j + 1] = key;
        i = i + 1;
    }
    return c;
}

/// sorted copy using the private recursive helper + caller comparator
pub func sorted_by<T>(a: Array<T>, cmp: (T, T) -> int): Array<T> {
    var c = a[0..a.len()];
    qsort_priv(c, 0, c.len() - 1, cmp);
    return c;
}

/// first element or nil (cross-module `T?` return)
pub func head<T>(a: Array<T>): T? {
    if a.len() == 0 {
        return nil;
    }
    return a[0];
}
