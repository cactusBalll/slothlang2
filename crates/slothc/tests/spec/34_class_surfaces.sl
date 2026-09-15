// spec: classes syntheses & surfaces — vtable/Display/hash/keyed (§2.5 extras)
trait Display {
    func to_str(): str;
}
trait Hashable {
    func __hash__(): int;
}
trait Equatable {
    func __eq__(other: Key): bool;
}
class Key impl Display, Equatable, Hashable {
    var x: int;
    func __init__(x: int) {
        this.x = x;
    }
    func to_str(): str {
        return "K${this.x}";
    }
    func __hash__(): int {
        return this.x;
    }
    func __eq__(other: Key): bool {
        return this.x == other.x;
    }
}
func main(): unit {
    let k = Key(3);
    print(k);                    // expect: K3
    print("${k}");               // expect: K3
    var m = @(Key(1): 10, Key(2): 20);
    print(len(m));               // expect: 2
    m[Key(3)] = 30;              // content hash: fresh equal Key(3) hits Key(3)'s slot
    print(len(m));               // expect: 3
    print(m[Key(3)]);            // expect: 30
    var s = 0;
    for (var e: m) {
        s = s + e.val;
    }
    print(s);                    // expect: 60
}

