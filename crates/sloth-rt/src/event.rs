//! Unified IO event backend: `select` / `poll` / `epoll` / `kqueue` / `io_uring`
//! behind one small C ABI. The sloth stdlib (`event.slt`) owns the queue
//! policy (tokens, timers, fiber reactor); this module only maps a set of
//! watched fds to a readiness list.
//!
//! Backends:
//! - `poll`/`select` rebuild their watch set from the registry each wait;
//! - `epoll` keeps kernel state and carries the sloth token in `epoll_data`;
//! - `io_uring` submits one-shot `POLL_ADD` requests and re-arms fired ones;
//! - `kqueue` is compiled for BSD/macOS only (unavailable on Linux).
//!
//! Every entry point is single-threaded-safe per handle; an `EvLoop` is owned
//! by one OS thread (matching the fiber model). A self-pipe `eventfd` lets
//! another thread wake a blocked wait.

use crate::strings;
use std::collections::HashMap;

// backend kinds
pub const EV_SELECT: i64 = 0;
pub const EV_POLL: i64 = 1;
pub const EV_EPOLL: i64 = 2;
pub const EV_KQUEUE: i64 = 3;
pub const EV_IOURING: i64 = 4;

// ops
const OP_ADD: i64 = 1;
const OP_MOD: i64 = 2;
const OP_DEL: i64 = 3;

// interest / revents bits (stable across backends)
const R_READ: i64 = 1;
const R_WRITE: i64 = 2;
const R_ERR: i64 = 4;
const R_HUP: i64 = 8;

/// reserved token for the wakeup eventfd
const TOKEN_WAKE: i64 = i64::MIN;

#[inline]
unsafe fn errno() -> i64 {
    -(*libc::__errno_location() as i64)
}

// ---------------- event buffer (out-param for wait) ----------------

#[repr(C)]
pub struct EvBuf {
    cap: usize,
    count: i64,
    tokens: *mut i64,
    revents: *mut i64,
}

unsafe fn evbuf_push(b: &mut EvBuf, token: i64, revents: i64) {
    let n = b.count as usize;
    if n >= b.cap {
        return;
    }
    *b.tokens.add(n) = token;
    *b.revents.add(n) = revents;
    b.count += 1;
}

