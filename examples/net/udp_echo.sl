// examples/net/udp_echo.sl
//
// UDP echo on the fiber event loop: a server fiber receives one datagram and
// sends it back, a client fiber sends and verifies the echo.

import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";

func main(): unit {
    let loop = EventLoop(kind_epoll());
    let server = udp_bind("127.0.0.1", 0, false);
    let port = local_port(server.raw_fd());

    loop.spawn(|w: Wait| -> unit {
        await_readable(loop, server.raw_fd());
        let b = io.bytes_new(256);
        let peer = addr_new();
        let n = server.try_recv_from(b, 0, 256, peer);
        if n > 0 {
            server.send_to(peer, b, 0, n);
        }
        addr_free(peer);
        io.bytes_free(b);
    });

    loop.spawn(|w: Wait| -> unit {
        let c = udp_bind("127.0.0.1", 0, false);
        let dst = addr_new();
        addr_set(dst, "127.0.0.1", port);
        let b = io.bytes_new(64);
        io.bytes_copy_from_str(b, 0, "ping");
        c.send_to(dst, b, 0, 4);

        let rb = io.bytes_new(256);
        let peer = addr_new();
        await_readable(loop, c.raw_fd());
        let n = c.try_recv_from(rb, 0, 256, peer);
        print("udp echo: " + io.bytes_to_str(rb, 0, n));
        addr_free(peer);
        addr_free(dst);
        io.bytes_free(b);
        io.bytes_free(rb);
        c.close();
    });

    loop.run();
    server.close();
    loop.close();
    print("udp echo OK");
}
