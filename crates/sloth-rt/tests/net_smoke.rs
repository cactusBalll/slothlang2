//! IO layer smoke suite: byte buffers, address parsing, socket lifecycle and
//! the select/poll/epoll/io_uring event backends. Runs in-process on loopback;
//! ports are ephemeral so parallel test binaries do not collide.

use sloth_rt::{bytes, event, net, strings};

const READ: i64 = 1;
const WRITE: i64 = 2;

const AF_INET: i64 = 2;
const SOCK_STREAM: i64 = 1;
const SOCK_DGRAM: i64 = 2;

fn inter(s: &str) -> i64 {
    let mut buf: Vec<u8> = Vec::with_capacity(s.len().max(1));
    buf.extend_from_slice(s.as_bytes());
    strings::sloth_str_intern(buf.as_ptr() as i64, s.len() as i64)
}

fn ip_str(h: i64) -> String {
    let n = strings::sloth_str_len(h);
    let mut out = String::new();
    for i in 0..n {
        out.push(strings::sloth_str_byte(h, i) as u8 as char);
    }
    out
}

/// one wait on `backend`, returning (token, revents) pairs into a Rust vec
fn wait(backend: i64, fds: &[(i64, i64, i64)], timeout_ms: i64) -> Vec<(i64, i64)> {
    let ev = event::sloth_ev_new(backend);
    assert_ne!(ev, 0, "backend {} unavailable", backend);
    let buf = event::sloth_evbuf_new(fds.len() as i64 + 4);
    for &(fd, mask, token) in fds {
        let r = event::sloth_ev_ctl(ev, 1, fd, mask, token);
        assert_eq!(r, 0, "ctl add failed");
    }
    let n = event::sloth_ev_poll(ev, timeout_ms, buf);
    assert!(n >= 0, "poll error {}", n);
    let mut out = Vec::new();
    for i in 0..n {
        out.push((
            event::sloth_evbuf_token(buf, i),
            event::sloth_evbuf_events(buf, i),
        ));
    }
    event::sloth_evbuf_free(buf);
    event::sloth_ev_free(ev);
    out
}

/// available backends, always including the portable ones
fn backends() -> Vec<i64> {
    let mut v = vec![event::EV_SELECT, event::EV_POLL, event::EV_EPOLL];
    if event::sloth_ev_available(event::EV_IOURING) != 0 {
        v.push(event::EV_IOURING);
    }
    v
}

// ---------------- bytes ----------------

#[test]
fn bytes_roundtrip() {
    let b = bytes::sloth_bytes_new(4);
    assert_ne!(b, 0);
    bytes::sloth_bytes_append(b, 'h' as i64);
    bytes::sloth_bytes_append(b, 'i' as i64);
    assert_eq!(bytes::sloth_bytes_len(b), 2);
    assert_eq!(bytes::sloth_bytes_get(b, 0), 'h' as i64);
    let s = bytes::sloth_bytes_as_str(b);
    assert_eq!(ip_str(s), "hi");
    bytes::sloth_bytes_set(b, 1, 'o' as i64);
    let s2 = bytes::sloth_bytes_to_str(b, 0, 2);
    assert_eq!(ip_str(s2), "ho");
    // growth
    for i in 0..1000 {
        bytes::sloth_bytes_append(b, (i % 256) as i64);
    }
    assert_eq!(bytes::sloth_bytes_len(b), 1002);
    bytes::sloth_bytes_free(b);
}

// ---------------- addresses ----------------

#[test]
fn addr_roundtrip() {
    let a = net::sloth_addr_new();
    assert_eq!(net::sloth_addr_set(a, inter("127.0.0.1"), 8080), 0);
    assert_eq!(ip_str(net::sloth_addr_ip(a)), "127.0.0.1");
    assert_eq!(net::sloth_addr_port(a), 8080);

    let a6 = net::sloth_addr_new();
    assert_eq!(net::sloth_addr_set(a6, inter("::1"), 443), 0);
    assert_eq!(net::sloth_addr_port(a6), 443);
    assert!(ip_str(net::sloth_addr_ip(a6)).contains(':'));

    // bad literal
    assert!(net::sloth_addr_set(a, inter("not-an-ip"), 1) < 0);
    net::sloth_addr_free(a);
    net::sloth_addr_free(a6);
}

// ---------------- backends: socketpair readiness ----------------

