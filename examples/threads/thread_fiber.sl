// TH-P3: thread + coroutine composition (design Appendix B). Each of 4 OS
// threads owns an independent fiber set (thread-confined); each fiber runs a
// cooperative task and delivers its result across threads through a Channel.
// N threads x m fibers = the 1:m x N concurrency shape of §5.4.

func main(): unit {
    let results = channel.new<int>(0);
    var workers: Array<JoinHandle<int>> = [];
    for (var t: 0..4) {
        workers.push(thread.spawn(|tid: int| -> int {
            var fibers: Array<Fiber<int>> = [];
            for (var k: 0..8) {
                fibers.push(fiber.create(|init: int| -> unit {
                    var acc = init;
                    var i = 0;
                    while (i < 100) {
                        acc = acc + i;
                        i = i + 1;
                        fiber.yield(acc);
                    }
                    results.send(acc);
                }, tid * 1000 + k));
            }
            // drive this thread's fiber set to completion
            for (var k: 0..8) {
                let f = fibers[k];
                while (fiber.resumable(f)) {
                    fiber.resume(f, 0);
                }
            }
            return 0;
        }, t));
    }
    for (var h: workers) {
        h.join();
    }

    var total = 0;
    var n = 0;
    while (n < 32) {
        let v: int? = results.recv();
        total = total + (v ?: 0);
        n = n + 1;
    }
    print(total);
    print("thread+fiber OK");
}
