//! rc core over the in-band object header (de-tag migration, PLAN §14).
//!
//! The word plane is untagged: a reference is a bare payload pointer (0 =
//! nil) and a value is its native word (int = i64, float = f64 bits,
//! bool = 0/1). Because a word no longer carries its own kind, the runtime
//! never guesses: every `retain`/`release` call is emitted by the compiler on
//! a statically-ref-typed word, and death cascades live in the compile-time
//! kind dtor: for objects and self-hosted containers it is a generated
//! `(payload, aux) -> i64` routine registered as the header `sdtor`; for the
//! remaining Rust-built kinds the header `dtor` reads their flags (arr `aux`
//! elref, closure env slot, channel/fiber `eref`). A `release(0)` (nil)
//! is the only inert input.
//!
//! Every rc-managed handle (object, array, map, string, lambda frame,
//! payload box, weak box) carries a hidden header 48 bytes BELOW its
//! payload pointer: `[cnt, size, dtor, aux, weak_head, weak_cnt]`.
//! `retain`/`release` bump the header count; reaching zero runs the entry's
//! death destructor (cascading field/element releases, mask-driven),
//! invalidates every weak box chained into the header's weak list, then
//! frees the header+payload chunk. No collector exists, so a missed release
//! leaks memory rather than crashing.
//!
//! Multithreading (TH): counts are atomic (`fetch_add` Relaxed /
//! `fetch_sub` Release + Acquire fence). The strong count and a separate
//! weak count form a split control block: when the last strong reference
//! drops, the payload dies and all weak targets are invalidated, but the
//! header chunk stays alive until the last weak box is released. That lets
//! `upgrade()` CAS-retain the target without racing the free (C++
//! `weak_ptr::lock` protocol).

use std::sync::atomic::{fence, AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// reference word -> raw payload pointer (de-tag: identity)
#[inline]
pub const fn w_unref(w: i64) -> usize {
    w as usize
}

/// raw payload pointer -> reference word (de-tag: identity)
#[inline]
pub const fn w_ref(p: usize) -> i64 {
    p as i64
}

/// int word -> raw value (de-tag: identity)
#[inline]
pub const fn dec_i(w: i64) -> i64 {
    w
}

/// raw int value -> word (de-tag: identity)
#[inline]
pub const fn enc_i(v: i64) -> i64 {
    v
}

/// float word bits -> raw f64 bits (de-tag: identity)
#[inline]
pub const fn dec_f_bits(w: i64) -> u64 {
    w as u64
}

/// raw f64 bits -> word (de-tag: identity)
#[inline]
pub const fn enc_f_bits(bits: u64) -> i64 {
    bits as i64
}

/// hidden header for a tracked payload (6 words, 48 bytes, payload-aligned)
#[repr(C)]
pub(crate) struct Hdr {
    /// strong reference count (atomic: shared-memory threads)
    cnt: AtomicU64,
    /// payload size in bytes (drives the relocating realloc copy)
    size: usize,
    /// death hook executed when the count reaches zero; Rust-built kinds only
    /// (their cascade reads the kind's compile-time flags)
    dtor: Option<fn(usize, u64)>,
    /// generated-code death cascade: a raw C-ABI pointer to a routine
    /// `(payload, aux) -> i64`. Installed for objects (codegen emits one
    /// `@...__cascade` per class: user `__dispose__` + reference-field
    /// releases) and for self-hosted containers via `__sloth_rc_new`; takes
    /// precedence over `dtor` when set.
    sdtor: Option<unsafe extern "C" fn(i64, i64) -> i64>,
    /// per-kind auxiliary word: object/cascade payload word (field count for
    /// objects), array element-ref flag (`elref`); interpreted by the kind's
    /// cascade/destructor
    aux: u64,
    /// intrusive chain of weak boxes targeting this payload
    weak_head: *mut WeakBox,
    /// number of live weak boxes targeting this payload; the header chunk is
    /// not freed at strong-zero until this reaches zero too
    weak_cnt: AtomicU64,
}

const HDR_WORDS: usize = 7;
const HDR_BYTES: usize = HDR_WORDS * 8;

/// weak box payload: `{target, hdr, next}` intrusive into the target header;
/// the box itself is rc-tracked (copy = retain, last release detaches+frees).
/// `hdr` keeps the (possibly dead) target header reachable while the box
/// lives so `upgrade` can CAS the target's strong count safely.
#[repr(C)]
pub(crate) struct WeakBox {
    /// raw target payload pointer; 0 = dead
    target: AtomicUsize,
    /// owning target header (kept alive by `weak_cnt`)
    hdr: *mut Hdr,
    next: *mut WeakBox,
}

// ---------------- weak-list spinlock ----------------

/// Guards mutation of any header's intrusive weak chain (link / unlink /
/// invalidate / relocate repoint). Held for very short critical sections and
/// never across a destructor call, so it cannot participate in a cycle.
static WEAK_LOCK: AtomicBool = AtomicBool::new(false);

struct WeakGuard;

impl WeakGuard {
    #[inline]
    fn lock() -> WeakGuard {
        while WEAK_LOCK
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while WEAK_LOCK.load(Ordering::Relaxed) {
                std::hint::spin_loop();
            }
        }
        WeakGuard
    }
}

