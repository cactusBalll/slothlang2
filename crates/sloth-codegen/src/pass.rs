//! Smoke tests: parse/print/round-trip textual MLIR with sloth.* ops.

use crate::context::Context;
use crate::module::Op;

const SRC: &str = r#"
module @sloth_test {
  func.func @add(%a : i64, %b : i64) -> i64 {
    %r = arith.addi %a, %b : i64
    %z = arith.constant 0 : i64
    %c = arith.cmpi ne, %a, %z : i64
    scf.if %c -> (i64) {
      scf.yield %r : i64
    } else {
      scf.yield %z : i64
    }
    return %r : i64
  }
  %g = "sloth.gc_alloc"() ({}) : () -> i64
}
"#;

pub fn smoke_all() -> Result<(), String> {
    let ctx = Context::new();
    let op = Op::parse(ctx.raw, SRC, "test.slt.mlir")?;
    let printed = op.print();
    if !printed.contains("arith.addi") || !printed.contains("sloth.gc_alloc") {
        return Err(format!("round-trip content mismatch:\n{}", printed));
    }
    Ok(())
}

// -------- irgen driver: build hello program and run --------

use crate::irgen::ModEmitter;
use sloth_frontend::parser::parse;

pub fn sloth_main_hello() -> Result<(), String> {
    let src = r#"
        var x: int = 21;
        var y: int = 21;
        print(x + y);
        print("hello world");
    "#;
    let prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    let mut me = ModEmitter::new("main");
    me.emit_module(&prog);
    for d in &me.diags {
        return Err(format!("codegen diag {}: {}", d.line, d.msg));
    }
    let ir0 = ModEmitter::take_ir(&mut me);
    let ir = crate::irgen::normalize_indices(&ir0);
    eprintln!("--- MLIR ---\n{}", ir);
    let ctx = Context::new();
    let op = Op::parse(ctx.raw, &ir, "hello.mlir")?;
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw)
        .map_err(|e| format!("pipeline: {}", e))?;
    let engine = crate::jit::Engine::new(&op, 2, &[lib_path()]);
    engine.invoke("sloth_main", &mut [])?;
    Ok(())
}

/// path of libsloth_rt.so produced by this workspace build
fn lib_path() -> String {
    // workspace-relative: CARGO_MANIFEST_DIR = .../crates/sloth-codegen
    let md = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    format!("{}/../../target/debug/libsloth_rt.so", md)
}

/// compile+run a sloth2 program through the JIT (run mode)
pub fn run_src(src: &str, mod_name: &str) -> Result<(), String> {
    let prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    let mut me = ModEmitter::new(mod_name);
    me.emit_module(&prog);
    let mut errs = me.diags.clone();
    if !errs.is_empty() {
        return Err(format!("codegen diags: {:?}", errs));
    }
    let ir0 = ModEmitter::take_ir(&mut me);
    let ir = crate::irgen::normalize_indices(&ir0);
    let ctx = Context::new();
    let op = match Op::parse(ctx.raw, &ir, "prog.mlir") {
        Ok(op2) => op2,
        Err(e) => {
            let _ = std::fs::write(format!("/tmp/opencode/{}/dump.mlir", mod_name), &ir);
            return Err(format!("MLIR parse: {}", e));
        }
    };
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw)
        .map_err(|e| format!("pipeline: {}", e))?;
    let e = crate::jit::Engine::new(&op, 2, &[lib_path()]);
    eprintln!("invokePacked target=sloth_main lib={}", lib_path());
    let r = e.invoke("sloth_main", &mut []);
    let _ = std::fs::write(format!("/tmp/opencode/{}/llvm-after.mlir", mod_name), op.print());
    r?;
    Ok(())
}

#[cfg(test)]
mod irgen_tests {
    use super::*;

    /// smoke: JIT prints "3" then "6" (sum of 1..=3 in a while loop)
    #[test]
    fn while_loop_sum() {
        let src = r#"
            var i: int = 0;
            var s: int = 0;
            while i < 3 {
                i = i + 1;
                s = s + i;
                print(i);
            }
            print(s);
        "#;
        run_src(src, "main").unwrap();
    }
}

#[cfg(test)]
mod irgen_more {
    use super::*;

    #[test]
    fn if_else_works() {
        let src = r#"
            var x: int = 3;
            if x > 2 {
                print(x);
            } else {
                print(0);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn range_for_works() {
        let src = r#"
            var s: int = 0;
            for i in 1..4 {
                s = s + i;
                print(i);
            }
            print(s);
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn str_concat_works() {
        let src = r#"
            var a: str = "abc";
            var b: str = "def";
            print(a + b);
        "#;
        run_src(src, "main").unwrap();
    }
}
