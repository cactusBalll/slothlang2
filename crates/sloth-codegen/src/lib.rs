//! sloth-codegen: MLIR backend for sloth2.

pub use mlir_sys as sys;

mod context;
mod module;
pub mod pass;
mod jit;
pub mod irgen;

pub fn selftest() -> Result<(), String> {
    pass::smoke_all()
}

/// parse any textual MLIR and print back (for IR probes)
pub fn parse_print_raw(src: &str, name: &str) -> Result<String, String> {
    let ctx = context::Context::new();
    let op = module::Op::parse(ctx.raw, src, name)?;
    Ok(op.print())
}