impl Drop for WeakGuard {
    #[inline]
    fn drop(&mut self) {
        WEAK_LOCK.store(false, Ordering::Release);
    }
}

// ---------------- weak API ----------------

#[no_mangle]
pub extern "C" fn __sloth_weak_new(h: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    unsafe {
        let b = rc_addr(std::mem::size_of::<WeakBox>(), Some(weak_dtor)) as *mut WeakBox;
        let thdr = hdr_of(w_unref(h)) as *mut Hdr;
        (*b).target = AtomicUsize::new(w_unref(h));
        (*b).hdr = thdr;
        let _g = WeakGuard::lock();
        (*b).next = (*thdr).weak_head;
        (*thdr).weak_head = b;
        (*thdr).weak_cnt.fetch_add(1, Ordering::Relaxed);
        w_ref(b as usize)
    }
}

/// weak box death: detach from the (maybe already dead) target; the rc core
/// frees the box chunk itself. Drops this box's weak reference and frees the
/// target header if the target had already died and no weak boxes remain.
fn weak_dtor(b: usize, _aux: u64) {
    unsafe {
        let bb = b as *mut WeakBox;
        let hdr = (*bb).hdr as *mut Hdr;
        if hdr.is_null() {
            return;
        }
        {
            let _g = WeakGuard::lock();
            let mut prev: *mut *mut WeakBox = &mut (*hdr).weak_head;
            while !(*prev).is_null() {
                if *prev == bb {
                    *prev = (*bb).next;
                    break;
                }
                prev = &mut (*(*prev)).next;
            }
        }
        let wc = (*hdr).weak_cnt.fetch_sub(1, Ordering::AcqRel);
        if wc == 1 && (*hdr).cnt.load(Ordering::Acquire) == 0 {
            libc::free(hdr as *mut libc::c_void);
        }
    }
}

