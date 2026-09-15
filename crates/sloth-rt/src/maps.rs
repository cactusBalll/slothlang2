//! Open-addressing hash map (linear probing).
//! Header layout: `[cap, used, kkind, buckets_ptr]` — the header stays put
//! across growth, only the bucket array is reallocated, so stored map handles
//! remain valid. Each bucket slot = 3 words `[used, key, value]`.
//! kkind: 0 = i64 keys, 1 = str keys (interned str handles compared by
//! content), 2 = object keys (pointer identity + hashed pointer; MVP
//! wp for Hashable)

use crate::arrays::{sloth_arr_new, sloth_arr_push};
use crate::gc::sloth_gc_alloc;
use crate::strings::StrT;
use crate::panics;

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
                        let _ = panics::sloth_panic_nokey(key);
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

#[cfg(test)]
mod tests {
    use crate::arrays::{sloth_arr_len, sloth_arr_get};
    use crate::maps::*;

    #[test]
    fn map_roundtrip_20_keys() {
        unsafe {
            let m = sloth_map_new(0);
            for i in 0..20 {
                sloth_map_set(m, i, i * 2);
            }
            assert_eq!(sloth_map_len(m), 20, "len after 20 sets");
            for i in 0..20 {
                assert_eq!(sloth_map_get(m, i), i * 2, "key {}", i);
            }
            let ks = sloth_map_keys(m);
            assert_eq!(sloth_arr_len(ks), 20, "keys array len");
            let vs = sloth_map_values(m);
            assert_eq!(sloth_arr_len(vs), 20, "values array len");
        }
    }

    /// two-key map: no phantom used slots, uniform probe
    #[test]
    fn map_small_two_keys() {
        unsafe {
            let m = sloth_map_new(0);
            sloth_map_set(m, 5, 50);
            sloth_map_set(m, 6, 60);
            assert_eq!(sloth_map_len(m), 2);
            assert_eq!(sloth_map_get(m, 5), 50);
            assert_eq!(sloth_map_get(m, 6), 60);
            let ks = sloth_map_keys(m);
            assert_eq!(sloth_arr_len(ks), 2, "keys array");
            let ks_len = sloth_arr_len(ks);
            let mut i = 0i64;
            let mut s50 = 0i64;
            let mut s60 = 0i64;
            while i < ks_len {
                let k = sloth_arr_get(ks, i);
                if k == 5 {
                    s50 = 1;
                }
                if k == 6 {
                    s60 = 1;
                }
                assert_eq!(sloth_map_get(m, k), k * 10);
                i += 1;
            }
            assert_eq!(s50, 1, "key 5 present");
            assert_eq!(s60, 1, "key 6 present");
        }
    }
}
