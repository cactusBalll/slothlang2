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

/// run canonicalize/cse then the func-to-llvm conversion pipeline
pub fn run_llvm_pipeline(ctx: sys::MlirContext, op: sys::MlirOperation) -> Result<(), String> {
    unsafe {
        let pm = sys::mlirPassManagerCreate(ctx);
        sys::mlirPassManagerAddOwnedPass(pm, sys::mlirCreateTransformsCanonicalizer());
        sys::mlirPassManagerAddOwnedPass(pm, sys::mlirCreateTransformsCSE());
        sys::mlirPassManagerAddOwnedPass(pm, sys::mlirCreateConversionConvertFuncToLLVMPass());
        sys::mlirPassManagerAddOwnedPass(pm, sys::mlirCreateConversionArithToLLVMConversionPass());
        sys::mlirPassManagerAddOwnedPass(pm, sys::mlirCreateConversionConvertIndexToLLVMPass());
        sys::mlirPassManagerAddOwnedPass(
            pm,
            sys::mlirCreateConversionConvertControlFlowToLLVMPass(),
        );
        sys::mlirPassManagerAddOwnedPass(
            pm,
            sys::mlirCreateConversionFinalizeMemRefToLLVMConversionPass(),
        );
        sys::mlirPassManagerAddOwnedPass(
            pm,
            sys::mlirCreateConversionReconcileUnrealizedCastsPass(),
        );
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
