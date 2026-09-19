// examples/net/event_backends.sl
//
// The I/O event queue across every backend: prints availability/names, then
// drives two sleeping fibers per backend to show the timer list in action.

import "sloth/io.slt";
import "sloth/event.slt";

func timer_order(kind: int): unit {
    let loop = EventLoop(kind);
    loop.spawn(|w: Wait| -> unit {
        event.sleep(loop, 30);
        print("  late");
    });
    loop.spawn(|w: Wait| -> unit {
        event.sleep(loop, 5);
        print("  early");
    });
    loop.run();
    loop.close();
}

func main(): unit {
    var kinds: Array<int> = [kind_select(), kind_poll(), kind_epoll(), kind_kqueue()];
    if backend_available(kind_iouring()) {
        kinds.push(kind_iouring());
    }
    var i = 0;
    while i < kinds.len() {
        let k = kinds[i];
        if backend_available(k) {
            print("backend: ${backend_name(k)} (available)");
            timer_order(k);
        } else {
            print("backend: ${backend_name(k)} (unavailable)");
        }
        i = i + 1;
    }
    print("event backends OK");
}
