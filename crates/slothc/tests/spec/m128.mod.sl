// spec: exported base class + virtual surface sibling for 128 (VD-P1; not run directly)
pub class Animal {
    func __init__() {
        return;
    }
    func kind(): str {
        return "animal";
    }
    func describe(): str {
        return this.kind();
    }
}
pub func newborn(): Animal {
    let a = Animal();
    return a;
}
pub func describe_animal(): str {
    let a = Animal();
    return a.describe();
}
