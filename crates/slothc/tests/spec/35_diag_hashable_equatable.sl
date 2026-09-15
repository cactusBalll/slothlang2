// spec: diagnostics — Hashable⇒Equatable contract (patch #31, design §2.5)
trait Hashable {
    func hashKey(): int;
}
class Key impl Hashable {
    var x: int = 1;
    func __init__() {
        return;
    }
    func hashKey(): int {
        return this.x;
    }
}
func main(): unit {
    var m = @(Key(): 3);
    print(m.len());
}
// diag: class `Key` impl `Hashable` must also impl `Equatable` (hash equality implies value equality)
