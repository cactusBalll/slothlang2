//! Low-level socket + address primitives (IO layer). Thin, `errno`-returning
//! wrappers over the platform socket API; the policy (event loop, servers)
//! lives in sloth stdlib. Every fallible call returns `-errno` (or a
//! non-negative fd/count), so the sloth side tests `r < 0` and passes the
//! value to `sloth_io_*` helpers.
//!
//! Sockets are created non-blocking + close-on-exec; the event backends rely
//! on the non-blocking contract.

use crate::bytes;
use crate::strings;

const INET6_ADDRSTRLEN: usize = 46;

extern "C" {
    fn inet_pton(af: libc::c_int, src: *const libc::c_char, dst: *mut libc::c_void) -> libc::c_int;
    fn inet_ntop(
        af: libc::c_int,
        src: *const libc::c_void,
        dst: *mut libc::c_char,
        size: libc::socklen_t,
    ) -> *const libc::c_char;
}

/// opaque address handle: a `sockaddr_storage` plus its used length
#[repr(C)]
pub struct AddrBox {
    pub storage: libc::sockaddr_storage,
    pub len: libc::socklen_t,
}

#[inline]
fn errno() -> i64 {
    unsafe { -(*libc::__errno_location() as i64) }
}

pub(crate) unsafe fn as_addr(h: i64) -> Option<&'static mut AddrBox> {
    if h == 0 {
        None
    } else {
        Some(&mut *(h as *mut AddrBox))
    }
}

/// borrow a `str` handle as an owned `String` (empty on nil)
unsafe fn str_owned(w: i64) -> String {
    match strings::str_of(w) {
        Some(t) => {
            String::from_utf8_lossy(std::slice::from_raw_parts((*t).data as *const u8, (*t).len))
                .into_owned()
        }
        None => String::new(),
    }
}

/// fill a `sockaddr_storage` from an IPv4/IPv6 literal + port (host order).
/// returns 0 or `-EINVAL` for a bad literal.
unsafe fn fill_addr(ip: &str, port: i64, out: *mut libc::sockaddr_storage) -> i64 {
    let mut cbuf: Vec<u8> = Vec::with_capacity(ip.len() + 1);
    cbuf.extend_from_slice(ip.as_bytes());
    cbuf.push(0);
    let p = cbuf.as_ptr() as *const libc::c_char;
    if ip.contains(':') {
        let sa = out as *mut libc::sockaddr_in6;
        (*sa).sin6_family = libc::AF_INET6 as libc::sa_family_t;
        (*sa).sin6_port = (port as u16).to_be();
        if inet_pton(
            libc::AF_INET6,
            p,
            &mut (*sa).sin6_addr as *mut _ as *mut libc::c_void,
        ) != 1
        {
            return -(libc::EINVAL as i64);
        }
    } else {
        let sa = out as *mut libc::sockaddr_in;
        (*sa).sin_family = libc::AF_INET as libc::sa_family_t;
        (*sa).sin_port = (port as u16).to_be();
        if inet_pton(
            libc::AF_INET,
            p,
            &mut (*sa).sin_addr as *mut _ as *mut libc::c_void,
        ) != 1
        {
            return -(libc::EINVAL as i64);
        }
    }
    0
}

unsafe fn addr_strlen(st: *const libc::sockaddr_storage) -> libc::socklen_t {
    match (*st).ss_family as i32 {
        libc::AF_INET6 => std::mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t,
        _ => std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
    }
}

/// build a stack address for `bind`/`connect`
unsafe fn make_addr(ip: &str, port: i64) -> Result<(libc::sockaddr_storage, libc::socklen_t), i64> {
    let mut st: libc::sockaddr_storage = std::mem::zeroed();
    let r = fill_addr(ip, port, &mut st);
    if r != 0 {
        return Err(r);
    }
    let len = addr_strlen(&st);
    Ok((st, len))
}

// ---------------- address handles ----------------

