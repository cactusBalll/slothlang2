//! Owning MLIR context.

use crate::sys;

pub struct Context {
    pub raw: sys::MlirContext,
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            let raw = sys::mlirContextCreate();
            // scheme A2: the custom `sloth` dialect is registered, so
            // unregistered dialects are no longer tolerated (typos surface
            // as parse errors).
            sys::mlirContextSetAllowUnregisteredDialects(raw, false);
            assert_eq!(sys::mlirContextGetAllowUnregisteredDialects(raw), false);
            let registry = sys::mlirDialectRegistryCreate();
            sys::mlirRegisterAllDialects(registry);
            crate::dialect::register_dialect(registry);
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
