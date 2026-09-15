// spec: optionals — is not nil narrowing, ?: fallback, Opt params (§3.6)
class Shape {
    func sides(): int {
        return 4;
    }
}
func report(p: Shape?): unit {
    if p is not nil {
        print(p.sides());       // expect: 4
    } else {
        print(0);               // expect: 0
    }
}
func pick(o: int?): int {
    return o ?: 7;
}
func main(): unit {
    report(nil);
    report(Shape());
    print(pick(nil));           // expect: 7
    print(pick(3));             // expect: 3
}

