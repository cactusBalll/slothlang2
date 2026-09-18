//! JIT ExecutionEngine wrapper.

use crate::sys;
use std::ffi::CString;

pub struct Engine {
    pub raw: sys::MlirExecutionEngine,
}

impl Engine {
    pub fn new(module: &crate::module::Op, opt: i32, libs: &[String]) -> Engine {
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
                if srefs.is_empty() {
                    std::ptr::null()
                } else {
                    srefs.as_ptr()
                },
                false,
            );
            Engine { raw: jit }
        }
    }
}

impl Engine {
    pub fn invoke(&self, name: &str, args: &mut [*mut libc::c_void]) -> Result<(), String> {
        unsafe {
            let cn = CString::new(name).unwrap();
            let r = sys::mlirExecutionEngineInvokePacked(
                self.raw,
                sys::mlirStringRefCreateFromCString(cn.as_ptr()),
                args.as_mut_ptr(),
            );
            if r.value == 1 {
                Ok(())
            } else {
                // look up address (panic indicator)
                let lookup = sys::mlirExecutionEngineLookupPacked(
                    self.raw,
                    sys::mlirStringRefCreateFromCString(cn.as_ptr()),
                );
                Err(format!("invoke {} failed (packed ptr {:?})", name, lookup))
            }
        }
    }
}

/// swallow pass-parser diagnostics so a failed parse is reported through the
/// return code rather than a fatal MLIR error
unsafe extern "C" fn discard_diag(_s: sys::MlirStringRef, _u: *mut std::ffi::c_void) {}

/// run the shared lowering pipeline (see `crate::pipeline::pass_names`).
/// `one-shot-bufferize` has no individual C-API creator, so the whole list is
/// driven as a textual pipeline after registering all passes.
pub fn run_llvm_pipeline(ctx: sys::MlirContext, op: sys::MlirOperation) -> Result<(), String> {
    unsafe {
        sys::mlirRegisterAllPasses();
        let pm = sys::mlirPassManagerCreate(ctx);
        let opm = sys::mlirPassManagerGetAsOpPassManager(pm);
        let pipe = CString::new(crate::pipeline::pass_pipeline_string()).unwrap();
        let added = sys::mlirOpPassManagerAddPipeline(
            opm,
            sys::mlirStringRefCreateFromCString(pipe.as_ptr()),
            Some(discard_diag),
            std::ptr::null_mut(),
        );
        if added.value != 1 {
            sys::mlirPassManagerDestroy(pm);
            return Err("pass pipeline failed to parse".to_string());
        }
        let r = sys::mlirPassManagerRunOnOp(pm, op);
        let ok = r.value == 1;
        sys::mlirPassManagerDestroy(pm);
        if ok {
            Ok(())
        } else {
            Err("pass pipeline failed".to_string())
        }
    }
}
