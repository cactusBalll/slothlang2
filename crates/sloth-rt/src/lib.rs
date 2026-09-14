//! sloth-rt: the sloth2 runtime library (libsloth_rt).
//! Console, GC, string pool, containers, panic — filled in incrementally.

#[no_mangle]
pub extern "C" fn sloth_rt_hello() {
    println!("libsloth_rt linked");
}

#[no_mangle]
pub extern "C" fn sloth_panic(msg: *const libc::c_char) -> ! {
    let s = unsafe {
        if msg.is_null() {
            "panic".to_string()
        } else {
            std::ffi::CStr::from_ptr(msg).to_string_lossy().to_string()
        }
    };
    eprintln!("sloth panic: {}", s);
    std::process::exit(1);
}

/// MVP allocator stand-in (replaced by Boehm GC in sloth-rt step 2).
#[no_mangle]
pub extern "C" fn sloth_gc_alloc(n: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::calloc(1, n) }
}

/// print an i64 value (display form); never returns useful value
#[no_mangle]
pub extern "C" fn sloth_rt_print_i64(v: i64) -> i64 {
    println!("{}", v);
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_f64(v: f64) -> i64 {
    println!("{}", v);
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_bool(v: i64) -> i64 {
    println!("{}", v != 0);
    0
}

// ---------------- string builders ----------------

// str32: 8-byte packed words; total length known at compile time.
#[repr(C)]
struct StrB {
    len: libc::size_t,
    cap: libc::size_t,
    data: *mut libc::c_void,
}

/// builder lifecycle: push chunks in,收回 handle; finalize interns the pool.
#[no_mangle]
pub extern "C" fn sloth_str_push(b: i64, w: i64, n: i64) -> i64 {
    unsafe {
        let p: *mut StrB = if b == 0 {
            let raw = libc::calloc(1, std::mem::size_of::<StrB>()) as *mut StrB;
            (*raw).cap = 0;
            raw as *mut StrB
        } else {
            b as *mut StrB
        };
        let un = (n as usize).min(8);
        if (*p).cap < (*p).len + un {
            let nc = ((*p).len + un + 16).next_power_of_two();
            (*p).data = libc::realloc((*p).data, nc);
            (*p).cap = nc;
        }
        let bytes = (w as u64).to_le_bytes();
        let dst = (*p).data as *mut libc::c_void;
        libc::memcpy((dst as *mut libc::c_char).offset((*p).len as isize) as *mut libc::c_void,
                     bytes.as_ptr() as *const libc::c_void, un);
        (*p).len += un;
        p as i64
    }
}

// ---------------- string pool interning ----------------

#[repr(C)]
struct StrT {
    len: libc::size_t,
    data: *mut libc::c_void,
}

/// interned string handle from raw bytes; used by string literals
#[no_mangle]
pub extern "C" fn sloth_str_intern(ptr: i64, len: i64) -> i64 {
    unsafe {
        let t = sloth_gc_alloc(std::mem::size_of::<StrT>() + len as libc::size_t + 1)
            as *mut libc::c_void;
        let td = t as *mut StrT;
        (*td).len = len as usize;
        let data = (t as *mut libc::c_void).offset(std::mem::size_of::<StrT>() as isize);
        libc::memcpy(data, ptr as *const libc::c_void, len as usize);
        // NUL terminate for easy C display
        libc::memset((data as *mut libc::c_char).offset(len as isize) as *mut libc::c_void, 0, 1);
        (*td).data = data;
        t as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_str(p: i64) -> i64 {
    use std::io::Write;
    unsafe {
        let td = p as *mut StrT;
        let sl = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
        let mut so = std::io::stdout();
        let _ = so.write_all(sl);
        let _ = so.write_all(b"\n");
        let _ = so.flush();
    }
    0
}

#[no_mangle]
pub extern "C" fn sloth_str_len(p: i64) -> i64 {
    unsafe { (*(p as *mut StrT)).len as i64 }
}

/// finalize: return the pooled interned string for a built byte buffer
#[no_mangle]
pub extern "C" fn sloth_str_finish(b: i64) -> i64 {
    unsafe {
        let p: *mut StrB = b as *mut StrB;
        let h = sloth_str_intern((*p).data as i64, (*p).len as i64);
        libc::free((*p).data);
        libc::free(p as *mut libc::c_void);
        h
    }
}

/// concatenate two pooled strings
#[no_mangle]
pub extern "C" fn sloth_str_concat(a: i64, b: i64) -> i64 {
    unsafe {
        let ta = a as *mut StrT;
        let tb = b as *mut StrT;
        let la = (*ta).len;
        let lb = (*tb).len;
        let h = sloth_gc_alloc(la + lb + 1 + std::mem::size_of::<StrT>()) as *mut libc::c_void;
        let td = h as *mut StrT;
        let dat = (h as *mut libc::c_void).offset(std::mem::size_of::<StrT>() as isize);
        libc::memcpy(dat, (*ta).data, la);
        libc::memcpy((dat as *mut libc::c_char).offset(la as isize) as *mut libc::c_void,
                     (*tb).data, lb);
        libc::memset((dat as *mut libc::c_char).offset((la + lb) as isize) as *mut libc::c_void, 0, 1);
        (*td).len = la + lb;
        (*td).data = dat;
        h as i64
    }
}

// ---------------- objects/classes ----------------

#[repr(C)]
struct ObjInfo {
    super_info: *mut libc::c_void,
    cls_id: i64,
}

#[no_mangle]
pub extern "C" fn sloth_cls_info(super_ptr: i64, cls_id: i64) -> i64 {
    unsafe {
        let o = sloth_gc_alloc(std::mem::size_of::<ObjInfo>()) as *mut ObjInfo;
        (*o).super_info = super_ptr as *mut libc::c_void;
        (*o).cls_id = cls_id;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_new(info_ptr: i64, n_words: i64) -> i64 {
    unsafe {
        let n = n_words.max(2) as libc::size_t;
        let o = sloth_gc_alloc(n * 8) as *mut i64;
        *o = info_ptr;
        o as i64
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

/// dynamic trait dispatch: vtable primitives.
/// vt: [capacity, slot0..] array of i64 raw function pointers
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

/// dyn receiver is not an implementing class (never returns)
#[no_mangle]
pub extern "C" fn sloth_panic_noimpl(cls_id: i64) -> i64 {
    eprintln!(
        "sloth panic: no impl for trait method on receiver (cls {})",
        cls_id
    );
    std::process::exit(1);
}
