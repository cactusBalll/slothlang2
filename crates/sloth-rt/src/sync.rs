//! Synchronization primitives (TH-P2): an opaque blocking `Mutex` over
//! `pthread_mutex_t` and an opaque `AtomicInt` cell with 63-bit word
//! semantics. Both are ARC objects (builtin opaque handles), so the emitted
//! retain/release protocol cleans them up automatically.

use crate::rc::{self, w_is_ref, w_unref};
use std::sync::atomic::{AtomicI64, Ordering};

// ---------------- Mutex ----------------

#[repr(C)]
struct MutexObj {
    m: libc::pthread_mutex_t,
}

fn mutex_dtor(p: usize, _aux: u64) {
    unsafe {
        libc::pthread_mutex_destroy(&mut (*(p as *mut MutexObj)).m);
        std::ptr::drop_in_place(p as *mut MutexObj);
    }
}

#[no_mangle]
pub extern "C" fn sloth_mutex_new() -> i64 {
    unsafe {
        let p = rc::rc_addr(std::mem::size_of::<MutexObj>(), Some(mutex_dtor))
            as *mut MutexObj;
        let m = &mut (*p).m;
        libc::pthread_mutex_init(m, std::ptr::null());
        rc::w_ref(p as usize)
    }
}

#[no_mangle]
pub extern "C" fn sloth_mutex_lock(m_w: i64) -> i64 {
    if w_is_ref(m_w) {
        unsafe {
            libc::pthread_mutex_lock(&mut (*(w_unref(m_w) as *mut MutexObj)).m);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn sloth_mutex_unlock(m_w: i64) -> i64 {
    if w_is_ref(m_w) {
        unsafe {
            libc::pthread_mutex_unlock(&mut (*(w_unref(m_w) as *mut MutexObj)).m);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn sloth_mutex_try_lock(m_w: i64) -> i64 {
    if !w_is_ref(m_w) {
        return 0;
    }
    unsafe {
        let r = libc::pthread_mutex_trylock(&mut (*(w_unref(m_w) as *mut MutexObj)).m);
        rc::enc_i((r == 0) as i64)
    }
}

/// `m.with(|g| { ... })`: lock, run the closure bridge `(env, 0)`, unlock.
/// The unique recommended acquisition point (design §4.3).
#[no_mangle]
pub extern "C" fn sloth_mutex_with(m_w: i64, f_w: i64) -> i64 {
    if !w_is_ref(m_w) || !w_is_ref(f_w) {
        return 0;
    }
    unsafe {
        let m = &mut (*(w_unref(m_w) as *mut MutexObj)).m;
        libc::pthread_mutex_lock(m);
        let fnptr_w = crate::objects::sloth_obj_field(f_w, rc::enc_i(0));
        let env = crate::objects::sloth_obj_field(f_w, rc::enc_i(1));
        let raw = (fnptr_w & !1) as usize;
        let cb: extern "C" fn(i64, i64) -> i64 = core::mem::transmute(raw);
        cb(env, 0);
        libc::pthread_mutex_unlock(m);
    }
    0
}

// ---------------- AtomicInt ----------------

#[repr(C)]
struct AtomicObj {
    v: AtomicI64,
}

fn atomic_dtor(p: usize, _aux: u64) {
    unsafe {
        std::ptr::drop_in_place(p as *mut AtomicObj);
    }
}

#[no_mangle]
pub extern "C" fn sloth_atomic_new(init_w: i64) -> i64 {
    unsafe {
        let p = rc::rc_addr(std::mem::size_of::<AtomicObj>(), Some(atomic_dtor))
            as *mut AtomicObj;
        std::ptr::write(
            p,
            AtomicObj {
                v: AtomicI64::new(init_w),
            },
        );
        rc::w_ref(p as usize)
    }
}

fn atomic_of(w: i64) -> Option<&'static AtomicI64> {
    if w_is_ref(w) {
        Some(unsafe { &(*(w_unref(w) as *const AtomicObj)).v })
    } else {
        None
    }
}

#[no_mangle]
pub extern "C" fn sloth_atomic_load(a_w: i64) -> i64 {
    atomic_of(a_w).map(|a| a.load(Ordering::SeqCst)).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn sloth_atomic_store(a_w: i64, v_w: i64) -> i64 {
    if let Some(a) = atomic_of(a_w) {
        a.store(v_w, Ordering::SeqCst);
    }
    0
}

#[no_mangle]
pub extern "C" fn sloth_atomic_add(a_w: i64, d_w: i64) -> i64 {
    let a = match atomic_of(a_w) {
        Some(a) => a,
        None => return 0,
    };
    loop {
        let cur = a.load(Ordering::SeqCst);
        let next = rc::enc_i(rc::dec_i(cur).wrapping_add(rc::dec_i(d_w)));
        match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return next,
            Err(_) => continue,
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_atomic_sub(a_w: i64, d_w: i64) -> i64 {
    let a = match atomic_of(a_w) {
        Some(a) => a,
        None => return 0,
    };
    loop {
        let cur = a.load(Ordering::SeqCst);
        let next = rc::enc_i(rc::dec_i(cur).wrapping_sub(rc::dec_i(d_w)));
        match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return next,
            Err(_) => continue,
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_atomic_cas(a_w: i64, old_w: i64, new_w: i64) -> i64 {
    let a = match atomic_of(a_w) {
        Some(a) => a,
        None => return 0,
    };
    match a.compare_exchange(old_w, new_w, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(_) => rc::enc_i(1),
        Err(_) => rc::enc_i(0),
    }
}
