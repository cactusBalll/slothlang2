//! Range values: a two-word rc box `{lo, hi}` carrying raw bounds.
//!
//! The word plane is a single i64, so a first-class `range` value (bound to a
//! variable, passed around) cannot inline both bounds. It rides a small
//! rc-tracked heap cell instead. `hi` is stored already normalized to the
//! exclusive upper bound (inclusive ranges add one at construction), so the
//! for-loop only needs a signed `<` test.

use crate::rc::{rc_addr, w_ref, w_unref};

/// pack `lo`/`hi` words into a fresh range handle
#[no_mangle]
pub extern "C" fn sloth_range_pack(lo: i64, hi: i64) -> i64 {
    unsafe {
        let b = rc_addr(16, None) as *mut i64;
        *b = lo;
        *b.add(1) = hi;
        w_ref(b as usize)
    }
}

/// lower bound word
#[no_mangle]
pub extern "C" fn sloth_range_lo(h_w: i64) -> i64 {
    if h_w != 0 {
        unsafe { *(w_unref(h_w) as *const i64) }
    } else {
        0
    }
}

/// exclusive upper bound word
#[no_mangle]
pub extern "C" fn sloth_range_hi(h_w: i64) -> i64 {
    if h_w != 0 {
        unsafe { *((w_unref(h_w) as *const i64).add(1)) }
    } else {
        0
    }
}
