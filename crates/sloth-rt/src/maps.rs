//! Open-addressing hash map (linear probing).
//! Header layout: `[cap, used, kkind, buckets_ptr]` — the header stays put
//! across growth, only the bucket array is reallocated, so stored map handles
//! remain valid. Each bucket slot = 4 words `[used, key, value, hash]`:
//! `hash` caches the caller-provided content hash for object keys (kkind: 2),
//! so growth rehashes without re-calling the monomorphized hash() (patch #35).
//! kkind: 0 = i64 keys, 1 = str keys (interned str handles compared by
//! content), 2 = object keys (content hash via the `hash()`-family call at
//! the call site; the legacy plain ops keep pointer identity as fallback)

use crate::alloc::sloth_rt_alloc;
use crate::arrays::{sloth_arr_new_k, sloth_arr_push};
use crate::panics;
use crate::rc::{track_owned, track_user_dtor};
use crate::strings::StrT;

const MAP_HDR_W: i64 = 4;
const MAP_SLOT_W: i64 = 4;

/// kkind encoding (patch C): low bits = key kind (0 int / 1 str / 2 object),
/// bit 8 = value words are refcounted (class/array/map/str values)
const MK_VREF: i64 = 1 << 8;

fn map_vref(kflag: i64) -> bool {
    kflag & MK_VREF != 0
}

fn map_kkind(kflag: i64) -> i64 {
    kflag & 3
}

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

fn map_alloc_buckets(m: i64, cap: i64) -> *mut i64 {
    let b = sloth_rt_alloc(((cap * MAP_SLOT_W) * 8) as libc::size_t) as *mut i64;
    // bucket buffer is an internal chunk owned by the map header
    track_owned(b as usize, m as usize);
    b
}

#[no_mangle]
pub extern "C" fn sloth_map_new(kkind: i64) -> i64 {
    unsafe {
        let cap = 8i64;
        let o = sloth_rt_alloc((MAP_HDR_W * 8) as libc::size_t) as *mut i64;
        *o = cap;
        *o.offset(1) = 0;
        *o.offset(2) = kkind;
        *o.offset(3) = 0;
        // death cascade: release ref-typed keys (kkind != 0), ref-typed
        // values (vref bit) and the bucket buffer
        track_user_dtor(o as usize, map_dtor);
        *o.offset(3) = map_alloc_buckets(o as i64, cap) as i64;
        o as i64
    }
}

