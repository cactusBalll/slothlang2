// spec: diagnostics — subclass ctor must call super.__init__ (patch #25)
class Animal {
    func __init__() {
        return;
    }
}
class Dog: Animal {
    func __init__() {
        return;
    }
}
func main(): unit {
    let d = Dog();
    print(1);
}
// diag: constructor of `Dog` must call super.__init__
