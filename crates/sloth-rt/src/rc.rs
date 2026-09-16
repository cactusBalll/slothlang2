//! Reference-count core over the deterministic allocator (ARC migration).
//!
//! Every rc-managed handle (object, array, map, string, lambda frame, and
//! internal buffer chunks) gets one entry keyed by the handle address.
//! `retain`/`release` move the counter; reaching zero runs the entry's death
//! destructor (cascading field/element releases), invalidates every weak box
//! pointing at the handle, detaches entry-owned children (internal buffers),
//! and then frees the chunk memory itself — no collector exists, so a
//! missed release leaks memory rather than crashing, and a release of an
//! unknown word is an inert no-op.
//!
//! Weak boxes live outside the counted heap (libc::malloc): they hold the raw
//! target address and are nulled when their target's count reaches zero.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    /// user-visible value (object/array/map/str/lambda frame)
    User,
    /// internal buffer owned by a parent handle (map buckets); shares the
    /// parent's lifetime but never outlives it
    Owned(usize),
}

pub(crate) struct Entry {
    cnt: u32,
    kind: Kind,
    /// weak boxes currently pointing at this handle
    weaks: Vec<*mut WeakBox>,
    /// patch C: type-driven destructor hook executed when the count reaches
    /// zero (cascades field/element/bucket releases for containers/objects)
    /// aux carries the handle's kind flags (e.g. array element refness)
    dtor: Option<fn(usize, u64)>,
    aux: u64,
}

#[repr(C)]
pub(crate) struct WeakBox {
    /// nonzero while the target is alive
    target: usize,
}

/// wrapper so the raw-pointer entries can live in a static Mutex
/// (single-threaded runtime: the mutex is only a safety belt)
pub(crate) struct RawMap(pub HashMap<usize, Entry>);
unsafe impl Send for RawMap {}
unsafe impl Sync for RawMap {}

fn table() -> &'static Mutex<RawMap> {
    static T: OnceLock<Mutex<RawMap>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(RawMap(HashMap::new())))
}

static DROPS: AtomicU64 = AtomicU64::new(0);

/// register a fresh user-owned handle produced by a runtime constructor
pub(crate) fn track_user(h: usize) {
    let mut t = table().lock().unwrap();
    t.0.insert(
        h,
        Entry {
            cnt: 1,
            kind: Kind::User,
            weaks: Vec::new(),
            dtor: None,
            aux: 0,
        },
    );
}

/// register with a death destructor (patch C cascade hook)
pub(crate) fn track_user_dtor(h: usize, dtor: fn(usize, u64)) {
    let mut t = table().lock().unwrap();
    t.0.insert(
        h,
        Entry {
            cnt: 1,
            kind: Kind::User,
            weaks: Vec::new(),
            dtor: Some(dtor),
            aux: 0,
        },
    );
}

/// set the kind-flag word of an entry (patch C: array element refness etc.)
pub(crate) fn set_aux(h: usize, aux: u64) {
    let mut t = table().lock().unwrap();
    if let Some(e) = t.0.get_mut(&h) {
        e.aux = aux;
    }
}

/// register an internal buffer owned by `parent`; it detaches (no cascade,
/// the parent owns it) when the parent entry dies
pub(crate) fn track_owned(h: usize, parent: usize) {
    let mut t = table().lock().unwrap();
    t.0.insert(
        h,
        Entry {
            cnt: 1,
            kind: Kind::Owned(parent),
            weaks: Vec::new(),
            dtor: None,
            aux: 0,
        },
    );
}

/// re-register a handle that was re-created in place (minimum viable: used by
/// producers whose chunk memory is recycled; count resets to 1)
#[allow(dead_code)]
pub(crate) fn reset(h: usize) {
    let mut t = table().lock().unwrap();
    t.0.insert(
        h,
        Entry {
            cnt: 1,
            kind: Kind::User,
            weaks: Vec::new(),
            dtor: None,
            aux: 0,
        },
    );
}

/// move count ownership from `old` to `new` (realloc path: the chunk was
/// relocated; any weak boxes follow the contents to the new address)
pub(crate) fn transfer(old: usize, new: usize) {
    let mut t = table().lock().unwrap();
    let e = t.0.remove(&old);
    match e {
        Some(e) => {
            for wb in &e.weaks {
                unsafe { (**wb).target = new };
            }
            t.0.entry(new).or_insert(e);
        }
        None => {
            t.0.entry(new).or_insert(Entry {
                cnt: 1,
                kind: Kind::User,
                weaks: Vec::new(),
                dtor: None,
                aux: 0,
            });
        }
    }
}

