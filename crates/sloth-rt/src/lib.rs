//! sloth-rt: the sloth2 runtime library (libsloth_rt).
//! Console, GC, string pool, containers, panic — filled in incrementally.

#[no_mangle]
pub extern "C" fn sloth_rt_hello() {
    println!("libsloth_rt linked");
}

#[no_mangle]
pub extern "C" fn sloth_panic(msg: *const libc::c_char) -> ! {
    let s = unsafe {
        if msg.is_null() {
            "panic".to_string()
        } else {
            std::ffi::CStr::from_ptr(msg).to_string_lossy().to_string()
        }
    };
    eprintln!("sloth panic: {}", s);
    std::process::exit(1);
}

/// MVP allocator stand-in (replaced by Boehm GC in sloth-rt step 2).
#[no_mangle]
pub extern "C" fn sloth_gc_alloc(n: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::calloc(1, n) }
}

/// print an i64 value (display form); never returns useful value
#[no_mangle]
pub extern "C" fn sloth_rt_print_i64(v: i64) -> i64 {
    println!("{}", v);
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_f64(v: f64) -> i64 {
    println!("{}", v);
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_bool(v: i64) -> i64 {
    println!("{}", v != 0);
    0
}

// ---------------- string builders ----------------

// str32: 8-byte packed words; total length known at compile time.
#[repr(C)]
struct StrB {
    len: libc::size_t,
    cap: libc::size_t,
    data: *mut libc::c_void,
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
struct StrT {
    len: libc::size_t,
    data: *mut libc::c_void,
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
        libc::memcpy(data, ptr as *const libc::c_void, len as usize);
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


unsafe fn strb_append(p: *mut StrB, src: *const libc::c_void, n: usize) {
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

unsafe fn strb_or_new(b: i64) -> *mut StrB {
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
        let p: *mut StrB = b as *mut StrB;
        let h = sloth_str_intern((*p).data as i64, (*p).len as i64);
        libc::free((*p).data);
        libc::free(p as *mut libc::c_void);
        h
    }
}

/// concatenate two pooled strings
#[no_mangle]
pub extern "C" fn sloth_str_concat(a: i64, b: i64) -> i64 {
    unsafe {
        let ta = a as *mut StrT;
        let tb = b as *mut StrT;
        let la = (*ta).len;
        let lb = (*tb).len;
        let h = sloth_gc_alloc(la + lb + 1 + std::mem::size_of::<StrT>()) as *mut libc::c_void;
        let td = h as *mut StrT;
        let dat = (h as *mut libc::c_void).offset(std::mem::size_of::<StrT>() as isize);
        libc::memcpy(dat, (*ta).data, la);
        libc::memcpy((dat as *mut libc::c_char).offset(la as isize) as *mut libc::c_void,
                     (*tb).data, lb);
        libc::memset((dat as *mut libc::c_char).offset((la + lb) as isize) as *mut libc::c_void, 0, 1);
        (*td).len = la + lb;
        (*td).data = dat;
        h as i64
    }
}

// ---------------- maps ----------------
// open-addressing hash map (linear probing).
// header layout: [cap, used, kkind, buckets_ptr] — the header stays put across
// growth, only the bucket array is reallocated, so stored map handles remain
// valid. Each bucket slot = 3 words [used, key, value].
// kkind: 0 = i64 keys, 1 = str keys (interned str handles compared by content),
// 2 = object keys (pointer identity + hashed pointer; MVP wp for Hashable)

const MAP_HDR_W: i64 = 4;
const MAP_SLOT_W: i64 = 3;

fn map_slot(m: i64, i: i64) -> *mut i64 {
    unsafe {
        let bp = *(m as *mut i64).offset(3) as *mut i64;
        bp.offset((i * MAP_SLOT_W) as isize)
    }
}

fn mix64(mut w: u64) -> u64 {
    w ^= w >> 33;
    w = w.wrapping_mul(0xff51afd7ed558ccd);
    w ^= w >> 33;
    w = w.wrapping_mul(0xc4ceb9fe1a85ec53);
    w ^= w >> 33;
    w
}

fn map_hash_i(k: i64) -> u64 {
    mix64(k as u64)
}

/// FNV-1a over an interned string's bytes
fn map_hash_s(h: i64) -> u64 {
    unsafe {
        let td = h as *const StrT;
        let len = (*td).len;
        let mut p = (*td).data as *const u8;
        let mut hash: u64 = 0xcbf29ce484222325;
        for _ in 0..len {
            hash ^= *p as u64;
            hash = hash.wrapping_mul(0x100000001b3);
            p = p.offset(1);
        }
        hash
    }
}

fn map_key_hash(kkind: i64, key: i64) -> u64 {
    if kkind == 1 {
        map_hash_s(key)
    } else if kkind == 2 {
        mix64(key as u64)
    } else {
        map_hash_i(key)
    }
}

fn map_streq(a: i64, b: i64) -> bool {
    unsafe {
        let ta = a as *const StrT;
        let tb = b as *const StrT;
        (*ta).len == (*tb).len
            && libc::memcmp((*ta).data, (*tb).data, (*ta).len as libc::size_t) == 0
    }
}

fn map_key_eq(kkind: i64, ka: i64, kb: i64) -> bool {
    if kkind == 1 {
        map_streq(ka, kb)
    } else if kkind == 2 {
        ka == kb
    } else {
        ka == kb
    }
}

fn map_alloc_buckets(cap: i64) -> *mut i64 {
    unsafe { sloth_gc_alloc(((cap * MAP_SLOT_W) * 8) as libc::size_t) as *mut i64 }
}

#[no_mangle]
pub extern "C" fn sloth_map_new(kkind: i64) -> i64 {
    unsafe {
        let cap = 8i64;
        let o = sloth_gc_alloc((MAP_HDR_W * 8) as libc::size_t) as *mut i64;
        *o = cap;
        *o.offset(1) = 0;
        *o.offset(2) = kkind;
        *o.offset(3) = map_alloc_buckets(cap) as i64;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_map_len(m: i64) -> i64 {
    unsafe { *(m as *mut i64).offset(1) }
}

/// place a used pair into the table (always the current m; no growth here)
fn map_insert_raw(m: i64, key: i64, val: i64) {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let kkind = *p.offset(2);
        let mut i = map_key_hash(kkind, key) % (cap as u64);
        loop {
            let s = map_slot(m, i as i64);
            if *s == 0 {
                *s = 1;
                *s.offset(1) = key;
                *s.offset(2) = val;
                *p.offset(1) += 1;
                return;
            }
            i = (i + 1) % (cap as u64);
        }
    }
}

/// grow: rehash into a fresh bucket array; the map handle stays valid
unsafe fn map_grow(m: i64) {
    let p = m as *mut i64;
    let cap = *p;
    let kkind = *p.offset(2);
    let used = *p.offset(1);
    let ncap = cap * 2;
    let old_bp = *p.offset(3) as *mut i64;
    let old_words = cap * MAP_SLOT_W;
    // swap out the bucket array first so reinserts land in the fresh table
    *p.offset(3) = map_alloc_buckets(ncap) as i64;
    *p = ncap;
    *p.offset(1) = 0; // reinserts re-count
    let mut k = 0i64;
    while k < old_words {
        if *old_bp.offset(k as isize) == 1 {
            map_insert_raw(m, *old_bp.offset((k + 1) as isize), *old_bp.offset((k + 2) as isize));
        }
        k += MAP_SLOT_W;
    }
}

/// find or create the slot for `key`; returns the slot pointer
fn map_upsert(m: i64, key: i64) -> *mut i64 {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let kkind = *p.offset(2);
        let used = *p.offset(1);
        let mut i = map_key_hash(kkind, key) % (cap as u64);
        let mut rounds = 0u64;
        loop {
            let s = map_slot(m, i as i64);
            if *s == 0 {
                // free slot: new pair (grow first if load too high)
                if used * 4 >= cap * 3 {
                    map_grow(m);
                    return map_upsert(m, key);
                }
                *s = 1;
                *s.offset(1) = key;
                *p.offset(1) = used + 1;
                return s;
            }
            if map_key_eq(kkind, *s.offset(1), key) {
                return s;
            }
            i = (i + 1) % (cap as u64);
            rounds += 1;
            if rounds > cap as u64 {
                // table exhausted: grow and retry
                map_grow(m);
                return map_upsert(m, key);
            }
        }
    }
}

/// missing map key (never returns normally)
#[no_mangle]
pub extern "C" fn sloth_panic_nokey(key: i64) -> i64 {
    eprintln!("sloth panic: map key not found ({})", key);
    std::process::exit(1);
}

macro_rules! map_get_impl {
    ($name:ident, $valty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(m: i64, key: i64) -> $valty {
            unsafe {
                let p = m as *mut i64;
                let cap = *p;
                let kkind = *p.offset(2);
                let mut i = map_key_hash(kkind, key) % (cap as u64);
                let mut rounds = 0u64;
                loop {
                    let s = map_slot(m, i as i64);
                    if *s == 0 || rounds > cap as u64 {
                        let _ = sloth_panic_nokey(key);
                        std::process::exit(1);
                    }
                    if map_key_eq(kkind, *s.offset(1), key) {
                        return *(s.offset(2) as *const $valty);
                    }
                    i = (i + 1) % (cap as u64);
                    rounds += 1;
                }
            }
        }
    };
}

macro_rules! map_set_impl {
    ($name:ident, $valty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(m: i64, key: i64, v: $valty) -> i64 {
            unsafe {
                let s = map_upsert(m, key);
                *(s.offset(2) as *mut $valty) = v;
                0
            }
        }
    };
}

map_get_impl!(sloth_map_get, i64);
map_get_impl!(sloth_map_get_f64, f64);
map_get_impl!(sloth_map_str_get, i64);
map_get_impl!(sloth_map_str_get_f64, f64);
map_set_impl!(sloth_map_set, i64);
map_set_impl!(sloth_map_set_f64, f64);
map_set_impl!(sloth_map_str_set, i64);
map_set_impl!(sloth_map_str_set_f64, f64);

/// array of key words
#[no_mangle]
pub extern "C" fn sloth_map_keys(m: i64) -> i64 {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let arr = sloth_arr_new(0);
        let mut i = 0i64;
        while i < cap {
            let s = map_slot(m, i);
            if *s == 1 {
                sloth_arr_push(arr, *s.offset(1));
            }
            i += 1;
        }
        arr
    }
}

#[no_mangle]
pub extern "C" fn sloth_map_values(m: i64) -> i64 {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let arr = sloth_arr_new(0);
        let mut i = 0i64;
        while i < cap {
            let s = map_slot(m, i);
            if *s == 1 {
                sloth_arr_push(arr, *s.offset(2));
            }
            i += 1;
        }
        arr
    }
}

// ---------------- arrays ----------------
// layout: [len, e0, e1, ...] (i64 words; f64 words routed via _f64 ops)

fn arr_index(a: i64, i: i64) -> *mut i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if i < 0 || i >= len {
            eprintln!("sloth panic: array index {} out of bounds (len {})", i, len);
            std::process::exit(1);
        }
        p.offset(i as isize + 2)
    }
}

