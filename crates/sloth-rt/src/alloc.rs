//! Deterministic behind-the-rc allocator (de-tag).
//!
//! Tracked chunks are allocated by the rc core (`rc_addr`): a hidden 48-byte
//! header followed by the user payload; the payload pointer is the rt handle.
//! Untracked allocations (class metadata, vtables, map bucket buffers) stay
//! plain calloc'd memory with no header and no counts.
//!
//! All C-ABI symbols keep their names; `__sloth_rt_realloc` now relocates the
//! header+payload pair through the rc core (the counts and the weak chain
//! follow the chunk) and returns the new handle word.

use crate::panics::panic_msg;
use crate::rc::relocate;

#[no_mangle]
pub extern "C" fn __sloth_rt_alloc(n: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::calloc(1, n) }
}

/// contents-preserving growth for tracked chunks (arrays push path);
/// `p` is the old handle word, `n` the new payload byte size — the returned
/// word supersedes the old one immediately
#[no_mangle]
pub extern "C" fn __sloth_rt_realloc(p: i64, n: usize) -> i64 {
    if p == 0 {
        panic_msg("realloc of a nil word");
    }
    unsafe { relocate(p, n) }
}
