//! Object model: allocation + class metadata. Object layout: word 0 = raw
//! ObjInfo pointer, word 1 = raw vtable pointer (rt-internal metadata),
//! fields from word 2 on — each field is a raw word (reference = payload
//! pointer, value = native word). The runtime carries NO layout knowledge for
//! objects: the per-class death cascade is emitted by codegen and registered
//! as the allocation's `sdtor` (`Hdr`), so value fields (int/float/bool words)
//! stay untouched without any runtime tag or mask.

use crate::rc::{rc_addr, rc_addr_sloth, w_ref, w_unref};

/// Per-class metadata: untracked plain-calloc block, never freed. Holds only
/// the runtime class id (virtual dispatch) and the display name; the death
/// cascade lives in generated code, not here.
#[repr(C)]
pub(crate) struct ObjInfo {
    cls_id: i64,
    /// display name of the concrete class (raw bytes, untracked, points into
    /// a codegen-emitted byte global); used by the `type_name` builtin
    name: *const u8,
    name_len: usize,
}
pub(crate) const OBJ_FIELD_OFFSET: isize = 2;

/// Build class metadata. `_super_w` stays in the ABI for the emission surface
/// but is unused: hierarchy is a compile-time concern and the runtime dispatch
/// key is `cls_id_w`.
#[no_mangle]
pub extern "C" fn __sloth_cls_info(_super_w: i64, cls_id_w: i64) -> i64 {
    unsafe {
        let o = crate::alloc::__sloth_rt_alloc(std::mem::size_of::<ObjInfo>()) as *mut ObjInfo;
        (*o).cls_id = cls_id_w;
        (*o).name = std::ptr::null();
        (*o).name_len = 0;
        // class metadata is never freed: the word is representational only
        w_ref(o as usize)
    }
}

/// register the runtime display name of a class (raw bytes pointer + length,
/// untracked). Called by codegen at instance construction and by the builtin
/// value-box builder; the bytes live in a codegen-emitted global.
#[no_mangle]
pub extern "C" fn __sloth_cls_name(info_w: i64, ptr_w: i64, len_w: i64) -> i64 {
    if info_w != 0 {
        unsafe {
            let o = w_unref(info_w) as *mut ObjInfo;
            (*o).name = w_unref(ptr_w) as *const u8;
            (*o).name_len = len_w.max(0) as usize;
        }
    }
    0
}

/// allocate an instance with `n_fields_w` = number of FIELD words (the two
/// leading metadata words are added here). `cascade_w` is the raw address of
/// the codegen-emitted per-class death cascade `(payload, aux) -> i64` (0 =
/// none); it is installed as the header's `sdtor` and releases exactly the
/// reference fields, after which the chunk is freed.
#[no_mangle]
pub extern "C" fn __sloth_obj_new(info_w: i64, n_fields_w: i64, cascade_w: i64) -> i64 {
    unsafe {
        let n_fields = n_fields_w.max(0);
        let info = w_unref(info_w);
        let o = rc_addr_sloth(
            (n_fields as usize + 2) * 8,
            n_fields as u64,
            cascade_w as usize,
        ) as *mut i64;
        *o = info as i64;
        // word 1 (vtable) is set separately; fields start zeroed (nil)
        w_ref(o as usize)
    }
}

/// allocate a closure box `{ fnptr, env }`. Field 0 is a raw function pointer
/// (not an rc handle), so closure boxes get a dedicated dtor that releases
/// only the environment field.
#[no_mangle]
pub extern "C" fn __sloth_closure_new(fnptr_w: i64, env_w: i64) -> i64 {
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
            crate::rc::__sloth_rc_release(env);
        }
    }
}

#[no_mangle]
pub extern "C" fn __sloth_obj_field(obj_w: i64, idx_w: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        *o.offset(idx_w as isize + OBJ_FIELD_OFFSET)
    }
}

#[no_mangle]
pub extern "C" fn __sloth_obj_set_field(obj_w: i64, idx_w: i64, val: i64) -> i64 {
    unsafe {
        let o = w_unref(obj_w) as *mut i64;
        *o.offset(idx_w as isize + OBJ_FIELD_OFFSET) = val;
        0
    }
}

/// runtime type name for a statically-known reference type: `w == 0` (nil) →
/// `"nil"`, otherwise a fresh `str` from the compile-time name bytes (owned
/// +1). Used by `type_name` on non-class reference types and reference
/// optionals, whose concrete type is known to the compiler.
#[no_mangle]
pub extern "C" fn __sloth_type_name_or(w: i64, ptr_w: i64, len_w: i64) -> i64 {
    if w == 0 {
        crate::strings::intern_bytes(b"nil".as_ptr() as usize, 3)
    } else {
        crate::strings::intern_bytes(w_unref(ptr_w), len_w.max(0))
    }
}

/// runtime class id of an object (raw from the type header)
#[no_mangle]
pub extern "C" fn __sloth_obj_cls_id(obj_w: i64) -> i64 {
    if obj_w == 0 {
        return 0;
    }
    unsafe {
        let info = *(w_unref(obj_w) as *mut *mut ObjInfo);
        if info.is_null() {
            0
        } else {
            (*info).cls_id
        }
    }
}

/// runtime display name of an object's concrete class as a fresh owned `str`
/// (+1). nil → `"nil"`; missing metadata/name → empty string.
#[no_mangle]
pub extern "C" fn __sloth_obj_type_name(obj_w: i64) -> i64 {
    unsafe {
        if obj_w == 0 {
            return crate::strings::intern_bytes(b"nil".as_ptr() as usize, 3);
        }
        let info = *(w_unref(obj_w) as *mut *mut ObjInfo);
        if info.is_null() {
            return crate::strings::intern_bytes(b"".as_ptr() as usize, 0);
        }
        let n = (*info).name;
        let l = (*info).name_len;
        if n.is_null() {
            crate::strings::intern_bytes(b"".as_ptr() as usize, 0)
        } else {
            crate::strings::intern_bytes(n as usize, l as i64)
        }
    }
}
