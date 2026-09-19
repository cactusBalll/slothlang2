// spec: HTTP/1.1 parser + router + fiber server (IO-P4).

import "sloth/io.slt";
import "sloth/net.slt";
import "sloth/event.slt";
import "sloth/http.slt";

func parser_tests(): unit {
    let req = parse_request("GET /hello?q=1 HTTP/1.1\r\nHost: X\r\nContent-Length: 5\r\n\r\nworld");
    if req is nil {
        print("parse-fail");
        return;
    }
    let r: Request = req ?: blank_request();
    print(r.method);
    print(r.route());
    print(r.query());
    print(r.body);
    print(r.header("host") ?: "none");
    print(r.header("missing") is nil);

    var resp = html("<h1>hi</h1>");
    let s = resp.serialize();
    print(str_starts_with(s, "HTTP/1.1 200 OK"));
    print(str_find(s, "Content-Length:", 0) >= 0);
    print(str_find(s, "\r\n\r\n<h1>hi</h1>", 0) >= 0);
}

func http_roundtrip(kind: int): unit {
    let loop = EventLoop(kind);
    let lst = tcp_listen("127.0.0.1", 0, 64, false);
    let port = lst.port_num();
    var router = Router();
    router.get("/hello", |q: Request| -> Response {
        return text(200, "hi");
    });
    router.get("/missing", |q: Request| -> Response {
        return not_found();
    });

    loop.spawn(|w: Wait| -> unit {
        serve_n(loop, lst, router, 1);
    });
    loop.spawn(|w: Wait| -> unit {
        let s = tcp_connect("127.0.0.1", port);
        await_writable(loop, s.raw_fd());
        write_str_all(loop, s, "GET /hello HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n");
        let b = io.bytes_new(4096);
        var total = 0;
        while true {
            let r = read_some(loop, s, b, total, 4096 - total);
            if r <= 0 {
                break;
            }
            total = total + r;
        }
        let resp = io.bytes_to_str(b, 0, total);
        if str_find(resp, " 200 ", 0) >= 0 and str_find(resp, "\r\n\r\nhi", 0) >= 0 {
            print("http 200");
        }
        s.close();
        io.bytes_free(b);
    });

    loop.run();
    lst.close();
    loop.close();
}

func main(): unit {
    parser_tests();
    var kinds: Array<int> = [kind_select(), kind_poll(), kind_epoll()];
    if backend_available(kind_iouring()) {
        kinds.push(kind_iouring());
    }
    var i = 0;
    while i < kinds.len() {
        http_roundtrip(kinds[i]);
        i = i + 1;
    }
    print("http OK");
    // expect: GET
    // expect: /hello
    // expect: q=1
    // expect: world
    // expect: X
    // expect: true
    // expect: http 200
    // expect: http OK
}
