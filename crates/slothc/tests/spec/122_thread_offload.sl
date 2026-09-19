// spec: compute offload (IO) — a fiber runs CPU-bound work on a fresh OS
// thread and suspends until it finishes, while the event loop keeps serving
// the other fibers. Exercises an imported generic stdlib function
// (`event.run_blocking`) over every available backend.

import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";

func heavy(seed: int): int {
    var x = seed;
    var i = 0;
    while i < 200000 {
        x = x * 1103515245 + 12345;
        i = i + 1;
    }
    return x;
}

func offload_once(kind: int): unit {
    let loop = EventLoop(kind);
    loop.spawn(|w: Wait| -> unit {
        let r = event.run_blocking(loop, heavy, 7);
        if r == heavy(7) {
            print("offload result ok");
        } else {
            print("offload result bad");
        }
    });
    loop.spawn(|w: Wait| -> unit {
        event.sleep(loop, 1);
        print("loop progressed");
    });
    loop.run();
    loop.close();
}

func main(): unit {
    var kinds: Array<int> = [kind_select(), kind_poll(), kind_epoll()];
    if backend_available(kind_iouring()) {
        kinds.push(kind_iouring());
    }

    var i = 0;
    while i < kinds.len() {
        offload_once(kinds[i]);
        i = i + 1;
    }
    print("thread offload OK");
    // expect: offload result ok
    // expect: loop progressed
    // expect: thread offload OK
}
