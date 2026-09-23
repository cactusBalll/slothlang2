//! Bare word-level memory primitives — the substrate the self-hosted
//! container prelude (`lib/prelude/containers.slt`) is built on.
//!
//! These are deliberately untyped: an address arrives as an `int` word and
//! slot indices are word offsets. `sloth_rt_alloc` (in `alloc.rs`) hands out
//! zeroed untracked chunks; `sloth_free` returns them. Together with the rc
//! core (`sloth_rc_new`/`sloth_rc_retain`/`sloth_rc_release`) this is the
//! entire surface the container implementation needs from the runtime.

/// free an untracked chunk allocated by `sloth_rt_alloc` (0 is inert)
#[no_mangle]
pub extern "C" fn sloth_free(p: i64) {
    if p != 0 {
        unsafe { libc::free(p as *mut libc::c_void) };
    }
}

/// load word `i` (i.e. `p[i]`) from a raw address
#[no_mangle]
pub extern "C" fn sloth_mem_load(p: i64, i: i64) -> i64 {
    unsafe { *((p as *mut i64).offset(i as isize)) }
}

/// store word `v` at `p[i]`
#[no_mangle]
pub extern "C" fn sloth_mem_store(p: i64, i: i64, v: i64) {
    unsafe { *((p as *mut i64).offset(i as isize)) = v };
}

/// copy `n` words from `src` to `dst` (non-overlapping)
#[no_mangle]
pub extern "C" fn sloth_mem_copy(dst: i64, src: i64, n: i64) {
    if n > 0 {
        unsafe { std::ptr::copy_nonoverlapping(src as *const i64, dst as *mut i64, n as usize) };
    }
}
