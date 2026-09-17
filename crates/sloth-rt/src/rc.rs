//! Tagged-word rc core over the in-band object header (ARC migration).
//!
//! Word tag (LSB): bit0 = 1 marks a refcounted handle (`ptr | 1`, payload is
//! 16-aligned); bit0 = 0 is a value word (63-bit int `v << 1`, f64
//! `(bits & !1) >> 1`, nil = 0) — retain/release/weak all no-op on value
//! words without ever touching memory.
//!
//! Every rc-managed handle (object, array, map, string, lambda frame,
//! payload box, weak box) carries a hidden header 48 bytes BELOW its
//! payload pointer: `[cnt, size, dtor, aux, weak_head, pad]`. `retain`/
//! `release` bump the header count; reaching zero runs the entry's death
//! destructor (cascading field/element releases — mask-free, tag-driven),
//! invalidates every weak box chained into the header's weak list, then
//! frees the header+payload chunk. No collector exists, so a missed
//! release leaks memory rather than crashing; a release of a value word
//! is an inert no-op by the tag bit alone.

use std::sync::atomic::{AtomicU64, Ordering};

/// tag bit of a word (LSB): 1 = refcounted handle, 0 = value word
pub const W_TAG: i64 = 1;
/// mask to clear the tag bit (handle -> raw payload pointer)
pub const W_UNTAG: i64 = !1;

#[inline]
pub const fn w_is_ref(w: i64) -> bool {
    w & W_TAG != 0
}

/// handle word -> raw payload pointer
#[inline]
pub const fn w_unref(w: i64) -> usize {
    (w & W_UNTAG) as usize
}

/// raw payload pointer -> handle word
#[inline]
pub const fn w_ref(p: usize) -> i64 {
    (p as i64) | W_TAG
}

/// tagged int word -> raw value (63-bit arithmetic, the sign rides bit `62`)
#[inline]
pub const fn dec_i(w: i64) -> i64 {
    w >> 1
}

/// raw int value -> tagged word (63-bit wrap semantics)
#[inline]
pub const fn enc_i(v: i64) -> i64 {
    v << 1
}

/// tagged f64 word bits -> raw f64 bits (1 mantissa LSB sacrificed)
#[inline]
pub const fn dec_f_bits(w: i64) -> u64 {
    (w as u64) << 1
}

/// raw f64 bits -> tagged word (mantissa LSB cleared, then shift in)
#[inline]
pub const fn enc_f_bits(bits: u64) -> i64 {
    ((bits & !1) as i64) >> 1
}

/// hidden header for a tracked payload (6 words, 48 bytes, payload-aligned)
#[repr(C)]
pub(crate) struct Hdr {
    cnt: u64,
    /// payload size in bytes (drives the relocating realloc copy)
    size: usize,
    /// death hook executed when the count reaches zero (cascade releases
    /// tagged words; mask-free)
    dtor: Option<fn(usize, u64)>,
    /// reserved kind flags (tag migration retired the elref aux)
    aux: u64,
    /// intrusive chain of weak boxes targeting this payload
    weak_head: *mut WeakBox,
    pad: u64,
}

const HDR_WORDS: usize = 6;
const HDR_BYTES: usize = HDR_WORDS * 8;

/// weak box payload: `{target, next}` intrusive into the target header;
/// the box itself is rc-tracked (copy = retain, last release detaches+frees)
#[repr(C)]
pub(crate) struct WeakBox {
    /// raw target payload pointer; 0 = dead
    target: usize,
    next: *mut WeakBox,
}

#[no_mangle]
pub extern "C" fn sloth_weak_new(h: i64) -> i64 {
    if !w_is_ref(h) {
        return 0;
    }
    unsafe {
        let b = rc_addr(std::mem::size_of::<WeakBox>(), Some(weak_dtor)) as *mut WeakBox;
        let thdr = hdr_of(w_unref(h)) as *mut Hdr;
        (*b).next = (*thdr).weak_head;
        (*thdr).weak_head = b;
        (*b).target = w_unref(h);
        w_ref(b as usize)
    }
}

/// weak box death: detach from the (maybe already dead) target; the rc core
/// frees the box chunk itself
fn weak_dtor(b: usize, _aux: u64) {
    unsafe {
        let bb = b as *mut WeakBox;
        let t = (*bb).target;
        if t == 0 {
            return;
        }
        let thdr = (t - HDR_BYTES) as *mut Hdr;
        // walk the target's weak chain and unlink this box
        let mut prev: *mut *mut WeakBox = &mut (*thdr).weak_head;
        while !(*prev).is_null() {
            if *prev == bb {
                *prev = (*bb).next;
                return;
            }
            prev = &mut (*(*prev)).next;
        }
    }
}

/// upgrade a weak box word to the target handle word (0 = dead)
#[no_mangle]
pub extern "C" fn sloth_weak_upgrade(w: i64) -> i64 {
    if !w_is_ref(w) {
        return 0;
    }
    let t = unsafe { (*(w_unref(w) as *mut WeakBox)).target };
    if t == 0 {
        0
    } else {
        w_ref(t)
    }
}

