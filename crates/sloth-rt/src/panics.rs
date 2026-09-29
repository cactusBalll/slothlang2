//! Panic entry points: all abort the process with a diagnosis on stderr.

/// Terminate without running `atexit`/destructors. Detached worker threads may
/// still be executing JIT-emitted code, and a full `process::exit` teardown
/// unmaps the JIT session under them → intermittent SIGSEGV (bug G6). `_exit`
/// stops the process immediately; stderr is flushed first so the diagnosis is
/// not lost. Shared by every terminal path (panic diagnostics and the
/// slice-assignment / opaque-token error exits) so no route reintroduces the
/// teardown race (bug B5).
pub(crate) fn exit_now(code: i32) -> ! {
    use std::io::Write;
    let _ = std::io::stderr().flush();
    unsafe { libc::_exit(code) }
}

#[no_mangle]
pub extern "C" fn __sloth_panic(msg: *const libc::c_char) -> ! {
    let s = unsafe {
        if msg.is_null() {
            "panic".to_string()
        } else {
            std::ffi::CStr::from_ptr(msg).to_string_lossy().to_string()
        }
    };
    eprintln!("sloth panic: {}", s);
    exit_now(1);
}

/// missing map key (never returns normally)
#[no_mangle]
pub extern "C" fn __sloth_panic_nokey(key: i64) -> i64 {
    eprintln!("sloth panic: map key not found ({})", key);
    exit_now(1);
}

/// dyn receiver is not an implementing class (never returns)
#[no_mangle]
pub extern "C" fn __sloth_panic_noimpl(cls_id: i64) -> i64 {
    eprintln!(
        "sloth panic: no impl for trait method on receiver (cls {})",
        cls_id
    );
    exit_now(1);
}

/// unwrap() on an err Result: unrecoverable, exit with diagnosis
#[no_mangle]
pub extern "C" fn __sloth_panic_unwrap() -> i64 {
    eprintln!("sloth panic: unwrap() on err Result");
    exit_now(1);
}

/// integer division/modulo by zero: unrecoverable (design §5.5)
#[no_mangle]
pub extern "C" fn __sloth_panic_divzero() -> i64 {
    eprintln!("sloth panic: integer division by zero");
    exit_now(1);
}

/// runtime panic with a fixed message (patch #34: the single exit path)
pub fn panic_msg(msg: &str) -> ! {
    eprintln!("sloth panic: {}", msg);
    exit_now(1);
}

/// container out-of-bounds: kind names the container ("array"/"pop" ...),
/// i the offending index (or key for kind "map"), len the container length
pub fn panic_oob(kind: &str, i: i64, len: i64) -> ! {
    eprintln!(
        "sloth panic: {} index {} out of bounds (len {})",
        kind, i, len
    );
    exit_now(1);
}

/// C-ABI out-of-bounds entry for the self-hosted container prelude (array
/// indexing / pop). `i` is the offending index, `len` the container length.
#[no_mangle]
pub extern "C" fn __sloth_panic_oob(i: i64, len: i64) -> i64 {
    panic_oob("array", i, len)
}

/// C-ABI pop-from-empty entry for the self-hosted container prelude.
#[no_mangle]
pub extern "C" fn __sloth_panic_pop(i: i64, len: i64) -> i64 {
    panic_oob("pop", i, len)
}

/// C-ABI out-of-bounds entry for `Array` range slicing (`a[lo..hi]`): `i` is
/// the exclusive end (or the offending `lo + len`), `len` the array length.
#[no_mangle]
pub extern "C" fn __sloth_panic_slice(i: i64, len: i64) -> i64 {
    panic_oob("array slice", i, len)
}

/// C-ABI length-mismatch entry for `Array` slice assignment
/// (`a[lo..hi] = src`): `got` is `src.len()`, `want` the slice length.
#[no_mangle]
pub extern "C" fn __sloth_panic_slice_assign(got: i64, want: i64) -> i64 {
    eprintln!(
        "sloth panic: array slice assignment length {} does not match target length {}",
        got, want
    );
    exit_now(1);
}
