//! Owning MLIR context.

use crate::sys;

pub struct Context {
    pub raw: sys::MlirContext,
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            let raw = sys::mlirContextCreate();
            sys::mlirContextSetAllowUnregisteredDialects(raw, true);
            assert_eq!(sys::mlirContextGetAllowUnregisteredDialects(raw), true);
            let registry = sys::mlirDialectRegistryCreate();
            sys::mlirRegisterAllDialects(registry);
            sys::mlirContextAppendDialectRegistry(raw, registry);
            sys::mlirContextLoadAllAvailableDialects(raw);
            sys::mlirRegisterAllLLVMTranslations(raw);
            sys::mlirDialectRegistryDestroy(registry);
            Context { raw }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::mlirContextDestroy(self.raw) }
    }
}