#[test]
fn socketpair_readiness_all_backends() {
    for backend in backends() {
        let mut fds = [0i32; 2];
        let r = unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
        assert_eq!(r, 0);
        net::sloth_net_set_nonblocking(fds[0] as i64);
        net::sloth_net_set_nonblocking(fds[1] as i64);

        // nothing ready at first
        let empty = wait(backend, &[(fds[0] as i64, READ, 7)], 20);
        assert!(
            empty.is_empty(),
            "backend {}: unexpected {}",
            backend,
            empty.len()
        );

        // peer writes -> read end becomes ready with our token
        let msg = b"ping";
        let w = unsafe { libc::write(fds[1], msg.as_ptr() as *const libc::c_void, msg.len()) };
        assert_eq!(w, msg.len() as isize);
        let got = wait(backend, &[(fds[0] as i64, READ, 7)], 2000);
        assert_eq!(got.len(), 1, "backend {}: no readiness", backend);
        assert_eq!(got[0].0, 7);
        assert_ne!(got[0].1 & READ, 0);

        // and the data is actually there
        let b = bytes::sloth_bytes_new(16);
        let n = net::sloth_net_recv(fds[0] as i64, b, 0, 16);
        assert_eq!(n, 4);
        assert_eq!(ip_str(bytes::sloth_bytes_as_str(b)), "ping");

        bytes::sloth_bytes_free(b);
        unsafe {
            libc::close(fds[0]);
            libc::close(fds[1]);
        }
    }
}

#[test]
fn writable_readiness_all_backends() {
    for backend in backends() {
        let mut fds = [0i32; 2];
        unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
        net::sloth_net_set_nonblocking(fds[0] as i64);
        let got = wait(backend, &[(fds[0] as i64, WRITE, 42)], 2000);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].0, 42);
        assert_ne!(got[0].1 & WRITE, 0);
        unsafe {
            libc::close(fds[0]);
            libc::close(fds[1]);
        }
    }
}

// ---------------- TCP loopback ----------------

fn make_listener() -> (i64, i64) {
    let lfd = net::sloth_net_socket(AF_INET, SOCK_STREAM, 0);
    assert!(lfd >= 0, "socket failed {}", lfd);
    assert_eq!(net::sloth_net_set_reuseaddr(lfd), 0);
    assert_eq!(net::sloth_net_bind(lfd, inter("127.0.0.1"), 0), 0);
    assert_eq!(net::sloth_net_listen(lfd, 64), 0);
    let port = net::sloth_net_local_port(lfd);
    assert!(port > 0, "local_port {}", port);
    (lfd, port)
}

#[test]
fn tcp_accept_connect_recv_all_backends() {
    for backend in backends() {
        let (lfd, port) = make_listener();

        // watch the listener for readability
        let ev = event::sloth_ev_new(backend);
        let buf = event::sloth_evbuf_new(16);
        assert_eq!(event::sloth_ev_ctl(ev, 1, lfd, READ, 1), 0);

        let cfd = net::sloth_net_socket(AF_INET, SOCK_STREAM, 0);
        assert!(cfd >= 0);
        let cr = net::sloth_net_connect(cfd, inter("127.0.0.1"), port);
        // non-blocking connect: 0 or -EINPROGRESS
        assert!(
            cr == 0 || net::sloth_io_would_block(cr) != 0,
            "connect {}",
            cr
        );

        let n = event::sloth_ev_poll(ev, 2000, buf);
        assert!(n >= 1, "backend {}: listener not ready", backend);
        assert_eq!(event::sloth_evbuf_token(buf, 0), 1);

        let afd = net::sloth_net_accept(lfd);
        assert!(afd >= 0, "accept {}", afd);
        net::sloth_net_set_nodelay(afd);

        // client -> server
        let msg = inter("hello-socket");
        let w = net::sloth_net_send_str(cfd, msg);
        assert_eq!(w, 12);
        let b = bytes::sloth_bytes_new(64);
        let mut got = 0;
        for _ in 0..100 {
            let r = net::sloth_net_recv(afd, b, got, 64 - got);
            if r > 0 {
                got += r;
                if got >= 12 {
                    break;
                }
            } else if net::sloth_io_would_block(r) != 0 {
                let _ = wait(backend, &[(afd, READ, 9)], 1000);
            } else {
                panic!("recv error {}", r);
            }
        }
        assert_eq!(got, 12);
        assert_eq!(ip_str(bytes::sloth_bytes_as_str(b)), "hello-socket");

        bytes::sloth_bytes_free(b);
        event::sloth_evbuf_free(buf);
        event::sloth_ev_free(ev);
        net::sloth_net_close(afd);
        net::sloth_net_close(cfd);
        net::sloth_net_close(lfd);
    }
}

// ---------------- UDP loopback ----------------

