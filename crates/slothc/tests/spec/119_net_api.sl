// spec: IO layer stdlib API (IO-P2) — byte buffers, clock, backend metadata,
// address parsing. Network-free so it is deterministic.

import "sloth/io.slt";
import "sloth/net.slt";

func main(): unit {
    // byte buffers
    let b = bytes_new(8);
    bytes_append(b, 104);
    bytes_append(b, 105);
    print(bytes_as_str(b));        // expect: hi
    print(bytes_len(b));           // expect: 2
    bytes_set(b, 1, 111);
    print(bytes_as_str(b));        // expect: ho
    print(bytes_get(b, 0));        // expect: 104
    bytes_free(b);

    // monotonic clock
    let t0 = now_ms();
    sleep_ms(1);
    print(now_ms() >= t0);         // expect: true

    // event backend metadata
    print(backend_available(kind_select()));   // expect: true
    print(backend_available(kind_epoll()));    // expect: true
    print(backend_name(kind_poll()));          // expect: poll
    print(would_block(-11));                 // expect: true
    print(would_block(-9));                  // expect: false
    print(ev_read() + ev_write());           // expect: 3

    // addresses
    let a = addr_new();
    print(addr_set(a, "127.0.0.1", 8080));   // expect: 0
    print(addr_ip(a));                       // expect: 127.0.0.1
    print(addr_port(a));                     // expect: 8080
    addr_free(a);

    // string helpers used by the HTTP parser
    print(str_find("GET / HTTP/1.1", " ", 0));   // expect: 3
    print(str_starts_with("GET /", "GET"));      // expect: true
    print(str_slice("hello world", 6, 5));       // expect: world
}
