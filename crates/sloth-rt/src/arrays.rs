//! Arrays of i64/f64 words. Layout: `[len, cap, e0, e1, ...]` (cap >= len;
//! push grows past cap by realloc). f64 accesses are routed via the `_f64` ops.

use crate::rc::{track_user_dtor, transfer};

/// bounds-checked element pointer
fn arr_index(a: i64, i: i64) -> *mut i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if i < 0 || i >= len {
            crate::panics::panic_oob("array", i, len);
        }
        p.offset(i as isize + 2)
    }
}

/// death cascade: release ref-typed elements when the array chunk dies
fn arr_dtor(p: usize, aux: u64) {
    unsafe {
        if aux == 0 {
            return;
        }
        let a = p as *mut i64;
        let len = *a;
        let mut i = 0i64;
        while i < len {
            let w = *a.offset(i as isize + 2);
            if w != 0 {
                crate::rc::sloth_rc_release(w);
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_new(len: i64) -> i64 {
    arr_new_impl(len, 0)
}

/// element-ref-aware array creation (patch C): elref = 1 marks elements as
/// refcounted words → death cascade releases each element's count
#[no_mangle]
pub extern "C" fn sloth_arr_new_k(len: i64, elref: i64) -> i64 {
    arr_new_impl(len, elref)
}

fn arr_new_impl(len: i64, elref: i64) -> i64 {
    unsafe {
        let n = len.max(0) as libc::size_t;
        let cap = (n * 2).next_power_of_two().max(8) as libc::size_t;
        let o = crate::alloc::sloth_rt_alloc((cap + 2) * 8) as *mut i64;
        *o = n as i64;
        *o.offset(1) = cap as i64;
        track_user_dtor(o as usize, arr_dtor);
        // stash elref in the count table via... entries carry aux; record it
        crate::rc::set_aux(o as usize, elref as u64);
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_len(a: i64) -> i64 {
    unsafe { *(a as *mut i64) }
}

/// append one i64 word; returns the (possibly moved) array handle —
/// GC_realloc relocates the buffer when growing, so callers MUST propagate
/// the returned handle (old input is freed immediately by the collector)
#[no_mangle]
pub extern "C" fn sloth_arr_push(a: i64, w: i64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        let mut base = a;
        if len >= cap {
            let nc = (cap * 2).max(8);
            let raw = crate::alloc::sloth_rt_realloc(
                a as *mut libc::c_void,
                (nc + 2) as libc::size_t * 8,
            );
            base = raw as i64;
            // count ownership follows the relocated chunk
            transfer(a as usize, base as usize);
            let p2 = base as *mut i64;
            *p2.offset(1) = nc;
        }
        let p2 = base as *mut i64;
        *p2.offset((len + 2) as isize) = w;
        *p2 = len + 1;
        base
    }
}

/// remove and return the last i64 word
#[no_mangle]
pub extern "C" fn sloth_arr_pop(a: i64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if len <= 0 {
            crate::panics::panic_oob("pop", len - 1, len);
        }
        *p = len - 1;
        *p.offset((len - 1) as isize + 2)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_push_f64(a: i64, w: f64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        let mut base = a;
        if len >= cap {
            // realloc moves the buffer: return the relocated handle
            let nc = (cap * 2).max(8);
            let raw = crate::alloc::sloth_rt_realloc(
                a as *mut libc::c_void,
                (nc + 2) as libc::size_t * 8,
            );
            base = raw as i64;
            // count ownership follows the relocated chunk
            transfer(a as usize, base as usize);
            let p2 = base as *mut i64;
            *p2.offset(1) = nc;
        }
        let p2 = base as *mut i64;
        *(p2.offset((len + 2) as isize) as *mut f64) = w;
        *p2 = len + 1;
        base
    }
}

/// remove and return the last f64 word
#[no_mangle]
pub extern "C" fn sloth_arr_pop_f64(a: i64) -> f64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if len <= 0 {
            crate::panics::panic_oob("pop", len - 1, len);
        }
        *p = len - 1;
        *((p.offset((len - 1) as isize + 2)) as *mut f64)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_get(a: i64, i: i64) -> i64 {
    unsafe { *arr_index(a, i) }
}

#[no_mangle]
pub extern "C" fn sloth_arr_get_f64(a: i64, i: i64) -> f64 {
    unsafe { *(arr_index(a, i) as *mut f64) }
}

#[no_mangle]
pub extern "C" fn sloth_arr_set(a: i64, i: i64, v: i64) -> i64 {
    unsafe {
        *arr_index(a, i) = v;
        0
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_set_f64(a: i64, i: i64, v: f64) -> i64 {
    unsafe {
        *(arr_index(a, i) as *mut f64) = v;
        0
    }
}
