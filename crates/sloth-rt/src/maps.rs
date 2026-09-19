//! Open-addressing hash map (linear probing) over raw words.
//! Header layout: `[cap, used, kflag, buckets_ptr]` — the header stays put
//! across growth, only the bucket array is reallocated, so stored map
//! handles remain valid. Each bucket slot = 4 words
//! `[used, key, value, hash]`: `hash` caches the caller-provided content
//! hash for object keys (kkind: 2), so growth rehashes without re-calling
//! the monomorphized hash(). kflag low bits = key kind (0 int / 1 str /
//! 2 object); bit 2 = `vref` (value words are references, so the death
//! cascade releases them). Keys are references iff kkind != 0; value
//! refness comes from VPREF. Bucket buffers are untracked internal chunks
//! freed by the map itself (grow frees the stale buffer directly, dying
//! with the header otherwise).

use crate::alloc::sloth_rt_alloc;
use crate::arrays::{sloth_arr_new_k, sloth_arr_push};
use crate::panics;
use crate::rc::{rc_addr, w_ref, w_unref};
use crate::strings::StrT;

const MAP_HDR_W: i64 = 4;
const MAP_SLOT_W: i64 = 4;

/// value-ref flag of kflag (bit 2): set when V is a reference type
const MAP_VREF: i64 = 4;

fn map_kkind_raw(kflag: i64) -> i64 {
    kflag & 3
}

/// key words are references iff the key kind is str (1) or object (2)
fn map_kref_raw(kflag: i64) -> bool {
    map_kkind_raw(kflag) != 0
}

fn map_vref_raw(kflag: i64) -> bool {
    kflag & MAP_VREF != 0
}

fn map_slot(m: usize, i: i64) -> *mut i64 {
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

/// FNV-1a over a `str`'s bytes (the key arrives as a tagged handle word; the
/// payload internals are raw; hashing is by content, not handle identity)
fn map_hash_s(h: i64) -> u64 {
    unsafe {
        let td = w_unref(h) as *const StrT;
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
    } else {
        map_hash_i(key)
    }
}

fn map_streq(a: i64, b: i64) -> bool {
    unsafe {
        let ta = w_unref(a) as *const StrT;
        let tb = w_unref(b) as *const StrT;
        (*ta).len == (*tb).len
            && libc::memcmp((*ta).data, (*tb).data, (*ta).len as libc::size_t) == 0
    }
}

fn map_key_eq(kkind: i64, ka: i64, kb: i64) -> bool {
    if kkind == 1 {
        map_streq(ka, kb)
    } else {
        ka == kb
    }
}

/// object-key equality: pointer identity, or cached-hash equality (the
/// Hashable⇒Equatable contract of the design: equal hash ⇒ equal value;
/// reserved-0 h is the fallback route and never implies equality)
fn map_key_eq_h(kkind: i64, ka: i64, ha: i64, kb: i64, hb: i64) -> bool {
    if kkind != 2 {
        return map_key_eq(kkind, ka, kb);
    }
    ka == kb || (ha != 0 && ha == hb)
}

fn map_alloc_buckets(cap: i64) -> *mut i64 {
    sloth_rt_alloc(((cap * MAP_SLOT_W) * 8) as libc::size_t) as *mut i64
}

#[no_mangle]
pub extern "C" fn sloth_map_new(kflag_w: i64) -> i64 {
    unsafe {
        let cap = 8i64;
        let p = rc_addr((MAP_HDR_W * 8) as usize, Some(map_dtor)) as *mut i64;
        *p = cap;
        *p.offset(1) = 0;
        // low 2 bits = key kind, bit 2 = value-ref flag
        *p.offset(2) = kflag_w & 7;
        *p.offset(3) = map_alloc_buckets(cap) as i64;
        w_ref(p as usize)
    }
}

/// death cascade: release each live pair's reference key/value (compile-time
/// refness flags), then free the bucket buffer (internal, untracked)
fn map_dtor(p: usize, _aux: u64) {
    unsafe {
        let m = p as *mut i64;
        let cap = *m;
        let kflag = map_kflag_raw(p);
        let kref = map_kref_raw(kflag);
        let vref = map_vref_raw(kflag);
        let bp = *m.offset(3) as *mut i64;
        if !bp.is_null() {
            let mut i = 0i64;
            while i < cap {
                let s = bp.offset((i * MAP_SLOT_W) as isize);
                if *s == 1 {
                    let k = *s.offset(1);
                    if kref && k != 0 {
                        crate::rc::sloth_rc_release(k);
                    }
                    let v = *s.offset(2);
                    if vref && v != 0 {
                        crate::rc::sloth_rc_release(v);
                    }
                }
                i += 1;
            }
            libc::free(bp as *mut libc::c_void);
        }
    }
}

fn map_kflag_raw(p: usize) -> i64 {
    unsafe { *(p as *mut i64).offset(2) }
}

#[no_mangle]
pub extern "C" fn sloth_map_len(m_w: i64) -> i64 {
    unsafe { *(w_unref(m_w) as *mut i64).offset(1) }
}

/// bucket index for a lookup/insert: object keys use the caller-provided
/// content hash h (mixed for distribution); builtin keys hash internally
fn map_bucket_i(kkind: i64, key: i64, h: i64, cap: i64) -> u64 {
    if kkind == 2 {
        mix64(h as u64) % (cap as u64)
    } else {
        map_key_hash(kkind, key) % (cap as u64)
    }
}

/// place a used pair with its cached hash into the table (always the current
/// m; no growth here)
fn map_insert_raw(m: usize, key: i64, val: i64, h: i64) {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let kflag = *p.offset(2);
        let kkind = map_kkind_raw(kflag);
        let mut i = map_bucket_i(kkind, key, h, cap);
        loop {
            let s = map_slot(m, i as i64);
            if *s == 0 {
                *s = 1;
                *s.offset(1) = key;
                *s.offset(2) = val;
                *s.offset(3) = h;
                *p.offset(1) += 1;
                return;
            }
            i = (i + 1) % (cap as u64);
        }
    }
}

