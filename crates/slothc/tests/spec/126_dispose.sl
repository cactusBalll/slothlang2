// spec: user-class `__dispose__()` death hook. When an object's strong count
// reaches zero the rt runs the hook before the refmask-driven field release
// (so fields are still readable inside it); inherited and generic classes
// resolve the hook like any other method.

class Res {
    var name: str;
    func __init__(n: str) { this.name = n; }
    func __dispose__() {
        print("close " + this.name);
    }
}

class Base {
    var tag: str;
    func __init__(t: str) { this.tag = t; }
    func __dispose__() {
        print("base " + this.tag);
    }
}

class Sub: Base {
    var extra: str;
    func __init__(t: str, e: str) {
        super.__init__(t);
        this.extra = e;
    }
}

class Box<T> {
    var v: T;
    func __init__(v: T) { this.v = v; }
    func __dispose__() {
        print("box");
    }
}

// scope exit runs the hook after the body's own output
func one(): unit {
    let r = Res("db");
    print("in");            // expect: in
}                           // expect: close db

// inherited hook (Sub has no __dispose__ of its own)
func two(): unit {
    let s = Sub("s1", "x");
    print(s.tag);           // expect: s1
}                           // expect: base s1

// generic instance hook
func three(): unit {
    let b = Box<int>(3);
    print(b.v);             // expect: 3
}                           // expect: box

// reassignment disposes the replaced object before the rest of the body
func reassign(): unit {
    var r = Res("a");
    r = Res("b");           // expect: close a
    print("done");          // expect: done
}                           // expect: close b

func main(): unit {
    let base = sloth_rc_live();
    one();
    two();
    three();
    reassign();
    print(sloth_rc_live() == base); // expect: true
}
