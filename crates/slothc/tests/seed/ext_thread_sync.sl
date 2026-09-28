// ext: Mutex + AtomicInt surfaces, including cross-thread try_lock and exact
// contention counts.

func atomic_surface(): unit {
    let a = atomic.new(0);
    print("load0=${a.load()}");
    a.add(5);
    print("add=${a.load()}");
    a.sub(2);
    print("sub=${a.load()}");
    a.store(9);
    print("store=${a.load()}");
    print("cas1=${a.cas(9, 10)}");
    print("cas2=${a.cas(9, 11)}");
    print("after=${a.load()}");
}

func mutex_surface(): unit {
    let m = mutex.new();
    print("try=${m.try_lock()}");
    m.unlock();
    m.lock();
    m.unlock();
    let b = atomic.new(0);
    m.with(|g: unit| -> unit {
        b.add(100);
    });
    print("with=${b.load()}");
}

func try_lock_held_by_other(): unit {
    let m = mutex.new();
    let go = channel.new<int>(0);
    let release = channel.new<int>(0);
    let h: JoinHandle<int> = thread.spawn(|x: int| -> int {
        m.lock();
        go.send(1);
        let r: int? = release.recv();
        m.unlock();
        return 0;
    }, 0);
    let g: int? = go.recv();
    print("worker holding=${g ?: -1}");
    print("try while held=${m.try_lock()}");
    release.send(1);
    h.join();
    print("try after release=${m.try_lock()}");
    m.unlock();
}

func atomic_contention(): unit {
    let a = atomic.new(0);
    let n = 4;
    let per = 50000;
    var hs: Array<JoinHandle<int>> = [];
    var i = 0;
    while i < n {
        hs.push(thread.spawn(|seed: int| -> int {
            var k = 0;
            while k < per {
                a.add(1);
                k = k + 1;
            }
            return 0;
        }, i));
        i = i + 1;
    }
    for h in hs { h.join(); }
    print("exact=${a.load() == n * per}");
}

func main(): unit {
    atomic_surface();
    mutex_surface();
    try_lock_held_by_other();
    atomic_contention();
    // expect: load0=0
    // expect: add=5
    // expect: sub=3
    // expect: store=9
    // expect: cas1=true
    // expect: cas2=false
    // expect: after=10
    // expect: try=true
    // expect: with=100
    // expect: worker holding=1
    // expect: try while held=false
    // expect: try after release=true
    // expect: exact=true
}
