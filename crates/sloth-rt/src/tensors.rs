//! Tensor descriptors (TE-P1): an `Array`-shaped rc payload carrying a
//! dynamic shape/stride header plus a non-tracked element buffer.
//!
//! Payload (7 tagged words): `[flags, ndim, shape_ptr, stride_ptr,
//! data_ptr, owner, total]`.
//! - `flags` bit0 = element kind (0 = int, 1 = float), bit1 = view;
//! - `shape_ptr`/`stride_ptr` are plain `malloc`'d `i64[ndim]` arrays;
//! - `data_ptr` is the element base of *this* tensor (row-major, 8-byte
//!   elements) — for a view it points into the owner's buffer at the view's
//!   offset, so a write through the view writes the owner (KV-cache rule);
//! - `owner` is the retained parent handle for a view (`0` = self-owned),
//!   keeping the storage alive with no collector;
//! - `total` is the owned allocation's element count (`0` for views).
//!
//! Views are cheap header clones. The death hook frees the shape/stride
//! arrays, then either releases the owner (view) or frees the data buffer.

use crate::rc::{
    dec_f_bits, dec_i, enc_f_bits, enc_i, rc_addr, sloth_rc_release, sloth_rc_retain, w_is_ref,
    w_ref, w_unref,
};

const HDR_WORDS: usize = 7;
const FLAG_FLOAT: i64 = 1;
const FLAG_VIEW: i64 = 2;
const ELEM: usize = 8;

#[inline]
unsafe fn shape_of(d: *mut i64) -> *mut i64 {
    *d.offset(2) as *mut i64
}
#[inline]
unsafe fn stride_of(d: *mut i64) -> *mut i64 {
    *d.offset(3) as *mut i64
}
#[inline]
unsafe fn data_of(d: *mut i64) -> *mut u8 {
    *d.offset(4) as *mut u8
}
#[inline]
unsafe fn kind_of(d: *mut i64) -> i64 {
    *d & FLAG_FLOAT
}
#[inline]
unsafe fn ndim_of(d: *mut i64) -> usize {
    *d.offset(1) as usize
}

/// element count = product of the shape
unsafe fn total_of(d: *mut i64) -> i64 {
    let n = ndim_of(d);
    let sh = shape_of(d);
    let mut acc: i64 = 1;
    let mut i = 0usize;
    while i < n {
        acc *= *sh.offset(i as isize);
        i += 1;
    }
    acc
}

/// allocate `ndim`-dimensional zeroed tensor of `kind` (tagged dim words)
unsafe fn tensor_new_impl(dims: &[i64], kind: i64) -> i64 {
    let n = dims.len();
    let shape = libc::malloc(n * 8) as *mut i64;
    let stride = libc::malloc(n * 8) as *mut i64;
    if shape.is_null() || stride.is_null() {
        crate::panics::panic_msg("out of memory");
    }
    let mut acc: i64 = 1;
    let mut i = n;
    while i > 0 {
        i -= 1;
        *shape.add(i) = dims[i];
        *stride.add(i) = acc;
        acc *= dims[i];
    }
    let data = libc::calloc(acc.max(1) as usize, ELEM) as *mut u8;
    if data.is_null() {
        crate::panics::panic_msg("out of memory");
    }
    let p = rc_addr(HDR_WORDS * 8, Some(tensor_dtor)) as *mut i64;
    *p = kind & FLAG_FLOAT;
    *p.offset(1) = n as i64;
    *p.offset(2) = shape as i64;
    *p.offset(3) = stride as i64;
    *p.offset(4) = data as i64;
    *p.offset(5) = 0;
    *p.offset(6) = acc;
    w_ref(p as usize)
}