/// release a weak box's slot ownership (detach + free; idempotent)
#[no_mangle]
pub extern "C" fn sloth_weak_release(w: i64) -> i64 {
    if w_is_ref(w) {
        sloth_rc_release(w);
    }
    0
}

/// number of live tracked handles (diagnostics; tagged int word)
#[no_mangle]
pub extern "C" fn sloth_rc_live() -> i64 {
    enc_i(RC_LIVE.load(Ordering::Relaxed) as i64)
}

static RC_LIVE: AtomicU64 = AtomicU64::new(0);

/// number of release calls executed (diagnostics; tagged int word)
#[no_mangle]
pub extern "C" fn sloth_rc_drops() -> i64 {
    enc_i(DROPS.load(Ordering::Relaxed) as i64)
}

static DROPS: AtomicU64 = AtomicU64::new(0);

/// allocate a tracked chunk: header + zeroed payload; returns the tagged
/// handle word. Payload zeroing keeps the rt contract (fresh slots are nil).
pub(crate) unsafe fn rc_addr(n: usize, dtor: Option<fn(usize, u64)>) -> usize {
    let raw = libc::malloc(n + HDR_BYTES) as *mut u8;
    let hdr = raw as *mut Hdr;
    *hdr = Hdr {
        cnt: 1,
        size: n,
        dtor,
        aux: 0,
        weak_head: std::ptr::null_mut(),
        pad: 0,
    };
    let payload = raw.add(HDR_BYTES);
    std::ptr::write_bytes(payload, 0, n);
    RC_LIVE.fetch_add(1, Ordering::Relaxed);
    payload as usize
}

/// header pointer of a raw payload
#[inline]
pub(crate) unsafe fn hdr_of(payload: usize) -> *mut Hdr {
    (payload - HDR_BYTES) as *mut Hdr
}

/// record a payload's auxiliary header word (object field count for the
/// death cascade; 0 for kinds that do not use it)
#[inline]
pub(crate) unsafe fn set_aux(payload: usize, aux: u64) {
    (*hdr_of(payload)).aux = aux;
}

/// retain: bump the count of a tracked handle (no-op for value words)
#[no_mangle]
pub extern "C" fn sloth_rc_retain(w: i64) -> i64 {
    if w_is_ref(w) {
        unsafe {
            let h = hdr_of(w_unref(w)) as *mut Hdr;
            (*h).cnt += 1;
        }
    }
    w
}

/// release: drop the count of a tracked handle; zero runs the death
/// destructor (cascading tagged-word releases), drains the weak chain and
/// frees header+payload (no-op for value words)
#[no_mangle]
pub extern "C" fn sloth_rc_release(w: i64) -> i64 {
    DROPS.fetch_add(1, Ordering::Relaxed);
    if !w_is_ref(w) {
        return w;
    }
    unsafe {
        let p = w_unref(w);
        let hdr = hdr_of(p);
        if (*hdr).cnt > 1 {
            (*hdr).cnt -= 1;
            return w;
        }
        let dtor = (*hdr).dtor.take();
        let aux = (*hdr).aux;
        let weaks = (*hdr).weak_head;
        // drains the weak chain first: boxes must see no live target
        let mut wb = weaks;
        while !wb.is_null() {
            let next = (*wb).next;
            (*wb).target = 0;
            (*wb).next = std::ptr::null_mut();
            wb = next;
        }
        RC_LIVE.fetch_sub(1, Ordering::Relaxed);
        if let Some(dtor) = dtor {
            dtor(p, aux);
        }
        libc::free(hdr_of(p) as *mut libc::c_void);
    }
    w
}

/// relocate a tracked chunk (realloc path): header followed by payload into
/// fresh memory; the weak chain's {target, next} nodes follow the contents
/// so upgrade() keeps resolving to the new payload. min(old.n, n) bytes are
/// copied from the old payload.
pub(crate) unsafe fn relocate(old_w: i64, n: usize) -> i64 {
    let old_p = w_unref(old_w);
    let old_hdr = hdr_of(old_p);
    let copy = (*old_hdr).size.min(n);
    let new_p = rc_addr(n, None);
    let new_hdr = hdr_of(new_p);
    // carry the identity (cnt/dtor/aux/weaks) to the new header field-wise
    // (UB-check friendly: copy_nonoverlapping::<Hdr> trips the alignment
    // guard on some toolchains even for legitimately aligned malloc blocks)
    (*new_hdr).cnt = (*old_hdr).cnt;
    (*new_hdr).size = n;
    (*new_hdr).dtor = (*old_hdr).dtor;
    (*new_hdr).aux = (*old_hdr).aux;
    (*new_hdr).weak_head = (*old_hdr).weak_head;
    (*new_hdr).pad = 0;
    std::ptr::copy_nonoverlapping(old_p as *const u8, new_p as *mut u8, copy);
    // weak chain nodes hold the old raw target: repoint them
    let mut wb = (*new_hdr).weak_head;
    while !wb.is_null() {
        (*wb).target = new_p;
        wb = (*wb).next;
    }
    libc::free(old_hdr as *mut libc::c_void);
    RC_LIVE.fetch_sub(1, Ordering::Relaxed);
    w_ref(new_p)
}
