//! Extern (`extern func`) support: example C-ABI math targets and opaque
//! extern-type tokens.

/// example `extern func` targets (§5.4): linked directly for JIT/AOT
#[no_mangle]
pub extern "C" fn sloth_extern_floor(x: f64) -> f64 {
    x.floor()
}

#[no_mangle]
pub extern "C" fn sloth_extern_powf(a: f64, b: f64) -> f64 {
    a.powf(b)
}

/// extern-type opaque token example: allocate a boxed int (op *mut c_void)
#[no_mangle]
pub extern "C" fn sloth_extern_tok_new() -> *mut libc::c_void {
    unsafe {
        let p = libc::malloc(8) as *mut i64;
        *p = 99;
        p as *mut libc::c_void
    }
}

#[no_mangle]
pub extern "C" fn sloth_extern_tok_val(t: *const libc::c_void) -> i64 {
    if t.is_null() {
        eprintln!("sloth panic: unreachable opaque token");
        std::process::exit(1);
    }
    unsafe { *(t as *const i64) }
}
