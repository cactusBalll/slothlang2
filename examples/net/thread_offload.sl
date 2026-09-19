// examples/net/thread_offload.sl
//
// A fiber offloads CPU-bound work to a fresh OS thread and suspends until it
// finishes (`event.run_blocking`), so the event loop keeps serving other
// fibers while the computation runs. Two fibers make the concurrency visible:
// the timer fiber prints while the compute thread is busy.

import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";

func heavy(rounds: int): int {
    var x = 1;
    var i = 0;
    while i < rounds {
        x = (x * 1103515245 + 12345) % 2147483647;
        i = i + 1;
    }
    return x;
}

func main(): unit {
    let loop = EventLoop(kind_epoll());

    loop.spawn(|w: Wait| -> unit {
        let r = event.run_blocking(loop, heavy, 4000000);
        print("compute result: ${r}");
    });
    loop.spawn(|w: Wait| -> unit {
        event.sleep(loop, 1);
        print("loop still serving");
    });

    loop.run();
    loop.close();
    print("compute offload OK");
}
