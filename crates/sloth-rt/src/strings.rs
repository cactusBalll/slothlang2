//! Interned strings (str32), string builders (StrB), and str printing.
//! Handles cross the boundary as tagged words; the StrT payload internals
//! (len/data) are raw. String builders stay untracked libc chunks (they
//! exist only inside a single expression and are finalized by finish).

use crate::rc;
use crate::rc::{rc_addr, w_ref, w_unref};

// ---------------- string builders (untracked internal chunks) ----------------

/// 8-byte packed words; total length known at compile time.
#[repr(C)]
pub(crate) struct StrB {
    pub len: libc::size_t,
    pub cap: libc::size_t,
    pub data: *mut libc::c_void,
}

/// builder lifecycle: push chunks in,收回 handle; finalize interns the pool.
#[no_mangle]
pub extern "C" fn sloth_str_push(b_w: i64, w: i64, n_w: i64) -> i64 {
    unsafe {
        let p: *mut StrB = if b_w == 0 {
            libc::calloc(1, std::mem::size_of::<StrB>()) as *mut StrB
        } else {
            w_unref(b_w) as *mut StrB
        };
        let un = (rc::dec_i(n_w) as usize).min(8);
        if (*p).cap < (*p).len + un {
            let nc = ((*p).len + un + 16).next_power_of_two();
            (*p).data = libc::realloc((*p).data, nc);
            (*p).cap = nc;
        }
        let bytes = (w as u64).to_le_bytes();
        let dst = (*p).data as *mut libc::c_void;
        libc::memcpy(
            (dst as *mut libc::c_char).offset((*p).len as isize) as *mut libc::c_void,
            bytes.as_ptr() as *const libc::c_void,
            un,
        );
        (*p).len += un;
        w_ref(p as usize)
    }
}

// ---------------- string pool interning ----------------

#[repr(C)]
pub struct StrT {
    pub len: libc::size_t,
    pub data: *mut libc::c_void,
}

/// interned string handle word from raw bytes; word-plane route (the
/// source pointer arrives tagged)
#[no_mangle]
pub extern "C" fn sloth_str_intern(ptr_w: i64, len_w: i64) -> i64 {
    intern_bytes(w_unref(ptr_w), rc::dec_i(len_w))
}

