//! Deterministic behind-the-rc allocator (tagged-word header migration).
//!
//! Tracked chunks are allocated by the rc core (`rc_addr`): a hidden 48-byte
//! header followed by the user payload; the payload pointer (tagged word) is
//! the rt handle. Untracked allocations (class metadata, vtables, map bucket
//! buffers) stay plain calloc'd memory with no header and no counts.
//!
//! All C-ABI symbols keep their names; `sloth_rt_realloc` now relocates the
//! header+payload pair through the rc core (the counts and the weak chain
//! follow the chunk) and returns the new tagged handle word.

use crate::panics::panic_msg;
use crate::rc::{relocate, w_is_ref};

#[no_mangle]
pub extern "C" fn sloth_rt_alloc(n: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::calloc(1, n) }
}

/// contents-preserving growth for tracked chunks (arrays push path);
/// `p` is the old handle word (tagged), `n` the new payload byte size
/// — the returned word supersedes the old one immediately
#[no_mangle]
pub extern "C" fn sloth_rt_realloc(p: i64, n: usize) -> i64 {
    if !w_is_ref(p) {
        panic_msg("realloc of a value word");
    }
    unsafe { relocate(p, n) }
}