/// array layout: [len, cap, e0..] (cap >= len; push grows past cap by realloc)
#[no_mangle]
pub extern "C" fn sloth_arr_new(len: i64) -> i64 {
    unsafe {
        let n = len.max(0) as libc::size_t;
        let cap = (n * 2).next_power_of_two().max(8) as libc::size_t;
        let o = sloth_gc_alloc((cap + 2) * 8) as *mut i64;
        *o = n as i64;
        *o.offset(1) = cap as i64;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_len(a: i64) -> i64 {
    unsafe { *(a as *mut i64) }
}

/// append one i64 word; returns the new length (call site usually ignores it)
#[no_mangle]
pub extern "C" fn sloth_arr_push(a: i64, w: i64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        if len >= cap {
            let nc = (cap * 2).max(8);
            let raw = libc::realloc(a as *mut libc::c_void, (nc + 2) as libc::size_t * 8);
            let p2 = raw as *mut i64;
            *p2.offset(1) = nc;
        }
        *p.offset((len + 2) as isize) = w;
        *p = len + 1;
        len + 1
    }
}

/// remove and return the last i64 word
#[no_mangle]
pub extern "C" fn sloth_arr_pop(a: i64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if len <= 0 {
            eprintln!("sloth panic: pop from empty array");
            std::process::exit(1);
        }
        *p = len - 1;
        *p.offset((len - 1) as isize + 2)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_push_f64(a: i64, w: f64) -> i64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        let cap = *p.offset(1);
        if len >= cap {
            let nc = (cap * 2).max(8);
            let raw = libc::realloc(a as *mut libc::c_void, (nc + 2) as libc::size_t * 8);
            let p2 = raw as *mut i64;
            *p2.offset(1) = nc;
        }
        *(p.offset((len + 2) as isize) as *mut f64) = w;
        *p = len + 1;
        len + 1
    }
}

