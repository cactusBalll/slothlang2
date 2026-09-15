//! Interned strings (str32), string builders (StrB), and str printing.

use crate::gc::sloth_gc_alloc;

// ---------------- string builders ----------------

/// 8-byte packed words; total length known at compile time.
#[repr(C)]
pub(crate) struct StrB {
    pub len: libc::size_t,
    pub cap: libc::size_t,
    pub data: *mut libc::c_void,
}

/// builder lifecycle: push chunks in,收回 handle; finalize interns the pool.
#[no_mangle]
pub extern "C" fn sloth_str_push(b: i64, w: i64, n: i64) -> i64 {
    unsafe {
        let p: *mut StrB = if b == 0 {
            let raw = libc::calloc(1, std::mem::size_of::<StrB>()) as *mut StrB;
            (*raw).cap = 0;
            raw as *mut StrB
        } else {
            b as *mut StrB
        };
        let un = (n as usize).min(8);
        if (*p).cap < (*p).len + un {
            let nc = ((*p).len + un + 16).next_power_of_two();
            (*p).data = libc::realloc((*p).data, nc);
            (*p).cap = nc;
        }
        let bytes = (w as u64).to_le_bytes();
        let dst = (*p).data as *mut libc::c_void;
        libc::memcpy((dst as *mut libc::c_char).offset((*p).len as isize) as *mut libc::c_void,
                     bytes.as_ptr() as *const libc::c_void, un);
        (*p).len += un;
        p as i64
    }
}

// ---------------- string pool interning ----------------

#[repr(C)]
pub(crate) struct StrT {
    pub len: libc::size_t,
    pub data: *mut libc::c_void,
}

/// interned string handle from raw bytes; used by string literals
#[no_mangle]
pub extern "C" fn sloth_str_intern(ptr: i64, len: i64) -> i64 {
    unsafe {
        let t = sloth_gc_alloc(std::mem::size_of::<StrT>() + len as libc::size_t + 1)
            as *mut libc::c_void;
        let td = t as *mut StrT;
        (*td).len = len as usize;
        let data = (t as *mut libc::c_void).offset(std::mem::size_of::<StrT>() as isize);
        if len != 0 {
            libc::memcpy(data, ptr as *const libc::c_void, len as usize);
        }
        // NUL terminate for easy C display
        libc::memset((data as *mut libc::c_char).offset(len as isize) as *mut libc::c_void, 0, 1);
        (*td).data = data;
        t as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_str(p: i64) -> i64 {
    use std::io::Write;
    unsafe {
        let td = p as *mut StrT;
        let sl = std::slice::from_raw_parts((*td).data as *const u8, (*td).len);
        let mut so = std::io::stdout();
        let _ = so.write_all(sl);
        let _ = so.write_all(b"\n");
        let _ = so.flush();
    }
    0
}

#[no_mangle]
pub extern "C" fn sloth_str_len(p: i64) -> i64 {
    unsafe { (*(p as *mut StrT)).len as i64 }
}

/// single-character string for iteration: `for (var c: "str")`
#[no_mangle]
pub extern "C" fn sloth_str_char(s: i64, i: i64) -> i64 {
    unsafe {
        let td = s as *mut StrT;
        let b = *(((*td).data as *const u8).offset(i as isize)) as u8;
        let t = sloth_gc_alloc(std::mem::size_of::<StrT>() + 2) as *mut StrT;
        let dat = (t as *mut libc::c_void).offset(std::mem::size_of::<StrT>() as isize);
        *(dat as *mut u8) = b;
        libc::memset((dat as *mut libc::c_char).offset(1) as *mut libc::c_void, 0, 1);
        (*t).len = 1;
        (*t).data = dat;
        t as i64
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

pub(crate) unsafe fn strb_or_new(b: i64) -> *mut StrB {
    if b == 0 {
        let raw = libc::calloc(1, std::mem::size_of::<StrB>()) as *mut StrB;
        (*raw).cap = 0;
        raw
    } else {
        b as *mut StrB
    }
}

/// push an interned (pooled) string's bytes onto a builder
#[no_mangle]
pub extern "C" fn sloth_str_pushp(b: i64, h: i64) -> i64 {
    unsafe {
        let p = strb_or_new(b);
        let td = h as *mut StrT;
        let n = (*td).len;
        strb_append(p, (*td).data, n);
        p as i64
    }
}

/// push an i64 rendered in decimal
#[no_mangle]
pub extern "C" fn sloth_str_push_i(b: i64, v: i64) -> i64 {
    unsafe {
        let s = format!("{}", v);
        let p = strb_or_new(b);
        strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
        p as i64
    }
}

/// push an f64 rendered with one decimal
#[no_mangle]
pub extern "C" fn sloth_str_push_f(b: i64, v: f64) -> i64 {
    unsafe {
        let s = format!("{}", v);
        let p = strb_or_new(b);
        strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
        p as i64
    }
}

/// push a bool rendered as true/false
#[no_mangle]
pub extern "C" fn sloth_str_push_b(b: i64, v: i64) -> i64 {
    unsafe {
        let s = if v != 0 { "true" } else { "false" };
        let p = strb_or_new(b);
        strb_append(p, s.as_ptr() as *const libc::c_void, s.len());
        p as i64
    }
}

/// finalize: return the pooled interned string for a built byte buffer
#[no_mangle]
pub extern "C" fn sloth_str_finish(b: i64) -> i64 {
    unsafe {
        // b == 0 (no chunks pushed at all, e.g. the `""` literal) makes a
        // fresh empty builder instead of dereferencing a NULL handle
        let p: *mut StrB = strb_or_new(b);
        let h = sloth_str_intern((*p).data as i64, (*p).len as i64);
        libc::free((*p).data);
        libc::free(p as *mut libc::c_void);
        h
    }
}

/// concatenate two pooled strings; the result is interned by content so
/// equal-content handles are identical (patch #36 intern-unification)
#[no_mangle]
pub extern "C" fn sloth_str_concat(a: i64, b: i64) -> i64 {
    unsafe {
        let ta = a as *const StrT;
        let tb = b as *const StrT;
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
        let h = sloth_str_intern(buf as i64, n as i64);
        libc::free(buf as *mut libc::c_void);
        h
    }
}

/// content equality of two pooled str words (patch #36): len + memcmp
#[no_mangle]
pub extern "C" fn sloth_str_eq(a: i64, b: i64) -> i64 {
    unsafe {
        let ta = a as *const StrT;
        let tb = b as *const StrT;
        ((*ta).len == (*tb).len
            && libc::memcmp((*ta).data, (*tb).data, (*ta).len as libc::size_t) == 0)
            as i64
    }
}

#[cfg(test)]
mod tests {
    /// str iteration returns single-byte strings
    #[test]
    fn str_char_roundtrip() {
        unsafe {
            let s = crate::strings::sloth_str_intern("ab\0".as_ptr() as i64, 2);
            assert_eq!(crate::strings::sloth_str_len(s), 2, "interned len");
            for (i, want) in (0..2).zip([b'a', b'b']) {
                let c = crate::strings::sloth_str_char(s, i);
                assert_eq!(crate::strings::sloth_str_len(c), 1, "char len @{}", i);
                let td = c as *mut crate::strings::StrT;
                let b = *(((*td).data as *const u8));
                assert_eq!(b, want, "char @{}", i);
            }
        }
    }

    #[test]
    fn print_str_smoke() {
        let s = crate::strings::sloth_str_intern("hi\0".as_ptr() as i64, 2);
        assert_eq!(crate::strings::sloth_rt_print_str(s), 0);
        let _ = crate::console::sloth_rt_print_i64(1);
    }
}
