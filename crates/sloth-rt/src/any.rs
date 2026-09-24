//! `any`: runtime-typed boxed values (top type) + `__sloth_rt_write` renderer.
//!
//! An `any` value is a single word: `0` = nil, otherwise the handle of an rc
//! box `{ desc, word }`. `desc` points at a compiler-emitted structural type
//! descriptor (a flat `i64[11]`), so the runtime can dispatch on the concrete
//! shape without re-tagging the untagged word plane: only values that actually
//! cross an `any` surface get boxed.
//!
//! Descriptor layout (word offsets):
//! `[kind, flags, type_id, name_ptr, name_len, elem, key, val, disp, cls_id, rank]`
//! - `flags` bit0 = REF (word is an rc handle to release), bit1 = NILCAPABLE
//!   (word 0 means nil rather than a real value);
//! - `elem`/`key`/`val` point at child descriptors (arrays/maps/optionals);
//! - `disp` is an `extern "C" fn(i64)->i64` that renders an object/dyn value
//!   to a fresh `str` (0 when the class has no `to_str`);
//! - `cls_id`/`rank` are the runtime class id (objects) and tensor rank.

use crate::boxopt::box_get;
use crate::rc::{__sloth_rc_release, __sloth_rc_retain, dec_i, rc_addr, w_ref, w_unref};
use crate::strings::{intern_bytes, strb_append, strb_or_new, StrT};

/// Array header `[len, cap, buf]` — the layout owned by the self-hosted
/// container prelude (`lib/prelude/containers.slt`). The renderer only needs
/// the length and the element-buffer pointer.
unsafe fn arr_parts(w: i64) -> (i64, *const i64) {
    let p = w_unref(w) as *const i64;
    (*p, *p.offset(2) as *const i64)
}

/// Walk every live `(key, value)` pair of a Map. Layout:
/// `[cap, used, kflag, buckets]`, slot `[used, key, value, hash]` (4 words).
unsafe fn map_pairs(w: i64, mut f: impl FnMut(i64, i64)) {
    let m = w_unref(w) as *const i64;
    let cap = *m;
    let bp = *m.offset(3) as *const i64;
    let mut i = 0i64;
    while i < cap {
        let s = bp.offset((i * 4) as isize);
        if *s == 1 {
            f(*s.offset(1), *s.offset(2));
        }
        i += 1;
    }
}

// ---------------- descriptor word offsets ----------------
pub const D_KIND: isize = 0;
pub const D_FLAGS: isize = 1;
pub const D_TYPEID: isize = 2;
pub const D_NAME: isize = 3;
pub const D_NAMELEN: isize = 4;
pub const D_ELEM: isize = 5;
pub const D_KEY: isize = 6;
pub const D_VAL: isize = 7;
pub const D_DISP: isize = 8;
pub const D_CLSID: isize = 9;
pub const D_RANK: isize = 10;
pub const DESC_WORDS: usize = 11;

// ---------------- flags ----------------
pub const FLAG_REF: i64 = 1;
pub const FLAG_NILCAP: i64 = 2;

// ---------------- kinds (mirrored by codegen) ----------------
pub const AK_UNIT: i64 = 0;
pub const AK_BOOL: i64 = 1;
pub const AK_I64: i64 = 2;
pub const AK_F64: i64 = 3;
pub const AK_STR: i64 = 4;
pub const AK_RANGE: i64 = 5;
pub const AK_ARRAY: i64 = 6;
pub const AK_MAP: i64 = 7;
pub const AK_OPT: i64 = 8;
pub const AK_TENSOR: i64 = 9;
pub const AK_FN: i64 = 10;
pub const AK_NAMED: i64 = 11;
pub const AK_DYN: i64 = 12;
pub const AK_WEAK: i64 = 13;
pub const AK_FIBER: i64 = 14;
pub const AK_JOINHANDLE: i64 = 15;
pub const AK_CHANNEL: i64 = 16;
pub const AK_MUTEX: i64 = 17;
pub const AK_ATOMICINT: i64 = 18;
pub const AK_ANY: i64 = 19;
pub const AK_U64: i64 = 20;
pub const AK_I8: i64 = 21;
pub const AK_U8: i64 = 22;
pub const AK_I16: i64 = 23;
pub const AK_U16: i64 = 24;
pub const AK_I32: i64 = 25;
pub const AK_U32: i64 = 26;

#[inline]
unsafe fn kind_of(desc: i64) -> i64 {
    *(desc as *const i64).offset(D_KIND)
}
#[inline]
unsafe fn flags_of(desc: i64) -> i64 {
    *(desc as *const i64).offset(D_FLAGS)
}
#[inline]
unsafe fn child(desc: i64, off: isize) -> i64 {
    *(desc as *const i64).offset(off)
}

