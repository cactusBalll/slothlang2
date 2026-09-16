//! Deterministic behind-the-rc allocator (ARC migration: replaces Boehm).
//!
//! Every container chunk is plain libc memory: fresh chunks come from
//! `calloc` (zeroed words — nil fields/unused slots contract), growth from
//! `realloc`. There is no collector and no background thread; chunk memory
//! is freed by the rc core (rc.rs) when an owner's count reaches zero, so
//! the kept C-ABI aliases below only translate sizes.

#[no_mangle]
pub extern "C" fn sloth_rt_alloc(n: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::calloc(1, n) }
}

/// contents-preserving growth (arrays push path); the returned pointer
/// supersedes the old one immediately (realloc moves) — the rc core's
/// `transfer` follows the count entry to the new address
#[no_mangle]
pub extern "C" fn sloth_rt_realloc(p: *mut libc::c_void, n: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::realloc(p, n) }
}
