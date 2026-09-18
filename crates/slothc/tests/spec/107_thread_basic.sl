// spec: TH-P1 threads — spawn/join value + ref payloads, detach, identity
func main(): unit {
    let h: JoinHandle<int> = thread.spawn(|seed: int| -> int {
        var s = 0;
        var i = 0;
        while (i < 100) {
            s = s + i;
            i = i + 1;
        }
        return seed + s;
    }, 1);
    print(h.join()); // expect: 4951

    let hs: JoinHandle<str> = thread.spawn(|s: str| -> str {
        return s + " world";
    }, "hello");
    print(hs.join()); // expect: hello world

    // detach after a channel handshake proves the worker finished
    let done = channel.new<int>(0);
    let hd = thread.spawn(|x: int| -> int {
        done.send(x);
        return x;
    }, 9);
    let v: int? = done.recv();
    print(v ?: -1); // expect: 9
    hd.detach();

    print(thread.current_id() > 0); // expect: true
    thread.yield_now();
    print("thread OK"); // expect: thread OK
}