/// grow: rehash into a fresh bucket array; the map handle stays valid; the
/// stale buffer is freed directly (no stale count entries any more)
unsafe fn map_grow(m: usize) {
    let p = m as *mut i64;
    let cap = *p;
    let kflag = *p.offset(2);
    let kkind = map_kkind_raw(kflag);
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
            map_insert_raw(
                m,
                *old_bp.offset((k + 1) as isize),
                *old_bp.offset((k + 2) as isize),
                *old_bp.offset((k + 3) as isize),
            );
        }
        k += MAP_SLOT_W;
    }
    let _ = (kkind, old_bp);
    libc::free(old_bp as *mut libc::c_void);
}

/// find or create the slot for `key` (h = cached content hash for obj keys);
/// returns the slot pointer
fn map_upsert(m: usize, key: i64, h: i64) -> *mut i64 {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let kflag = *p.offset(2);
        let kkind = map_kkind_raw(kflag);
        let used = *p.offset(1);
        let mut i = map_bucket_i(kkind, key, h, cap);
        let mut rounds = 0u64;
        loop {
            let s = map_slot(m, i as i64);
            if *s == 0 {
                // free slot: new pair (grow first if load too high)
                if used * 4 >= cap * 3 {
                    map_grow(m);
                    return map_upsert(m, key, h);
                }
                *s = 1;
                *s.offset(1) = key;
                *s.offset(3) = h;
                *p.offset(1) = used + 1;
                return s;
            }
            if map_key_eq_h(kkind, *s.offset(1), *s.offset(3), key, h) {
                // overwrite: the caller handed us a +1 on the new key, so
                // release the old key and swap the new one into the slot
                // (leaving the stale pointer behind would double-free it);
                // then evict the old value's count
                let kref = map_kref_raw(kflag);
                let vref = map_vref_raw(kflag);
                let ok = *s.offset(1);
                if kref && ok != 0 {
                    crate::rc::sloth_rc_release(ok);
                }
                *s.offset(1) = key;
                *s.offset(3) = h;
                let ov = *s.offset(2);
                if vref && ov != 0 {
                    crate::rc::sloth_rc_release(ov);
                }
                return s;
            }
            i = (i + 1) % (cap as u64);
            rounds += 1;
            if rounds > cap as u64 {
                // table exhausted: grow and retry
                map_grow(m);
                return map_upsert(m, key, h);
            }
        }
    }
}

