//! Arrays of tagged words with a *stable handle*.
//!
//! The rc-tracked payload is a fixed 3-word header `[len, cap, buf]`; the
//! elements live in a separate, untracked buffer (`buf`, `cap * 8` bytes).
//! Growth reallocates only the buffer, never the header, so the tagged
//! handle word never moves — every alias (locals, params, closure captures,
//! nested container slots) keeps pointing at the same live array.
//!
//! Every element word is tagged (ref | 1, int `v<<1`, f64 `(bits&!1)>>1`,
//! nil=0); rt treats elements as opaque tagged words, indexes are decoded
//! ints.

use crate::rc::{rc_addr, w_is_ref, w_ref, w_unref};

/// header words: `[len, cap, buf]`
const HDR_WORDS: usize = 3;

/// elements buffer pointer of an array handle
unsafe fn arr_buf(w: i64) -> *mut i64 {
    *((w_unref(w) as *mut i64).offset(2)) as *mut i64
}

/// bounds-checked element pointer (`i` arrives as a tagged index word)
fn arr_index(w: i64, i_w: i64) -> *mut i64 {
    unsafe {
        let p = w_unref(w) as *mut i64;
        let i = crate::rc::dec_i(i_w);
        let len = *p;
        if i < 0 || i >= len {
            crate::panics::panic_oob("array", i, len);
        }
        arr_buf(w).offset(i as isize)
    }
}

/// zeroed elements buffer (fresh slots read as nil)
fn alloc_buf(cap: i64) -> *mut i64 {
    let n = cap.max(1) as libc::size_t;
    let b = unsafe { libc::calloc(n, 8) as *mut i64 };
    if b.is_null() {
        crate::panics::panic_msg("out of memory");
    }
    b
}

/// death cascade: release every element word, then free the elements buffer
/// (value elements are inert no-ops thanks to the tag checks)
fn arr_dtor(p: usize, _aux: u64) {
    unsafe {
        let a = p as *mut i64;
        let len = *a;
        let buf = *a.offset(2) as *mut i64;
        let mut i = 0i64;
        while i < len {
            let w = *buf.offset(i as isize);
            if w != 0 && w_is_ref(w) {
                crate::rc::sloth_rc_release(w);
            }
            i += 1;
        }
        if !buf.is_null() {
            libc::free(buf as *mut libc::c_void);
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
        let n = crate::rc::dec_i(len_w).max(0);
        let cap = ((n as usize) * 2).next_power_of_two().max(8) as i64;
        let h = rc_addr(HDR_WORDS * 8, Some(arr_dtor));
        let p = h as *mut i64;
        *p = n;
        *p.offset(1) = cap;
        *p.offset(2) = alloc_buf(cap) as i64;
        w_ref(h)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_len(w: i64) -> i64 {
    unsafe { crate::rc::enc_i(*(w_unref(w) as *mut i64)) }
}

/// append one tagged word; the handle is stable (only the elements buffer
/// grows), so the same word is returned and aliases stay valid
#[no_mangle]
pub extern "C" fn sloth_arr_push(a: i64, w: i64) -> i64 {
    unsafe {
        let p = w_unref(a) as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        if len >= cap {
            let nc = (cap * 2).max(8);
            let nb = alloc_buf(nc);
            let old = *p.offset(2) as *mut i64;
            std::ptr::copy_nonoverlapping(old, nb, len as usize);
            libc::free(old as *mut libc::c_void);
            *p.offset(2) = nb as i64;
            *p.offset(1) = nc;
        }
        *(*p.offset(2) as *mut i64).offset(len as isize) = w;
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
        *(*p.offset(2) as *mut i64).offset((len - 1) as isize)
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
