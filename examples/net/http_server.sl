// examples/net/http_server.sl
//
// Minimal HTTP/1.1 server on the fiber event loop: a router with GET/POST
// handlers, keep-alive-capable responses, and a client that drives GET and
// POST requests. Self-contained so it terminates.

import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";
import "sloth/http.slt";

func roundtrip(loop: EventLoop, port: int, raw: str): str {
    let s = tcp_connect("127.0.0.1", port);
    await_writable(loop, s.raw_fd());
    write_str_all(loop, s, raw);
    let b = io.bytes_new(8192);
    var total = 0;
    while total < 8192 {
        let r = read_some(loop, s, b, total, 8192 - total);
        if r <= 0 {
            break;
        }
        total = total + r;
    }
    let out = io.bytes_to_str(b, 0, total);
    s.close();
    io.bytes_free(b);
    return out;
}

func main(): unit {
    let loop = EventLoop(kind_epoll());
    let lst = tcp_listen("127.0.0.1", 0, 64, false);
    let port = lst.port_num();

    var router = Router();
    router.get("/", |q: Request| -> Response {
        return html("<h1>sloth-http</h1>");
    });
    router.get("/hello", |q: Request| -> Response {
        var name = "world";
        let s = q.query();
        if s != "" {
            name = s;
        }
        return text(200, "hello " + name);
    });
    router.post("/echo", |q: Request| -> Response {
        return text(200, q.body);
    });

    // two connections: GET /hello?sloth and POST /echo with a body
    loop.spawn(|w: Wait| -> unit {
        serve_n(loop, lst, router, 2);
    });

    var results: Array<int> = [0, 0];
    loop.spawn(|w: Wait| -> unit {
        let a = roundtrip(loop, port, "GET /hello?sloth HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n");
        if str_find(a, " 200 ", 0) >= 0 and str_find(a, "\r\n\r\nhello sloth", 0) >= 0 {
            results[0] = 1;
        }
        print("GET /hello done");
    });
    loop.spawn(|w: Wait| -> unit {
        let a = roundtrip(loop, port, "POST /echo HTTP/1.1\r\nHost: x\r\nContent-Length: 11\r\nConnection: close\r\n\r\nhello world");
        if str_find(a, " 200 ", 0) >= 0 and str_find(a, "\r\n\r\nhello world", 0) >= 0 {
            results[1] = 1;
        }
        print("POST /echo done");
    });

    loop.run();
    lst.close();
    loop.close();

    if results[0] == 1 and results[1] == 1 {
        print("http server OK");
    } else {
        print("http server FAIL");
    }
}
