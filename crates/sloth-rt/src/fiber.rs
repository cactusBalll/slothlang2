//! Stackful coroutines (fiber 2.0): native stack switching over ARC.
//!
//! Each fiber owns an independently `mmap`'d stack with a trailing guard page;
//! switching saves the ABI callee-saved registers + stack pointer and restores
//! the target's. The payload word crosses the boundary untouched (tagged i64),
//! so codegen never needs a marshalling step.
//!
//! Ownership: the payload of `resume`/`yield`/`transfer`/`create` is delivered
//! by *retaining* it on the receiving side (a plain borrowed argument at the
//! call site) — the runtime keeps counts balanced, so missed transfer
//! instrumentation cannot dangle.
//!
//! `sloth_fiber_switch_asm` and `sloth_fiber_trampoline` are hand-written in
//! `global_asm!` per architecture (x86_64 SysV / aarch64 AAPCS64); they are
//! internal to this shared object (the compiler only names the high-level
//! `sloth_fiber_*` entry points).

use crate::panics;
use crate::rc::{self, w_is_ref, w_unref};
use crate::strings::StrT;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

// ---------------- fiber states ----------------

const STATE_NEW: i64 = 0;
const STATE_SUSPENDED: i64 = 1;
const STATE_RUNNING: i64 = 2;
const STATE_DONE: i64 = 3;
const STATE_ERROR: i64 = 4;

/// callee-saved register file + stack pointer (arch-specific population)
#[repr(C)]
pub(crate) struct Ctx {
    pub regs: [u64; 24],
}

impl Ctx {
    const fn zero() -> Ctx {
        Ctx { regs: [0; 24] }
    }
}

#[repr(C)]
pub(crate) struct FiberObj {
    state: i64,
    /// resumer to return to on yield / completion (raw, caller is live)
    prev: *mut FiberObj,
    /// owned payload delivered to this fiber, taken by its next yield
    inbox: i64,
    stack_base: *mut u8,
    stack_size: usize,
    ctx: Ctx,
    /// entry closure word `{tagged fnptr, env}` (owned)
    entry: i64,
    /// first argument handed to the entry closure (owned until started)
    init: i64,
    /// setjmp landing pad used by cooperative cancellation
    jb: [u64; 32],
    cancel: i64,
}

// ---------------- assembly primitives ----------------

extern "C" {
    /// save the running context into `save`, restore `restore`.
    fn sloth_fiber_switch_asm(save: *mut Ctx, restore: *const Ctx);
    /// first-run entry point: reads the fiber pointer out of the restored ctx.
    fn sloth_fiber_trampoline();
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    ".text",
    ".globl sloth_fiber_switch_asm",
    ".type sloth_fiber_switch_asm, @function",
    "sloth_fiber_switch_asm:",
    "mov [rdi + 0], rsp",
    "mov [rdi + 8], rbx",
    "mov [rdi + 16], rbp",
    "mov [rdi + 24], r12",
    "mov [rdi + 32], r13",
    "mov [rdi + 40], r14",
    "mov [rdi + 48], r15",
    "mov rsp, [rsi + 0]",
    "mov rbx, [rsi + 8]",
    "mov rbp, [rsi + 16]",
    "mov r12, [rsi + 24]",
    "mov r13, [rsi + 32]",
    "mov r14, [rsi + 40]",
    "mov r15, [rsi + 48]",
    "ret",
    ".size sloth_fiber_switch_asm, .-sloth_fiber_switch_asm",
    ".globl sloth_fiber_trampoline",
    ".type sloth_fiber_trampoline, @function",
    "sloth_fiber_trampoline:",
    "mov rdi, r12",
    "call sloth_fiber_entry",
    "ud2",
    ".size sloth_fiber_trampoline, .-sloth_fiber_trampoline",
);

