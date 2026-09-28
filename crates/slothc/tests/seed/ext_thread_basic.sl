// ext: thread basics — spawn/join with value and `str` payloads, result
// ownership, detach after a channel handshake, current_id, yield_now.

func sum_to(n: int): int {
    var s = 0;
    var i = 0;
    while i < n {
        s = s + i;
        i = i + 1;
    }
    return s;
}

func main(): unit {
    let h: JoinHandle<int> = thread.spawn(|seed: int| -> int {
        return seed + sum_to(100);
    }, 1);
    print(h.join());

    let hs: JoinHandle<str> = thread.spawn(|s: str| -> str {
        return s + " world";
    }, "hello");
    print(hs.join());

    // inferred handle type
    let hi = thread.spawn(|seed: int| -> int {
        return seed * 3;
    }, 7);
    print(hi.join());

    // detach once a channel handshake proves the worker has run
    let done = channel.new<int>(0);
    let hd = thread.spawn(|x: int| -> int {
        done.send(x);
        return x;
    }, 9);
    let v: int? = done.recv();
    print(v ?: -1);
    hd.detach();

    let id0 = thread.current_id();
    print(id0 > 0);
    let hw: JoinHandle<int> = thread.spawn(|seed: int| -> int {
        return thread.current_id();
    }, 0);
    let idw = hw.join();
    print(idw != id0);
    thread.yield_now();
    print("thread OK");
    // expect: 4951
    // expect: hello world
    // expect: 21
    // expect: 9
    // expect: true
    // expect: true
    // expect: thread OK
}