// ---------------- any box ----------------

/// death hook: release the boxed payload when it is a reference word
fn any_dtor(p: usize, _aux: u64) {
    unsafe {
        let b = p as *mut i64;
        let desc = *b;
        let word = *b.add(1);
        if desc != 0 && flags_of(desc) & FLAG_REF != 0 && word != 0 {
            __sloth_rc_release(word);
        }
    }
}

/// box a value word under `desc`; nil-capable word 0 collapses to `any` nil.
/// Reference payloads are retained so the box owns its own +1.
#[no_mangle]
pub extern "C" fn __sloth_any_from(desc_w: i64, word: i64) -> i64 {
    if desc_w == 0 {
        return 0;
    }
    unsafe {
        if word == 0 && flags_of(desc_w) & FLAG_NILCAP != 0 {
            return 0;
        }
        let b = rc_addr(16, Some(any_dtor)) as *mut i64;
        *b = desc_w;
        *b.add(1) = word;
        if flags_of(desc_w) & FLAG_REF != 0 && word != 0 {
            __sloth_rc_retain(word);
        }
        w_ref(b as usize)
    }
}

/// descriptor of an `any` (0 for nil / invalid)
#[no_mangle]
pub extern "C" fn __sloth_any_desc(box_w: i64) -> i64 {
    if box_w == 0 {
        0
    } else {
        unsafe { *(w_unref(box_w) as *const i64) }
    }
}

/// payload word of an `any` (0 for nil)
#[no_mangle]
pub extern "C" fn __sloth_any_word(box_w: i64) -> i64 {
    if box_w == 0 {
        0
    } else {
        unsafe { *((w_unref(box_w) as *const i64).add(1)) }
    }
}

/// kind of an `any` payload (`-1` for nil)
#[no_mangle]
pub extern "C" fn __sloth_any_kind(box_w: i64) -> i64 {
    if box_w == 0 {
        return -1;
    }
    unsafe {
        let d = __sloth_any_desc(box_w);
        if d == 0 {
            -1
        } else {
            kind_of(d)
        }
    }
}

/// runtime class id of the boxed object (i64::MIN when not an object/dyn)
#[no_mangle]
pub extern "C" fn __sloth_any_cls_id(box_w: i64) -> i64 {
    if box_w == 0 {
        return i64::MIN;
    }
    unsafe {
        let d = __sloth_any_desc(box_w);
        if d != 0 && matches!(kind_of(d), AK_NAMED | AK_DYN) {
            crate::objects::__sloth_obj_cls_id(__sloth_any_word(box_w))
        } else {
            i64::MIN
        }
    }
}

/// retain (return) the box: used when narrowing materializes a new owner slot
#[no_mangle]
pub extern "C" fn __sloth_any_retain(box_w: i64) -> i64 {
    if box_w != 0 {
        __sloth_rc_retain(box_w);
    }
    box_w
}

/// retain the boxed payload (for narrowing a reference down to its concrete
/// type); value payloads are returned untouched.
#[no_mangle]
pub extern "C" fn __sloth_any_ref(box_w: i64) -> i64 {
    if box_w == 0 {
        return 0;
    }
    unsafe {
        let d = __sloth_any_desc(box_w);
        let w = __sloth_any_word(box_w);
        if w != 0 && flags_of(d) & FLAG_REF != 0 {
            __sloth_rc_retain(w);
        }
        w
    }
}

/// `typeid(any)`: object class id, else the structural type id
#[no_mangle]
pub extern "C" fn __sloth_any_type_id(box_w: i64) -> i64 {
    if box_w == 0 {
        return 0;
    }
    unsafe {
        let d = __sloth_any_desc(box_w);
        if kind_of(d) == AK_NAMED || kind_of(d) == AK_DYN {
            crate::objects::__sloth_obj_cls_id(__sloth_any_word(box_w))
        } else {
            child(d, D_TYPEID)
        }
    }
}

/// `type_name(any)` as a fresh owned `str` (+1)
#[no_mangle]
pub extern "C" fn __sloth_any_type_name(box_w: i64) -> i64 {
    if box_w == 0 {
        return intern_bytes(b"nil".as_ptr() as usize, 3);
    }
    unsafe {
        let d = __sloth_any_desc(box_w);
        let k = kind_of(d);
        if k == AK_NAMED || k == AK_DYN {
            return crate::objects::__sloth_obj_type_name(__sloth_any_word(box_w));
        }
        let np = child(d, D_NAME);
        let nl = child(d, D_NAMELEN);
        if np == 0 {
            intern_bytes(b"any".as_ptr() as usize, 3)
        } else {
            intern_bytes(np as usize, nl)
        }
    }
}