#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    ".text",
    ".globl sloth_fiber_switch_asm",
    ".type sloth_fiber_switch_asm, @function",
    "sloth_fiber_switch_asm:",
    "mov x9, sp",
    "str x9, [x0, #0]",
    "stp x19, x20, [x0, #8]",
    "stp x21, x22, [x0, #24]",
    "stp x23, x24, [x0, #40]",
    "stp x25, x26, [x0, #56]",
    "stp x27, x28, [x0, #72]",
    "str x29, [x0, #88]",
    "str x30, [x0, #96]",
    "stp d8, d9, [x0, #104]",
    "stp d10, d11, [x0, #120]",
    "stp d12, d13, [x0, #136]",
    "stp d14, d15, [x0, #152]",
    "ldr x9, [x1, #0]",
    "mov sp, x9",
    "ldp x19, x20, [x1, #8]",
    "ldp x21, x22, [x1, #24]",
    "ldp x23, x24, [x1, #40]",
    "ldp x25, x26, [x1, #56]",
    "ldp x27, x28, [x1, #72]",
    "ldr x29, [x1, #88]",
    "ldr x30, [x1, #96]",
    "ldp d8, d9, [x1, #104]",
    "ldp d10, d11, [x1, #120]",
    "ldp d12, d13, [x1, #136]",
    "ldp d14, d15, [x1, #152]",
    "ret",
    ".size sloth_fiber_switch_asm, .-sloth_fiber_switch_asm",
    ".globl sloth_fiber_trampoline",
    ".type sloth_fiber_trampoline, @function",
    "sloth_fiber_trampoline:",
    "mov x0, x19",
    "bl sloth_fiber_entry",
    "brk #0",
    ".size sloth_fiber_trampoline, .-sloth_fiber_trampoline",
);

// ---------------- current fiber ----------------

static mut CUR: *mut FiberObj = std::ptr::null_mut();

/// process-stack sentinel fiber: the resumer context of top-level code
static mut MAIN: FiberObj = FiberObj {
    state: STATE_RUNNING,
    prev: std::ptr::null_mut(),
    inbox: 0,
    stack_base: std::ptr::null_mut(),
    stack_size: 0,
    ctx: Ctx::zero(),
    entry: 0,
    init: 0,
    jb: [0; 32],
    cancel: 0,
};

#[inline]
unsafe fn cur() -> *mut FiberObj {
    if CUR.is_null() {
        CUR = core::ptr::addr_of_mut!(MAIN);
    }
    CUR
}

// ---------------- guard page diagnostics ----------------

const MAX_GUARDS: usize = 128;
static mut GUARDS: [(usize, usize); MAX_GUARDS] = [(0, 0); MAX_GUARDS];
static GUARD_N: AtomicUsize = AtomicUsize::new(0);

unsafe fn register_guard(lo: usize, hi: usize) {
    let n = GUARD_N.load(Ordering::Relaxed);
    if n < MAX_GUARDS {
        GUARDS[n] = (lo, hi);
        GUARD_N.store(n + 1, Ordering::Relaxed);
    }
}

unsafe fn unregister_guard(lo: usize) {
    let n = GUARD_N.load(Ordering::Relaxed);
    for i in 0..n {
        if GUARDS[i].0 == lo {
            GUARDS[i] = GUARDS[n - 1];
            GUARD_N.store(n - 1, Ordering::Relaxed);
            return;
        }
    }
}

extern "C" fn segv_handler(_sig: libc::c_int, info: *mut libc::siginfo_t, _ctx: *mut libc::c_void) {
    let addr = if info.is_null() {
        0
    } else {
        unsafe { (*info).si_addr() as usize }
    };
    unsafe {
        let n = GUARD_N.load(Ordering::Relaxed);
        for i in 0..n {
            let (lo, hi) = GUARDS[i];
            if addr >= lo && addr < hi {
                let msg = b"sloth panic: fiber stack overflow\n";
                libc::write(2, msg.as_ptr() as *const libc::c_void, msg.len());
                libc::_exit(1);
            }
        }
        // not a fiber stack: restore default disposition and let it re-fault
        libc::signal(libc::SIGSEGV, libc::SIG_DFL);
    }
}

static INSTALL_SIG: AtomicUsize = AtomicUsize::new(0);
const ALT_SIZE: usize = 64 * 1024;
static mut ALT_STACK: [u8; ALT_SIZE] = [0; ALT_SIZE];

unsafe fn install_sigsegv() {
    if INSTALL_SIG.swap(1, Ordering::Relaxed) != 0 {
        return;
    }
    // overflow faults cannot run a handler on the exhausted fiber stack
    let ss = libc::stack_t {
        ss_sp: core::ptr::addr_of_mut!(ALT_STACK) as *mut libc::c_void,
        ss_flags: 0,
        ss_size: ALT_SIZE,
    };
    libc::sigaltstack(&ss, std::ptr::null_mut());
    let mut sa: libc::sigaction = std::mem::zeroed();
    sa.sa_sigaction = segv_handler as *const () as usize;
    sa.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK;
    libc::sigemptyset(&mut sa.sa_mask);
    libc::sigaction(libc::SIGSEGV, &sa, std::ptr::null_mut());
}

// ---------------- stack allocation ----------------

const DEFAULT_STACK: usize = 256 * 1024;

