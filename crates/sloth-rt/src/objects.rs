//! Object model: allocation, class type headers, tag-driven field cascade.
//! Object layout: word 0 = raw ObjInfo pointer, word 1 = raw vtable pointer
//! (rt-internal metadata, never tagged words), fields from word 2 on — each
//! field is a tagged user word; the death cascade releases every word with
//! the tag bit set (no per-class mask needed any more).

use crate::rc::{rc_addr, w_is_ref, w_ref, w_unref};

#[repr(C)]
pub(crate) struct ObjInfo {
    super_info: *mut libc::c_void,
    cls_id: i64,
    /// tag migration: the per-class ref mask is gone (death cascade is
    /// tag-driven); the class keeps only structural metadata
    n_fields: i64,
}
pub(crate) const OBJ_FIELD_OFFSET: isize = 2;

#[no_mangle]
pub extern "C" fn sloth_cls_info(super_w: i64, cls_id_w: i64) -> i64 {
    unsafe {
        let o = crate::alloc::sloth_rt_alloc(std::mem::size_of::<ObjInfo>()) as *mut ObjInfo;
        (*o).super_info = if w_is_ref(super_w) {
            w_unref(super_w) as *mut libc::c_void
        } else {
            std::ptr::null_mut()
        };
        (*o).cls_id = crate::rc::dec_i(cls_id_w);
        (*o).n_fields = 0;
        // class metadata is never freed: the word is representational only
        // (not rc-tracked) — still tagged so the word plane stays uniform
        w_ref(o as usize)
    }
}

/// legacy ref-mask registration: retired by the tag bit (no-op)
#[no_mangle]
pub extern "C" fn sloth_cls_refmask(_info_w: i64, _mask: i64, _n: i64) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn sloth_obj_new(info_w: i64, n_fields_w: i64) -> i64 {
    unsafe {
        let n_words = crate::rc::dec_i(n_fields_w).max(0) + 2;
        let info = if w_is_ref(info_w) {
            w_unref(info_w) as usize
        } else {
            0
        };
        let o = rc_addr(n_words as usize * 8, Some(obj_dtor)) as *mut i64;
        *o = info as i64;
        // word 1 (vtable) is set separately; fields start zeroed (nil)
        w_ref(o as usize)
    }
}

/// death cascade: release every tagged field word of the dying instance
fn obj_dtor(p: usize, _aux: u64) {
    unsafe {
        let o = p as *mut i64;
        let info = *o as *mut ObjInfo;
        if info.is_null() {
            return;
        }
        let nf = (*info).n_fields;
        let mut i = 0i64;
        while i < nf {
            let w = *o.offset(i as isize + OBJ_FIELD_OFFSET);
            if w != 0 && w_is_ref(w) {
                crate::rc::sloth_rc_release(w);
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_field(obj_w: i64, idx_w: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        let i = crate::rc::dec_i(idx_w);
        *o.offset(i as isize + OBJ_FIELD_OFFSET)
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_set_field(obj_w: i64, idx_w: i64, val: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        let i = crate::rc::dec_i(idx_w);
        *o.offset(i as isize + OBJ_FIELD_OFFSET) = val;
        0
    }
}

/// runtime class id of an object (raw from the type header, decoded at the
/// boundary since the word plane carries it tagged)
#[no_mangle]
pub extern "C" fn sloth_obj_cls_id(obj_w: i64) -> i64 {
    unsafe {
        let info = *(w_unref(obj_w) as *mut *mut ObjInfo);
        crate::rc::enc_i((*info).cls_id)
    }
}