/// structural descriptor equality (cross-module safe: compares kinds/names,
/// recursing element/key/value/rank)
unsafe fn desc_eq(a: i64, b: i64) -> bool {
    if a == b {
        return true;
    }
    if a == 0 || b == 0 {
        return false;
    }
    let ka = kind_of(a);
    if ka != kind_of(b) {
        return false;
    }
    match ka {
        AK_ARRAY | AK_OPT => desc_eq(child(a, D_ELEM), child(b, D_ELEM)),
        AK_MAP => {
            desc_eq(child(a, D_KEY), child(b, D_KEY)) && desc_eq(child(a, D_VAL), child(b, D_VAL))
        }
        AK_TENSOR => {
            child(a, D_RANK) == child(b, D_RANK) && desc_eq(child(a, D_ELEM), child(b, D_ELEM))
        }
        AK_NAMED | AK_DYN => {
            let la = child(a, D_NAMELEN);
            let lb = child(b, D_NAMELEN);
            la == lb
                && la >= 0
                && libc::memcmp(
                    child(a, D_NAME) as *const libc::c_void,
                    child(b, D_NAME) as *const libc::c_void,
                    la as libc::size_t,
                ) == 0
        }
        _ => true,
    }
}

/// `x is T` for non-class targets: exact structural match of the boxed value's
/// descriptor against the target descriptor. Class targets use
/// `__sloth_any_cls_id` + a compile-time ancestor chain instead.
#[no_mangle]
pub extern "C" fn __sloth_any_is(box_w: i64, target_desc: i64) -> i64 {
    if box_w == 0 || target_desc == 0 {
        return 0;
    }
    unsafe {
        let d = __sloth_any_desc(box_w);
        crate::rc::enc_i(desc_eq(d, target_desc) as i64)
    }
}

// ---------------- rendering ----------------

unsafe fn append(p: *mut crate::strings::StrB, bytes: &[u8]) {
    strb_append(p, bytes.as_ptr() as *const libc::c_void, bytes.len());
}

unsafe fn append_str_handle(p: *mut crate::strings::StrB, h: i64) {
    if h == 0 {
        return;
    }
    let td = w_unref(h) as *const StrT;
    strb_append(p, (*td).data, (*td).len);
}

unsafe fn append_i(p: *mut crate::strings::StrB, v: i64) {
    let s = format!("{}", v);
    strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
}

unsafe fn append_f(p: *mut crate::strings::StrB, v: f64) {
    let s = format!("{}", v);
    strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
}

unsafe fn render_tensor(
    p: *mut crate::strings::StrB,
    dim: usize,
    shape: &[i64],
    stride: &[i64],
    base: i64,
    data: *const u8,
    is_float: bool,
) {
    append(p, b"[");
    let n = shape[dim];
    let mut i = 0i64;
    while i < n {
        if i > 0 {
            append(p, b", ");
        }
        let off = base + i * stride[dim];
        if dim + 1 == shape.len() {
            let cell = data.offset((off * 8) as isize) as *const i64;
            let raw = *cell;
            if is_float {
                append_f(p, f64::from_bits(raw as u64));
            } else {
                append_i(p, raw);
            }
        } else {
            render_tensor(p, dim + 1, shape, stride, off, data, is_float);
        }
        i += 1;
    }
    append(p, b"]");
}

