//! Mutable byte buffers (IO): an opaque, untracked handle holding a growable
//! `[u8]`. Networking recv/send, HTTP framing and the event backends operate
//! on these so hot payloads stay in the runtime instead of crossing the word
//! plane byte-by-byte.
//!
//! Ownership: `Bytes` is an `extern type` from sloth's perspective, so codegen
//! never emits ARC retain/release on it. The handle is a `libc::malloc`'d
//! `BytesObj` whose data pointer is realloc'd; the sloth side must call
//! `__sloth_bytes_free` (a missed free leaks, matching the runtime's
//! "leak rather than dangle" discipline).

#[repr(C)]
pub struct BytesObj {
    len: usize,
    cap: usize,
    data: *mut u8,
}

unsafe fn as_bytes<'a>(h: i64) -> Option<&'a mut BytesObj> {
    if h == 0 {
        None
    } else {
        Some(&mut *(h as *mut BytesObj))
    }
}

#[no_mangle]
pub extern "C" fn __sloth_bytes_new(cap: i64) -> i64 {
    unsafe {
        let c = cap.max(1) as usize;
        let p = libc::calloc(1, std::mem::size_of::<BytesObj>()) as *mut BytesObj;
        if p.is_null() {
            return 0;
        }
        let data = libc::malloc(c) as *mut u8;
        if data.is_null() {
            libc::free(p as *mut libc::c_void);
            return 0;
        }
        (*p).len = 0;
        (*p).cap = c;
        (*p).data = data;
        p as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_bytes_free(h: i64) -> i64 {
    unsafe {
        if let Some(b) = as_bytes(h) {
            if !b.data.is_null() {
                libc::free(b.data as *mut libc::c_void);
            }
            libc::free(b as *mut BytesObj as *mut libc::c_void);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_bytes_len(h: i64) -> i64 {
    unsafe { as_bytes(h).map(|b| b.len as i64).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn __sloth_bytes_cap(h: i64) -> i64 {
    unsafe { as_bytes(h).map(|b| b.cap as i64).unwrap_or(0) }
}

/// grow the backing store to at least `cap` bytes (contents preserved)
#[no_mangle]
pub extern "C" fn __sloth_bytes_ensure(h: i64, cap: i64) -> i64 {
    unsafe {
        let b = match as_bytes(h) {
            Some(b) => b,
            None => return 0,
        };
        if cap <= 0 {
            return 0;
        }
        let want = cap as usize;
        if b.cap >= want {
            return 0;
        }
        let mut nc = b.cap.max(16);
        while nc < want {
            nc *= 2;
        }
        let np = libc::realloc(b.data as *mut libc::c_void, nc) as *mut u8;
        if np.is_null() {
            return -1;
        }
        b.data = np;
        b.cap = nc;
    }
    0
}

/// declare `n` bytes valid (used after recv fills the buffer externally via
/// the internal pointer; also allows parsers to reserve room)
#[no_mangle]
pub extern "C" fn __sloth_bytes_set_len(h: i64, n: i64) -> i64 {
    unsafe {
        if let Some(b) = as_bytes(h) {
            let want = n.max(0) as usize;
            if want > b.cap && __sloth_bytes_ensure(h, n) != 0 {
                return -1;
            }
            b.len = want;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn __sloth_bytes_get(h: i64, i: i64) -> i64 {
    unsafe {
        let b = match as_bytes(h) {
            Some(b) => b,
            None => return -1,
        };
        if i < 0 || i as usize >= b.len {
            return -1;
        }
        *b.data.add(i as usize) as i64
    }
}

#[no_mangle]
pub extern "C" fn __sloth_bytes_set(h: i64, i: i64, v: i64) -> i64 {
    unsafe {
        let b = match as_bytes(h) {
            Some(b) => b,
            None => return -1,
        };
        let idx = i.max(0) as usize;
        if idx >= b.cap && __sloth_bytes_ensure(h, i + 1) != 0 {
            return -1;
        }
        *b.data.add(idx) = v as u8;
        if idx >= b.len {
            b.len = idx + 1;
        }
    }
    0
}

/// write `v` into `n` bytes starting at `off` (extends length as needed)
#[no_mangle]
pub extern "C" fn __sloth_bytes_fill(h: i64, off: i64, v: i64, n: i64) -> i64 {
    unsafe {
        let b = match as_bytes(h) {
            Some(b) => b,
            None => return -1,
        };
        let start = off.max(0) as usize;
        let cnt = n.max(0) as usize;
        let end = start + cnt;
        if end > b.cap && __sloth_bytes_ensure(h, end as i64) != 0 {
            return -1;
        }
        std::ptr::write_bytes(b.data.add(start), v as u8, cnt);
        if end > b.len {
            b.len = end;
        }
    }
    0
}

/// append one byte, growing as needed; returns the new length
#[no_mangle]
pub extern "C" fn __sloth_bytes_append(h: i64, v: i64) -> i64 {
    unsafe {
        let b = match as_bytes(h) {
            Some(b) => b,
            None => return -1,
        };
        let end = b.len + 1;
        if end > b.cap && __sloth_bytes_ensure(h, end as i64) != 0 {
            return -1;
        }
        *b.data.add(b.len) = v as u8;
        b.len = end;
        b.len as i64
    }
}

/// copy a `str`'s bytes into the buffer at `off`, overwriting; returns the
/// number of bytes written (length is not implicitly extended past `off+n`)
#[no_mangle]
pub extern "C" fn __sloth_bytes_copy_from_str(h: i64, off: i64, s_w: i64) -> i64 {
    unsafe {
        let b = match as_bytes(h) {
            Some(b) => b,
            None => return -1,
        };
        let td = match crate::strings::str_of(s_w) {
            Some(t) => t,
            None => return 0,
        };
        let start = off.max(0) as usize;
        let end = start + td.len;
        if end > b.cap && __sloth_bytes_ensure(h, end as i64) != 0 {
            return -1;
        }
        if td.len != 0 {
            libc::memcpy(
                b.data.add(start) as *mut libc::c_void,
                td.data as *const libc::c_void,
                td.len,
            );
        }
        if end > b.len {
            b.len = end;
        }
        td.len as i64
    }
}

/// fresh `str` from `[off, off+len)` (clamped to the valid length)
#[no_mangle]
pub extern "C" fn __sloth_bytes_to_str(h: i64, off: i64, len: i64) -> i64 {
    unsafe {
        let b = match as_bytes(h) {
            Some(b) => b,
            None => return 0,
        };
        let start = off.max(0) as usize;
        if start > b.len {
            return crate::strings::intern_bytes(b.data as usize, 0);
        }
        let mut n = len.max(0) as usize;
        if start + n > b.len {
            n = b.len - start;
        }
        crate::strings::intern_bytes(b.data.add(start) as usize, n as i64)
    }
}

/// fresh `str` holding the whole buffer
#[no_mangle]
pub extern "C" fn __sloth_bytes_as_str(h: i64) -> i64 {
    __sloth_bytes_to_str(h, 0, __sloth_bytes_len(h))
}

/// internal: raw data pointer (0 for nil) — used by net/socket wrappers
pub(crate) unsafe fn data_ptr(b: &BytesObj) -> *mut u8 {
    b.data
}

/// internal: access the object (0 when nil)
pub(crate) unsafe fn obj<'a>(h: i64) -> Option<&'a mut BytesObj> {
    as_bytes(h)
}