#[no_mangle]
pub extern "C" fn __sloth_addr_new() -> i64 {
    unsafe {
        let p = libc::calloc(1, std::mem::size_of::<AddrBox>()) as *mut AddrBox;
        if p.is_null() {
            return 0;
        }
        p as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_addr_free(h: i64) -> i64 {
    if h != 0 {
        unsafe { libc::free(h as *mut libc::c_void) };
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_addr_set(h: i64, ip_w: i64, port: i64) -> i64 {
    unsafe {
        let a = match as_addr(h) {
            Some(a) => a,
            None => return -(libc::EINVAL as i64),
        };
        let ip = str_owned(ip_w);
        let r = fill_addr(&ip, port, &mut a.storage);
        if r != 0 {
            return r;
        }
        a.len = addr_strlen(&a.storage);
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_addr_ip(h: i64) -> i64 {
    unsafe {
        let a = match as_addr(h) {
            Some(a) => a,
            None => return strings::intern_bytes(std::ptr::null::<u8>() as usize, 0),
        };
        let mut buf = [0i8; INET6_ADDRSTRLEN];
        let src = if a.storage.ss_family as i32 == libc::AF_INET6 {
            &(*(&a.storage as *const _ as *const libc::sockaddr_in6)).sin6_addr as *const _
                as *const libc::c_void
        } else {
            &(*(&a.storage as *const _ as *const libc::sockaddr_in)).sin_addr as *const _
                as *const libc::c_void
        };
        let p = inet_ntop(
            a.storage.ss_family as libc::c_int,
            src,
            buf.as_mut_ptr(),
            buf.len() as libc::socklen_t,
        );
        if p.is_null() {
            return strings::intern_bytes(std::ptr::null::<u8>() as usize, 0);
        }
        let s = std::ffi::CStr::from_ptr(buf.as_ptr()).to_bytes();
        strings::intern_bytes(s.as_ptr() as usize, s.len() as i64)
    }
}

#[no_mangle]
pub extern "C" fn __sloth_addr_port(h: i64) -> i64 {
    unsafe {
        let a = match as_addr(h) {
            Some(a) => a,
            None => return 0,
        };
        if a.storage.ss_family as i32 == libc::AF_INET6 {
            u16::from_be((*(&a.storage as *const _ as *const libc::sockaddr_in6)).sin6_port) as i64
        } else {
            u16::from_be((*(&a.storage as *const _ as *const libc::sockaddr_in)).sin_port) as i64
        }
    }
}

// ---------------- socket lifecycle ----------------

#[no_mangle]
pub extern "C" fn __sloth_net_socket(domain: i64, kind: i64, proto: i64) -> i64 {
    unsafe {
        let fd = libc::socket(
            domain as libc::c_int,
            kind as libc::c_int | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC,
            proto as libc::c_int,
        );
        if fd < 0 {
            return errno();
        }
        fd as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_net_close(fd: i64) -> i64 {
    if fd < 0 {
        return 0;
    }
    unsafe {
        if libc::close(fd as libc::c_int) != 0 {
            return errno();
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_net_shutdown(fd: i64, how: i64) -> i64 {
    unsafe {
        if libc::shutdown(fd as libc::c_int, how as libc::c_int) != 0 {
            return errno();
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_net_set_nonblocking(fd: i64) -> i64 {
    unsafe {
        let fl = libc::fcntl(fd as libc::c_int, libc::F_GETFL, 0);
        if fl < 0 {
            return errno();
        }
        if libc::fcntl(fd as libc::c_int, libc::F_SETFL, fl | libc::O_NONBLOCK) < 0 {
            return errno();
        }
    }
    0
}

/// clear O_NONBLOCK (blocking thread-per-connection model)
#[no_mangle]
pub extern "C" fn __sloth_net_set_blocking(fd: i64) -> i64 {
    unsafe {
        let fl = libc::fcntl(fd as libc::c_int, libc::F_GETFL, 0);
        if fl < 0 {
            return errno();
        }
        if libc::fcntl(fd as libc::c_int, libc::F_SETFL, fl & !libc::O_NONBLOCK) < 0 {
            return errno();
        }
    }
    0
}

unsafe fn set_int_opt(fd: i64, level: libc::c_int, name: libc::c_int, val: libc::c_int) -> i64 {
    if libc::setsockopt(
        fd as libc::c_int,
        level,
        name,
        &val as *const _ as *const libc::c_void,
        std::mem::size_of::<libc::c_int>() as libc::socklen_t,
    ) != 0
    {
        return errno();
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_net_set_reuseaddr(fd: i64) -> i64 {
    unsafe { set_int_opt(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1) }
}

#[no_mangle]
pub extern "C" fn __sloth_net_set_reuseport(fd: i64) -> i64 {
    unsafe { set_int_opt(fd, libc::SOL_SOCKET, libc::SO_REUSEPORT, 1) }
}

#[no_mangle]
pub extern "C" fn __sloth_net_set_nodelay(fd: i64) -> i64 {
    unsafe { set_int_opt(fd, libc::IPPROTO_TCP, libc::TCP_NODELAY, 1) }
}

#[no_mangle]
pub extern "C" fn __sloth_net_bind(fd: i64, ip_w: i64, port: i64) -> i64 {
    unsafe {
        let ip = str_owned(ip_w);
        let (mut st, len) = match make_addr(&ip, port) {
            Ok(x) => x,
            Err(e) => return e,
        };
        if libc::bind(
            fd as libc::c_int,
            &mut st as *mut _ as *mut libc::sockaddr,
            len,
        ) != 0
        {
            return errno();
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_net_listen(fd: i64, backlog: i64) -> i64 {
    unsafe {
        if libc::listen(fd as libc::c_int, backlog as libc::c_int) != 0 {
            return errno();
        }
    }
    0
}

/// accept a connection (returns a non-blocking fd, or `-errno`; `-EAGAIN`
/// means no pending connection)
#[no_mangle]
pub extern "C" fn __sloth_net_accept(fd: i64) -> i64 {
    unsafe {
        let c = libc::accept4(
            fd as libc::c_int,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC,
        );
        if c < 0 {
            return errno();
        }
        c as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_net_connect(fd: i64, ip_w: i64, port: i64) -> i64 {
    unsafe {
        let ip = str_owned(ip_w);
        let (mut st, len) = match make_addr(&ip, port) {
            Ok(x) => x,
            Err(e) => return e,
        };
        if libc::connect(
            fd as libc::c_int,
            &mut st as *mut _ as *mut libc::sockaddr,
            len,
        ) != 0
        {
            return errno();
        }
    }
    0
}

// ---------------- data transfer ----------------

#[no_mangle]
pub extern "C" fn __sloth_net_recv(fd: i64, b: i64, off: i64, len: i64) -> i64 {
    let bo = match unsafe { bytes::obj(b) } {
        Some(x) => x,
        None => return -(libc::EINVAL as i64),
    };
    if off < 0 || len <= 0 {
        return -(libc::EINVAL as i64);
    }
    let end = off + len;
    if bytes::__sloth_bytes_ensure(b, end) != 0 {
        return -(libc::ENOMEM as i64);
    }
    let cur = bytes::__sloth_bytes_len(b);
    unsafe {
        let dst = bytes::data_ptr(bo).add(off as usize) as *mut libc::c_void;
        let n = libc::recv(fd as libc::c_int, dst, len as usize, 0);
        if n < 0 {
            return errno();
        }
        let total = off + n as i64;
        if total > cur {
            bytes::__sloth_bytes_set_len(b, total);
        }
        n as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_net_send(fd: i64, b: i64, off: i64, len: i64) -> i64 {
    let bo = match unsafe { bytes::obj(b) } {
        Some(x) => x,
        None => return -(libc::EINVAL as i64),
    };
    if off < 0 || len < 0 {
        return -(libc::EINVAL as i64);
    }
    let cur = bytes::__sloth_bytes_len(b);
    let avail = (cur - off).max(0);
    let n = len.min(avail);
    unsafe {
        let src = bytes::data_ptr(bo).add(off as usize) as *const libc::c_void;
        let w = libc::send(fd as libc::c_int, src, n as usize, libc::MSG_NOSIGNAL);
        if w < 0 {
            return errno();
        }
        w as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_net_send_str(fd: i64, s_w: i64) -> i64 {
    unsafe {
        let t = match strings::str_of(s_w) {
            Some(t) => t as *const strings::StrT,
            None => return 0,
        };
        let w = libc::send(
            fd as libc::c_int,
            (*t).data as *const libc::c_void,
            (*t).len,
            libc::MSG_NOSIGNAL,
        );
        if w < 0 {
            return errno();
        }
        w as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_net_recvfrom(fd: i64, b: i64, off: i64, len: i64, addr: i64) -> i64 {
    let bo = match unsafe { bytes::obj(b) } {
        Some(x) => x,
        None => return -(libc::EINVAL as i64),
    };
    if off < 0 || len <= 0 {
        return -(libc::EINVAL as i64);
    }
    let end = off + len;
    if bytes::__sloth_bytes_ensure(b, end) != 0 {
        return -(libc::ENOMEM as i64);
    }
    let cur = bytes::__sloth_bytes_len(b);
    unsafe {
        let dst = bytes::data_ptr(bo).add(off as usize) as *mut libc::c_void;
        let (st, slen): (*mut libc::sockaddr, *mut libc::socklen_t) = if addr != 0 {
            let a = match as_addr(addr) {
                Some(a) => a,
                None => return -(libc::EINVAL as i64),
            };
            a.len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
            (
                &mut a.storage as *mut _ as *mut libc::sockaddr,
                &mut a.len as *mut libc::socklen_t,
            )
        } else {
            (std::ptr::null_mut(), std::ptr::null_mut())
        };
        let n = libc::recvfrom(fd as libc::c_int, dst, len as usize, 0, st, slen);
        if n < 0 {
            return errno();
        }
        let total = off + n as i64;
        if total > cur {
            bytes::__sloth_bytes_set_len(b, total);
        }
        n as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_net_sendto(fd: i64, b: i64, off: i64, len: i64, addr: i64) -> i64 {
    let bo = match unsafe { bytes::obj(b) } {
        Some(x) => x,
        None => return -(libc::EINVAL as i64),
    };
    if off < 0 || len < 0 {
        return -(libc::EINVAL as i64);
    }
    let a = match unsafe { as_addr(addr) } {
        Some(a) => a,
        None => return -(libc::EINVAL as i64),
    };
    let cur = bytes::__sloth_bytes_len(b);
    let avail = (cur - off).max(0);
    let n = len.min(avail);
    unsafe {
        let src = bytes::data_ptr(bo).add(off as usize) as *const libc::c_void;
        let w = libc::sendto(
            fd as libc::c_int,
            src,
            n as usize,
            libc::MSG_NOSIGNAL,
            &a.storage as *const _ as *const libc::sockaddr,
            a.len,
        );
        if w < 0 {
            return errno();
        }
        w as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_net_peer_addr(fd: i64, addr: i64) -> i64 {
    unsafe {
        let a = match as_addr(addr) {
            Some(a) => a,
            None => return -(libc::EINVAL as i64),
        };
        a.len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        if libc::getpeername(
            fd as libc::c_int,
            &mut a.storage as *mut _ as *mut libc::sockaddr,
            &mut a.len as *mut libc::socklen_t,
        ) != 0
        {
            return errno();
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_net_local_port(fd: i64) -> i64 {
    unsafe {
        let mut st: libc::sockaddr_storage = std::mem::zeroed();
        let mut len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        if libc::getsockname(
            fd as libc::c_int,
            &mut st as *mut _ as *mut libc::sockaddr,
            &mut len,
        ) != 0
        {
            return errno();
        }
        if st.ss_family as i32 == libc::AF_INET6 {
            u16::from_be((*(&st as *const _ as *const libc::sockaddr_in6)).sin6_port) as i64
        } else {
            u16::from_be((*(&st as *const _ as *const libc::sockaddr_in)).sin_port) as i64
        }
    }
}

// ---------------- errno helpers ----------------

#[no_mangle]
pub extern "C" fn __sloth_io_errno() -> i64 {
    unsafe { *libc::__errno_location() as i64 }
}

/// is `r` (a `-errno` from this module) a would-block condition?
#[no_mangle]
pub extern "C" fn __sloth_io_would_block(r: i64) -> i64 {
    let e = -r;
    (e == libc::EAGAIN as i64 || e == libc::EWOULDBLOCK as i64 || e == libc::EINPROGRESS as i64)
        as i64
}

/// is `r` a closed/EOF condition?
#[no_mangle]
pub extern "C" fn __sloth_io_conn_closed(r: i64) -> i64 {
    let e = -r;
    (e == libc::ECONNRESET as i64
        || e == libc::ECONNABORTED as i64
        || e == libc::EPIPE as i64
        || e == libc::ENOTCONN as i64) as i64
}