/// remove and return the last f64 word
#[no_mangle]
pub extern "C" fn sloth_arr_pop_f64(a: i64) -> f64 {
    unsafe {
        let p = a as *mut i64;
        let len = *p;
        if len <= 0 {
            eprintln!("sloth panic: pop from empty array");
            std::process::exit(1);
        }
        *p = len - 1;
        *((p.offset((len - 1) as isize + 2)) as *mut f64)
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_get(a: i64, i: i64) -> i64 {
    unsafe { *arr_index(a, i) }
}

#[no_mangle]
pub extern "C" fn sloth_arr_get_f64(a: i64, i: i64) -> f64 {
    unsafe { *(arr_index(a, i) as *mut f64) }
}

#[no_mangle]
pub extern "C" fn sloth_arr_set(a: i64, i: i64, v: i64) -> i64 {
    unsafe {
        *arr_index(a, i) = v;
        0
    }
}

#[no_mangle]
pub extern "C" fn sloth_arr_set_f64(a: i64, i: i64, v: f64) -> i64 {
    unsafe {
        *(arr_index(a, i) as *mut f64) = v;
        0
    }
}

/// dynamic trait dispatch: vtable primitives.

#[repr(C)]
struct ObjInfo {
    super_info: *mut libc::c_void,
    cls_id: i64,
}

#[no_mangle]
pub extern "C" fn sloth_cls_info(super_ptr: i64, cls_id: i64) -> i64 {
    unsafe {
        let o = sloth_gc_alloc(std::mem::size_of::<ObjInfo>()) as *mut ObjInfo;
        (*o).super_info = super_ptr as *mut libc::c_void;
        (*o).cls_id = cls_id;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_new(info_ptr: i64, n_words: i64) -> i64 {
    unsafe {
        let n = n_words.max(2) as libc::size_t;
        let o = sloth_gc_alloc(n * 8) as *mut i64;
        *o = info_ptr;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_obj_field(obj: i64, idx: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(idx as isize + 2) }
}

#[no_mangle]
pub extern "C" fn sloth_obj_field_f64(obj: i64, idx: i64) -> f64 {
    unsafe { *(obj as *mut f64).offset(idx as isize + 2) }
}

#[no_mangle]
pub extern "C" fn sloth_obj_set_field(obj: i64, idx: i64, val: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(idx as isize + 2) = val }
    0
}

#[no_mangle]
pub extern "C" fn sloth_obj_set_field_f64(obj: i64, idx: i64, val: f64) -> i64 {
    unsafe { *(obj as *mut f64).offset(idx as isize + 2) = val }
    0
}

/// runtime class id of an object (from its type header), used by dyn dispatch
#[no_mangle]
pub extern "C" fn sloth_obj_cls_id(obj: i64) -> i64 {
    unsafe {
        let info = *(obj as *mut *mut libc::c_void);
        (*(info as *mut ObjInfo)).cls_id
    }
}

/// dynamic trait dispatch: vtable primitives.
/// vt: [capacity, slot0..] array of i64 raw function pointers
#[no_mangle]
pub extern "C" fn sloth_vt_new(cap: i64) -> i64 {
    unsafe {
        let n = cap.max(1) as libc::size_t;
        let o = sloth_gc_alloc((n + 1) * 8) as *mut i64;
        *o = n as i64;
        o as i64
    }
}

#[no_mangle]
pub extern "C" fn sloth_vt_set(vt: i64, slot: i64, fp: i64) -> i64 {
    unsafe {
        let p = vt as *mut i64;
        let cap = *p;
        if slot >= 0 && slot < cap {
            *p.offset(slot as isize + 1) = fp;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn sloth_vt_get(vt: i64, slot: i64) -> i64 {
    unsafe {
        if vt == 0 {
            return 0;
        }
        let p = vt as *mut i64;
        let cap = *p;
        if slot >= 0 && slot < cap {
            *(p.offset(slot as isize + 1))
        } else {
            0
        }
    }
}

/// object header word 1: pointer to this class's vtable
#[no_mangle]
pub extern "C" fn sloth_obj_set_vtable(obj: i64, vt: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(1) = vt }
    0
}

#[no_mangle]
pub extern "C" fn sloth_obj_vtable(obj: i64) -> i64 {
    unsafe { *(obj as *mut i64).offset(1) }
}

/// dyn receiver is not an implementing class (never returns)
#[no_mangle]
pub extern "C" fn sloth_panic_noimpl(cls_id: i64) -> i64 {
    eprintln!(
        "sloth panic: no impl for trait method on receiver (cls {})",
        cls_id
    );
    std::process::exit(1);
}

#[cfg(test)]
mod map_tests {
    #[test]
    fn map_roundtrip_20_keys() {
        unsafe {
            let m = crate::sloth_map_new(0);
            for i in 0..20 {
                crate::sloth_map_set(m, i, i * 2);
            }
            assert_eq!(crate::sloth_map_len(m), 20, "len after 20 sets");
            for i in 0..20 {
                assert_eq!(crate::sloth_map_get(m, i), i * 2, "key {}", i);
            }
            let ks = crate::sloth_map_keys(m);
            assert_eq!(crate::sloth_arr_len(ks), 20, "keys array len");
            let vs = crate::sloth_map_values(m);
            assert_eq!(crate::sloth_arr_len(vs), 20, "values array len");
        }
    }

    /// two-key map: no phantom used slots, uniform probe
    #[test]
    fn map_small_two_keys() {        unsafe {
            let m = crate::sloth_map_new(0);
            crate::sloth_map_set(m, 5, 50);
            crate::sloth_map_set(m, 6, 60);
            assert_eq!(crate::sloth_map_len(m), 2);
            assert_eq!(crate::sloth_map_get(m, 5), 50);
            assert_eq!(crate::sloth_map_get(m, 6), 60);
            let ks = crate::sloth_map_keys(m);
            assert_eq!(crate::sloth_arr_len(ks), 2, "keys array");
            let mut s50 = 0i64;
            let mut s60 = 0i64;
            for i in 0..2 {
                let k = crate::sloth_arr_get(ks, i);
                if k == 5 {
                    s50 = 1;
                }
                if k == 6 {
                    s60 = 1;
                }
                assert_eq!(crate::sloth_map_get(m, k), k * 10);
            }
            assert_eq!(s50, 1, "key 5 present");
            assert_eq!(s60, 1, "key 6 present");
        }
    }
}
