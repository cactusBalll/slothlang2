//! Dynamic trait dispatch: vtable primitives and the object-header link.
//! vt payload: `[raw_capacity(mem), slot0..]` — slots hold tagged fn ptr
//! words (bit0 set); word 1 of an object payload holds a tagged vtable
//! pointer word. rt-internal metadata (capacity) stays raw.

use crate::alloc::sloth_rt_alloc;
use crate::rc::{dec_i, w_is_ref, w_ref, w_unref};

#[no_mangle]
pub extern "C" fn sloth_vt_new(cap_w: i64) -> i64 {
    unsafe {
        let n = dec_i(cap_w).max(1) as libc::size_t;
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
        let slot = dec_i(slot_w);
        if slot >= 0 && slot < cap {
            *p.offset(slot as isize + 1) = fp_w;
        }
        0
    }
}

/// read the tagged fn ptr of a slot (0 when unset — empty chunks are zeroed)
#[no_mangle]
pub extern "C" fn sloth_vt_get(vt_w: i64, slot_w: i64) -> i64 {
    unsafe {
        if !w_is_ref(vt_w) {
            return 0;
        }
        let raw = w_unref(vt_w);
        if raw == 0 {
            return 0;
        }
        let p = raw as *mut i64;
        let cap = *p;
        let slot = dec_i(slot_w);
        if slot >= 0 && slot < cap {
            *(p.offset(slot as isize + 1))
        } else {
            0
        }
    }
}

/// object header word 1: tagged pointer to this class's vtable
#[no_mangle]
pub extern "C" fn sloth_obj_set_vtable(obj_w: i64, vt_w: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        *o.offset(1) = if w_is_ref(vt_w) {
            w_unref(vt_w) as i64
        } else {
            0
        };
    }
    0
}

/// (word-plane: the vtable pointer is stored raw internally and re-tagged;
/// empty = 0 stays 0 so the nil check survives the round trip)
#[no_mangle]
pub extern "C" fn sloth_obj_vtable(obj_w: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        let raw = *o.offset(1);
        if raw == 0 {
            0
        } else {
            w_ref(raw as usize)
        }
    }
}