#[no_mangle]
pub extern "C" fn sloth_evbuf_new(cap: i64) -> i64 {
    unsafe {
        let c = cap.max(1) as usize;
        let p = libc::calloc(1, std::mem::size_of::<EvBuf>()) as *mut EvBuf;
        if p.is_null() {
            return 0;
        }
        (*p).cap = c;
        (*p).count = 0;
        (*p).tokens = libc::malloc(c * 8) as *mut i64;
        (*p).revents = libc::malloc(c * 8) as *mut i64;
        (*p).count = 0;
        p as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_evbuf_free(h: i64) -> i64 {
    unsafe {
        if h != 0 {
            let b = &mut *(h as *mut EvBuf);
            libc::free(b.tokens as *mut libc::c_void);
            libc::free(b.revents as *mut libc::c_void);
            libc::free(b as *mut EvBuf as *mut libc::c_void);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn sloth_evbuf_count(h: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    unsafe { (*(h as *const EvBuf)).count }
}

#[no_mangle]
pub extern "C" fn sloth_evbuf_token(h: i64, i: i64) -> i64 {
    unsafe {
        let b = &*(h as *const EvBuf);
        if i < 0 || i >= b.count {
            return 0;
        }
        *b.tokens.add(i as usize)
    }
}

#[no_mangle]
pub extern "C" fn sloth_evbuf_events(h: i64, i: i64) -> i64 {
    unsafe {
        let b = &*(h as *const EvBuf);
        if i < 0 || i >= b.count {
            return 0;
        }
        *b.revents.add(i as usize)
    }
}

// ---------------- loop state ----------------

#[derive(Clone, Copy)]
struct Reg {
    fd: i32,
    events: i64,
    token: i64,
    gen: u32,
}

pub struct EvLoop {
    kind: i64,
    epfd: i32,
    wake_fd: i32,
    regs: Vec<Reg>,
    by_token: HashMap<i64, usize>,
    #[cfg(target_os = "linux")]
    ring: Option<Box<IoUring>>,
}

unsafe fn loop_of<'a>(h: i64) -> Option<&'a mut EvLoop> {
    if h == 0 {
        None
    } else {
        Some(&mut *(h as *mut EvLoop))
    }
}

fn make_eventfd() -> i32 {
    unsafe { libc::eventfd(0, libc::EFD_NONBLOCK | libc::EFD_CLOEXEC) }
}

fn drain_eventfd(fd: i32) {
    unsafe {
        let mut v: u64 = 0;
        let _ = libc::read(fd, &mut v as *mut u64 as *mut libc::c_void, 8);
    }
}

// ---------------- standalone eventfd (cross-thread completion signal) ----------------
//
// A worker OS thread signals one of these when a compute task finishes; the
// owning `EventLoop` watches the fd and wakes the waiting fiber. The counter
// nature of eventfd makes "signal before the watch is registered" safe: the
// fd stays readable until drained.

/// create a nonblocking eventfd; returns the fd or `-errno`
#[no_mangle]
pub extern "C" fn sloth_async_new() -> i64 {
    let fd = make_eventfd();
    if fd < 0 {
        unsafe { errno() }
    } else {
        fd as i64
    }
}

/// signal an eventfd (add 1); returns 0 or `-errno`
#[no_mangle]
pub extern "C" fn sloth_async_signal(fd: i64) -> i64 {
    let v: u64 = 1;
    let r = unsafe { libc::write(fd as i32, &v as *const u64 as *const libc::c_void, 8) };
    if r < 0 {
        unsafe { errno() }
    } else {
        0
    }
}

/// drain an eventfd (nonblocking read); returns 0 or `-errno`
#[no_mangle]
pub extern "C" fn sloth_async_drain(fd: i64) -> i64 {
    let mut v: u64 = 0;
    let r = unsafe { libc::read(fd as i32, &mut v as *mut u64 as *mut libc::c_void, 8) };
    if r < 0 {
        unsafe { errno() }
    } else {
        0
    }
}

/// close an eventfd; returns 0 or `-errno`
#[no_mangle]
pub extern "C" fn sloth_async_free(fd: i64) -> i64 {
    if fd < 0 {
        return 0;
    }
    let r = unsafe { libc::close(fd as i32) };
    if r < 0 {
        unsafe { errno() }
    } else {
        0
    }
}

// ---------------- availability / naming ----------------

#[cfg(target_os = "linux")]
fn probe_iouring() -> bool {
    use std::sync::atomic::{AtomicI8, Ordering};
    static PROBED: AtomicI8 = AtomicI8::new(-1);
    let cached = PROBED.load(Ordering::Relaxed);
    if cached >= 0 {
        return cached == 1;
    }
    let ok = unsafe {
        let mut p: uring::IoUringParams = std::mem::zeroed();
        let fd = libc::syscall(
            libc::SYS_io_uring_setup,
            8usize as libc::c_long,
            &mut p as *mut uring::IoUringParams as libc::c_long,
        );
        if fd < 0 {
            false
        } else {
            libc::close(fd as libc::c_int);
            true
        }
    };
    PROBED.store(ok as i8, Ordering::Relaxed);
    ok
}

#[cfg(not(target_os = "linux"))]
fn probe_iouring() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn sloth_ev_available(kind: i64) -> i64 {
    let ok = match kind {
        EV_SELECT | EV_POLL | EV_EPOLL => true,
        EV_IOURING => probe_iouring(),
        // kqueue is not implemented yet: report it unavailable on every
        // platform rather than advertise a backend that would fail at poll
        // time. The ABI already reserves the kind so a BSD backend can be
        // slotted in without touching the sloth stdlib.
        EV_KQUEUE => false,
        _ => false,
    };
    ok as i64
}

#[no_mangle]
pub extern "C" fn sloth_ev_backend_name(kind: i64) -> i64 {
    let name = match kind {
        EV_SELECT => "select",
        EV_POLL => "poll",
        EV_EPOLL => "epoll",
        EV_KQUEUE => "kqueue",
        EV_IOURING => "io_uring",
        _ => "unknown",
    };
    strings::intern_bytes(name.as_ptr() as usize, name.len() as i64)
}

// ---------------- lifecycle ----------------

#[no_mangle]
pub extern "C" fn sloth_ev_new(kind: i64) -> i64 {
    if sloth_ev_available(kind) == 0 {
        return 0;
    }
    let wake_fd = make_eventfd();
    if wake_fd < 0 {
        return 0;
    }
    let mut ev = EvLoop {
        kind,
        epfd: -1,
        wake_fd,
        regs: Vec::new(),
        by_token: HashMap::new(),
        #[cfg(target_os = "linux")]
        ring: None,
    };
    if kind == EV_EPOLL {
        let ep = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
        if ep < 0 {
            unsafe { libc::close(wake_fd) };
            return 0;
        }
        ev.epfd = ep;
        // register the wakeup eventfd
        let mut e = libc::epoll_event {
            events: libc::EPOLLIN as u32,
            u64: TOKEN_WAKE as u64,
        };
        let _ = unsafe { libc::epoll_ctl(ep, libc::EPOLL_CTL_ADD, wake_fd, &mut e) };
    } else if kind == EV_IOURING {
        #[cfg(target_os = "linux")]
        {
            match unsafe { IoUring::new(64) } {
                Some(r) => ev.ring = Some(Box::new(r)),
                None => {
                    unsafe { libc::close(wake_fd) };
                    return 0;
                }
            }
        }
    }
    Box::into_raw(Box::new(ev)) as i64
}

#[no_mangle]
pub extern "C" fn sloth_ev_free(h: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    unsafe {
        let ev = Box::from_raw(h as *mut EvLoop);
        if ev.epfd >= 0 {
            libc::close(ev.epfd);
        }
        if ev.wake_fd >= 0 {
            libc::close(ev.wake_fd);
        }
        // ring drops its mappings
        drop(ev);
    }
    0
}

/// cross-thread wake: make a blocked `sloth_ev_poll` return promptly
#[no_mangle]
pub extern "C" fn sloth_ev_wakeup(h: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    unsafe {
        let ev = &*(h as *const EvLoop);
        let v: u64 = 1;
        let _ = libc::write(ev.wake_fd, &v as *const u64 as *const libc::c_void, 8);
    }
    0
}

// ---------------- control ----------------

fn map_interest(events: i64) -> i32 {
    let mut m = 0;
    if events & R_READ != 0 {
        m |= libc::EPOLLIN;
    }
    if events & R_WRITE != 0 {
        m |= libc::EPOLLOUT;
    }
    m
}

fn unmap_interest(revents: i32) -> i64 {
    let mut r = 0i64;
    if revents & (libc::EPOLLIN | libc::EPOLLPRI) != 0 {
        r |= R_READ;
    }
    if revents & libc::EPOLLOUT != 0 {
        r |= R_WRITE;
    }
    if revents & libc::EPOLLERR != 0 {
        r |= R_ERR;
    }
    if revents & (libc::EPOLLHUP | libc::EPOLLRDHUP) != 0 {
        r |= R_HUP;
    }
    r
}

#[no_mangle]
pub extern "C" fn sloth_ev_ctl(h: i64, op: i64, fd: i64, events: i64, token: i64) -> i64 {
    let ev = match unsafe { loop_of(h) } {
        Some(e) => e,
        None => return -(libc::EINVAL as i64),
    };
    if ev.kind == EV_EPOLL {
        let eop = match op {
            OP_ADD => libc::EPOLL_CTL_ADD,
            OP_MOD => libc::EPOLL_CTL_MOD,
            OP_DEL => libc::EPOLL_CTL_DEL,
            _ => return -(libc::EINVAL as i64),
        };
        let mut e = libc::epoll_event {
            events: (map_interest(events) | libc::EPOLLERR | libc::EPOLLHUP | libc::EPOLLRDHUP)
                as u32,
            u64: token as u64,
        };
        let r = unsafe { libc::epoll_ctl(ev.epfd, eop, fd as i32, &mut e) };
        if r != 0 {
            return unsafe { errno() };
        }
        return 0;
    }
    // registry-based backends
    match op {
        OP_ADD => {
            let gen = 1u32;
            ev.regs.push(Reg {
                fd: fd as i32,
                events,
                token,
                gen,
            });
            ev.by_token.insert(token, ev.regs.len() - 1);
            #[cfg(target_os = "linux")]
            if ev.kind == EV_IOURING {
                if let Some(r) = ev.ring.as_mut() {
                    unsafe { r.arm(fd as i32, events, token, gen) };
                }
            }
            0
        }
        OP_MOD => {
            if let Some(&i) = ev.by_token.get(&token) {
                let gen = ev.regs[i].gen.wrapping_add(1).max(1);
                ev.regs[i] = Reg {
                    fd: fd as i32,
                    events,
                    token,
                    gen,
                };
                #[cfg(target_os = "linux")]
                if ev.kind == EV_IOURING {
                    if let Some(r) = ev.ring.as_mut() {
                        unsafe { r.arm(fd as i32, events, token, gen) };
                    }
                }
            } else {
                return sloth_ev_ctl(h, OP_ADD, fd, events, token);
            }
            0
        }
        OP_DEL => {
            if let Some(i) = ev.by_token.remove(&token) {
                ev.regs.swap_remove(i);
                // rebuild the token->index map for the swapped element
                if i < ev.regs.len() {
                    let t = ev.regs[i].token;
                    ev.by_token.insert(t, i);
                }
            }
            0
        }
        _ => -(libc::EINVAL as i64),
    }
}

// ---------------- wait ----------------

#[no_mangle]
pub extern "C" fn sloth_ev_poll(h: i64, timeout_ms: i64, out: i64) -> i64 {
    let ev = match unsafe { loop_of(h) } {
        Some(e) => e,
        None => return -(libc::EINVAL as i64),
    };
    if out == 0 {
        return -(libc::EINVAL as i64);
    }
    unsafe {
        let buf = &mut *(out as *mut EvBuf);
        buf.count = 0;
    }
    match ev.kind {
        EV_EPOLL => poll_epoll(ev, timeout_ms, out),
        EV_POLL => poll_poll(ev, timeout_ms, out),
        EV_SELECT => poll_select(ev, timeout_ms, out),
        EV_IOURING => {
            #[cfg(target_os = "linux")]
            {
                poll_iouring(ev, timeout_ms, out)
            }
            #[cfg(not(target_os = "linux"))]
            {
                let _ = (ev, out);
                -(libc::ENOSYS as i64)
            }
        }
        _ => -(libc::ENOSYS as i64),
    }
}

fn poll_epoll(ev: &mut EvLoop, timeout_ms: i64, out: i64) -> i64 {
    let n = ev.regs.len().max(16) + 8;
    let mut evs: Vec<libc::epoll_event> =
        vec![libc::epoll_event { events: 0, u64: 0 }; n.min(4096)];
    let r = unsafe {
        libc::epoll_wait(
            ev.epfd,
            evs.as_mut_ptr(),
            evs.len() as libc::c_int,
            timeout_ms as libc::c_int,
        )
    };
    if r < 0 {
        let e = unsafe { -(*libc::__errno_location() as i64) };
        if unsafe { *libc::__errno_location() } == libc::EINTR {
            return 0;
        }
        return e;
    }
    unsafe {
        let buf = &mut *(out as *mut EvBuf);
        for i in 0..r as usize {
            let e = evs[i];
            let token = e.u64 as i64;
            let re = e.events as i32;
            if token == TOKEN_WAKE {
                drain_eventfd(ev.wake_fd);
                continue;
            }
            evbuf_push(buf, token, unmap_interest(re));
        }
        buf.count
    }
}

fn poll_poll(ev: &mut EvLoop, timeout_ms: i64, out: i64) -> i64 {
    let mut pfds: Vec<libc::pollfd> = Vec::with_capacity(ev.regs.len() + 1);
    let mut tokens: Vec<i64> = Vec::with_capacity(ev.regs.len() + 1);
    pfds.push(libc::pollfd {
        fd: ev.wake_fd,
        events: libc::POLLIN,
        revents: 0,
    });
    tokens.push(TOKEN_WAKE);
    for r in &ev.regs {
        let mut pe = 0i16;
        if r.events & R_READ != 0 {
            pe |= libc::POLLIN;
        }
        if r.events & R_WRITE != 0 {
            pe |= libc::POLLOUT;
        }
        pfds.push(libc::pollfd {
            fd: r.fd,
            events: pe,
            revents: 0,
        });
        tokens.push(r.token);
    }
    let r = unsafe {
        libc::poll(
            pfds.as_mut_ptr(),
            pfds.len() as libc::nfds_t,
            timeout_ms as libc::c_int,
        )
    };
    if r < 0 {
        if unsafe { *libc::__errno_location() } == libc::EINTR {
            return 0;
        }
        return unsafe { errno() };
    }
    unsafe {
        let buf = &mut *(out as *mut EvBuf);
        for (i, p) in pfds.iter().enumerate() {
            if p.revents == 0 {
                continue;
            }
            if tokens[i] == TOKEN_WAKE {
                drain_eventfd(ev.wake_fd);
                continue;
            }
            let mut re = 0i64;
            if p.revents & libc::POLLIN != 0 {
                re |= R_READ;
            }
            if p.revents & libc::POLLOUT != 0 {
                re |= R_WRITE;
            }
            if p.revents & libc::POLLERR != 0 {
                re |= R_ERR;
            }
            if p.revents & libc::POLLHUP != 0 {
                re |= R_HUP;
            }
            evbuf_push(buf, tokens[i], re);
        }
        buf.count
    }
}

/// portable `fd_set` image (16 * u64 = FD_SETSIZE bits); passed to `select` as
/// `libc::fd_set` whose layout is identical on 64-bit Linux
#[repr(C)]
#[derive(Clone, Copy)]
struct FdSet {
    bits: [u64; 16],
}

impl FdSet {
    fn zero() -> FdSet {
        FdSet { bits: [0; 16] }
    }
    fn set(&mut self, fd: i32) {
        let f = fd as usize;
        self.bits[f / 64] |= 1u64 << (f % 64);
    }
    fn isset(&self, fd: i32) -> bool {
        let f = fd as usize;
        self.bits[f / 64] & (1u64 << (f % 64)) != 0
    }
}

fn poll_select(ev: &mut EvLoop, timeout_ms: i64, out: i64) -> i64 {
    let mut rfds = FdSet::zero();
    let mut wfds = FdSet::zero();
    let mut maxfd: i32 = ev.wake_fd;
    rfds.set(ev.wake_fd);
    for r in &ev.regs {
        if r.fd < 0 || r.fd as usize >= libc::FD_SETSIZE {
            continue;
        }
        if r.events & R_READ != 0 {
            rfds.set(r.fd);
        }
        if r.events & R_WRITE != 0 {
            wfds.set(r.fd);
        }
        if r.fd > maxfd {
            maxfd = r.fd;
        }
    }
    let mut tv = libc::timeval {
        tv_sec: (timeout_ms.max(0)) / 1000,
        tv_usec: (timeout_ms.max(0) % 1000) * 1000,
    };
    let tvp: *mut libc::timeval = if timeout_ms < 0 {
        std::ptr::null_mut()
    } else {
        &mut tv as *mut libc::timeval
    };
    let r = unsafe {
        libc::select(
            maxfd + 1,
            &mut rfds as *mut FdSet as *mut libc::fd_set,
            &mut wfds as *mut FdSet as *mut libc::fd_set,
            std::ptr::null_mut(),
            tvp,
        )
    };
    if r < 0 {
        if unsafe { *libc::__errno_location() } == libc::EINTR {
            return 0;
        }
        return unsafe { errno() };
    }
    unsafe {
        let buf = &mut *(out as *mut EvBuf);
        if rfds.isset(ev.wake_fd) {
            drain_eventfd(ev.wake_fd);
        }
        for reg in &ev.regs {
            if reg.fd < 0 || reg.fd as usize >= libc::FD_SETSIZE {
                continue;
            }
            let mut re = 0i64;
            if rfds.isset(reg.fd) {
                re |= R_READ;
            }
            if wfds.isset(reg.fd) {
                re |= R_WRITE;
            }
            if re != 0 {
                evbuf_push(buf, reg.token, re);
            }
        }
        buf.count
    }
}

// ---------------- io_uring (Linux) ----------------

#[cfg(target_os = "linux")]
mod uring {
    pub const IORING_OFF_SQ_RING: i64 = 0;
    pub const IORING_OFF_CQ_RING: i64 = 0x8000000;
    pub const IORING_OFF_SQES: i64 = 0x10000000;
    pub const IORING_OP_POLL_ADD: u8 = 6;

    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    pub struct SqOff {
        pub head: u32,
        pub tail: u32,
        pub ring_mask: u32,
        pub ring_entries: u32,
        pub flags: u32,
        pub dropped: u32,
        pub array: u32,
        pub resv1: u32,
        pub resv2: u64,
    }

    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    pub struct CqOff {
        pub head: u32,
        pub tail: u32,
        pub ring_mask: u32,
        pub ring_entries: u32,
        pub overflow: u32,
        pub cqes: u32,
        pub flags: u32,
        pub resv1: u32,
        pub resv2: u64,
    }

    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    pub struct IoUringParams {
        pub sq_entries: u32,
        pub cq_entries: u32,
        pub flags: u32,
        pub sq_thread_cpu: u32,
        pub sq_thread_idle: u32,
        pub features: u32,
        pub wq_fd: u32,
        pub resv: [u32; 3],
        pub sq_off: SqOff,
        pub cq_off: CqOff,
    }

    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    pub struct Sqe {
        pub opcode: u8,
        pub flags: u8,
        pub ioprio: u16,
        pub fd: i32,
        pub off: u64,
        pub addr: u64,
        pub len: u32,
        pub poll32_events: u32,
        pub user_data: u64,
        pub buf_index: u16,
        pub personality: u16,
        pub splice_fd_in: i32,
        pub addr3: u64,
        pub pad2: u64,
    }

    #[repr(C)]
    #[derive(Default, Clone, Copy)]
    pub struct Cqe {
        pub user_data: u64,
        pub res: i32,
        pub flags: u32,
    }
}

#[cfg(target_os = "linux")]
pub struct IoUring {
    fd: i32,
    sq_ring: *mut u8,
    cq_ring: *mut u8,
    sqes: *mut u8,
    sq_ring_sz: usize,
    cq_ring_sz: usize,
    sqes_sz: usize,
    params: Box<uring::IoUringParams>,
    submitted_tail: u32,
}

#[cfg(target_os = "linux")]
unsafe impl Send for IoUring {}

#[cfg(target_os = "linux")]
impl IoUring {
    unsafe fn new(entries: u32) -> Option<IoUring> {
        use uring::*;
        let mut params: Box<uring::IoUringParams> = Box::new(std::mem::zeroed());
        let fd = libc::syscall(
            libc::SYS_io_uring_setup,
            entries as libc::c_long,
            params.as_mut() as *mut uring::IoUringParams as libc::c_long,
        );
        if fd < 0 {
            return None;
        }
        let fd = fd as i32;
        let sq_sz = (params.sq_off.array + params.sq_entries * 4) as usize;
        let cq_sz = (params.cq_off.cqes + params.cq_entries * 16) as usize;
        let sqes_sz = (params.sq_entries as usize) * 64;
        let map = |len: usize, off: i64| -> *mut u8 {
            let p = libc::mmap(
                std::ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_POPULATE,
                fd,
                off,
            );
            if p == libc::MAP_FAILED {
                std::ptr::null_mut()
            } else {
                p as *mut u8
            }
        };
        let sq_ring = map(sq_sz, IORING_OFF_SQ_RING);
        let cq_ring = map(cq_sz, IORING_OFF_CQ_RING);
        let sqes = map(sqes_sz, IORING_OFF_SQES);
        if sq_ring.is_null() || cq_ring.is_null() || sqes.is_null() {
            libc::close(fd);
            return None;
        }
        Some(IoUring {
            fd,
            sq_ring,
            cq_ring,
            sqes,
            sq_ring_sz: sq_sz,
            cq_ring_sz: cq_sz,
            sqes_sz,
            params,
            submitted_tail: 0,
        })
    }

    #[inline]
    unsafe fn sq_head(&self) -> *mut u32 {
        self.sq_ring.add(self.params.sq_off.head as usize) as *mut u32
    }
    #[inline]
    unsafe fn sq_tail(&self) -> *mut u32 {
        self.sq_ring.add(self.params.sq_off.tail as usize) as *mut u32
    }
    #[inline]
    unsafe fn sq_mask(&self) -> u32 {
        *(self.sq_ring.add(self.params.sq_off.ring_mask as usize) as *const u32)
    }
    #[inline]
    unsafe fn sq_array(&self) -> *mut u32 {
        self.sq_ring.add(self.params.sq_off.array as usize) as *mut u32
    }
    #[inline]
    unsafe fn cq_head(&self) -> *mut u32 {
        self.cq_ring.add(self.params.cq_off.head as usize) as *mut u32
    }
    #[inline]
    unsafe fn cq_tail(&self) -> *mut u32 {
        self.cq_ring.add(self.params.cq_off.tail as usize) as *mut u32
    }
    #[inline]
    unsafe fn cq_mask(&self) -> u32 {
        *(self.cq_ring.add(self.params.cq_off.ring_mask as usize) as *const u32)
    }
    #[inline]
    unsafe fn cqes(&self) -> *mut uring::Cqe {
        self.cq_ring.add(self.params.cq_off.cqes as usize) as *mut uring::Cqe
    }

    unsafe fn queue(&mut self, sqe: &uring::Sqe) -> bool {
        let tail = *self.sq_tail();
        let head = *self.sq_head();
        if tail.wrapping_sub(head) >= self.params.sq_entries {
            return false;
        }
        let idx = tail & self.sq_mask();
        let dst = self.sqes.add(idx as usize * 64) as *mut uring::Sqe;
        std::ptr::write(dst, *sqe);
        *self.sq_array().add(idx as usize) = idx;
        *self.sq_tail() = tail.wrapping_add(1);
        true
    }

    unsafe fn flush(&mut self) {
        let tail = *self.sq_tail();
        let to_submit = tail.wrapping_sub(self.submitted_tail);
        if to_submit == 0 {
            return;
        }
        let _ = libc::syscall(
            libc::SYS_io_uring_enter,
            self.fd as libc::c_long,
            to_submit as libc::c_long,
            0usize as libc::c_long,
            0usize as libc::c_long,
            0usize as libc::c_long,
            0usize as libc::c_long,
        );
        self.submitted_tail = tail;
    }

    /// submit a one-shot POLL_ADD for `fd`; user_data packs (gen, token)
    unsafe fn arm(&mut self, fd: i32, events: i64, token: i64, gen: u32) {
        let uring::Sqe { .. } = uring::Sqe::default();
        let mut pe: u32 = 0;
        if events & R_READ != 0 {
            pe |= libc::POLLIN as u32;
        }
        if events & R_WRITE != 0 {
            pe |= libc::POLLOUT as u32;
        }
        let sqe = uring::Sqe {
            opcode: uring::IORING_OP_POLL_ADD,
            fd,
            poll32_events: pe,
            user_data: pack(gen, token),
            ..Default::default()
        };
        if !self.queue(&sqe) {
            self.flush();
            let _ = self.queue(&sqe);
        }
    }

    unsafe fn drain(&mut self, buf: &mut EvBuf, regs: &[Reg]) {
        let mask = self.cq_mask();
        loop {
            let head = *self.cq_head();
            let tail = *self.cq_tail();
            if head == tail {
                break;
            }
            let idx = head & mask;
            let cqe = std::ptr::read(self.cqes().add(idx as usize));
            *self.cq_head() = head.wrapping_add(1);
            if cqe.res < 0 {
                // -ECANCELED / -EBADF from a retired wait: ignore
                continue;
            }
            let (gen, token) = unpack(cqe.user_data);
            let mask_poll = cqe.res as u32;
            let mut re = 0i64;
            if mask_poll & (libc::POLLIN as u32) != 0 {
                re |= R_READ;
            }
            if mask_poll & (libc::POLLOUT as u32) != 0 {
                re |= R_WRITE;
            }
            if mask_poll & (libc::POLLERR as u32) != 0 {
                re |= R_ERR;
            }
            if mask_poll & (libc::POLLHUP as u32) != 0 {
                re |= R_HUP;
            }
            // accept only if the registration is still current
            let cur = regs.iter().find(|r| r.token == token);
            match cur {
                Some(r) if r.gen == gen => {
                    evbuf_push(buf, token, re);
                    // re-arm the one-shot poll for the next wait
                    let ngen = gen.wrapping_add(1).max(1);
                    self.arm(r.fd, r.events, token, ngen);
                }
                _ => {}
            }
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for IoUring {
    fn drop(&mut self) {
        unsafe {
            if !self.sqes.is_null() {
                libc::munmap(self.sqes as *mut libc::c_void, self.sqes_sz);
            }
            if !self.cq_ring.is_null() {
                libc::munmap(self.cq_ring as *mut libc::c_void, self.cq_ring_sz);
            }
            if !self.sq_ring.is_null() {
                libc::munmap(self.sq_ring as *mut libc::c_void, self.sq_ring_sz);
            }
            if self.fd >= 0 {
                libc::close(self.fd);
            }
        }
    }
}

#[cfg(target_os = "linux")]
#[inline]
fn pack(gen: u32, token: i64) -> u64 {
    ((gen as u64) << 32) | ((token as u32) as u64)
}

#[cfg(target_os = "linux")]
#[inline]
fn unpack(v: u64) -> (u32, i64) {
    ((v >> 32) as u32, (v & 0xffff_ffff) as u32 as i32 as i64)
}

#[cfg(target_os = "linux")]
fn poll_iouring(ev: &mut EvLoop, timeout_ms: i64, out: i64) -> i64 {
    let ring = match ev.ring.as_mut() {
        Some(r) => r,
        None => return -(libc::ENOSYS as i64),
    };
    unsafe {
        ring.flush();
        let buf = &mut *(out as *mut EvBuf);
        // ready completions may already be queued (sync poll)
        ring.drain(buf, &ev.regs);
        if buf.count > 0 {
            return buf.count;
        }
        // wait for CQ readiness on the ring fd
        let mut pfd = libc::pollfd {
            fd: ring.fd,
            events: libc::POLLIN,
            revents: 0,
        };
        let n = libc::poll(&mut pfd, 1, timeout_ms as libc::c_int);
        if n < 0 {
            if *libc::__errno_location() == libc::EINTR {
                return 0;
            }
            return errno();
        }
        if n > 0 {
            ring.drain(buf, &ev.regs);
        }
        buf.count
    }
}
