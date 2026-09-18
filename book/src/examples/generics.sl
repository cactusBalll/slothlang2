func first<T>(xs: Array<T>): T {
    return xs[0];
}

func twice<T>(x: T): Array<T> {
    return [x, x];
}

class Box<T> {
    var v: T;
    func set(x: T) { this.v = x; }
    func get(): T { return this.v; }
}

func main() {
    var a = [10, 20];
    print(first(a));         // 10  T=int 实例
    var s = ["x", "y"];
    print(first(s));         // x   T=str 实例
    print(first<int>(a));    // 10  显式类型实参
    let b = twice("z");
    print(b[0] + b[1]);      // zz
    let bi = Box<int>();
    bi.set(7);
    print(bi.get());         // 7
    let bf = Box<float>();
    bf.set(1.5);
    print(bf.get());         // 1.5
}
