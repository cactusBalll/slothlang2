//! Single source of truth for the MLIR lowering pipeline (TE-P2).
//!
//! Both the JIT (`jit::run_llvm_pipeline`, via the C-API textual pipeline)
//! and the AOT build (`slothc::build_mode_r`, via `mlir-opt --<name>`) consume
//! this list, eliminating the previous two-place hardcoding drift.
//!
//! The tensor segment (`one-shot-bufferize` … `convert-math-to-llvm`) is
//! inserted before the func/arith conversions so `linalg`/`scf`/`math` lower
//! to `llvm` in the same run. Channel-B operators stay in the `memref` domain
//! and bypass bufferization entirely (design D2), so the bufferizer only ever
//! sees function-local `tensor` values.

/// full ordered pass list, names as accepted by `mlir-opt` and the C-API
/// textual pipeline parser
pub fn pass_names() -> &'static [&'static str] {
    &[
        "canonicalize",
        "cse",
        "one-shot-bufferize",
        "linalg-fuse-elementwise-ops",
        "convert-linalg-to-loops",
        "convert-scf-to-cf",
        "convert-math-to-llvm",
        "convert-func-to-llvm",
        "convert-arith-to-llvm",
        "convert-index-to-llvm",
        "convert-cf-to-llvm",
        "finalize-memref-to-llvm",
        "reconcile-unrealized-casts",
    ]
}

/// comma-separated form for `mlirOpPassManagerAddPipeline`
pub fn pass_pipeline_string() -> String {
    pass_names().join(",")
}
