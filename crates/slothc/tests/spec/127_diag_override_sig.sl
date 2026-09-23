// spec: diagnostics — a subclass override must keep the ancestor's ABI
// (arity + word kinds); base-typed virtual dispatch reuses the ancestor ABI
class Base {
    func f(x: int): int {
        return x;
    }
}
class SubArity: Base {
    func f(x: int, y: int): int {
        return x;
    }
}
class Base2 {
    func g(x: float): float {
        return x;
    }
}
class SubAbi: Base2 {
    func g(x: int): int {
        return x;
    }
}
func main(): unit {
    print(0);
}
// diag: arity want 1, got 2
// diag: param 1: ABI word mismatch
