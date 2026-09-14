//! sloth-codegen: MLIR backend for sloth2.

pub use mlir_sys as sys;

mod context;
mod module;
mod pass;
mod jit;

pub fn selftest() -> Result<(), String> {
    pass::smoke_all()
}