/// death cascade: release each live pair's key/value words, then the pair
/// owns nothing; the Owned(parent) bucket buffers detach separately
fn map_dtor(p: usize, _aux: u64) {
    unsafe {
        let m = p as *mut i64;
        let cap = *m;
        let kflag = *m.offset(2);
        let kkind = map_kkind(kflag);
        let vref = map_vref(kflag);
        let bp = *m.offset(3) as *mut i64;
        if bp.is_null() {
            return;
        }
        let mut i = 0i64;
        while i < cap {
            let s = bp.offset((i * MAP_SLOT_W) as isize);
            if *s == 1 {
                if kkind != 0 {
                    let k = *s.offset(1);
                    if k != 0 {
                        crate::rc::sloth_rc_release(k);
                    }
                }
                if vref {
                    let v = *s.offset(2);
                    if v != 0 {
                        crate::rc::sloth_rc_release(v);
                    }
                }
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_map_len(m: i64) -> i64 {
    unsafe { *(m as *mut i64).offset(1) }
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
fn map_insert_raw(m: i64, key: i64, val: i64, h: i64) {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let kflag = *p.offset(2);
        let kkind = map_kkind(kflag);
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

/// grow: rehash into a fresh bucket array; the map handle stays valid
unsafe fn map_grow(m: i64) {
    let p = m as *mut i64;
    let cap = *p;
    let kflag = *p.offset(2);
    let kkind = map_kkind(kflag);
    let used = *p.offset(1);
    let ncap = cap * 2;
    let old_bp = *p.offset(3) as *mut i64;
    let old_words = cap * MAP_SLOT_W;
    // swap out the bucket array first so reinserts land in the fresh table
    // (fresh buffer is owned by the same header; the stale buffer's count
    // entry dies with the header)
    *p.offset(3) = map_alloc_buckets(m, ncap) as i64;
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
    let _ = (kkind, used);
}

/// find or create the slot for `key` (h = cached content hash for obj keys);
/// returns the slot pointer
fn map_upsert(m: i64, key: i64, h: i64) -> *mut i64 {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let kflag = *p.offset(2);
        let kkind = map_kkind(kflag);
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
                // overwrite: evict the old pair's counts (kkind != 0 => ref
                // key; vref => ref value)
                let kflag2 = *p.offset(2);
                if map_kkind(kflag2) != 0 {
                    let ok = *s.offset(1);
                    if ok != 0 {
                        crate::rc::sloth_rc_release(ok);
                    }
                }
                if map_vref(kflag2) {
                    let ov = *s.offset(2);
                    if ov != 0 {
                        crate::rc::sloth_rc_release(ov);
                    }
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
    ($name:ident, $valty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(m: i64, key: i64) -> $valty {
            unsafe {
                let p = m as *mut i64;
                let cap = *p;
                let kflag = *p.offset(2);
                let kkind = map_kkind(kflag);
                let mut i = map_bucket_i(kkind, key, 0, cap);
                let mut rounds = 0u64;
                loop {
                    let s = map_slot(m, i as i64);
                    if *s == 0 || rounds > cap as u64 {
                        let _ = panics::sloth_panic_nokey(key);
                        std::process::exit(1);
                    }
                    if map_key_eq_h(kkind, *s.offset(1), *s.offset(3), key, 0) {
                        return *(s.offset(2) as *const $valty);
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
    ($name:ident, $valty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(m: i64, key: i64, h: i64) -> $valty {
            unsafe {
                let p = m as *mut i64;
                let cap = *p;
                let kflag = *p.offset(2);
                let kkind = map_kkind(kflag);
                let mut i = map_bucket_i(kkind, key, h, cap);
                let mut rounds = 0u64;
                loop {
                    let s = map_slot(m, i as i64);
                    if *s == 0 || rounds > cap as u64 {
                        let _ = panics::sloth_panic_nokey(key);
                        std::process::exit(1);
                    }
                    if map_key_eq_h(kkind, *s.offset(1), *s.offset(3), key, h) {
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
                let s = map_upsert(m, key, 0);
                *(s.offset(2) as *mut $valty) = v;
                0
            }
        }
    };
}

/// object-key set with a caller-provided content hash (patch #35)
macro_rules! map_set_h_impl {
    ($name:ident, $valty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(m: i64, key: i64, h: i64, v: $valty) -> i64 {
            unsafe {
                let s = map_upsert(m, key, h);
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
map_get_h_impl!(sloth_map_get_h, i64);
map_get_h_impl!(sloth_map_get_h_f64, f64);
map_set_h_impl!(sloth_map_set_h, i64);
map_set_h_impl!(sloth_map_set_h_f64, f64);

/// array of key words
#[no_mangle]
pub extern "C" fn sloth_map_keys(m: i64) -> i64 {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let kkind = map_kkind(*p.offset(2));
        // keys are refcounted words for str/object key kinds (patch C)
        let mut arr = sloth_arr_new_k(0, (kkind != 0) as i64);
        let mut i = 0i64;
        while i < cap {
            let s = map_slot(m, i);
            if *s == 1 {
                let k = *s.offset(1);
                // the result array owns its copy of each ref key
                if kkind != 0 && k != 0 {
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
pub extern "C" fn sloth_map_values(m: i64) -> i64 {
    unsafe {
        let p = m as *mut i64;
        let cap = *p;
        let vref = map_vref(*p.offset(2));
        // values re-own their slots (values never change hands: released on
        // map death; the array retains each extracted value)
        let mut arr = sloth_arr_new_k(0, map_vref(*p.offset(2)) as i64);
        let _ = vref;
        let mut i = 0i64;
        while i < cap {
            let s = map_slot(m, i);
            if *s == 1 {
                let v = *s.offset(2);
                if map_vref(*p.offset(2)) && v != 0 {
                    crate::rc::sloth_rc_retain(v);
                }
                arr = sloth_arr_push(arr, v);
            }
            i += 1;
        }
        arr
    }
}
