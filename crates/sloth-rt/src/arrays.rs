//! Arrays of i64/f64 words. Layout: `[len, cap, e0, e1, ...]` (cap >= len;
//! push grows past cap by realloc). f64 accesses are routed via the `_f64` ops.

/// bounds-checked element pointer
fn arr_index(a: i64, i: i64) -> *mut i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if i < 0 || i >= len {
            eprintln!("sloth panic: array index {} out of bounds (len {})", i, len);
            std::process::exit(1);
        }
        p.offset(i as isize + 2)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_new(len: i64) -> i64 {
    unsafe {
        let n = len.max(0) as libc::size_t;
        let cap = (n * 2).next_power_of_two().max(8) as libc::size_t;
        let o = crate::gc::sloth_gc_alloc((cap + 2) * 8) as *mut i64;
        *o = n as i64;
        *o.offset(1) = cap as i64;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_len(a: i64) -> i64 {
    unsafe { *(a as *mut i64) }
}

/// append one i64 word; returns the new length (call site usually ignores it)
#[no_mangle]
pub extern "C" fn sloth_arr_push(a: i64, w: i64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        if len >= cap {
            let nc = (cap * 2).max(8);
            let raw = libc::realloc(a as *mut libc::c_void, (nc + 2) as libc::size_t * 8);
            let p2 = raw as *mut i64;
            *p2.offset(1) = nc;
        }
        *p.offset((len + 2) as isize) = w;
        *p = len + 1;
        len + 1
    }
}

/// remove and return the last i64 word
#[no_mangle]
pub extern "C" fn sloth_arr_pop(a: i64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if len <= 0 {
            eprintln!("sloth panic: pop from empty array");
            std::process::exit(1);
        }
        *p = len - 1;
        *p.offset((len - 1) as isize + 2)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_push_f64(a: i64, w: f64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        if len >= cap {
            let nc = (cap * 2).max(8);
            let raw = libc::realloc(a as *mut libc::c_void, (nc + 2) as libc::size_t * 8);
            let p2 = raw as *mut i64;
            *p2.offset(1) = nc;
        }
        *(p.offset((len + 2) as isize) as *mut f64) = w;
        *p = len + 1;
        len + 1
    }
}

/// remove and return the last f64 word
#[no_mangle]
pub extern "C" fn sloth_arr_pop_f64(a: i64) -> f64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if len <= 0 {
            eprintln!("sloth panic: pop from empty array");
            std::process::exit(1);
        }
        *p = len - 1;
        *((p.offset((len - 1) as isize + 2)) as *mut f64)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_get(a: i64, i: i64) -> i64 {
    unsafe { *arr_index(a, i) }
}

#[no_mangle]
pub extern "C" fn sloth_arr_get_f64(a: i64, i: i64) -> f64 {
    unsafe { *(arr_index(a, i) as *mut f64) }
}

#[no_mangle]
pub extern "C" fn sloth_arr_set(a: i64, i: i64, v: i64) -> i64 {
    unsafe {
        *arr_index(a, i) = v;
        0
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_set_f64(a: i64, i: i64, v: f64) -> i64 {
    unsafe {
        *(arr_index(a, i) as *mut f64) = v;
        0
    }
}
