// spec: IO event queue (IO-P3) — the sloth EventLoop reactor drives timers and
// TCP echo across stackful fibers, on every available backend.

import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";

func timer_order(kind: int): unit {
    let loop = EventLoop(kind);
    loop.spawn(|w: Wait| -> unit {
        event.sleep(loop, 25);
        print("late");
    });
    loop.spawn(|w: Wait| -> unit {
        event.sleep(loop, 5);
        print("early");
    });
    loop.run();
    loop.close();
}

func echo_once(kind: int): unit {
    let loop = EventLoop(kind);
    let lst = tcp_listen("127.0.0.1", 0, 64, false);
    let port = lst.port_num();

    loop.spawn(|w: Wait| -> unit {
        let fd = accept_blocking(loop, lst);
        let s = stream_from_fd(fd);
        let b = io.bytes_new(64);
        let n = read_some(loop, s, b, 0, 64);
        if n > 0 {
            write_all(loop, s, b, 0, n);
        }
        s.close();
        io.bytes_free(b);
    });

    loop.spawn(|w: Wait| -> unit {
        let s = tcp_connect("127.0.0.1", port);
        await_writable(loop, s.raw_fd());
        write_str_all(loop, s, "ping");
        let b = io.bytes_new(64);
        var got = 0;
        while got < 4 {
            let r = read_some(loop, s, b, got, 64 - got);
            if r <= 0 {
                break;
            }
            got = got + r;
        }
        print("echo: " + io.bytes_to_str(b, 0, got));
        s.close();
        io.bytes_free(b);
    });

    loop.run();
    lst.close();
    loop.close();
}

func main(): unit {
    var kinds: Array<int> = [kind_select(), kind_poll(), kind_epoll()];
    if backend_available(kind_iouring()) {
        kinds.push(kind_iouring());
    }

    var i = 0;
    while i < kinds.len() {
        let k = kinds[i];
        timer_order(k);
        echo_once(k);
        i = i + 1;
    }
    print("event queue OK");
    // expect: early
    // expect: late
    // expect: echo: ping
    // expect: event queue OK
}
