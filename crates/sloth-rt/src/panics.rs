//! Panic entry points: all abort the process with a diagnosis on stderr.

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

/// missing map key (never returns normally)
#[no_mangle]
pub extern "C" fn sloth_panic_nokey(key: i64) -> i64 {
    eprintln!("sloth panic: map key not found ({})", key);
    std::process::exit(1);
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

/// unwrap() on an err Result: unrecoverable, exit with diagnosis
#[no_mangle]
pub extern "C" fn sloth_panic_unwrap() -> i64 {
    eprintln!("sloth panic: unwrap() on err Result");
    std::process::exit(1);
}
