//! Auto-boxed builtin value types (int/float/bool) for `dyn Trait`.
//!
//! Value types have no object header, so a value coerced to a `dyn Trait`
//! surface is wrapped in a synthetic object: word 0 = a per-kind `ObjInfo`
//! (reserved negative class id, ref mask 0 so the value field is never
//! released), word 1 = the builtin kind's vtable, field 0 = the raw value
//! word. The box is an ordinary rc object, so death cascades need no special
//! case.
//!
//! The kind stays cheap on the word plane: `0 = int`, `1 = float`, `2 = bool`.
//! Trait methods for the predefined surfaces (Display/Equatable/Hashable/
//! Comparable) are served by the three helpers below; codegen emits one thin
//! bridge per (kind, method) that forwards to them. Operands that reach the
//! comparison helper are box handles (or nil): codegen boxes builtin values
//! at `dyn` surfaces before the call.

use crate::rc::{dec_f_bits, dec_i, enc_i};

/// reserved runtime class ids for builtin value types (user classes start at 0)
pub const CLS_INT: i64 = -1;
pub const CLS_FLOAT: i64 = -2;
pub const CLS_BOOL: i64 = -3;

const KIND_INT: i64 = 0;
const KIND_FLOAT: i64 = 1;
const KIND_BOOL: i64 = 2;

fn kind_cls(kind: i64) -> i64 {
    match kind {
        KIND_FLOAT => CLS_FLOAT,
        KIND_BOOL => CLS_BOOL,
        _ => CLS_INT,
    }
}

// ---------------- per-kind class metadata (lazily built, never freed) ----------------

use std::sync::atomic::{AtomicI64, Ordering};
static INFO_INT: AtomicI64 = AtomicI64::new(0);
static INFO_FLOAT: AtomicI64 = AtomicI64::new(0);
static INFO_BOOL: AtomicI64 = AtomicI64::new(0);

fn info_slot(kind: i64) -> &'static AtomicI64 {
    match kind {
        KIND_FLOAT => &INFO_FLOAT,
        KIND_BOOL => &INFO_BOOL,
        _ => &INFO_INT,
    }
}

/// `ObjInfo` word for a builtin kind (kind arrives raw)
#[no_mangle]
pub extern "C" fn sloth_builtin_info(kind_w: i64) -> i64 {
    let kind = kind_w;
    let slot = info_slot(kind);
    let cur = slot.load(Ordering::Acquire);
    if cur != 0 {
        return cur;
    }
    // ref mask stays 0: field 0 holds a value word, never an rc handle
    let h = crate::objects::sloth_cls_info(0, kind_cls(kind));
    let _ = slot.compare_exchange(0, h, Ordering::AcqRel, Ordering::Acquire);
    slot.load(Ordering::Acquire)
}

// ---------------- box helpers ----------------

/// field 0 (the payload word) of a box; nil reads 0
fn payload(w: i64) -> i64 {
    if w != 0 {
        crate::objects::sloth_obj_field(w, 0)
    } else {
        0
    }
}

/// payload word of a box (nil = 0)
#[no_mangle]
pub extern "C" fn sloth_dyn_unbox(box_w: i64) -> i64 {
    payload(box_w)
}

/// Display `to_str`: render the boxed value into a fresh `str` handle
#[no_mangle]
pub extern "C" fn sloth_dyn_to_str(box_w: i64, kind_w: i64) -> i64 {
    let kind = kind_w;
    let v = payload(box_w);
    let b = match kind {
        KIND_FLOAT => crate::strings::sloth_str_push_f(0, f64::from_bits(dec_f_bits(v))),
        KIND_BOOL => crate::strings::sloth_str_push_b(0, v),
        _ => crate::strings::sloth_str_push_i(0, v),
    };
    crate::strings::sloth_str_finish(b)
}

/// kind of an operand: a box handle carries it in the reserved class id; nil
/// falls back to the caller's kind
fn operand_kind(w: i64, fallback: i64) -> i64 {
    if w != 0 {
        match crate::objects::sloth_obj_cls_id(w) {
            CLS_FLOAT => KIND_FLOAT,
            CLS_BOOL => KIND_BOOL,
            CLS_INT => KIND_INT,
            _ => fallback,
        }
    } else {
        fallback
    }
}

fn mix64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

/// Hashable `__hash__`: int word, content-stable for the builtin kind
#[no_mangle]
pub extern "C" fn sloth_dyn_hash(box_w: i64, kind_w: i64) -> i64 {
    let kind = kind_w;
    let v = payload(box_w);
    match kind {
        KIND_FLOAT => enc_i(mix64(dec_f_bits(v)) as i64),
        // int/bool payload is already the raw integer word
        _ => v,
    }
}

/// decode a payload word to a comparable raw f64 (int/bool widen)
fn as_f64(kind: i64, v: i64) -> f64 {
    match kind {
        KIND_FLOAT => f64::from_bits(dec_f_bits(v)),
        _ => dec_i(v) as f64,
    }
}

/// comparison family. `op`: 0 eq, 1 ne, 2 lt, 3 le, 4 gt, 5 ge.
/// Operands are box handles or nil (codegen boxes builtin values at dyn
/// surfaces).
#[no_mangle]
pub extern "C" fn sloth_dyn_binop(a_w: i64, b_w: i64, kind_w: i64, op_w: i64) -> i64 {
    let kind = kind_w;
    let op = op_w;
    let r = if b_w == 0 {
        // nil only equals nil (no nil box can reach here, so false)
        false
    } else {
        let ka = operand_kind(a_w, kind);
        let kb = operand_kind(b_w, kind);
        if ka == KIND_FLOAT || kb == KIND_FLOAT {
            let x = as_f64(ka, payload(a_w));
            let y = as_f64(kb, payload(b_w));
            match op {
                0 => x == y,
                1 => x != y,
                2 => x < y,
                3 => x <= y,
                4 => x > y,
                _ => x >= y,
            }
        } else {
            let x = dec_i(payload(a_w));
            let y = dec_i(payload(b_w));
            match op {
                0 => x == y,
                1 => x != y,
                2 => x < y,
                3 => x <= y,
                4 => x > y,
                _ => x >= y,
            }
        }
    };
    enc_i(r as i64)
}
