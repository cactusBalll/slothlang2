//! GC allocator primitives shared by every runtime container.
//!
//! Backed by Boehm-Demers-Weiser (libgc, §5.1 MVP):
//! allocations are garbage collected via conservative mark & sweep;
//! no manual free path exists today (we never `libc::free` a GC chunk).
//!
//! GC_init runs lazily at the first allocation; single-threaded programs
//! (the only supported execution model, no fibers §7.1) hit it on the main
//! thread, which is the only thread libgc registers implicitly.

extern "C" {
    fn GC_init();
    fn GC_malloc(n: libc::size_t) -> *mut libc::c_void;
    fn GC_realloc(p: *mut libc::c_void, n: libc::size_t) -> *mut libc::c_void;
    fn GC_get_heap_size() -> libc::size_t;
    fn GC_get_gc_no() -> libc::size_t; // collection count for diagnostics
    fn GC_gcollect();
    fn GC_allow_register_threads();
    fn GC_get_stack_base(base: *mut GcStackBase) -> i32;
    fn GC_register_my_thread(base: *const GcStackBase) -> i32;
}

#[repr(C)]
struct GcStackBase {
    mem_base: *mut libc::c_char,
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    reg_base: *mut libc::c_void,
}

static INIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static INIT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

thread_local! {
    static REGISTERED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// collector entry precondition: initialized collector + current thread
/// registered (the initializing thread is registered by GC_init, other
/// threads self-register here)
fn init() {
    use std::sync::atomic::Ordering;
    // serialize collector bring-up + first-use registration so parallel
    let g = INIT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    REGISTERED.with(|r| {
        if !r.get() {
            if !INIT.swap(true, Ordering::AcqRel) {
                unsafe {
                    GC_init();
                    // non-interceptor threads may self-register (gc.h §registration)
                    GC_allow_register_threads();
                }
                // GC_init registers the initializing thread implicitly
                r.set(true);
                return;
            }
            unsafe {
                let mut base = GcStackBase { mem_base: std::ptr::null_mut() };
                if GC_get_stack_base(&mut base) == 0 {
                    GC_register_my_thread(&base);
                }
            }
            r.set(true);
        }
    });
    drop(g);
}

#[no_mangle]
pub extern "C" fn sloth_gc_alloc(n: libc::size_t) -> *mut libc::c_void {
    init();
    unsafe { GC_malloc(n) }
}

/// GC-tracked growth (replaces libc::realloc callers); contents preserved,
/// the returned pointer supersedes the old one immediately (realloc moves).
#[no_mangle]
pub extern "C" fn sloth_gc_realloc(
    p: *mut libc::c_void,
    n: libc::size_t,
) -> *mut libc::c_void {
    init();
    unsafe { GC_realloc(p, n) }
}

/// total heap bytes currently reserved by the collector (diagnostics)
#[no_mangle]
pub extern "C" fn sloth_gc_heap_size() -> libc::size_t {
    init();
    unsafe { GC_get_heap_size() }
}

/// number of collections so far (diagnostics)
#[no_mangle]
pub extern "C" fn sloth_gc_collections() -> libc::size_t {
    init();
    unsafe { GC_get_gc_no() }
}

/// full mark & sweep collection on demand; returns collection count
#[no_mangle]
pub extern "C" fn sloth_gc_collect() -> libc::size_t {
    init();
    unsafe { GC_gcollect() };
    unsafe { GC_get_gc_no() }
}
