//! Dynamic trait dispatch: vtable primitives and the object-header link.
//! vt: `[capacity, slot0..]` array of i64 raw function pointers.

use crate::gc::sloth_gc_alloc;

#[no_mangle]
pub extern "C" fn sloth_vt_new(cap: i64) -> i64 {
    unsafe {
        let n = cap.max(1) as libc::size_t;
        let o = sloth_gc_alloc((n + 1) * 8) as *mut i64;
        *o = n as i64;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_vt_set(vt: i64, slot: i64, fp: i64) -> i64 {
    unsafe {
        let p = vt as *mut i64;
        let cap = *p;
        if slot >= 0 && slot < cap {
            *p.offset(slot as isize + 1) = fp;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn sloth_vt_get(vt: i64, slot: i64) -> i64 {
    unsafe {
        if vt == 0 {
            return 0;
        }
        let p = vt as *mut i64;
        let cap = *p;
        if slot >= 0 && slot < cap {
            *(p.offset(slot as isize + 1))
        } else {
            0
        }
    }
}

/// object header word 1: pointer to this class's vtable
#[no_mangle]
pub extern "C" fn sloth_obj_set_vtable(obj: i64, vt: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(1) = vt }
    0
}

#[no_mangle]
pub extern "C" fn sloth_obj_vtable(obj: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(1) }
}
