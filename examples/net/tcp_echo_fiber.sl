// examples/net/tcp_echo_fiber.sl
//
// TCP echo server built on the fiber event loop, exercised on every available
// backend (select / poll / epoll / io_uring). Each backend runs a server fiber
// (accept + echo) and a client fiber on one EventLoop; the process exits when
// both finish, so this is CI-friendly.

import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";

func echo_roundtrip(kind: int): bool {
    let loop = EventLoop(kind);
    let lst = tcp_listen("127.0.0.1", 0, 64, false);
    let port = lst.port_num();

    loop.spawn(|w: Wait| -> unit {
        let fd = accept_blocking(loop, lst);
        let s = stream_from_fd(fd);
        let b = io.bytes_new(256);
        let n = read_some(loop, s, b, 0, 256);
        if n > 0 {
            write_all(loop, s, b, 0, n);
        }
        s.close();
        io.bytes_free(b);
    });

    var hit: Array<int> = [0];
    loop.spawn(|w: Wait| -> unit {
        let s = tcp_connect("127.0.0.1", port);
        await_writable(loop, s.raw_fd());
        write_str_all(loop, s, "hello over ${backend_name(kind)}");
        let b = io.bytes_new(256);
        var got = 0;
        while true {
            let r = read_some(loop, s, b, got, 256 - got);
            if r <= 0 {
                break;
            }
            got = got + r;
        }
        if io.bytes_to_str(b, 0, got) == "hello over ${backend_name(kind)}" {
            hit[0] = 1;
        }
        s.close();
        io.bytes_free(b);
    });

    loop.run();
    lst.close();
    loop.close();
    return hit[0] == 1;
}

func main(): unit {
    var kinds: Array<int> = [kind_select(), kind_poll(), kind_epoll()];
    if backend_available(kind_iouring()) {
        kinds.push(kind_iouring());
    }
    var i = 0;
    var pass = 0;
    while i < kinds.len() {
        let k = kinds[i];
        if echo_roundtrip(k) {
            print("${backend_name(k)}: echo ok");
            pass = pass + 1;
        } else {
            print("${backend_name(k)}: echo FAIL");
        }
        i = i + 1;
    }
    if pass == kinds.len() {
        print("tcp echo (fiber) OK");
    } else {
        print("tcp echo (fiber) FAIL");
    }
}
