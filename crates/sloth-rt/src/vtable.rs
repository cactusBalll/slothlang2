//! Dynamic trait dispatch: vtable primitives and the object-header link.
//! vt payload: `[raw_capacity(mem), slot0..]` — slots hold raw fn ptr words;
//! word 1 of an object payload holds a raw vtable pointer (0 = none).
//! rt-internal metadata (capacity) stays raw.

use crate::alloc::sloth_rt_alloc;
use crate::rc::{w_ref, w_unref};

#[no_mangle]
pub extern "C" fn sloth_vt_new(cap_w: i64) -> i64 {
    unsafe {
        let n = cap_w.max(1) as libc::size_t;
        let o = sloth_rt_alloc((n + 1) * 8) as *mut i64;
        *o = n as i64;
        w_ref(o as usize)
    }
}

#[no_mangle]
pub extern "C" fn sloth_vt_set(vt_w: i64, slot_w: i64, fp_w: i64) -> i64 {
    unsafe {
        let p = w_unref(vt_w) as *mut i64;
        let cap = *p;
        let slot = slot_w;
        if slot >= 0 && slot < cap {
            *p.offset(slot as isize + 1) = fp_w;
        }
        0
    }
}

/// read the raw fn ptr of a slot (0 when unset — empty chunks are zeroed)
#[no_mangle]
pub extern "C" fn sloth_vt_get(vt_w: i64, slot_w: i64) -> i64 {
    unsafe {
        if vt_w == 0 {
            return 0;
        }
        let p = w_unref(vt_w) as *mut i64;
        let cap = *p;
        let slot = slot_w;
        if slot >= 0 && slot < cap {
            *(p.offset(slot as isize + 1))
        } else {
            0
        }
    }
}

/// object header word 1: store the raw vtable payload pointer
#[no_mangle]
pub extern "C" fn sloth_obj_set_vtable(obj_w: i64, vt_w: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        *o.offset(1) = vt_w;
    }
    0
}

/// object header word 1 (raw vtable pointer; 0 = none)
#[no_mangle]
pub extern "C" fn sloth_obj_vtable(obj_w: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        *o.offset(1)
    }
}
