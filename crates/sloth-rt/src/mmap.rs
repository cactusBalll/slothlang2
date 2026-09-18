//! Read-only file mapping (design D7): the llama2.c checkpoint path.
//!
//! `sloth_mmap(str) -> ByteBuffer` maps the whole file; the opaque handle is a
//! `BufHdr*` (length + base). `sloth_mmap_i32` reads little-endian config
//! fields, and `sloth_tensor_from_f32_ptr` (in `tensors`) widens the f32 weight
//! region into an f64 tensor (the documented non-zero-copy correction).

use crate::panics;
use crate::rc::w_unref;
use crate::strings::StrT;

#[repr(C)]
pub struct BufHdr {
    pub len: i64,
    pub data: *mut u8,
}

/// map `path` read-only; panics on any failure
#[no_mangle]
pub extern "C" fn sloth_mmap(path_w: i64) -> i64 {
    unsafe {
        let td = w_unref(path_w) as *const StrT;
        let n = (*td).len;
        let mut cpath: Vec<u8> = Vec::with_capacity(n + 1);
        if n > 0 {
            cpath.extend_from_slice(std::slice::from_raw_parts((*td).data as *const u8, n));
        }
        cpath.push(0);
        let fd = libc::open(cpath.as_ptr() as *const libc::c_char, libc::O_RDONLY);
        if fd < 0 {
            panics::panic_msg("mmap: cannot open file");
        }
        let mut st: libc::stat = std::mem::zeroed();
        if libc::fstat(fd, &mut st) != 0 {
            libc::close(fd);
            panics::panic_msg("mmap: fstat failed");
        }
        let size = st.st_size as usize;
        let p = if size == 0 {
            std::ptr::null_mut()
        } else {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ,
                libc::MAP_PRIVATE,
                fd,
                0,
            )
        };
        libc::close(fd);
        if p == libc::MAP_FAILED {
            panics::panic_msg("mmap: mmap failed");
        }
        let h = libc::malloc(std::mem::size_of::<BufHdr>()) as *mut BufHdr;
        if h.is_null() {
            panics::panic_msg("mmap: out of memory");
        }
        (*h).len = size as i64;
        (*h).data = p as *mut u8;
        h as i64
    }
}

/// byte length of the mapping (raw i64; sloth side declares `int`)
#[no_mangle]
pub extern "C" fn sloth_mmap_len(h: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    unsafe { (*(h as *const BufHdr)).len }
}

/// little-endian i32 at byte `off` (raw C-ABI i64 arg; bounds-checked)
#[no_mangle]
pub extern "C" fn sloth_mmap_i32(h: i64, off: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    unsafe {
        let hd = &*(h as *const BufHdr);
        if off < 0 || off + 4 > hd.len {
            panics::panic_oob("mmap i32", off, hd.len);
        }
        let p = hd.data.add(off as usize) as *const i32;
        *p as i64
    }
}
