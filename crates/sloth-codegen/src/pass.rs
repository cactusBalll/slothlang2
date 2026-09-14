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
