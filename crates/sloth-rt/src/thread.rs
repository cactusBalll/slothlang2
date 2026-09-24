//! OS-level threads (TH-P1): shared-memory concurrency over the atomic ARC
//! core. A spawned thread runs a first-class closure value through the same
//! uniform bridge ABI as fibers `(env, arg) -> i64`; the handle is an ARC
//! object holding the join state, the worker's result and the OS thread.
//!
//! Ownership: spawn retains the entry closure and the payload argument; the
//! worker delivers its return word as an owned +1 into the handle (or sheds
//! it with the handle if the thread is detached / abandoned). Each worker
//! holds an extra reference on its own handle for its whole lifetime, so a
//! dropped handle cannot free state the worker still writes.

use crate::panics;
use crate::rc::{self, w_unref};
use std::sync::{Condvar, Mutex};

struct ThreadState {
    done: bool,
    joined: bool,
    detached: bool,
}

struct ThreadHandle {
    state: Mutex<ThreadState>,
    cond: Condvar,
    /// owned entry closure `{tagged fnptr, env}` (retained by spawn)
    entry: i64,
    /// owned argument word (retained by spawn iff `aref`)
    arg: i64,
    /// argument type carries references (0 = value arg, stored raw)
    aref: i64,
    /// owned worker result (transferred to `join`, else shed by the dtor)
    result: i64,
    /// result type carries references (0 = value result, stored raw)
    rref: i64,
    /// OS thread reaper; `None` once joined or detached
    joinable: Option<std::thread::JoinHandle<()>>,
}

/// worker body: run the entry closure, publish the result, drop the worker's
/// self-reference. Keeps the raw pointer valid throughout (self +1).
unsafe fn worker(ptr: usize) {
    let h = ptr as *mut ThreadHandle;
    let entry = (*h).entry;
    let arg = (*h).arg;
    let fnptr_w = crate::objects::__sloth_obj_field(entry, rc::enc_i(0));
    let env = crate::objects::__sloth_obj_field(entry, rc::enc_i(1));
    let raw = fnptr_w as usize;
    let cb: extern "C" fn(i64, i64) -> i64 = core::mem::transmute(raw);
    let res = cb(env, arg);
    {
        let mut st = (*h).state.lock().unwrap();
        (*h).result = res;
        st.done = true;
        (*h).cond.notify_all();
    }
    // TH-P2: a thread must have driven its fiber set to Done/Error before exit
    crate::fiber::__sloth_fiber_thread_exit();
    // release the worker's self-reference last (may run the dtor)
    rc::__sloth_rc_release(rc::w_ref(h as usize));
}

fn thread_dtor(p: usize, _aux: u64) {
    unsafe {
        let h = &mut *(p as *mut ThreadHandle);
        #[cfg(debug_assertions)]
        {
            let st = h.state.lock().unwrap();
            if !st.joined && !st.detached {
                drop(st);
                panics::panic_msg("abandoned thread handle (join or detach it)");
            }
        }
        // shed any untaken result (reference results only), then the owned
        // closure + reference argument
        if h.rref != 0 && h.result != 0 {
            rc::__sloth_rc_release(h.result);
        }
        if h.aref != 0 && h.arg != 0 {
            rc::__sloth_rc_release(h.arg);
        }
        if h.entry != 0 {
            rc::__sloth_rc_release(h.entry);
        }
        std::ptr::drop_in_place(p as *mut ThreadHandle);
    }
}

#[no_mangle]
pub extern "C" fn __sloth_thread_spawn(entry_w: i64, arg_w: i64, aref: i64, rref: i64) -> i64 {
    unsafe {
        let p = rc::rc_addr(std::mem::size_of::<ThreadHandle>(), Some(thread_dtor))
            as *mut ThreadHandle;
        std::ptr::write(
            p,
            ThreadHandle {
                state: Mutex::new(ThreadState {
                    done: false,
                    joined: false,
                    detached: false,
                }),
                cond: Condvar::new(),
                entry: if entry_w == 0 {
                    0
                } else {
                    rc::__sloth_rc_retain(entry_w)
                },
                arg: if aref != 0 && arg_w != 0 {
                    rc::__sloth_rc_retain(arg_w)
                } else {
                    arg_w
                },
                aref,
                result: 0,
                rref,
                joinable: None,
            },
        );
        let hw = rc::w_ref(p as usize);
        // worker self-reference: keeps the handle (and the worker's writes)
        // valid even if the caller drops its handle immediately
        rc::__sloth_rc_retain(hw);
        let ptr = p as usize;
        match std::thread::Builder::new().spawn(move || worker(ptr)) {
            Ok(jh) => {
                (*p).joinable = Some(jh);
                hw
            }
            Err(_) => {
                // never return a dead handle: exit with a diagnosis (the
                // worker +1 and caller +1 leak, but the process is ending)
                panics::panic_msg("thread: cannot spawn OS thread");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn __sloth_thread_join(h_w: i64) -> i64 {
    if h_w == 0 {
        return 0;
    }
    unsafe {
        let h = w_unref(h_w) as *mut ThreadHandle;
        let jh = {
            let mut st = (*h).state.lock().unwrap();
            if st.joined {
                drop(st);
                panics::panic_msg("thread handle joined twice");
            }
            if st.detached {
                drop(st);
                panics::panic_msg("cannot join a detached thread handle");
            }
            while !st.done {
                st = (*h).cond.wait(st).unwrap();
            }
            st.joined = true;
            (*h).joinable.take()
        };
        if let Some(jh) = jh {
            let _ = jh.join();
        }
        // transfer the owned result out (zero the slot so the dtor cannot
        // double-release it)
        let r = (*h).result;
        (*h).result = 0;
        r
    }
}

#[no_mangle]
pub extern "C" fn __sloth_thread_detach(h_w: i64) -> i64 {
    if h_w == 0 {
        return 0;
    }
    unsafe {
        let h = w_unref(h_w) as *mut ThreadHandle;
        let jh = {
            let mut st = (*h).state.lock().unwrap();
            if st.joined || st.detached {
                drop(st);
                panics::panic_msg("thread handle already joined/detached");
            }
            st.detached = true;
            (*h).joinable.take()
        };
        // dropping the std handle detaches the OS thread
        drop(jh);
    }
    0
}

/// current thread identity (pthread_self; stable for the thread's lifetime)
#[no_mangle]
pub extern "C" fn __sloth_thread_current_id() -> i64 {
    rc::enc_i(unsafe { libc::pthread_self() } as usize as i64)
}

/// cooperative yield (`sched_yield`; spin-wait companions)
#[no_mangle]
pub extern "C" fn __sloth_thread_yield_now() -> i64 {
    std::thread::yield_now();
    0
}