unsafe fn alloc_stack(bytes: usize) -> (*mut u8, usize) {
    let page = 4096usize;
    let want = if bytes == 0 { DEFAULT_STACK } else { bytes };
    let total = (want + page - 1) & !(page - 1);
    let total = total + page; // trailing guard page at the low end
    let p = libc::mmap(
        std::ptr::null_mut(),
        total,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_STACK,
        -1,
        0,
    );
    if p == libc::MAP_FAILED {
        panics::panic_msg("fiber: cannot allocate stack");
    }
    let base = p as *mut u8;
    if libc::mprotect(base as *mut libc::c_void, page, libc::PROT_NONE) != 0 {
        libc::munmap(base as *mut libc::c_void, total);
        panics::panic_msg("fiber: cannot set guard page");
    }
    install_sigsegv();
    register_guard(base as usize, base as usize + page);
    (base, total)
}

/// prepare a first-run context: stack top + trampoline return address
unsafe fn prime_ctx(f: *mut FiberObj) {
    let ff = &mut *f;
    let top = ff.stack_base as usize + ff.stack_size;
    #[cfg(target_arch = "x86_64")]
    {
        let sp = top & !15;
        let slot = sp - 8;
        *(slot as *mut usize) = sloth_fiber_trampoline as *const () as usize;
        ff.ctx.regs[0] = slot as u64; // rsp
        ff.ctx.regs[3] = f as u64; // r12 carries the fiber pointer
    }
    #[cfg(target_arch = "aarch64")]
    {
        let sp = top & !15;
        ff.ctx.regs[0] = sp as u64; // sp
        ff.ctx.regs[9] = f as u64; // x19 carries the fiber pointer
        ff.ctx.regs[11] = sloth_fiber_trampoline as usize as u64; // x30 (lr)
    }
}

// ---------------- fiber lifecycle ----------------

unsafe fn fiber_setup(entry_w: i64, init_w: i64, stack_w: i64) -> i64 {
    let (base, total) = alloc_stack(rc::dec_i(stack_w).max(0) as usize);
    let p = rc::rc_addr(std::mem::size_of::<FiberObj>(), Some(fiber_dtor)) as *mut FiberObj;
    let f = &mut *p;
    f.state = STATE_NEW;
    f.prev = std::ptr::null_mut();
    f.inbox = 0;
    f.stack_base = base;
    f.stack_size = total;
    f.ctx = Ctx::zero();
    // entry is retained; init is retained and consumed by the entry closure
    f.entry = if entry_w == 0 {
        0
    } else {
        rc::sloth_rc_retain(entry_w)
    };
    f.init = if init_w == 0 {
        0
    } else {
        rc::sloth_rc_retain(init_w)
    };
    f.cancel = 0;
    prime_ctx(p);
    rc::w_ref(p as usize)
}

#[no_mangle]
pub extern "C" fn sloth_fiber_create(entry_w: i64, init_w: i64) -> i64 {
    unsafe { fiber_setup(entry_w, init_w, 0) }
}

#[no_mangle]
pub extern "C" fn sloth_fiber_create_with(entry_w: i64, init_w: i64, stack_w: i64) -> i64 {
    unsafe { fiber_setup(entry_w, init_w, stack_w) }
}

fn fiber_dtor(p: usize, _aux: u64) {
    unsafe {
        let f = &mut *(p as *mut FiberObj);
        #[cfg(debug_assertions)]
        if f.state == STATE_SUSPENDED {
            panics::panic_msg("abandoned suspended fiber");
        }
        if f.state == STATE_NEW && f.init != 0 {
            rc::sloth_rc_release(f.init);
        }
        if f.inbox != 0 {
            rc::sloth_rc_release(f.inbox);
        }
        if f.entry != 0 {
            rc::sloth_rc_release(f.entry);
        }
        if !f.stack_base.is_null() {
            unregister_guard(f.stack_base as usize);
            libc::munmap(f.stack_base as *mut libc::c_void, f.stack_size);
            f.stack_base = std::ptr::null_mut();
        }
    }
}

extern "C" {
    fn setjmp(env: *mut u64) -> libc::c_int;
    fn longjmp(env: *mut u64, val: libc::c_int) -> !;
}

