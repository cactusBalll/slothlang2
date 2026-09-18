class Animal {
    var tag: int;
    func __init__(t: int) { this.tag = t; }
    func who(): str { return "animal"; }
}

class Dog: Animal {
    var loud: int;
    func __init__(l: int) {
        super.__init__(7);       // 子类构造器必须显式调用
        this.loud = l;
    }
    func who(): str { return "dog"; }
}

func main() {
    let d = Dog(3);
    print(d.tag);                // 7  继承字段
    print(d.who());              // dog

    let a: Animal = d;           // 上转
    print(a.who());              // dog  按运行时类型虚分派

    if a is Dog {                // is 收窄
        print(a.loud);           // 3
    }

    // 数组字面量求最近公共祖先 -> Array<Animal>
    let xs = [Animal(1), Dog(2)];
    print(xs.len());             // 2
    print(xs[1].who());          // dog
}
