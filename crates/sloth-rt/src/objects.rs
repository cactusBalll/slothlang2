//! Object model: allocation, class type headers, mask-driven field cascade.
//! Object layout: word 0 = raw ObjInfo pointer, word 1 = raw vtable pointer
//! (rt-internal metadata), fields from word 2 on — each field is a raw word
//! (reference = payload pointer, value = native word). The death cascade
//! releases each field whose bit is set in the class's compile-time refness
//! mask (`ObjInfo.refmask`), so value fields (int/float/bool words) stay
//! untouched without any runtime tag.

use crate::rc::{rc_addr, w_ref, w_unref};

/// Per-class metadata: untracked plain-calloc block, never freed. Holds the
/// runtime class id (virtual dispatch) and the boxed field refness mask
/// (bit i = field i is a reference word; base-class-first layout). The field
/// count rides `Hdr.aux`, so the cascade only needs the mask.
#[repr(C)]
pub(crate) struct ObjInfo {
    cls_id: i64,
    refmask: u64,
}
pub(crate) const OBJ_FIELD_OFFSET: isize = 2;

/// Build class metadata. `_super_w` stays in the ABI for the emission surface
/// but is unused: hierarchy is a compile-time concern and the runtime dispatch
/// key is `cls_id_w`. The ref mask is registered separately by
/// `sloth_cls_refmask`.
#[no_mangle]
pub extern "C" fn sloth_cls_info(_super_w: i64, cls_id_w: i64) -> i64 {
    unsafe {
        let o = crate::alloc::sloth_rt_alloc(std::mem::size_of::<ObjInfo>()) as *mut ObjInfo;
        (*o).cls_id = cls_id_w;
        (*o).refmask = 0;
        // class metadata is never freed: the word is representational only
        w_ref(o as usize)
    }
}

/// mark the refcounted fields of a class (codegen knows the field kinds);
/// `mask` bit i = field i (base-class-first layout) is a reference word.
/// The field count is recorded on every instance's header `aux`, so the
/// mask is enough to drive the death cascade.
#[no_mangle]
pub extern "C" fn sloth_cls_refmask(info_w: i64, mask: i64, _n: i64) -> i64 {
    if info_w != 0 {
        unsafe {
            let o = w_unref(info_w) as *mut ObjInfo;
            (*o).refmask = mask as u64;
        }
    }
    0
}

/// allocate an instance with `n_fields_w` = number of FIELD words (the two
/// leading metadata words are added here). The field count is recorded in the
/// header `aux` so the death cascade knows how many words to walk.
#[no_mangle]
pub extern "C" fn sloth_obj_new(info_w: i64, n_fields_w: i64) -> i64 {
    unsafe {
        let n_fields = n_fields_w.max(0);
        let info = w_unref(info_w);
        let o = rc_addr((n_fields as usize + 2) * 8, Some(obj_dtor)) as *mut i64;
        crate::rc::set_aux(o as usize, n_fields as u64);
        *o = info as i64;
        // word 1 (vtable) is set separately; fields start zeroed (nil)
        w_ref(o as usize)
    }
}

/// allocate a closure box `{ fnptr, env }`. Field 0 is a raw function pointer
/// (not an rc handle), so closure boxes get a dedicated dtor that releases
/// only the environment field.
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
        if env != 0 {
            crate::rc::sloth_rc_release(env);
        }
    }
}

/// death cascade: release every reference field selected by the class ref
/// mask. The field count rides the header `aux` (value fields are masked
/// out), so the cascade is mask-driven and info-less boxes are handled by
/// their own destructors.
fn obj_dtor(p: usize, aux: u64) {
    unsafe {
        let o = p as *mut i64;
        let info = *o as *mut ObjInfo;
        if info.is_null() {
            return;
        }
        let mask = (*info).refmask;
        let nf = (aux as i64).min(64);
        let mut i = 0i64;
        while i < nf {
            if mask & (1u64 << i) != 0 {
                let w = *o.offset(i as isize + OBJ_FIELD_OFFSET);
                if w != 0 {
                    crate::rc::sloth_rc_release(w);
                }
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_field(obj_w: i64, idx_w: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        *o.offset(idx_w as isize + OBJ_FIELD_OFFSET)
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_set_field(obj_w: i64, idx_w: i64, val: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        *o.offset(idx_w as isize + OBJ_FIELD_OFFSET) = val;
        0
    }
}

/// runtime class id of an object (raw from the type header)
#[no_mangle]
pub extern "C" fn sloth_obj_cls_id(obj_w: i64) -> i64 {
    unsafe {
        let info = *(w_unref(obj_w) as *mut *mut ObjInfo);
        if info.is_null() {
            0
        } else {
            (*info).cls_id
        }
    }
}
