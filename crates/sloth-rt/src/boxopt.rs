//! Value-optional boxes: immutable one-word payload cells (int / float /
//! bool PVOID-lifted from the "word + 0 = nil" convention).
//!
//! An optional of a value type is a 1-word slot holding either 0 (= nil, the
//! same word null set every other optional surface uses) or the handle of a
//! heap box holding the payload. A box is an rc-managed user value: every
//! copy counts (retain/release via the rc core), and reaching zero frees
//! the chunk. Because the payload never shares storage with the box handle,
//! an `int?` value 0 is a real box pointer and never confuses with nil.

use crate::alloc::sloth_rt_alloc;
use crate::rc::track_user;

/// allocate a box holding an integer payload
#[no_mangle]
pub extern "C" fn sloth_box_new(v: i64) -> i64 {
    unsafe {
        let b = sloth_rt_alloc(8) as *mut i64;
        *b = v;
        track_user(b as usize);
        b as i64
    }
}

/// allocate a box holding a float payload
#[no_mangle]
pub extern "C" fn sloth_box_new_f64(v: f64) -> i64 {
    unsafe {
        let b = sloth_rt_alloc(8) as *mut i64;
        *(b as *mut f64) = v;
        track_user(b as usize);
        b as i64
    }
}

/// payload read (integer route; 0 = nil reads as 0 — callers that matter
/// never dereference a nil box: non-nil was checked or the value is consumed
/// under unwrap-or-0 semantics)
#[no_mangle]
pub extern "C" fn sloth_box_get(h: i64) -> i64 {
    if h == 0 {
        0
    } else {
        unsafe { *(h as *const i64) }
    }
}

/// payload read (float route; nil reads 0.0)
#[no_mangle]
pub extern "C" fn sloth_box_get_f64(h: i64) -> f64 {
    if h == 0 {
        0.0
    } else {
        unsafe { *(h as *const f64) }
    }
}