/// append the rendered form of `(desc, word)` to the builder
unsafe fn render(b: i64, desc: i64, word: i64) -> i64 {
    let p = strb_or_new(b);
    if desc == 0 {
        append(p, b"nil");
        return w_ref(p as usize);
    }
    let k = kind_of(desc);
    match k {
        AK_UNIT => append(p, b"()"),
        AK_I64 => append_i(p, dec_i(word)),
        AK_I8 | AK_I16 | AK_I32 | AK_U8 | AK_U16 | AK_U32 => append_i(p, dec_i(word)),
        AK_U64 => {
            let s = format!("{}", word as u64);
            strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
        }
        AK_F64 => append_f(p, f64::from_bits(word as u64)),
        AK_BOOL => append(p, if word != 0 { b"true" } else { b"false" }),
        AK_STR => {
            if word == 0 {
                append(p, b"nil");
            } else {
                append_str_handle(p, word);
            }
        }
        AK_RANGE => {
            if word == 0 {
                append(p, b"nil");
            } else {
                // frozen range-box layout `[lo, hi]` (lib/prelude/core.slt)
                let r = w_unref(word) as *const i64;
                append_i(p, *r);
                append(p, b"..");
                append_i(p, *r.add(1));
            }
        }
        AK_OPT => {
            let elem = child(desc, D_ELEM);
            if word == 0 {
                append(p, b"nil");
            } else if elem != 0 && matches!(kind_of(elem), AK_I64 | AK_U64 | AK_F64 | AK_BOOL) {
                let payload = box_get(word);
                return render(w_ref(p as usize), elem, payload);
            } else {
                return render(w_ref(p as usize), elem, word);
            }
        }
        AK_ARRAY => {
            let elem = child(desc, D_ELEM);
            if word == 0 {
                append(p, b"nil");
            } else {
                append(p, b"[");
                let (n, buf) = unsafe { arr_parts(word) };
                let mut i = 0i64;
                while i < n {
                    if i > 0 {
                        append(p, b", ");
                    }
                    let w = unsafe { *buf.offset(i as isize) };
                    render(w_ref(p as usize), elem, w);
                    i += 1;
                }
                append(p, b"]");
            }
        }
        AK_MAP => {
            let kd = child(desc, D_KEY);
            let vd = child(desc, D_VAL);
            if word == 0 {
                append(p, b"nil");
            } else {
                append(p, b"{");
                let mut first = true;
                unsafe {
                    map_pairs(word, |kw, vw| {
                        if !first {
                            append(p, b", ");
                        }
                        first = false;
                        render(w_ref(p as usize), kd, kw);
                        append(p, b": ");
                        render(w_ref(p as usize), vd, vw);
                    });
                }
                append(p, b"}");
            }
        }
        AK_NAMED | AK_DYN => {
            if word == 0 {
                append(p, b"nil");
                return w_ref(p as usize);
            }
            let disp = child(desc, D_DISP);
            if disp != 0 {
                let f: extern "C" fn(i64) -> i64 = std::mem::transmute(disp as usize);
                let s = f(word);
                append_str_handle(p, s);
                if s != 0 {
                    __sloth_rc_release(s);
                }
            } else {
                let np = child(desc, D_NAME);
                let nl = child(desc, D_NAMELEN);
                if np != 0 {
                    let bytes = std::slice::from_raw_parts(np as *const u8, nl.max(0) as usize);
                    append(p, bytes);
                }
            }
        }
        AK_TENSOR => {
            if word == 0 {
                append(p, b"nil");
            } else {
                let (is_float, shape, stride, data) = crate::tensors::tensor_parts(word);
                if shape.is_empty() {
                    append(p, b"tensor([])");
                } else {
                    append(p, b"tensor(");
                    render_tensor(p, 0, &shape, &stride, 0, data, is_float);
                    append(p, b")");
                }
            }
        }
        AK_ANY => {
            if word == 0 {
                append(p, b"nil");
            } else {
                let d = __sloth_any_desc(word);
                let w = __sloth_any_word(word);
                return render(w_ref(p as usize), d, w);
            }
        }
        AK_FN => append(p, b"<fn>"),
        AK_WEAK => append(p, b"<weak>"),
        AK_FIBER => append(p, b"<fiber>"),
        AK_JOINHANDLE => append(p, b"<handle>"),
        AK_CHANNEL => append(p, b"<channel>"),
        AK_MUTEX => append(p, b"<mutex>"),
        AK_ATOMICINT => append(p, b"<atomic>"),
        _ => {
            let np = child(desc, D_NAME);
            let nl = child(desc, D_NAMELEN);
            if np != 0 {
                let bytes = std::slice::from_raw_parts(np as *const u8, nl.max(0) as usize);
                append(p, bytes);
            }
        }
    }
    w_ref(p as usize)
}

/// `__sloth_rt_write(v: any): str` — render a value to a fresh owned `str`,
/// dispatching on the boxed runtime type and recursing through containers.
#[no_mangle]
pub extern "C" fn __sloth_rt_write(v: i64) -> i64 {
    if v == 0 {
        return intern_bytes(b"nil".as_ptr() as usize, 3);
    }
    let b = unsafe {
        let d = __sloth_any_desc(v);
        let w = __sloth_any_word(v);
        render(0, d, w)
    };
    crate::strings::__sloth_str_finish(b)
}

/// `__sloth_rt_puts(v: str): unit` — print a string followed by a newline.
#[no_mangle]
pub extern "C" fn __sloth_rt_puts(s_w: i64) -> i64 {
    use std::io::Write;
    unsafe {
        if s_w != 0 {
            let td = w_unref(s_w) as *const StrT;
            let sl = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
            let mut so = std::io::stdout();
            let _ = so.write_all(sl);
        }
        let mut so = std::io::stdout();
        let _ = so.write_all(b"\n");
        let _ = so.flush();
    }
    0
}
