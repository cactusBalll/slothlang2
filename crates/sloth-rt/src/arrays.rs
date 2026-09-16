//! Arrays of tagged words. Layout: `[len, cap, e0, e1, ...]` (cap >= len;
//! push grows past cap by relocating through the rc core). Every element
//! word is tagged (ref | 1, int `v<<1`, f64 `(bits&!1)>>1`, nil=0); rt
//! treats elements as opaque tagged words, indexes are decoded ints.

use crate::rc::{rc_addr, w_is_ref, w_ref, w_unref};

/// bounds-checked element pointer (`i` arrives as a tagged index word)
fn arr_index(w: i64, i_w: i64) -> *mut i64 {
    unsafe {
        let p = w_unref(w) as *mut i64;
        let i = crate::rc::dec_i(i_w);
        let len = *p;
        if i < 0 || i >= len {
            crate::panics::panic_oob("array", i, len);
        }
        p.offset(i as isize + 2)
    }
}

/// death cascade: release every element word (tag checks make value
/// elements inert no-ops — no elref mask needed)
fn arr_dtor(p: usize, _aux: u64) {
    unsafe {
        let a = p as *mut i64;
        let len = *a;
        let mut i = 0i64;
        while i < len {
            let w = *a.offset(i as isize + 2);
            if w != 0 && w_is_ref(w) {
                crate::rc::sloth_rc_release(w);
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_new(len_w: i64) -> i64 {
    arr_new_impl(len_w)
}

/// element-ref-aware creation signature kept (the tag bit replaced the
/// elref mask; the parameter is ignored — callers keep shape)
#[no_mangle]
pub extern "C" fn sloth_arr_new_k(len_w: i64, _elref: i64) -> i64 {
    arr_new_impl(len_w)
}

fn arr_new_impl(len_w: i64) -> i64 {
    unsafe {
        let n = crate::rc::dec_i(len_w).max(0) as libc::size_t;
        let cap = (n * 2).next_power_of_two().max(8) as libc::size_t;
        let p = rc_addr((cap + 2) * 8, Some(arr_dtor)) as *mut i64;
        *p = n as i64;
        *p.offset(1) = cap as i64;
        w_ref(p as usize)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_len(w: i64) -> i64 {
    unsafe { crate::rc::enc_i(*(w_unref(w) as *mut i64)) }
}

/// append one tagged word; returns the (possibly relocated) array handle —
/// growth moves the header+payload, so callers MUST propagate the handle
#[no_mangle]
pub extern "C" fn sloth_arr_push(a: i64, w: i64) -> i64 {
    unsafe {
        let p = w_unref(a) as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        if len >= cap {
            let nc = (cap * 2).max(8);
            let a2 = crate::alloc::sloth_rt_realloc(a, ((nc + 2) * 8) as usize);
            let p2 = w_unref(a2) as *mut i64;
            *p2.offset(1) = nc;
            *p2.offset((len + 2) as isize) = w;
            *p2 = len + 1;
            return a2;
        }
        *p.offset((len + 2) as isize) = w;
        *p = len + 1;
        a
    }
}

/// remove and return the last tagged word
#[no_mangle]
pub extern "C" fn sloth_arr_pop(a: i64) -> i64 {
    unsafe {
        let p = w_unref(a) as *mut i64;
        let len = *p;
        if len <= 0 {
            crate::panics::panic_oob("pop", len - 1, len);
        }
        *p = len - 1;
        *p.offset((len - 1) as isize + 2)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_get(a: i64, i: i64) -> i64 {
    unsafe { *arr_index(a, i) }
}

#[no_mangle]
pub extern "C" fn sloth_arr_set(a: i64, i: i64, v: i64) -> i64 {
    unsafe {
        *arr_index(a, i) = v;
        0
    }
}
