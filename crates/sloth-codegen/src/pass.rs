//! JIT driver: compile a sloth2 program to textual MLIR, lower it, and run it
//! through ORC. This module backs `slothc run`; the MLIR round-trip probes that
//! used to live here were early-development smoke tests superseded by the
//! `spec`/`seed` suites.

use crate::context::Context;
use crate::irgen::ModEmitter;
use crate::jit::Engine;
use crate::module::Op;

/// Leave the process once a `slothc run` program has returned. `run` is a
/// one-shot mode: returning from `run_src` would drop the `Engine`, unmapping
/// the JIT session while detached worker threads may still be executing
/// JIT-emitted code (bug G6/B5). Flush the buffered stdout/stderr first, then
/// `_exit(0)` — the same no-teardown discipline the panic paths use.
fn exit_after_run() -> ! {
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();
    unsafe { libc::_exit(0) }
}

/// path of libsloth_rt.so produced by this workspace build
fn lib_path() -> String {
    // running binary is at target/debug/slothc; resolve relative to the exe
    let exe = std::env::current_exe().unwrap_or_default();
    let exe_dir = exe.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let lib = exe_dir.join("libsloth_rt.so");
    if lib.exists() {
        return lib.display().to_string();
    }
    // cargo test runs with cwd = crate dir: go up to workspace target/debug
    let md = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let c = format!("{}/../../target/debug/libsloth_rt.so", md);
    if std::path::Path::new(&c).exists() {
        return c;
    }
    // last resort: absolute unknown
    "/home/undatus63/slothlang2/target/debug/libsloth_rt.so".to_string()
}

/// compile+run a sloth2 program through the JIT (run mode)
pub fn run_src(src: &str, mod_name: &str) -> Result<(), String> {
    let mut prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    sloth_frontend::ast::assign_ids(&mut prog);
    // Pass 1: semantic analysis owns diagnostics and the NodeId type table.
    let sem = crate::sem::analyze_program(&prog, mod_name)?;
    // Pass 2: emission, consulting the Pass 1 type side table.
    let mut me = ModEmitter::new(mod_name);
    me.check_mode = false;
    me.seed_mono_plan(sem.mono);
    me.seed_typed(sem.types);
    me.emit_module(&prog);
    let errs = me.diags.clone();
    if !errs.is_empty() {
        return Err(format!("codegen diags: {:?}", errs));
    }
    let ir = ModEmitter::take_ir(&mut me);
    let ctx = Context::new();
    let op = match Op::parse(ctx.raw, &ir, "prog.mlir") {
        Ok(op2) => op2,
        Err(e) => {
            let _ = std::fs::write(format!("/tmp/opencode/{}/dump.mlir", mod_name), &ir);
            return Err(format!("MLIR parse: {}", e));
        }
    };
    crate::dialect::lower_parsed(&op)?;
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw).map_err(|e| format!("pipeline: {}", e))?;
    let e = crate::jit::Engine::new(&op, 2, &[lib_path()]);
    eprintln!("invokePacked target=sloth_main lib={}", lib_path());
    let r = e.invoke("sloth_main", &mut []);
    let _ = std::fs::write(
        format!("/tmp/opencode/{}/llvm-after.mlir", mod_name),
        op.print(),
    );
    r?;
    // do not drop the engine under live detached workers (bug B5)
    exit_after_run()
}

/// compile+run a multi-module program through the JIT
pub fn run_src_multimod(src: &str, base: &std::path::Path) -> Result<(), String> {
    let ir0 = crate::irgen::compile_multimod(src, base).map_err(|e| format!("lower: {}", e))?;
    let ctx = Context::new();
    let op = match Op::parse(ctx.raw, &ir0, "prog.mlir") {
        Ok(op2) => op2,
        Err(e) => {
            let _ = std::fs::write("/tmp/opencode/main/dump.mlir", &ir0);
            return Err(format!("MLIR parse: {}", e));
        }
    };
    crate::dialect::lower_parsed(&op)?;
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw).map_err(|e| format!("pipeline: {}", e))?;
    let e = Engine::new(&op, 2, &[lib_path()]);
    eprintln!("invokePacked target=sloth_main lib={}", lib_path());
    let r = e.invoke("sloth_main", &mut []);
    r?;
    // do not drop the engine under live detached workers (bug B5)
    exit_after_run()
}

#[cfg(test)]
mod dialect_migration {
    use crate::irgen::ModEmitter;

    /// ARC emission must go through the `sloth` dialect and the single-point
    /// funnel (compile_to_ir) must lower it away before anyone sees the IR.
    #[test]
    fn arc_ops_are_sloth_then_lowered() {
        let src = r#"
            var a: str = "abc";
            var b: str = "def";
            print(a + b);
        "#;
        let prog = sloth_frontend::parser::parse(src).unwrap();
        let mut me = ModEmitter::new("main");
        me.emit_module(&prog);
        assert!(me.diags.is_empty(), "diags: {:?}", me.diags);
        let raw = ModEmitter::take_ir(&mut me);
        assert!(
            raw.contains("sloth.rc_release"),
            "raw emission must use the sloth dialect:\n{}",
            raw
        );
        let lowered = crate::irgen::compile_to_ir(src, "main").unwrap();
        assert!(
            !lowered.contains("sloth."),
            "lowered output leaked sloth dialect:\n{}",
            lowered
        );
        assert!(
            lowered.contains("@__sloth_rc_release"),
            "lowered output missing rt call:\n{}",
            lowered
        );
    }
}