#[test]
fn udp_echo() {
    let sfd = net::sloth_net_socket(AF_INET, SOCK_DGRAM, 0);
    assert_eq!(net::sloth_net_bind(sfd, inter("127.0.0.1"), 0), 0);
    let port = net::sloth_net_local_port(sfd);
    assert!(port > 0);
    let cfd = net::sloth_net_socket(AF_INET, SOCK_DGRAM, 0);

    let a = net::sloth_addr_new();
    assert_eq!(net::sloth_addr_set(a, inter("127.0.0.1"), port), 0);
    let b = bytes::sloth_bytes_new(32);
    bytes::sloth_bytes_copy_from_str(b, 0, inter("datagram"));
    let w = net::sloth_net_sendto(cfd, b, 0, 8, a);
    assert_eq!(w, 8);

    let rb = bytes::sloth_bytes_new(32);
    let peer = net::sloth_addr_new();
    let mut n = net::sloth_net_recvfrom(sfd, rb, 0, 32, peer);
    for _ in 0..50 {
        if n > 0 {
            break;
        }
        let _ = wait(event::EV_POLL, &[(sfd, READ, 3)], 1000);
        n = net::sloth_net_recvfrom(sfd, rb, 0, 32, peer);
    }
    assert_eq!(n, 8);
    assert_eq!(ip_str(bytes::sloth_bytes_as_str(rb)), "datagram");
    assert_eq!(
        net::sloth_addr_port(peer),
        0 + net::sloth_net_local_port(cfd)
    );

    bytes::sloth_bytes_free(b);
    bytes::sloth_bytes_free(rb);
    net::sloth_addr_free(a);
    net::sloth_addr_free(peer);
    net::sloth_net_close(sfd);
    net::sloth_net_close(cfd);
}

// ---------------- backend metadata ----------------

#[test]
fn backend_names_and_availability() {
    assert_eq!(
        ip_str(event::sloth_ev_backend_name(event::EV_SELECT)),
        "select"
    );
    assert_eq!(ip_str(event::sloth_ev_backend_name(event::EV_POLL)), "poll");
    assert_eq!(
        ip_str(event::sloth_ev_backend_name(event::EV_EPOLL)),
        "epoll"
    );
    assert_eq!(
        ip_str(event::sloth_ev_backend_name(event::EV_IOURING)),
        "io_uring"
    );
    assert_eq!(event::sloth_ev_available(event::EV_SELECT), 1);
    assert_eq!(event::sloth_ev_available(event::EV_POLL), 1);
    assert_eq!(event::sloth_ev_available(event::EV_EPOLL), 1);
    // kqueue is not available on Linux
    if cfg!(target_os = "linux") {
        assert_eq!(event::sloth_ev_available(event::EV_KQUEUE), 0);
    }
}

#[test]
fn wakeup_breaks_a_blocked_wait() {
    use std::sync::Arc;
    // wait on a socketpair read end that never fires; a helper thread wakes it
    for backend in [event::EV_SELECT, event::EV_POLL, event::EV_EPOLL] {
        let mut fds = [0i32; 2];
        unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
        net::sloth_net_set_nonblocking(fds[0] as i64);
        let ev = event::sloth_ev_new(backend);
        let buf = event::sloth_evbuf_new(8);
        assert_eq!(event::sloth_ev_ctl(ev, 1, fds[0] as i64, READ, 5), 0);
        let evp = Arc::new(ev as usize);
        let h = {
            let evp = evp.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(50));
                event::sloth_ev_wakeup(*evp as i64);
            })
        };
        let t0 = std::time::Instant::now();
        let n = event::sloth_ev_poll(ev, 5000, buf);
        let dt = t0.elapsed();
        assert!(n >= 0);
        assert!(
            dt < std::time::Duration::from_millis(3000),
            "wakeup too slow"
        );
        h.join().unwrap();
        event::sloth_evbuf_free(buf);
        event::sloth_ev_free(ev);
        unsafe {
            libc::close(fds[0]);
            libc::close(fds[1]);
        }
    }
}

#[test]
fn infinite_timeout_returns_when_ready() {
    // exercises the -1 (infinite) timeout path on every backend; data is
    // buffered before the wait, so a correct backend returns immediately
    for backend in backends() {
        let mut fds = [0i32; 2];
        unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
        net::sloth_net_set_nonblocking(fds[0] as i64);
        let w = unsafe { libc::write(fds[1], b"z".as_ptr() as *const libc::c_void, 1) };
        assert_eq!(w, 1);
        let t0 = std::time::Instant::now();
        let got = wait(backend, &[(fds[0] as i64, READ, 11)], -1);
        assert_eq!(
            got.len(),
            1,
            "backend {}: infinite wait returned nothing",
            backend
        );
        assert!(t0.elapsed() < std::time::Duration::from_millis(2000));
        unsafe {
            libc::close(fds[0]);
            libc::close(fds[1]);
        }
    }
}
