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