/// raw internal intern entry (returns the tagged handle word)
pub(crate) fn intern_bytes(ptr: usize, len: i64) -> i64 {
    unsafe {
        let p = rc_addr(std::mem::size_of::<StrT>() + len as usize + 1, None) as *mut libc::c_void;
        let td = p as *mut StrT;
        (*td).len = len as usize;
        let data = (p as *mut u8).offset(std::mem::size_of::<StrT>() as isize);
        if len != 0 {
            libc::memcpy(
                data as *mut libc::c_void,
                ptr as *const libc::c_void,
                len as usize,
            );
        }
        // NUL terminate for easy C display
        *data.offset(len as isize) = 0;
        (*td).data = data as *mut libc::c_void;
        w_ref(p as usize)
    }
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_str(p_w: i64) -> i64 {
    use std::io::Write;
    unsafe {
        let td = w_unref(p_w) as *const StrT;
        let sl = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
        let mut so = std::io::stdout();
        let _ = so.write_all(sl);
        let _ = so.write_all(b"\n");
        let _ = so.flush();
    }
    0
}

/// string length (the word plane carries it tagged)
#[no_mangle]
pub extern "C" fn sloth_str_len(p_w: i64) -> i64 {
    unsafe { rc::enc_i((*(w_unref(p_w) as *const StrT)).len as i64) }
}

/// number of Unicode scalar values in a pooled string (str iteration bound).
/// Falls back to the byte length for non-UTF-8 content.
#[no_mangle]
pub extern "C" fn sloth_str_clen(p_w: i64) -> i64 {
    unsafe {
        let td = w_unref(p_w) as *const StrT;
        let bytes = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
        let n = std::str::from_utf8(bytes)
            .map(|s| s.chars().count())
            .unwrap_or(bytes.len());
        rc::enc_i(n as i64)
    }
}

/// i-th character (Unicode scalar) as a one-char pooled string; the index is
/// a CHAR index, matching `sloth_str_clen` (design §3.5: str iterates by
/// character). Non-UTF-8 content falls back to byte slicing.
#[no_mangle]
pub extern "C" fn sloth_str_char(s_w: i64, i_w: i64) -> i64 {
    unsafe {
        let td = w_unref(s_w) as *const StrT;
        let bytes = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
        let idx = rc::dec_i(i_w) as usize;
        let (start, end) = match std::str::from_utf8(bytes) {
            Ok(s) => match s.char_indices().nth(idx) {
                Some((b, ch)) => (b, b + ch.len_utf8()),
                None => (bytes.len(), bytes.len()),
            },
            Err(_) => {
                if idx < bytes.len() {
                    (idx, idx + 1)
                } else {
                    (bytes.len(), bytes.len())
                }
            }
        };
        let n = end - start;
        let p = rc_addr(std::mem::size_of::<StrT>() + n + 1, None) as *mut libc::c_void;
        let t = p as *mut StrT;
        let dat = (p as *mut libc::c_char).offset(std::mem::size_of::<StrT>() as isize)
            as *mut libc::c_void;
        if n != 0 {
            libc::memcpy(dat, bytes.as_ptr().add(start) as *const libc::c_void, n);
        }
        *(dat as *mut u8).add(n) = 0;
        (*t).len = n;
        (*t).data = dat;
        w_ref(p as usize)
    }
}

pub(crate) unsafe fn strb_append(p: *mut StrB, src: *const libc::c_void, n: usize) {
    if (*p).cap < (*p).len + n {
        let nc = ((*p).len + n + 16).next_power_of_two();
        (*p).data = libc::realloc((*p).data, nc);
        (*p).cap = nc;
    }
    libc::memcpy(
        ((*p).data as *mut libc::c_char).offset((*p).len as isize) as *mut libc::c_void,
        src,
        n,
    );
    (*p).len += n;
}

pub(crate) unsafe fn strb_or_new(b_w: i64) -> *mut StrB {
    if b_w == 0 {
        let raw = libc::calloc(1, std::mem::size_of::<StrB>()) as *mut StrB;
        (*raw).cap = 0;
        raw
    } else {
        w_unref(b_w) as *mut StrB
    }
}

/// push an interned (pooled) string's bytes onto a builder
#[no_mangle]
pub extern "C" fn sloth_str_pushp(b_w: i64, h_w: i64) -> i64 {
    unsafe {
        let p = strb_or_new(b_w);
        let td = w_unref(h_w) as *mut StrT;
        let n = (*td).len;
        strb_append(p, (*td).data, n);
        w_ref(p as usize)
    }
}

/// push an i64 rendered in decimal (value arrives tagged)
#[no_mangle]
pub extern "C" fn sloth_str_push_i(b_w: i64, v_w: i64) -> i64 {
    unsafe {
        let s = format!("{}", rc::dec_i(v_w));
        let p = strb_or_new(b_w);
        strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
        w_ref(p as usize)
    }
}

/// push an f64 rendered with one decimal (raw f64 route: callers decode)
#[no_mangle]
pub extern "C" fn sloth_str_push_f(b_w: i64, v: f64) -> i64 {
    unsafe {
        let s = format!("{}", v);
        let p = strb_or_new(b_w);
        strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
        w_ref(p as usize)
    }
}

/// push a bool rendered as true/false (value arrives tagged)
#[no_mangle]
pub extern "C" fn sloth_str_push_b(b_w: i64, v_w: i64) -> i64 {
    unsafe {
        let s = if rc::dec_i(v_w) != 0 { "true" } else { "false" };
        let p = strb_or_new(b_w);
        strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
        w_ref(p as usize)
    }
}

/// value-optional box interpolation (kind: 0 = int, 1 = float, 2 = bool):
/// nil renders as "nil" — 0/0.0/false inside a box never collide
/// (the box handle arrives tagged; payload words are decoded)
#[no_mangle]
pub extern "C" fn sloth_str_push_opt(b_w: i64, h_w: i64, kind_w: i64) -> i64 {
    unsafe {
        if h_w == 0 {
            let p = strb_or_new(b_w);
            strb_append(p, b"nil\0".as_ptr() as *const libc::c_void, 3);
            return w_ref(p as usize);
        }
        let kind = rc::dec_i(kind_w);
        let p = strb_or_new(b_w);
        match kind {
            1 => {
                let bits = *(w_unref(h_w) as *const i64);
                let s = format!("{}", f64::from_bits((bits as u64) << 1));
                strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
            }
            2 => {
                let b = rc::dec_i(*(w_unref(h_w) as *const i64));
                let s = if b != 0 { "true" } else { "false" };
                strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
            }
            _ => {
                let i = rc::dec_i(*(w_unref(h_w) as *const i64));
                let s = format!("{}", i);
                strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
            }
        }
        w_ref(p as usize)
    }
}

/// finalize: return the pooled interned string for a built byte buffer
#[no_mangle]
pub extern "C" fn sloth_str_finish(b_w: i64) -> i64 {
    unsafe {
        // b == 0 (no chunks pushed at all, e.g. the `""` literal) makes a
        // fresh empty builder instead of dereferencing a NULL handle
        let p: *mut StrB = strb_or_new(b_w);
        let h = intern_bytes((*p).data as usize, (*p).len as i64);
        libc::free((*p).data);
        libc::free(p as *mut libc::c_void);
        h
    }
}

/// concatenate two pooled strings; the result is interned by content so
/// equal-content handles are identical (patch #36 intern-unification)
#[no_mangle]
pub extern "C" fn sloth_str_concat(a_w: i64, b_w: i64) -> i64 {
    unsafe {
        let ta = w_unref(a_w) as *const StrT;
        let tb = w_unref(b_w) as *const StrT;
        let la = (*ta).len;
        let lb = (*tb).len;
        let n = la + lb;
        let buf = libc::malloc((n + 1) as libc::size_t) as *mut libc::c_char;
        libc::memcpy(buf as *mut libc::c_void, (*ta).data, la);
        libc::memcpy(
            (buf as *mut libc::c_void).offset(la as isize),
            (*tb).data,
            lb,
        );
        *(buf.offset(n as isize)) = 0;
        let h = intern_bytes(buf as usize, n as i64);
        libc::free(buf as *mut libc::c_void);
        h
    }
}

/// content equality of two pooled str words (patch #36): len + memcmp
#[no_mangle]
pub extern "C" fn sloth_str_eq(a_w: i64, b_w: i64) -> i64 {
    unsafe {
        let ta = w_unref(a_w) as *const StrT;
        let tb = w_unref(b_w) as *const StrT;
        let eq = (*ta).len == (*tb).len
            && libc::memcmp((*ta).data, (*tb).data, (*ta).len as libc::size_t) == 0;
        rc::enc_i(eq as i64)
    }
}

/// lexicographic byte comparison (TE-P4 tokenizer): raw C-ABI args, returns a
/// raw negative/zero/positive i64 (the codegen re-encodes extern int returns)
#[no_mangle]
pub extern "C" fn sloth_str_cmp(a_w: i64, b_w: i64) -> i64 {
    unsafe {
        let ta = w_unref(a_w) as *const StrT;
        let tb = w_unref(b_w) as *const StrT;
        let n = (*ta).len.min((*tb).len);
        let c = libc::memcmp((*ta).data, (*tb).data, n);
        if c != 0 {
            return c as i64;
        }
        if (*ta).len < (*tb).len {
            -1
        } else if (*ta).len > (*tb).len {
            1
        } else {
            0
        }
    }
}

/// i-th raw byte of a pooled string (raw C-ABI index in, raw byte out)
#[no_mangle]
pub extern "C" fn sloth_str_byte(s_w: i64, i: i64) -> i64 {
    unsafe {
        let td = w_unref(s_w) as *const StrT;
        if i < 0 || i as usize >= (*td).len {
            crate::panics::panic_oob("str byte", i, (*td).len as i64);
        }
        *((*td).data as *const u8).offset(i as isize) as i64
    }
}

/// byte slice `[start, start+len)` as a fresh pooled string (raw C-ABI ints)
#[no_mangle]
pub extern "C" fn sloth_str_slice(s_w: i64, start: i64, len: i64) -> i64 {
    unsafe {
        let td = w_unref(s_w) as *const StrT;
        if start < 0 || len < 0 || start + len > (*td).len as i64 {
            crate::panics::panic_oob("str slice", start + len, (*td).len as i64);
        }
        let data = ((*td).data as *const u8).offset(start as isize);
        intern_bytes(data as usize, len)
    }
}

/// one raw byte as a pooled string (raw C-ABI byte in); used by the tokenizer
/// for `<0xNN>` raw-byte vocabulary entries
#[no_mangle]
pub extern "C" fn sloth_str_of_byte(v: i64) -> i64 {
    let b = (v & 0xff) as u8;
    intern_bytes((&b as *const u8) as usize, 1)
}

/// write a pooled string's bytes to stdout with no trailing newline (generation
/// output must be printable piece-by-piece)
#[no_mangle]
pub extern "C" fn sloth_rt_write_str(p_w: i64) -> i64 {
    use std::io::Write;
    unsafe {
        let td = w_unref(p_w) as *const StrT;
        let sl = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
        let mut so = std::io::stdout();
        let _ = so.write_all(sl);
        let _ = so.flush();
    }
    0
}
