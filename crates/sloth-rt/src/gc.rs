//! GC allocator primitives shared by every runtime container.

/// MVP allocator stand-in (replaced by Boehm GC in sloth-rt step 2).
#[no_mangle]
pub extern "C" fn sloth_gc_alloc(n: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::calloc(1, n) }
}
