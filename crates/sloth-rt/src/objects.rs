//! Object model: allocation, class type headers, header-resident fields.

use crate::alloc::sloth_rt_alloc;

#[repr(C)]
pub(crate) struct ObjInfo {
    super_info: *mut libc::c_void,
    cls_id: i64,
    /// rc migration patch C: per-field ref mask (bit i = field i is a
    /// refcounted word); drives the cascade on instance death
    refmask: u64,
    n_fields: i64,
}

#[no_mangle]
pub extern "C" fn sloth_cls_info(super_ptr: i64, cls_id: i64) -> i64 {
    unsafe {
        let o = sloth_rt_alloc(std::mem::size_of::<ObjInfo>()) as *mut ObjInfo;
        (*o).super_info = super_ptr as *mut libc::c_void;
        (*o).cls_id = cls_id;
        (*o).refmask = 0;
        (*o).n_fields = 0;
        o as i64
    }
}

/// mark the refcounted fields of a class (codegen knows the field kinds);
/// `mask` bit i = field i (base-class-first layout) is a ref word
#[no_mangle]
pub extern "C" fn sloth_cls_refmask(info_ptr: i64, mask: i64, n_fields: i64) -> i64 {
    unsafe {
        let o = info_ptr as *mut ObjInfo;
        (*o).refmask = mask as u64;
        (*o).n_fields = n_fields;
        0
    }
}

/// object layout: word 0 = ObjInfo header, word 1 = vtable pointer, fields after
#[no_mangle]
pub extern "C" fn sloth_obj_new(info_ptr: i64, n_words: i64) -> i64 {
    unsafe {
        let n = n_words.max(2) as libc::size_t;
        let o = sloth_rt_alloc(n * 8) as *mut i64;
        *o = info_ptr;
        // death cascade: release each ref-typed field via the class mask
        crate::rc::track_user_dtor(o as usize, obj_dtor);
        o as i64
    }
}

/// death cascade: release every ref-masked field word of the dying instance
fn obj_dtor(p: usize, _aux: u64) {
    unsafe {
        let o = p as *mut i64;
        let info = *o as *mut ObjInfo;
        if info.is_null() {
            return;
        }
        let mask = (*info).refmask;
        let nf = (*info).n_fields;
        let mut i = 0i64;
        while i < nf {
            if mask & (1u64 << i) != 0 {
                let w = *o.offset(i as isize + 2);
                if w != 0 {
                    crate::rc::sloth_rc_release(w);
                }
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_field(obj: i64, idx: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(idx as isize + 2) }
}

#[no_mangle]
pub extern "C" fn sloth_obj_field_f64(obj: i64, idx: i64) -> f64 {
    unsafe { *(obj as *mut f64).offset(idx as isize + 2) }
}

#[no_mangle]
pub extern "C" fn sloth_obj_set_field(obj: i64, idx: i64, val: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(idx as isize + 2) = val }
    0
}

#[no_mangle]
pub extern "C" fn sloth_obj_set_field_f64(obj: i64, idx: i64, val: f64) -> i64 {
    unsafe { *(obj as *mut f64).offset(idx as isize + 2) = val }
    0
}

/// runtime class id of an object (from its type header), used by dyn dispatch
#[no_mangle]
pub extern "C" fn sloth_obj_cls_id(obj: i64) -> i64 {
    unsafe {
        let info = *(obj as *mut *mut libc::c_void);
        (*(info as *mut ObjInfo)).cls_id
    }
}
