// spec: TH-P2 Mutex + AtomicInt surfaces
func main(): unit {
    let a = atomic.new(0);
    print(a.load()); // expect: 0
    a.add(5);
    print(a.load()); // expect: 5
    a.sub(2);
    print(a.load()); // expect: 3
    a.store(9);
    print(a.load()); // expect: 9
    print(a.cas(9, 10)); // expect: true
    print(a.cas(9, 11)); // expect: false
    print(a.load()); // expect: 10

    let m = mutex.new();
    print(m.try_lock()); // expect: true
    m.unlock();
    m.lock();
    a.add(1);
    m.unlock();
    print(a.load()); // expect: 11

    let b = atomic.new(0);
    m.with(|g: unit| -> unit {
        b.add(100);
    });
    print(b.load()); // expect: 100
    print("sync OK"); // expect: sync OK
}