macro_rules! map_get_impl {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(m_w: i64, key: i64) -> i64 {
            unsafe {
                let m = w_unref(m_w);
                let cap = *(m as *mut i64);
                let kflag = *(m as *mut i64).offset(2);
                let kkind = map_kkind_raw(kflag);
                let mut i = map_bucket_i(kkind, key, 0, cap);
                let mut rounds = 0u64;
                loop {
                    let s = map_slot(m, i as i64);
                    if *s == 0 || rounds > cap as u64 {
                        let _ = panics::sloth_panic_nokey(crate::rc::dec_i(key));
                        std::process::exit(1);
                    }
                    if map_key_eq_h(kkind, *s.offset(1), *s.offset(3), key, 0) {
                        return *(s.offset(2) as *const i64);
                    }
                    i = (i + 1) % (cap as u64);
                    rounds += 1;
                }
            }
        }
    };
}

/// object-key get with a caller-provided content hash (patch #35)
macro_rules! map_get_h_impl {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(m_w: i64, key: i64, h: i64) -> i64 {
            unsafe {
                let m = w_unref(m_w);
                let cap = *(m as *mut i64);
                let kflag = *(m as *mut i64).offset(2);
                let kkind = map_kkind_raw(kflag);
                let mut i = map_bucket_i(kkind, key, h, cap);
                let mut rounds = 0u64;
                loop {
                    let s = map_slot(m, i as i64);
                    if *s == 0 || rounds > cap as u64 {
                        let _ = panics::sloth_panic_nokey(crate::rc::dec_i(key));
                        std::process::exit(1);
                    }
                    if map_key_eq_h(kkind, *s.offset(1), *s.offset(3), key, h) {
                        return *(s.offset(2) as *const i64);
                    }
                    i = (i + 1) % (cap as u64);
                    rounds += 1;
                }
            }
        }
    };
}

macro_rules! map_set_impl {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(m_w: i64, key: i64, v: i64) -> i64 {
            unsafe {
                let m = w_unref(m_w);
                let s = map_upsert(m, key, 0);
                *(s.offset(2) as *mut i64) = v;
                0
            }
        }
    };
}

/// object-key set with a caller-provided content hash (patch #35)
macro_rules! map_set_h_impl {
    ($name:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(m_w: i64, key: i64, h: i64, v: i64) -> i64 {
            unsafe {
                let m = w_unref(m_w);
                let s = map_upsert(m, key, h);
                *(s.offset(2) as *mut i64) = v;
                0
            }
        }
    };
}

map_get_impl!(sloth_map_get);
map_get_impl!(sloth_map_str_get);
map_set_impl!(sloth_map_set);
map_set_impl!(sloth_map_str_set);
map_get_h_impl!(sloth_map_get_h);
map_set_h_impl!(sloth_map_set_h);

/// array of key words
#[no_mangle]
pub extern "C" fn sloth_map_keys(m_w: i64) -> i64 {
    unsafe {
        let m = w_unref(m_w);
        let cap = *(m as *mut i64);
        let kflag = map_kflag_raw(m);
        let kref = map_kref_raw(kflag);
        // the result array owns a copy of each reference key
        let mut arr = sloth_arr_new_k(0, if kref { 1 } else { 0 });
        let mut i = 0i64;
        while i < cap {
            let s = map_slot(m, i);
            if *s == 1 {
                let k = *s.offset(1);
                if kref && k != 0 {
                    crate::rc::sloth_rc_retain(k);
                }
                arr = sloth_arr_push(arr, k);
            }
            i += 1;
        }
        arr
    }
}

#[no_mangle]
pub extern "C" fn sloth_map_values(m_w: i64) -> i64 {
    unsafe {
        let m = w_unref(m_w);
        let cap = *(m as *mut i64);
        let vref = map_vref_raw(map_kflag_raw(m));
        // values re-own their reference slots
        let mut arr = sloth_arr_new_k(0, if vref { 1 } else { 0 });
        let mut i = 0i64;
        while i < cap {
            let s = map_slot(m, i);
            if *s == 1 {
                let v = *s.offset(2);
                if vref && v != 0 {
                    crate::rc::sloth_rc_retain(v);
                }
                arr = sloth_arr_push(arr, v);
            }
            i += 1;
        }
        arr
    }
}
