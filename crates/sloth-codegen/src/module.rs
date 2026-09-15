//! Owning wrappers for MlirOperation / parsed modules and textual round-trip.

use crate::sys;
use std::ffi::{c_void, CString};

/// Owns an MlirOperation (e.g. a module op).
pub struct Op {
    pub raw: sys::MlirOperation,
}

impl Op {
    /// Parse textual MLIR in `ctx`; `source_name` is used for locations.
    pub fn parse(ctx: sys::MlirContext, source: &str, source_name: &str) -> Result<Op, String> {
        let src = CString::new(source).map_err(|e| e.to_string())?;
        let nm = CString::new(source_name).map_err(|e| e.to_string())?;
        unsafe {
            let op = sys::mlirOperationCreateParse(
                ctx,
                sys::mlirStringRefCreateFromCString(src.as_ptr()),
                sys::mlirStringRefCreateFromCString(nm.as_ptr()),
            );
            if op.ptr.is_null() {
                Err(diagnostic(ctx))
            } else {
                Ok(Op { raw: op })
            }
        }
    }

    pub fn print(&self) -> String {
        unsafe {
            let mut out: Vec<u8> = Vec::new();
            let cb: sys::MlirStringCallback = Some(print_cb);
            let ud = &mut out as *mut Vec<u8> as *mut c_void;
            sys::mlirOperationPrint(self.raw, cb, ud);
            String::from_utf8_lossy(&out).into_owned()
        }
    }
}

unsafe extern "C" fn print_cb(ref_: sys::MlirStringRef, user_data: *mut c_void) {
    let out = &mut *(user_data as *mut Vec<u8>);
    let len = ref_.length as usize;
    let bytes = std::slice::from_raw_parts(ref_.data as *const u8, len);
    out.extend_from_slice(bytes);
}

impl Drop for Op {
    fn drop(&mut self) {
        unsafe { sys::mlirOperationDestroy(self.raw) }
    }
}

unsafe fn diagnostic(ctx: sys::MlirContext) -> String {
    // Diagnostics engine default emits to stderr; derive a simple message.
    let _ = ctx;
    "MLIR parse failed (diagnostic printed to stderr)".to_string()
}
