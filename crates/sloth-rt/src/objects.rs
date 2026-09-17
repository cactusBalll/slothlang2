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

/// allocate an instance with `n_fields_w` = number of FIELD words (the two
/// leading metadata words are added here). The field count is recorded in the
/// header `aux` so the death cascade knows how many words to walk — this also
/// covers info-less boxes (closure/lambda frames), whose word 0 is null.
#[no_mangle]
pub extern "C" fn sloth_obj_new(info_w: i64, n_fields_w: i64) -> i64 {
    unsafe {
        let n_fields = crate::rc::dec_i(n_fields_w).max(0);
        let info = if w_is_ref(info_w) {
            w_unref(info_w) as usize
        } else {
            0
        };
        let o = rc_addr((n_fields as usize + 2) * 8, Some(obj_dtor)) as *mut i64;
        crate::rc::set_aux(o as usize, n_fields as u64);
        *o = info as i64;
        // word 1 (vtable) is set separately; fields start zeroed (nil)
        w_ref(o as usize)
    }
}

/// allocate a closure box `{ tagged fnptr, env }`. Field 0 is a *tagged*
/// function pointer (bit 0 set to mark it), which the generic tag-driven
/// cascade would mistake for an rc handle — so closure boxes get a dedicated
/// dtor that releases only the environment field.
#[no_mangle]
pub extern "C" fn sloth_closure_new(fnptr_w: i64, env_w: i64) -> i64 {
    unsafe {
        let o = rc_addr((2 + 2) * 8, Some(closure_dtor)) as *mut i64;
        *o = 0;
        *o.offset(1) = 0;
        *o.offset(OBJ_FIELD_OFFSET) = fnptr_w;
        *o.offset(OBJ_FIELD_OFFSET + 1) = env_w;
        w_ref(o as usize)
    }
}

/// closure death: release the environment (field 1) only
fn closure_dtor(p: usize, _aux: u64) {
    unsafe {
        let o = p as *mut i64;
        let env = *o.offset(OBJ_FIELD_OFFSET + 1);
        if env != 0 && w_is_ref(env) {
            crate::rc::sloth_rc_release(env);
        }
    }
}

/// death cascade: release every tagged field word of the dying instance. The
/// field count rides the header `aux` (tag-gated releases keep value words
/// inert), so the cascade is mask-free and info-less boxes work too.
fn obj_dtor(p: usize, aux: u64) {
    unsafe {
        let o = p as *mut i64;
        let nf = aux as i64;
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
