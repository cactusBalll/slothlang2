//! Value-optional boxes: immutable one-word payload cells (de-tag).
//!
//! An optional of a value type is a 1-word slot holding either 0 (= nil, the
//! same word null set every other optional surface uses) or the handle of a
//! heap box holding the payload word. Value 0 inside a box is a real handle
//! and never confuses with nil. Callers encode/decode at the codec boundary,
//! so a box is a plain word cell.
//!
//! The C-ABI faces `__sloth_box_new`/`__sloth_box_get` are self-hosted in
//! `lib/prelude/core.slt`; the functions below are the Rust-internal
//! equivalents (fiber/channel construction, `any` rendering) over the same
//! frozen `[payload]` layout — keep both sides in sync.

use crate::rc::{rc_addr, w_ref, w_unref};

/// allocate a box holding a raw payload word (Rust-internal; no C export)
pub fn box_new(v: i64) -> i64 {
    unsafe {
        let b = rc_addr(8, None) as *mut i64;
        *b = v;
        w_ref(b as usize)
    }
}

/// payload read (nil reads 0 — callers never dereference a nil box except
/// under unwrap-or-0 semantics) (Rust-internal; no C export)
pub fn box_get(h_w: i64) -> i64 {
    if h_w != 0 {
        unsafe { *(w_unref(h_w) as *const i64) }
    } else {
        0
    }
}
