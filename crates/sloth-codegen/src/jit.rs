//! JIT ExecutionEngine wrapper.

use crate::sys;
use std::ffi::CString;

pub struct Engine {
    pub raw: sys::MlirExecutionEngine,
}

impl Engine {
    pub fn new(
        module: &crate::module::Op,
        opt: i32,
        libs: &[String],
    ) -> Engine {
        unsafe {
            let refs: Vec<CString> = libs
                .iter()
                .map(|l| CString::new(l.clone()).unwrap())
                .collect();
            let srefs: Vec<sys::MlirStringRef> = refs
                .iter()
                .map(|c| sys::mlirStringRefCreateFromCString(c.as_ptr()))
                .collect();
            let jit = sys::mlirExecutionEngineCreate(
                sys::mlirModuleFromOperation(module.raw),
                opt,
                srefs.len() as i32,
                if srefs.is_empty() { std::ptr::null() } else { srefs.as_ptr() },
                false,
            );
            Engine { raw: jit }
        }
    }
}

impl Engine {
    pub fn register(&self, name: &str, sym: *mut libc::c_void) {
        unsafe {
            let cn = CString::new(name).unwrap();
            sys::mlirExecutionEngineRegisterSymbol(
                self.raw,
                sys::mlirStringRefCreateFromCString(cn.as_ptr()),
                sym,
            );
        }
    }

    pub fn invoke(&self, name: &str, args: &mut [*mut libc::c_void]) -> Result<(), String> {
        unsafe {
            let cn = CString::new(name).unwrap();
            let r = sys::mlirExecutionEngineInvokePacked(
                self.raw,
                sys::mlirStringRefCreateFromCString(cn.as_ptr()),
                args.as_mut_ptr(),
            );
            if r.value == 0 {
                Err(format!("invoke {} failed", name))
            } else {
                Ok(())
            }
        }
    }
}