/// death hook: free the header arrays, then release the owner (view) or the
/// data buffer (self-owned)
fn tensor_dtor(p: usize, _aux: u64) {
    unsafe {
        let d = p as *mut i64;
        let sh = shape_of(d);
        let st = stride_of(d);
        if !sh.is_null() {
            libc::free(sh as *mut libc::c_void);
        }
        if !st.is_null() {
            libc::free(st as *mut libc::c_void);
        }
        let owner = *d.offset(5);
        if owner != 0 {
            sloth_rc_release(owner);
        } else {
            let data = data_of(d);
            if !data.is_null() {
                libc::free(data as *mut libc::c_void);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_tensor_new_1(d0: i64, kind: i64) -> i64 {
    unsafe { tensor_new_impl(&[dec_i(d0)], dec_i(kind)) }
}

#[no_mangle]
pub extern "C" fn sloth_tensor_new_2(d0: i64, d1: i64, kind: i64) -> i64 {
    unsafe { tensor_new_impl(&[dec_i(d0), dec_i(d1)], dec_i(kind)) }
}

#[no_mangle]
pub extern "C" fn sloth_tensor_new_3(d0: i64, d1: i64, d2: i64, kind: i64) -> i64 {
    unsafe { tensor_new_impl(&[dec_i(d0), dec_i(d1), dec_i(d2)], dec_i(kind)) }
}

/// view construction: `off` element offset along the flattened parent data,
/// `drop` != 0 drops dim 0 (`t[i]`), else dim 0 is kept with the new length
/// `len0` (`t[a..b]`). Bounds-checked.
#[no_mangle]
pub extern "C" fn sloth_tensor_view(t: i64, off_w: i64, drop_w: i64, len0_w: i64) -> i64 {
    if !w_is_ref(t) {
        crate::panics::panic_msg("view of a nil tensor");
    }
    unsafe {
        let base = w_unref(t) as *mut i64;
        let kind = kind_of(base);
        let ndim = ndim_of(base);
        let sh = shape_of(base);
        let st = stride_of(base);
        let data = data_of(base);
        if ndim == 0 {
            crate::panics::panic_msg("view of a rank-0 tensor");
        }
        let off = dec_i(off_w);
        let drop = dec_i(drop_w) != 0;
        let len0 = dec_i(len0_w);
        let d0 = *sh;
        if off < 0 || off > d0 {
            crate::panics::panic_oob("tensor", off, d0);
        }
        if !drop && (len0 < 0 || off + len0 > d0) {
            crate::panics::panic_oob("tensor", off + len0, d0);
        }
        let nn = if drop { ndim - 1 } else { ndim };
        let ns = libc::malloc(nn * 8) as *mut i64;
        let nst = libc::malloc(nn * 8) as *mut i64;
        if ns.is_null() || nst.is_null() {
            crate::panics::panic_msg("out of memory");
        }
        if drop {
            let mut i = 0usize;
            while i < nn {
                *ns.add(i) = *sh.add(i + 1);
                *nst.add(i) = *st.add(i + 1);
                i += 1;
            }
        } else {
            let mut i = 0usize;
            while i < nn {
                *ns.add(i) = *sh.add(i);
                *nst.add(i) = *st.add(i);
                i += 1;
            }
            *ns = len0;
        }
        // `off` indexes dim 0, so the element offset scales by stride[0]
        let ndata = data.offset((off * *st) as isize * ELEM as isize);
        let owner = sloth_rc_retain(t);
        let p = rc_addr(HDR_WORDS * 8, Some(tensor_dtor)) as *mut i64;
        *p = kind | FLAG_VIEW;
        *p.offset(1) = nn as i64;
        *p.offset(2) = ns as i64;
        *p.offset(3) = nst as i64;
        *p.offset(4) = ndata as i64;
        *p.offset(5) = owner;
        *p.offset(6) = 0;
        w_ref(p as usize)
    }
}

/// rank-1 element read (bounds-checked) as a tagged scalar word
#[no_mangle]
pub extern "C" fn sloth_tensor_get1(t: i64, i_w: i64) -> i64 {
    if !w_is_ref(t) {
        return 0;
    }
    unsafe {
        let d = w_unref(t) as *mut i64;
        let sh = shape_of(d);
        let data = data_of(d);
        let i = dec_i(i_w);
        if i < 0 || i >= *sh {
            crate::panics::panic_oob("tensor", i, *sh);
        }
        let raw = *(data.offset((i as usize * ELEM) as isize) as *const i64);
        if kind_of(d) == FLAG_FLOAT {
            enc_f_bits(raw as u64)
        } else {
            enc_i(raw)
        }
    }
}

/// rank-1 element write (bounds-checked); the tagged value word decodes by kind
#[no_mangle]
pub extern "C" fn sloth_tensor_set1(t: i64, i_w: i64, v: i64) -> i64 {
    if !w_is_ref(t) {
        return 0;
    }
    unsafe {
        let d = w_unref(t) as *mut i64;
        let sh = shape_of(d);
        let data = data_of(d);
        let i = dec_i(i_w);
        if i < 0 || i >= *sh {
            crate::panics::panic_oob("tensor", i, *sh);
        }
        let cell = data.offset((i as usize * ELEM) as isize) as *mut i64;
        if kind_of(d) == FLAG_FLOAT {
            *cell = dec_f_bits(v) as i64;
        } else {
            *cell = dec_i(v);
        }
        0
    }
}

/// element-wise copy `dst = src` (same kind and element count); used for
/// view assignment `kc[l][pos] = k`
#[no_mangle]
pub extern "C" fn sloth_tensor_copy_into(dst: i64, src: i64) -> i64 {
    if !w_is_ref(dst) || !w_is_ref(src) {
        crate::panics::panic_msg("tensor copy of a nil operand");
    }
    unsafe {
        let dd = w_unref(dst) as *mut i64;
        let sd = w_unref(src) as *mut i64;
        if kind_of(dd) != kind_of(sd) {
            crate::panics::panic_msg("tensor copy element kind mismatch");
        }
        let dn = total_of(dd);
        let sn = total_of(sd);
        if dn != sn {
            crate::panics::panic_msg("tensor copy shape size mismatch");
        }
        if dn > 0 && data_of(sd) != data_of(dd) {
            std::ptr::copy_nonoverlapping(data_of(sd), data_of(dd), dn as usize * ELEM);
        }
        0
    }
}

/// copy the leading elements of an `Array<T>` into a tensor (construction)
#[no_mangle]
pub extern "C" fn sloth_tensor_copy_from_array(t: i64, arr: i64) -> i64 {
    if !w_is_ref(t) || !w_is_ref(arr) {
        return 0;
    }
    unsafe {
        let d = w_unref(t) as *mut i64;
        let data = data_of(d) as *mut i64;
        let kind = kind_of(d);
        let tn = total_of(d);
        let ap = w_unref(arr) as *mut i64;
        let an = *ap;
        let abuf = *ap.offset(2) as *mut i64;
        let n = an.min(tn);
        let mut i = 0i64;
        while i < n {
            let w = *abuf.offset(i as isize);
            *data.offset(i as isize) = if kind == FLAG_FLOAT {
                dec_f_bits(w) as i64
            } else {
                dec_i(w)
            };
            i += 1;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn sloth_tensor_rank(t: i64) -> i64 {
    if !w_is_ref(t) {
        return 0;
    }
    enc_i(unsafe { ndim_of(w_unref(t) as *mut i64) } as i64)
}

#[no_mangle]
pub extern "C" fn sloth_tensor_dim(t: i64, axis_w: i64) -> i64 {
    if !w_is_ref(t) {
        return 0;
    }
    unsafe {
        let d = w_unref(t) as *mut i64;
        let n = ndim_of(d);
        let a = dec_i(axis_w);
        if a < 0 || a as usize >= n {
            crate::panics::panic_oob("tensor axis", a, n as i64);
        }
        enc_i(*shape_of(d).offset(a as isize))
    }
}

/// zero-fill the tensor's elements in place
#[no_mangle]
pub extern "C" fn sloth_tensor_fill_zero(t: i64) -> i64 {
    if !w_is_ref(t) {
        return 0;
    }
    unsafe {
        let d = w_unref(t) as *mut i64;
        let n = total_of(d);
        if n > 0 {
            std::ptr::write_bytes(data_of(d), 0, n as usize * ELEM);
        }
    }
    0
}
