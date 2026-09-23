//! FFI to the C++/TableGen `sloth` dialect sidecar (scheme A2) and the
//! single-point post-parse lowering used by both JIT and AOT paths.

use crate::module::Op;
use crate::sys;

extern "C" {
    fn slothRegisterDialect(registry: sys::MlirDialectRegistry);
    fn slothLowerModule(module: sys::MlirModule) -> sys::MlirLogicalResult;
}

/// Register the custom dialect into `registry` (call after
/// `mlirRegisterAllDialects`, before the context loads available dialects).
pub(crate) fn register_dialect(registry: sys::MlirDialectRegistry) {
    unsafe { slothRegisterDialect(registry) }
}

/// Lower every `sloth.*` op in a freshly parsed module to standard dialects.
/// Fails when any `sloth.*` op survives (leak gate).
pub(crate) fn lower_parsed(op: &Op) -> Result<(), String> {
    let module = unsafe { sys::mlirModuleFromOperation(op.raw) };
    let r = unsafe { slothLowerModule(module) };
    if r.value == 1 {
        // second gate: printed text must contain no `sloth.` op prefix
        // (symbol names like @sloth_rc_retain use `_`, not `.`)
        let printed = op.print();
        if printed.contains("sloth.") {
            return Err("sloth dialect leak after lowering".to_string());
        }
        Ok(())
    } else {
        Err("sloth lowering failed (leftover sloth.* op)".to_string())
    }
}

/// parse + lower + print: the single funnel used by `slothc ir` / AOT so
/// downstream consumers (`mlir-opt`, book goldens) only ever see standard
/// dialects.
pub(crate) fn lower_text(ir: &str, source_name: &str) -> Result<String, String> {
    let ctx = crate::context::Context::new();
    let op = Op::parse(ctx.raw, ir, source_name)?;
    lower_parsed(&op)?;
    Ok(op.print())
}