/// retain: bump the count of a tracked handle (no-op for untracked words such
/// as nil or non-ref integers)
#[no_mangle]
pub extern "C" fn sloth_rc_retain(h: i64) -> i64 {
    if h != 0 {
        let mut t = table().lock().unwrap();
        if let Some(e) = t.0.get_mut(&(h as usize)) {
            e.cnt += 1;
        }
    }
    h
}

/// release: drop the count of a tracked handle; zero detaches the entry,
/// invalidates its weak boxes and drops entry-owned children
#[no_mangle]
pub extern "C" fn sloth_rc_release(h: i64) -> i64 {
    DROPS.fetch_add(1, Ordering::Relaxed);
    if h == 0 {
        return 0;
    }
    // snapshot: hold the lock ONLY for this table surgery WITHOUT running
    // the destructor cascade, child draining or the final free — those run
    // after the guard is dropped, since child releases re-enter this
    // function recursively (std Mutex is not reentrant: cascade-under-lock
    // used to deadlock the harness)
    let (dtor, aux, weaks) = {
        let mut t = table().lock().unwrap();
        if !t.0.contains_key(&(h as usize)) {
            return h;
        }
        let e = t.0.get_mut(&(h as usize)).unwrap();
        if e.cnt > 1 {
            e.cnt -= 1;
            return h;
        }
        let dtor = e.dtor.take();
        let aux = e.aux;
        let weaks = std::mem::take(&mut e.weaks);
        t.0.remove(&(h as usize));
        (dtor, aux, weaks)
    };
    for wb in weaks {
        unsafe { (*wb).target = 0 };
    }
    if let Some(dtor) = dtor {
        dtor(h as usize, aux);
    }
    // owned children (map buckets, also stale grow buffers): detach + free
    let kids: Vec<usize> = {
        let mut t = table().lock().unwrap();
        let kids: Vec<usize> =
            t.0.iter()
                .filter(|(_k2, e2)| matches!(e2.kind, Kind::Owned(p) if p == h as usize))
                .map(|(k2, _)| *k2)
                .collect();
        for kid in &kids {
            t.0.remove(kid);
        }
        kids
    };
    for kid in kids {
        unsafe { libc::free(kid as *mut libc::c_void) };
    }
    unsafe { libc::free(h as *mut libc::c_void) };
    h
}

/// number of live tracked handles (diagnostics)
#[no_mangle]
pub extern "C" fn sloth_rc_live() -> i64 {
    table().lock().unwrap().0.len() as i64
}

/// number of release calls executed (diagnostics)
#[no_mangle]
pub extern "C" fn sloth_rc_drops() -> i64 {
    DROPS.load(Ordering::Relaxed) as i64
}

/// weak reference: a malloc'd box holding the raw target address; the box
/// is independent memory, so it does not keep the target alive. The box
/// itself is rc-tracked (copy = retain, last release detaches + frees)
#[no_mangle]
pub extern "C" fn sloth_weak_new(h: i64) -> i64 {
    unsafe {
        let b = libc::malloc(std::mem::size_of::<WeakBox>()) as *mut WeakBox;
        (*b).target = h as usize;
        crate::rc::track_user_dtor(b as usize, weak_dtor);
        let mut t = table().lock().unwrap();
        if let Some(e) = t.0.get_mut(&(h as usize)) {
            e.weaks.push(b);
        }
        b as i64
    }
}

/// weak box death: detach from the (maybe already dead) target; the rc core
/// frees the box chunk itself
fn weak_dtor(b: usize, _aux: u64) {
    unsafe {
        let bb = b as *mut WeakBox;
        let mut t = table().lock().unwrap();
        if let Some(e) = t.0.get_mut(&(*bb).target) {
            e.weaks.retain(|x| *x != bb);
        }
    }
}

/// upgrade a weak box to the target handle (0 = dead)
#[no_mangle]
pub extern "C" fn sloth_weak_upgrade(w: i64) -> i64 {
    if w == 0 {
        return 0;
    }
    unsafe { (*(w as *mut WeakBox)).target as i64 }
}

/// release a weak box's slot ownership (detach + free; idempotent)
#[no_mangle]
pub extern "C" fn sloth_weak_release(w: i64) -> i64 {
    if w != 0 {
        unsafe {
            sloth_rc_release(w);
        }
    }
    0
}