/// upgrade a weak box word to the target handle word (0 = dead). The returned
/// handle carries an owned +1 (codegen must not retain it again): the CAS
/// either wins the race with the last strong release or fails on a dead
/// target — it never resurrects a destroyed object, and the header stays
/// mapped while this box lives.
#[no_mangle]
pub extern "C" fn __sloth_weak_upgrade(w: i64) -> i64 {
    if w == 0 {
        return 0;
    }
    unsafe {
        let bb = w_unref(w) as *mut WeakBox;
        let hdr = (*bb).hdr as *mut Hdr;
        if hdr.is_null() {
            return 0;
        }
        // hold the weak lock so a concurrent relocate cannot free the old
        // payload between the target read and the retain
        let _g = WeakGuard::lock();
        let t = (*bb).target.load(Ordering::Acquire);
        if t == 0 {
            return 0;
        }
        loop {
            let c = (*hdr).cnt.load(Ordering::Acquire);
            if c == 0 {
                return 0;
            }
            match (*hdr)
                .cnt
                .compare_exchange_weak(c, c + 1, Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
        w_ref(t)
    }
}

/// release a weak box's slot ownership (detach + free; idempotent)
#[no_mangle]
pub extern "C" fn __sloth_weak_release(w: i64) -> i64 {
    if w != 0 {
        __sloth_rc_release(w);
    }
    0
}

// ---------------- diagnostics ----------------

/// number of live tracked handles (diagnostics; raw int word)
#[no_mangle]
pub extern "C" fn __sloth_rc_live() -> i64 {
    enc_i(RC_LIVE.load(Ordering::Relaxed) as i64)
}

static RC_LIVE: AtomicU64 = AtomicU64::new(0);

/// number of release calls executed (diagnostics; raw int word)
#[no_mangle]
pub extern "C" fn __sloth_rc_drops() -> i64 {
    enc_i(DROPS.load(Ordering::Relaxed) as i64)
}

static DROPS: AtomicU64 = AtomicU64::new(0);

// ---------------- core ----------------

/// allocate a tracked chunk: header + zeroed payload; returns the raw
/// handle word. Payload zeroing keeps the rt contract (fresh slots are nil).
pub(crate) unsafe fn rc_addr(n: usize, dtor: Option<fn(usize, u64)>) -> usize {
    rc_addr_full(n, dtor, None, 0)
}

/// allocate a tracked chunk whose death cascade is generated code: `sdtor`
/// is a raw C-ABI pointer `(payload, aux) -> i64` (0 = none). Used for objects
/// (per-class `@...__cascade`) and self-hosted containers.
pub(crate) unsafe fn rc_addr_sloth(n: usize, aux: u64, sdtor: usize) -> usize {
    let s = if sdtor == 0 {
        None
    } else {
        Some(std::mem::transmute::<
            usize,
            unsafe extern "C" fn(i64, i64) -> i64,
        >(sdtor))
    };
    rc_addr_full(n, None, s, aux)
}

unsafe fn rc_addr_full(
    n: usize,
    dtor: Option<fn(usize, u64)>,
    sdtor: Option<unsafe extern "C" fn(i64, i64) -> i64>,
    aux: u64,
) -> usize {
    let raw = libc::malloc(n + HDR_BYTES) as *mut u8;
    let hdr = raw as *mut Hdr;
    *hdr = Hdr {
        cnt: AtomicU64::new(1),
        size: n,
        dtor,
        sdtor,
        aux,
        weak_head: std::ptr::null_mut(),
        weak_cnt: AtomicU64::new(0),
    };
    let payload = raw.add(HDR_BYTES);
    std::ptr::write_bytes(payload, 0, n);
    RC_LIVE.fetch_add(1, Ordering::Relaxed);
    payload as usize
}

/// `__sloth_rc_new(nbytes, aux, dtor)` — bare tracked allocation for the
/// self-hosted container prelude. `dtor` is the raw address of a compiled
/// sloth `__dispose__` routine `(payload, aux) -> i64` (0 = none).
#[no_mangle]
pub extern "C" fn __sloth_rc_new(nbytes: i64, aux: i64, dtor: i64) -> i64 {
    if nbytes < 0 {
        crate::panics::panic_msg("rc_new: negative payload size");
    }
    unsafe { w_ref(rc_addr_sloth(nbytes as usize, aux as u64, dtor as usize)) }
}

/// header pointer of a raw payload
#[inline]
pub(crate) unsafe fn hdr_of(payload: usize) -> *mut Hdr {
    (payload - HDR_BYTES) as *mut Hdr
}

/// retain: bump the count of a tracked handle (nil is inert)
#[no_mangle]
pub extern "C" fn __sloth_rc_retain(w: i64) -> i64 {
    if w != 0 {
        unsafe {
            let h = hdr_of(w_unref(w)) as *mut Hdr;
            (*h).cnt.fetch_add(1, Ordering::Relaxed);
        }
    }
    w
}

/// release: drop the count of a tracked handle; zero runs the death
/// destructor (cascading mask-driven releases), invalidates the weak chain
/// and frees header+payload once no weak box remains (nil is inert)
#[no_mangle]
pub extern "C" fn __sloth_rc_release(w: i64) -> i64 {
    DROPS.fetch_add(1, Ordering::Relaxed);
    if w == 0 {
        return w;
    }
    unsafe {
        let p = w_unref(w);
        let hdr = hdr_of(p);
        if (*hdr).cnt.fetch_sub(1, Ordering::Release) != 1 {
            return w;
        }
        // last strong reference: acquire the dtor's reads/writes
        fence(Ordering::Acquire);
        let dtor = (*hdr).dtor.take();
        let sdtor = (*hdr).sdtor.take();
        let aux = (*hdr).aux;
        // Snapshot whether any weak box existed *before* the death cascade.
        // Ownership of the final `free` must be unique: if weak boxes exist,
        // the last `weak_dtor` (which may itself run inside this cascade when
        // the object owns a weak box to itself / a weak back-edge) frees the
        // header; otherwise this path does. Reading `weak_cnt` again *after*
        // the cascade would race with a `weak_dtor` running during it and
        // double-free (bug A2 / OPT1).
        let had_weak = (*hdr).weak_cnt.load(Ordering::Acquire) != 0;
        // invalidate every weak box (their own rc keeps the boxes alive).
        // The header chunk itself is retained while weak boxes exist.
        {
            let _g = WeakGuard::lock();
            let mut wb = (*hdr).weak_head;
            while !wb.is_null() {
                (*wb).target.store(0, Ordering::Release);
                wb = (*wb).next;
            }
        }
        RC_LIVE.fetch_sub(1, Ordering::Relaxed);
        if let Some(sd) = sdtor {
            // self-hosted `__dispose__`: (payload, aux)
            sd(p as i64, aux as i64);
        } else if let Some(dtor) = dtor {
            dtor(p, aux);
        }
        if !had_weak {
            libc::free(hdr as *mut libc::c_void);
        }
    }
    w
}

/// relocate a tracked chunk (realloc path): header followed by payload into
/// fresh memory; the weak chain's {target, hdr} nodes follow the contents
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
    (*new_hdr)
        .cnt
        .store((*old_hdr).cnt.load(Ordering::Relaxed), Ordering::Relaxed);
    (*new_hdr).size = n;
    (*new_hdr).dtor = (*old_hdr).dtor;
    (*new_hdr).sdtor = (*old_hdr).sdtor;
    (*new_hdr).aux = (*old_hdr).aux;
    (*new_hdr).weak_head = (*old_hdr).weak_head;
    (*new_hdr).weak_cnt.store(
        (*old_hdr).weak_cnt.load(Ordering::Relaxed),
        Ordering::Relaxed,
    );
    std::ptr::copy_nonoverlapping(old_p as *const u8, new_p as *mut u8, copy);
    // weak chain nodes hold the old raw target: repoint them (under the weak
    // lock so a concurrent upgrade sees a consistent {target, hdr})
    {
        let _g = WeakGuard::lock();
        let mut wb = (*new_hdr).weak_head;
        while !wb.is_null() {
            (*wb).target.store(new_p, Ordering::Release);
            (*wb).hdr = new_hdr;
            wb = (*wb).next;
        }
    }
    libc::free(old_hdr as *mut libc::c_void);
    RC_LIVE.fetch_sub(1, Ordering::Relaxed);
    w_ref(new_p)
}