/// first-run body: invoke the entry closure with the pre-bound `init`, then
/// retire the fiber. Also the cancellation landing pad (setjmp returns twice).
#[no_mangle]
pub(crate) extern "C" fn sloth_fiber_entry(f: *mut FiberObj) -> ! {
    unsafe {
        let ff = &mut *f;
        if setjmp(ff.jb.as_mut_ptr()) != 0 {
            terminate(f);
        }
        ff.state = STATE_RUNNING;
        let init = ff.init;
        ff.init = 0;
        let entry = ff.entry;
        let fnptr_w = crate::objects::sloth_obj_field(entry, rc::enc_i(0));
        let env = crate::objects::sloth_obj_field(entry, rc::enc_i(1));
        let raw = (fnptr_w & !1) as usize;
        let cb: extern "C" fn(i64, i64) -> i64 = core::mem::transmute(raw);
        cb(env, init);
        // the closure ABI borrows its argument: drop the fiber's owning +1
        if init != 0 {
            rc::sloth_rc_release(init);
        }
        terminate(f);
    }
}

/// retire the running fiber: deliver nil to its resumer and never return
unsafe fn terminate(f: *mut FiberObj) -> ! {
    let ff = &mut *f;
    if ff.state != STATE_ERROR {
        ff.state = STATE_DONE;
    }
    // an unconsumed resume payload (closure that never yielded) is released
    if ff.inbox != 0 {
        rc::sloth_rc_release(ff.inbox);
        ff.inbox = 0;
    }
    let to = ff.prev;
    (*to).inbox = 0;
    CUR = to;
    sloth_fiber_switch_asm(&mut ff.ctx, &(*to).ctx);
    core::hint::unreachable_unchecked()
}

// ---------------- public coroutine API ----------------

