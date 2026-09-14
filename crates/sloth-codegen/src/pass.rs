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
use crate::jit::Engine;
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
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw)
        .map_err(|e| format!("pipeline: {}", e))?;
    let e = Engine::new(&op, 2, &[lib_path()]);
    eprintln!("invokePacked target=sloth_main lib={}", lib_path());
    let r = e.invoke("sloth_main", &mut []);
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

#[cfg(test)]
mod irgen_p3 {
    use super::*;

    #[test]
    fn break_continue_work() {
        let src = r#"
            var i: int = 0;
            while true {
                i = i + 1;
                if i > 5 {
                    break;
                }
                if i % 2 == 0 {
                    continue;
                }
                print(i);
            }
            print(i);
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn float_arith_works() {
        let src = r#"
            var x: float = 1.5;
            var y: float = 2.5;
            print(x + y);
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn str_len_works() {
        let src = r#"
            var a: str = "abcde";
            print(len(a));
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn fn_call_works() {
        let src = r#"
            func add(a: int, b: int) -> int {
                return a + b;
            }
            print(add(1, 2));
        "#;
        run_src(src, "main").unwrap();
    }
}

#[cfg(test)]
mod irgen_p3b {
    use super::*;

    #[test]
    fn str_display_int_works() {
        let src = r#"
            var n: int = 15;
            print(n * 2);
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn logical_ops_work() {
        let src = r#"
            var a: int = 1;
            var b: int = 2;
            if a == 1 && b > 0 {
                print(7);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn nested_fn_calls_work() {
        let src = r#"
            func inc(x: int) -> int {
                return x + 1;
            }
            func twice(x: int) -> int {
                return inc(x) + inc(x);
            }
            print(twice(3));
        "#;
        run_src(src, "main").unwrap();
    }
}

#[cfg(test)]
mod irgen_p4 {
    use super::*;

    #[test]
    fn class_field_access_works() {
        let src = r#"
            class Pt {
                var x: int;
                func __init__(a: int) {
                    this.x = a;
                }
            }
            var p = Pt(7);
            print(p.x);
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn multimodb_works() {
        let d = std::env::temp_dir().join("sloth_mm");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(d.join("lib.mm.sl"), "func twofold(a: int) -> int {\n    return a + a;\n}").unwrap();
        let src = "import \"lib.mm.sl\";\nvar x = twofold(6);\nprint(x);\n";
        run_src_multimod(src, &d).unwrap();
    }

    #[test]
    fn multimod_qualified_global_works() {
        let d = std::env::temp_dir().join("sloth_mmq");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(
            d.join("cfg.mm.sl"),
            "pub var counter = 20;\nfunc twice(v: int) -> int {\n    return v + v;\n}\n",
        )
        .unwrap();
        let src = "import \"cfg.mm.sl\" as cfg;\nprint(cfg.twice(3));\nprint(cfg.counter);\n";
        run_src_multimod(src, &d).unwrap();
    }

    #[test]
    fn multimod_foreign_class_works() {
        let d = std::env::temp_dir().join("sloth_mmc");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(
            d.join("lib.mm.sl"),
            "pub class Counter {\n    var n: int;\n    func __init__(a: int) {\n        this.n = a;\n    }\n    func bump(d: int) -> int {\n        this.n = this.n + d;\n        return this.n;\n    }\n}\n",
        )
        .unwrap();
        let src = "import \"lib.mm.sl\" as lib;\nvar c = lib.Counter(1);\nprint(c.bump(4));\nprint(c.n);\n";
        run_src_multimod(src, &d).unwrap();
    }

    #[test]
    fn lambda_plain_works() {
        let src = r#"
            var add = |a: int, b: int| { return a + b; };
            print(add(2, 3));
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn lambda_captures_works() {
        let src = r#"
            var base = 10;
            var inc = |x: int| { return x + base; };
            print(inc(5));
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn class_method_call_works() {
        let src = r#"
            class Counter {
                var n: int;
                func __init__(a: int) {
                    this.n = a;
                }
                func bump(d: int) -> int {
                    this.n = this.n + d;
                    return this.n;
                }
            }
            var c = Counter(1);
            print(c.bump(4));
        "#;
        run_src(src, "main").unwrap();
    }
}

#[cfg(test)]
mod irgen_p3c {
    use super::*;

    /// single inheritance: fields laid out across the chain, inherited
    /// method called statically, ctor height includes base fields
    #[test]
    fn class_inherit_fields_works() {
        let src = r#"
            class A {
                var x: int;
                func __init__(v: int) {
                    this.x = v;
                }
                func getx() -> int {
                    return this.x;
                }
            }
            class B: A {
                var y: int;
                func __init__() {
                    super.__init__(5);
                    this.y = 2;
                }
                func total() -> int {
                    return this.x + this.y;
                }
            }
            var b = B();
            print(b.x);
            print(b.y);
            print(b.getx());
            print(b.total());
        "#;
        run_src(src, "main").unwrap();
    }

    /// method override + super.method() delegation (in this-based dispatch)
    #[test]
    fn class_super_method_works() {
        let src = r#"
            class Base {
                var n: int;
                func __init__(v: int) {
                    this.n = v;
                }
                func v() -> int {
                    return this.n;
                }
            }
            class Sub: Base {
                var m: int;
                func __init__(a: int, b: int) {
                    super.__init__(a);
                    this.m = b;
                }
                func v() -> int {
                    return super.v() + this.m;
                }
            }
            var s = Sub(3, 10);
            print(s.v());
        "#;
        run_src(src, "main").unwrap();
    }

    /// super.x = v: write an inherited field slot through this
    #[test]
    fn class_super_assign_works() {
        let src = r#"
            class P {
                var x: int;
                func __init__() {
                    this.x = 0;
                }
            }
            class C: P {
                func __init__() {
                    super.__init__();
                    super.x = 9;
                }
            }
            var c = C();
            print(c.x);
        "#;
        run_src(src, "main").unwrap();
    }

    /// unit-return methods and unit-return locals don't bind call results
    #[test]
    fn unit_calls_no_bind_works() {
        let src = r#"
            func shout(msg: str) {
                print(msg);
            }
            class K {
                var n: int;
                func __init__(v: int) {
                    this.n = v;
                }
                func announce() {
                    print(this.n);
                }
            }
            shout("hi");
            var k = K(4);
            k.announce();
        "#;
        run_src(src, "main").unwrap();
    }
}

#[cfg(test)]
mod irgen_p3d {
    use super::*;

    /// dyn Trait: runtime dispatch over implementing classes
    #[test]
    fn trait_dyn_dispatch_works() {
        let src = r#"
            trait Speaker {
                func say(): unit;
            }
            class Fish impl Speaker {
                func say(): unit {
                    print("glub");
                }
            }
            class Dog impl Speaker {
                func say(): unit {
                    print("woof");
                }
            }
            func announce(s: dyn Speaker) {
                s.say();
            }
            announce(Fish());
            announce(Dog());
            var s: dyn Speaker = Dog();
            s.say();
        "#;
        run_src(src, "main").unwrap();
    }

    /// inherited method satisfies a trait impl; trait-typed param position
    #[test]
    fn trait_inherited_impl_works() {
        let src = r#"
            trait Counter {
                func bump(d: int): int;
            }
            class Base {
                var n: int;
                func __init__(n: int) {
                    this.n = n;
                }
                func bump(d: int) -> int {
                    this.n = this.n + d;
                    return this.n;
                }
            }
            class Inc: Base impl Counter {
                var step: int;
                func __init__(n: int) {
                    super.__init__(n);
                    this.step = 1;
                }
            }
            func twosteps(c: dyn Counter) -> int {
                return c.bump(1) + c.bump(1);
            }
            var i = Inc(0);
            print(twosteps(i));
        "#;
        run_src(src, "main").unwrap();
    }

    /// vtable entry resolves to the overriding method
    #[test]
    fn trait_override_dispatch_works() {
        let src = r#"
            trait Shape {
                func area(): int;
            }
            class Base impl Shape {
                var w: int;
                func __init__(w: int) {
                    this.w = w;
                }
                func area(): int {
                    return 0;
                }
            }
            class Sq: Base impl Shape {
                var h: int;
                func __init__(w: int, h: int) {
                    super.__init__(w);
                    this.h = h;
                }
                func area(): int {
                    return this.w * this.h;
                }
            }
            func show(s: dyn Shape) {
                print(s.area());
            }
            var b = Base(9);
            var q = Sq(3, 4);
            show(b);
            show(q);
        "#;
        run_src(src, "main").unwrap();
    }

    /// broken impl (missing trait method) reports a codegen diagnostic
    #[test]
    fn trait_missing_method_diag() {
        let src = r#"
            trait Speaker {
                func say(): unit;
            }
            class Silent impl Speaker {
            }
            var s = Silent();
        "#;
        let prog = sloth_frontend::parser::parse(src).unwrap();
        let mut me = ModEmitter::new("main");
        me.emit_module(&prog);
        assert!(
            !me.diags.is_empty(),
            "expected a diagnostic for missing trait method"
        );
    }

    /// unknown trait name in impl reports a codegen diagnostic
    #[test]
    fn trait_unknown_impl_diag() {
        let src = r#"
            class Ghost impl Nowhere {
            }
            var g = Ghost();
        "#;
        let prog = sloth_frontend::parser::parse(src).unwrap();
        let mut me = ModEmitter::new("main");
        me.emit_module(&prog);
        assert!(
            !me.diags.is_empty(),
            "expected a diagnostic for unknown trait"
        );
    }
}
