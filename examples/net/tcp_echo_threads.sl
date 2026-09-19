// examples/net/tcp_echo_threads.sl
//
// Blocking thread-per-connection TCP echo: a server thread accepts N
// connections and spawns one handler thread per connection; the main thread
// runs the client. Demonstrates the `thread.*` model as a contrast to the
// fiber event loop.

import "sloth/io.slt";
import "sloth/net.slt";

func handle_conn(s: TcpStream): int {
    s.set_blocking();
    let b = io.bytes_new(1024);
    let n = s.try_read(b, 0, 1024);
    if n > 0 {
        var sent = 0;
        while sent < n {
            let w = s.try_write(b, sent, n - sent);
            if w <= 0 {
                break;
            }
            sent = sent + w;
        }
    }
    s.shutdown_both();
    s.close();
    io.bytes_free(b);
    return n;
}

func main(): unit {
    let n = 8;
    let lst = tcp_listen_blocking("127.0.0.1", 0, 128);
    let port = lst.port_num();

    let server = thread.spawn(|unused: int| -> int {
        var hs: Array<JoinHandle<int>> = [];
        var i = 0;
        while i < n {
            let fd = lst.try_accept();
            let s = stream_from_fd(fd);
            hs.push(thread.spawn(|x: int| -> int {
                return handle_conn(s);
            }, 0));
            i = i + 1;
        }
        for (var h: hs) {
            h.join();
        }
        return 0;
    }, 0);

    var ok: Array<int> = [0];
    var c = 0;
    while c < n {
        let s = tcp_connect_blocking("127.0.0.1", port);
        let msg = "ping-${c}";
        s.write_str(msg);
        let b = io.bytes_new(256);
        var got = 0;
        while got < 256 {
            let r = s.try_read(b, got, 256 - got);
            if r <= 0 {
                break;
            }
            got = got + r;
        }
        if io.bytes_to_str(b, 0, got) == msg {
            ok[0] = ok[0] + 1;
        }
        s.close();
        io.bytes_free(b);
        c = c + 1;
    }

    server.join();
    lst.close();
    print(ok[0]);
    if ok[0] == n {
        print("tcp echo (threads) OK");
    } else {
        print("tcp echo (threads) FAIL");
    }
}