#[no_mangle]
pub extern "C" fn sloth_fiber_resume(f_w: i64, v_w: i64, box_w: i64) -> i64 {
    if !w_is_ref(f_w) {
        return 0;
    }
    unsafe {
        let f = w_unref(f_w) as *mut FiberObj;
        if (*f).state != STATE_NEW && (*f).state != STATE_SUSPENDED {
            return 0;
        }
        (*f).inbox = if v_w == 0 {
            0
        } else {
            rc::sloth_rc_retain(v_w)
        };
        let from = cur();
        (*f).prev = from;
        CUR = f;
        sloth_fiber_switch_asm(&mut (*from).ctx, &(*f).ctx);
        CUR = from;
        let got = (*from).inbox;
        (*from).inbox = 0;
        if (*f).state == STATE_SUSPENDED {
            if box_w != 0 {
                return crate::boxopt::sloth_box_new(got);
            }
            got
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_fiber_transfer(f_w: i64, v_w: i64, box_w: i64) -> i64 {
    if !w_is_ref(f_w) {
        return 0;
    }
    unsafe {
        let f = w_unref(f_w) as *mut FiberObj;
        if (*f).state != STATE_NEW && (*f).state != STATE_SUSPENDED {
            return 0;
        }
        (*f).inbox = if v_w == 0 {
            0
        } else {
            rc::sloth_rc_retain(v_w)
        };
        let from = cur();
        if (*f).state == STATE_NEW {
            // bootstrap a never-started fiber so its first yield has a target
            (*f).prev = from;
        }
        CUR = f;
        sloth_fiber_switch_asm(&mut (*from).ctx, &(*f).ctx);
        CUR = from;
        let got = (*from).inbox;
        (*from).inbox = 0;
        if (*f).state == STATE_SUSPENDED {
            if box_w != 0 {
                return crate::boxopt::sloth_box_new(got);
            }
            got
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_fiber_yield(v_w: i64) -> i64 {
    unsafe {
        let f = cur();
        if (*f).prev.is_null() {
            panics::panic_msg("fiber.yield outside a fiber");
        }
        if (*f).cancel != 0 {
            longjmp((*f).jb.as_mut_ptr(), 1);
        }
        // consume the resume payload that woke us *before* suspending, so a
        // later resume cannot overwrite an undelivered value
        let incoming = (*f).inbox;
        (*f).inbox = 0;
        (*f).state = STATE_SUSPENDED;
        let to = (*f).prev;
        (*to).inbox = if v_w == 0 {
            0
        } else {
            rc::sloth_rc_retain(v_w)
        };
        CUR = to;
        sloth_fiber_switch_asm(&mut (*f).ctx, &(*to).ctx);
        CUR = f;
        (*f).state = STATE_RUNNING;
        if (*f).cancel != 0 {
            longjmp((*f).jb.as_mut_ptr(), 1);
        }
        incoming
    }
}

#[no_mangle]
pub extern "C" fn sloth_fiber_error(msg_w: i64) -> i64 {
    unsafe {
        if w_is_ref(msg_w) {
            let td = w_unref(msg_w) as *const StrT;
            let sl = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
            let mut e = std::io::stderr();
            let _ = e.write_all(b"sloth fiber error: ");
            let _ = e.write_all(sl);
            let _ = e.write_all(b"\n");
        }
        let f = cur();
        (*f).state = STATE_ERROR;
        terminate(f);
    }
}

#[no_mangle]
pub extern "C" fn sloth_fiber_check(f_w: i64) -> i64 {
    if !w_is_ref(f_w) {
        return 0;
    }
    unsafe {
        let f = w_unref(f_w) as *const FiberObj;
        rc::enc_i(((*f).state == STATE_ERROR) as i64)
    }
}

#[no_mangle]
pub extern "C" fn sloth_fiber_resumable(f_w: i64) -> i64 {
    if !w_is_ref(f_w) {
        return 0;
    }
    unsafe {
        let f = w_unref(f_w) as *const FiberObj;
        let s = (*f).state;
        rc::enc_i((s == STATE_NEW || s == STATE_SUSPENDED) as i64)
    }
}

/// cooperative cancellation: mark a suspended fiber and resume it so its next
/// `yield` longjmps to the entry landing pad. The fiber stack is reclaimed;
/// owned references in skipped frames are abandoned (leak, never dangling).
#[no_mangle]
pub extern "C" fn sloth_fiber_cancel(f_w: i64) -> i64 {
    if !w_is_ref(f_w) {
        return 0;
    }
    unsafe {
        let f = w_unref(f_w) as *mut FiberObj;
        if (*f).state != STATE_SUSPENDED {
            return 0;
        }
        (*f).cancel = 1;
        (*f).inbox = 0;
        let from = cur();
        (*f).prev = from;
        CUR = f;
        sloth_fiber_switch_asm(&mut (*from).ctx, &(*f).ctx);
        CUR = from;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn counting_entry(_env: i64, _init: i64) -> i64 {
        let mut i = 1i64;
        while i <= 3 {
            let _got = sloth_fiber_yield(rc::enc_i(i));
            i += 1;
        }
        0
    }

    #[test]
    fn switch_roundtrip() {
        let base = rc::dec_i(rc::sloth_rc_live());
        let fp = rc::w_ref(counting_entry as *const () as usize);
        let clo = crate::objects::sloth_closure_new(fp, 0);
        let f = sloth_fiber_create(clo, rc::enc_i(7));
        let mut got = Vec::new();
        loop {
            let r = sloth_fiber_resume(f, rc::enc_i(100), 0);
            if r == 0 {
                break;
            }
            got.push(rc::dec_i(r));
            assert!(got.len() < 10, "runaway fiber");
        }
        assert_eq!(got, vec![1, 2, 3]);
        assert_eq!(rc::dec_i(sloth_fiber_resumable(f)), 0);
        rc::sloth_rc_release(f);
        rc::sloth_rc_release(clo);
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), base);
    }

    extern "C" fn million_entry(_env: i64, _init: i64) -> i64 {
        let mut i = 0i64;
        while i < 1_000_000 {
            let _ = sloth_fiber_yield(rc::enc_i(0));
            i += 1;
        }
        0
    }

    #[test]
    fn million_switches() {
        let base = rc::dec_i(rc::sloth_rc_live());
        let fp = rc::w_ref(million_entry as *const () as usize);
        let clo = crate::objects::sloth_closure_new(fp, 0);
        let f = sloth_fiber_create(clo, 0);
        let mut n = 0i64;
        loop {
            // box=1 makes a yielded value 0 distinguishable from completion nil
            let r = sloth_fiber_resume(f, 0, 1);
            if r == 0 {
                break;
            }
            rc::sloth_rc_release(r);
            n += 1;
        }
        assert_eq!(n, 1_000_000);
        rc::sloth_rc_release(f);
        rc::sloth_rc_release(clo);
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), base);
    }

    extern "C" fn cancel_entry(_env: i64, _init: i64) -> i64 {
        loop {
            let _ = sloth_fiber_yield(rc::enc_i(1));
        }
    }

    #[test]
    fn cancel_unwinds() {
        let base = rc::dec_i(rc::sloth_rc_live());
        let fp = rc::w_ref(cancel_entry as *const () as usize);
        let clo = crate::objects::sloth_closure_new(fp, 0);
        let f = sloth_fiber_create(clo, 0);
        let r = sloth_fiber_resume(f, 0, 0);
        assert_eq!(rc::dec_i(r), 1);
        assert_eq!(rc::dec_i(sloth_fiber_resumable(f)), 1);
        sloth_fiber_cancel(f);
        assert_eq!(rc::dec_i(sloth_fiber_resumable(f)), 0);
        rc::sloth_rc_release(f);
        rc::sloth_rc_release(clo);
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), base);
    }
}
