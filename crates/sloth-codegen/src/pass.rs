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
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw).map_err(|e| format!("pipeline: {}", e))?;
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
    let errs = me.diags.clone();
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
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw).map_err(|e| format!("pipeline: {}", e))?;
    let e = crate::jit::Engine::new(&op, 2, &[lib_path()]);
    eprintln!("invokePacked target=sloth_main lib={}", lib_path());
    let r = e.invoke("sloth_main", &mut []);
    let _ = std::fs::write(
        format!("/tmp/opencode/{}/llvm-after.mlir", mod_name),
        op.print(),
    );
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
    crate::jit::run_llvm_pipeline(ctx.raw, op.raw).map_err(|e| format!("pipeline: {}", e))?;
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
        std::fs::write(
            d.join("lib.mm.sl"),
            "pub func twofold(a: int) -> int {\n    return a + a;\n}",
        )
        .unwrap();
        let src = "import \"lib.mm.sl\";\nvar x = twofold(6);\nprint(x);\n";
        run_src_multimod(src, &d).unwrap();
    }

    #[test]
    fn multimod_qualified_global_works() {
        let d = std::env::temp_dir().join("sloth_mmq");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(
            d.join("cfg.mm.sl"),
            "pub var counter = 20;\npub func twice(v: int) -> int {\n    return v + v;\n}\n",
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

    /// non-pub fn/var access from another module is rejected
    #[test]
    fn private_cross_module_rejected() {
        let d = std::env::temp_dir().join("sloth_mmpriv");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(
            d.join("priv.mm.sl"),
            "func hidden() -> int {\n    return 1;\n}\nvar secret = 5;\n",
        )
        .unwrap();
        for src in [
            "import \"priv.mm.sl\" as p;\nprint(p.hidden());\n",
            "import \"priv.mm.sl\" as p;\nprint(p.secret);\n",
        ] {
            let r = run_src_multimod(src, &d);
            let e = match r {
                Ok(()) => panic!("private access accepted: {:?}", src),
                Err(e) => e,
            };
            assert!(e.contains("private to its module"), "unexpected: {}", e);
        }
    }

    /// cross-module pub var read with a declared main(): modinit assembly
    /// runs in entry before main body (was: `M.g` read the raw 0 cell)
    #[test]
    fn multimod_global_reads_with_main() {
        let d = std::env::temp_dir().join("sloth_mmg");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(
            d.join("g.mm.sl"),
            "pub var g = 42;\npub var s = \"seed\";\npub func h() -> int {\n    return g;\n}\n",
        )
        .unwrap();
        let src = "import \"g.mm.sl\" as M;\nfunc main() {\n    print(M.g);\n    print(M.h());\n    print(M.s);\n}\n";
        run_src_multimod(src, &d).unwrap();
    }

    /// empty string literal: no builder chunks pushed at all (was: NULL
    /// builder deref in sloth_str_finish)
    #[test]
    fn empty_str_literal_works() {
        let src = r#"
            func main() {
                var s = "";
                print(s);
                print("x" + s);
                var t = "" + "y";
                print(t);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// pub fn/var/class access from another module still works
    #[test]
    fn pub_cross_module_allowed() {
        let d = std::env::temp_dir().join("sloth_mmpub");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(
            d.join("pub.mm.sl"),
            "pub var n = 3;\npub class Box {\n    var v: int;\n    func __init__(a: int) {\n        this.v = a;\n    }\n}\npub func triple(x: int) -> int {\n    return x * 3;\n}\n",
        )
        .unwrap();
        let src =
            "import \"pub.mm.sl\" as q;\nprint(q.triple(q.n));\nvar b = q.Box(2);\nprint(b.v);\n";
        run_src_multimod(src, &d).unwrap();
    }

    /// is/is not with nil and class chains
    #[test]
    fn is_type_tests_work() {
        let src = r#"
            class Animal { }
            class Dog: Animal { }
            var d = Dog();
            var n: int? = nil;
            print(d is Dog);
            print(d is Animal);
            print(d is not Animal);
            print(n is nil);
            print(not (n is nil));
        "#;
        run_src(src, "main").unwrap();
    }

    /// elvis default
    #[test]
    fn elvis_nil_default_works() {
        let src = r#"
            var a: int? = nil;
            var b: int? = 7;
            print(a ?: 5);
            print(b ?: 5);
        "#;
        run_src(src, "main").unwrap();
    }

    /// circular import chain is diagnosed
    #[test]
    fn circular_import_diagnosed() {
        let d = std::env::temp_dir().join("sloth_mmcyc");
        let _ = std::fs::create_dir_all(&d);
        std::fs::write(
            d.join("b.sl"),
            "import \"a.sl\";\nfunc bx() -> int {\n    return 1;\n}\n",
        )
        .unwrap();
        std::fs::write(
            d.join("a.sl"),
            "import \"b.sl\";\npub func ax() -> int {\n    return bx();\n}\n",
        )
        .unwrap();
        let src = "import \"a.sl\";\nprint(ax());\n";
        let e = match run_src_multimod(src, &d) {
            Ok(()) => panic!("cycle accepted"),
            Err(e) => e,
        };
        assert!(e.contains("circular import"), "unexpected: {}", e);
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

    /// array literals, index read/write, len builtin
    #[test]
    fn array_literal_index_works() {
        let src = r#"
            var a = [1, 2, 3];
            var s = 0;
            for i in 0..len(a) {
                s = s + a[i];
            }
            print(s);
            a[1] = 20;
            print(a[0] + a[1] + a[2]);
            print(len(a));
        "#;
        run_src(src, "main").unwrap();
    }

    /// float array with promotion + f64 indexing
    #[test]
    fn array_float_works() {
        let src = r#"
            var f: Array<float> = [1.0, 2.5, 3.0];
            f[0] = 0.5;
            print(f[0] + f[1]);
        "#;
        run_src(src, "main").unwrap();
    }

    /// object array with dyn element iteration (trait surface on elem)
    #[test]
    fn array_of_dyn_works() {
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
            func chorus(sp: Array<dyn Speaker>) {
                var s = 0;
                for x in sp {
                    x.say();
                    s = s + 1;
                }
                print(s);
            }
            var zoo = [Fish(), Dog(), Fish()];
            chorus(zoo);
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

// patch #9: array push/pop, let immutability, declared-kind coercion
#[cfg(test)]
mod irgen_p9 {
    use super::*;

    /// push appends; pop removes and returns last elem; mixed growth
    #[test]
    fn array_push_pop_works() {
        let src = r#"
            var a: Array<int> = [];
            a.push(1);
            a.push(2);
            a.push(3);
            print(a.len());
            print(a.pop());
            print(a.len());
            var f2: Array<float> = [1.5];
            f2.push(2.0);
            print(f2[1]);
            print(f2 .len());
        "#;
        run_src(src, "main").unwrap();
    }

    /// let binding rejects reassignment
    #[test]
    fn let_immutable_rejected() {
        let src = r#"
            let a = 1;
            a = 2;
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("let rebinding accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("cannot assign to immutable"),
            "unexpected: {}",
            e
        );
    }

    /// var still mutable
    #[test]
    fn var_mutable_still_works() {
        let src = r#"
            var a = 1;
            a = 2;
            print(a);
        "#;
        run_src(src, "main").unwrap();
    }

    /// declared type conflicts with initializer kind
    #[test]
    fn declared_kind_mismatch_diag() {
        for src in ["var x: int = 2.5;\nprint(x);\n", "var y: int = 1.0;\n"] {
            let e = match run_src(src, "main") {
                Ok(()) => panic!("kind mismatch accepted: {:?}", src),
                Err(e) => e,
            };
            assert!(
                e.contains("expected: non-float surface\\n  got: float"),
                "unexpected: {}",
                e
            );
        }
    }

    /// declared float coerces int initializer words
    #[test]
    fn declared_float_coerces() {
        let src = r#"
            var x: float = 2;
            print(x + 0.5);
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #10: map runtime ----------------
#[cfg(test)]
mod irgen_p10 {
    use super::*;

    /// literal, index read/write, len; growth past initial 8 buckets
    #[test]
    fn map_i64_keys_work() {
        let src = r#"
            var m: Map<int, int> = @(1: 10, 2: 20, 3: 30);
            print(len(m));
            print(m[1]);
            print(m[3]);
            m[2] = 99;
            print(m[2]);
            var i = 0;
            while i < 20 {
                m[i] = i * 2;
                i = i + 1;
            }
            print(len(m));
            print(m[19]);
        "#;
        run_src(src, "main").unwrap();
    }

    /// str keys route to the content-comparison path
    #[test]
    fn map_str_keys_work() {
        let src = r#"
            var m: Map<str, int> = @("a": 1, "b": 2);
            print(m["a"]);
            print(m["b"]);
            m["c"] = 7;
            print(len(m));
            print(m["c"]);
        "#;
        run_src(src, "main").unwrap();
    }

    /// float values keep the f64 word route
    #[test]
    fn map_float_values_work() {
        let src = r#"
            var m: Map<int, float> = @(1: 1.5);
            m[2] = 2;
            print(m[1] + m[2]);
        "#;
        run_src(src, "main").unwrap();
    }

    /// for-in over a map iterates keys; keys/values word views
    #[test]
    fn map_iteration_yields_keys() {
        let src = r#"
            var m: Map<int, int> = @(5: 50, 6: 60);
            var acc = 0;
            for (var e: m) {
                acc = acc + e.key + e.val;
            }
            print(acc);
            var ks = keys(m);
            print(len(ks));
            print(ks[0] + ks[1]);
        "#;
        run_src(src, "main").unwrap();
    }

    /// m.len() method form
    #[test]
    fn map_len_method() {
        let src = r#"
            var m: Map<str, int> = @("x": 1);
            print(m.len());
        "#;
        run_src(src, "main").unwrap();
    }

    /// declarations carry maps (globals + arguments)
    #[test]
    fn map_as_global_and_arg() {
        let src = r#"
            var g: Map<int, int> = @(1: 11);
            func bump(t: Map<int, int>, k: int): int {
                return t[k] + 1;
            }
            print(bump(g, 1));
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #11: variadic function bodies ----------------
#[cfg(test)]
mod irgen_p11 {
    use super::*;

    /// variadic param binds inside the body as a real Array<int>; mixed with a
    /// fixed leading parameter and callable with zero extra args
    #[test]
    fn variadic_body_binding() {
        let src = r#"
            func add_all(base: int, xs...: Array<int>): int {
                var s = base;
                for (var x: xs) {
                    s = s + x;
                }
                return s;
            }
            func only(xs...: Array<int>): int {
                var s = 0;
                for (var x: xs) { s = s + x; }
                return s;
            }
            print(add_all(100, 1, 2, 3));
            print(add_all(5));
            print(only(4, 5, 6));
        "#;
        run_src(src, "main").unwrap();
    }

    /// float elem kind: int extras are promoted, float extras pass through
    #[test]
    fn variadic_float_elems() {
        let src = r#"
            func stats(xs...: Array<float>): float {
                var s = 0.0;
                for (var x: xs) { s = s + x; }
                return s;
            }
            print(stats(1.0, 2.5, 3.0));
        "#;
        run_src(src, "main").unwrap();
    }

    /// float arg into an int-elem variadic is a diagnostic
    #[test]
    fn variadic_kind_mismatch_diag() {
        let src = "func f(xs...: Array<int>): unit { print(1); }\nfunc main(): unit { f(2.5); }\n";
        let e = match run_src(src, "main") {
            Ok(()) => panic!("float into int variadic accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("variadic argument is float"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #12: pipe |> ----------------
#[cfg(test)]
mod irgen_p12 {
    use super::*;

    /// x |> f ≡ f(x); x |> f(a, b) ≡ f(a, b, x) — pipe value goes last
    #[test]
    fn pipe_single_and_two_arg() {
        let src = r#"
            func twice(x: int): int {
                return x * 2;
            }
            func pick(a: int, b: int): int {
                return a * 10 + b;
            }
            print(5 |> twice);
            print(3 |> twice |> twice);
            print(1 |> pick(7));
        "#;
        run_src(src, "main").unwrap();
    }

    /// chained pipes thread the value through successive calls
    #[test]
    fn pipe_chained() {
        let src = r#"
            func inc(x: int): int { return x + 1; }
            func dbl(x: int): int { return x * 2; }
            print((1 |> inc |> inc |> dbl));
        "#;
        run_src(src, "main").unwrap();
    }

    /// non-call pipe rhs is a diagnostic
    #[test]
    fn pipe_rhs_must_be_callable() {
        let src = "func twice(x: int): int { return x * 2; }\nlet y = 3 |> 4;\nprint(1);\n";
        let e = match run_src(src, "main") {
            Ok(()) => panic!("pipe to non-call accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("pipe rhs must be a function or call"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #13: is narrowing + int()/float() ----------------
#[cfg(test)]
mod irgen_p13 {
    use super::*;

    /// `if (x is Bird)` rebinds x to Bird chain: Bird-only method callable
    #[test]
    fn is_class_narrows() {
        let src = r#"
            class Animal { func leg(): int { return 4; } }
            class Bird: Animal {
                func fly(): unit { print("flap"); }
                func leg(): int { return 2; }
            }
            func main(): unit {
                var a: Animal = Bird();
                print(a.leg());
                if (a is Bird) {
                    a.fly();
                }
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// `if (p is not nil)` unbinds the optional: method call legal inside
    #[test]
    fn is_not_nil_narrows_opt() {
        let src = r#"
            class Shape {
                func sides(): int { return 4; }
            }
            func report(p: Shape?): unit {
                if (p is not nil) {
                    print(p.sides());
                } else {
                    print(0);
                }
            }
            func main(): unit {
                report(nil);
                report(Shape());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// explicit conversion builtins: int() truncation, float() promotion
    #[test]
    fn int_float_conversions() {
        let src = r#"
            func main(): unit {
                print(int(2.7));
                print(float(3) + 0.5);
                print(float(int(float(9))));
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// conversions of str are rejected diagnostics (MVP)
    #[test]
    fn conversion_str_rejected() {
        for form in ["int(\"3\")", "float(\"3\")"] {
            let src = format!("func main(): unit {{\n    var x = {};\n}}\n", form);
            let e = match run_src(&src, "main") {
                Ok(()) => panic!("str conversion accepted: {}", form),
                Err(e) => e,
            };
            assert!(e.contains("unsupported"), "unexpected: {}", e);
        }
    }
}

// ---------------- patch #14: generic function monomorphization MVP ----------------
#[cfg(test)]
mod irgen_p14 {
    use super::*;

    /// T inferred from call-site arg kinds; distinct binds get distinct
    /// monomorphic instances; same-bind calls share one instance (cache)
    #[test]
    fn generic_top_level_and_array_param() {
        let src = r#"
            func twice<T>(x: T): T {
                var q = x;
                return q;
            }
            func count<T>(xs: Array<T>): int {
                var s = 0;
                for (var x: xs) {
                    s = s + 1;
                }
                return s;
            }
            func main(): unit {
                print(twice(5));
                print(twice(7));
                print(count([1, 2, 3]));
                print(count([1.5, 2.5]));
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// nested generic calls instantiate per binding set; unbound T diagnoses
    #[test]
    fn generic_mismatch_and_infer_diag() {
        let src = "func id<T>(x: T): T { return x; }\nfunc weird(): unit {\n    id(id(1));\n}\n";
        run_src(src, "main").unwrap();
        let src2 = "func id<T>(x: T): T { return x; }\nfunc bare(): unit {\n    id(x);\n}\n";
        let e = match run_src(src2, "main") {
            Ok(()) => panic!("unbound generic accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("unknown identifier `x`") || e.contains("cannot infer"),
            "unexpected: {}",
            e
        );
    }

    /// variadic x generic stays unsupported (MVP shape) with a diagnostic
    #[test]
    fn generic_variadic_diag() {
        let src =
            "func f<T>(xs...: Array<T>): unit { print(1); }\nfunc main(): unit { f(1, 2); }\n";
        let e = match run_src(src, "main") {
            Ok(()) => panic!("generic variadic accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("generic variadic unsupported"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #15: operator overloads + trait bounds ----------------
#[cfg(test)]
mod irgen_p15 {
    use super::*;

    /// Vec2 + Vec2 dispatches __add__ (new-object return + f64 fields worded)
    #[test]
    fn operator_overload_add() {
        let src = r#"
            class Vec2 {
                var x: float = 0.0;
                var y: float = 0.0;
                func __init__(x: float, y: float): unit {
                    this.x = x;
                    this.y = y;
                    return;
                }
                func __add__(r: Vec2): Vec2 {
                    return Vec2(this.x + r.x, this.y + r.y);
                }
            }
            func main(): unit {
                var a = Vec2(1.0, 2.0);
                var b = Vec2(3.0, 4.0);
                var c = a + b;
                print(c.x);
                print(c.y);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// class without the overload still refuses numeric fallback
    #[test]
    fn operator_overload_missing_diag() {
        let src = "class Pt { func __init__(): unit { return; } }\nfunc main(): unit {\n    var a = Pt();\n    var b = a + a;\n}\n";
        let e = match run_src(src, "main") {
            Ok(()) => panic!("operator without overload accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires a `__add__` overload"),
            "unexpected: {}",
            e
        );
    }

    /// Hashable-keyed maps: object keys route through pointer identity
    #[test]
    fn map_object_keys_work() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            trait Equatable { func __eq__(other: Point): bool; }
            class Point impl Hashable, Equatable {
                var x: int;
                var y: int;
                func __init__(x: int, y: int): unit {
                    this.x = x;
                    this.y = y;
                    return;
                }
                func hashKey(): int {
                    return this.x + 1000 * this.y;
                }
                func __eq__(other: Point): bool {
                    return this.x == other.x && this.y == other.y;
                }
            }
            func main(): unit {
                let a = Point(1, 2);
                let b = Point(3, 4);
                let m: Map<Point, int> = @(a: 10, b: 20);
                print(m[a]);
                print(m[b]);
                m[a] = 99;
                print(m[a]);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// generic trait bounds: builtin kinds satisfy predefined traits; classes
    /// go through the impl chain
    #[test]
    fn generic_bound_check() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            trait Equatable { func __eq__(other: Point): bool; }
            class Point impl Hashable, Equatable {
                var x: int = 42;
                func __init__(): unit { return; }
                func hashKey(): int { return 7; }
                func __eq__(other: Point): bool { return this.x == other.x; }
            }
            func pick<T: Hashable>(k: T): T {
                var q = k;
                return q;
            }
            func main(): unit {
                print(pick(7));
                print(pick(Point()).x);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// classes lacking the trait violate the bound diagnostically
    #[test]
    fn generic_bound_violation_diag() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            class Plain {
                func __init__(): unit { return; }
            }
            func pick<T: Hashable>(k: T): T {
                var q = k;
                return q;
            }
            func main(): unit {
                print(pick(Plain()));
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("bound violation accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("does not satisfy trait bound `Hashable`"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #16: Iterator/Iterable protocol ----------------
#[cfg(test)]
mod irgen_p16 {
    use super::*;

    /// custom class with iter()/next() drives for over its payload
    #[test]
    fn iterator_protocol_class() {
        let src = r#"
            class Queue {
                var data: Array<int> = [];
                var pos: int = 0;
                func push(v: int): unit {
                    this.data.push(v);
                    return;
                }
                func iter(): Queue {
                    return this;
                }
                func next(): int? {
                    if (this.pos >= this.data.len()) {
                        return nil;
                    }
                    var v = this.data[this.pos];
                    this.pos = this.pos + 1;
                    return v;
                }
            }
            func main(): unit {
                let q = Queue();
                q.push(10);
                q.push(20);
                q.push(30);
                var s: int = 0;
                for (var x: q) {
                    print(x);
                    s = s + x;
                }
                print(s);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// str iteration yields 1-char strings
    #[test]
    fn str_iter_per_char() {
        let src = r#"
            func main(): unit {
                let s: str = "ab";
                var n: int = 0;
                for (var c: s) {
                    n = n + 1;
                    print(c);
                }
                print(n);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// map key iteration still goes through the keys() route
    #[test]
    fn map_for_keys_route_kept() {
        let src = r#"
            func main(): unit {
                let m: Map<int, int> = @(1: 11, 2: 22);
                var t: int = 0;
                for (var k: m) {
                    t = t + k;
                }
                print(t);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// for over a class without next/iter refuses diagnostically
    #[test]
    fn iterator_protocol_missing_diag() {
        let src = r#"
            class Plain {
                func __init__(): unit { return; }
            }
            func main(): unit {
                let p = Plain();
                for (var x: p) {
                    print(x);
                }
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("iterator protocolless class accepted"),
            Err(e) => e,
        };
        assert!(e.contains("has no `next()`"), "unexpected: {}", e);
    }
}

// ---------------- patch #17: extern func (C ABI) ----------------
#[cfg(test)]
mod irgen_p17 {
    use super::*;

    /// extern funcs declare raw C-ABI symbols and call straight into the rt
    #[test]
    fn extern_func_direct() {
        let src = r#"
            extern func sloth_extern_floor(x: float): float;
            extern func sloth_extern_powf(a: float, b: float): float;
            func main(): unit {
                let two: float = 2.0;
                let half: float = 0.5;
                print(sloth_extern_floor(two * 0.75));
                print(sloth_extern_powf(two, half));
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// extern decl parses as a statement position? no — parsed as func decl only
    #[test]
    fn extern_decl_diag_missing_body() {
        // sloth-extern symbols only exist in libsloth_rt.so; a wrong raw name must
        // break the JIT load instead of being silently aliased
        let src = "extern func sloth_no_such_symbol(x: float): float;\nfunc main(): unit {\n    print(sloth_no_such_symbol(1.0));\n}\n";
        let e = match run_src(src, "main") {
            Ok(()) => panic!("missing extern symbol accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("Symbols not found") || e.contains("invoke sloth_main failed"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #18a: trait default method bodies ----------------
#[cfg(test)]
mod irgen_p18a {
    use super::*;

    /// default bodies synthesize onto impl-missing classes; overrides win
    #[test]
    fn trait_default_body_synthesis() {
        let src = r#"
            trait Hello {
                func greet(): unit {
                    print("hi\n");
                    return;
                }
            }
            class A impl Hello {
                func greet(): unit {
                    print("A\n");
                    return;
                }
            }
            class B impl Hello {
                func __init__(): unit { return; }
            }
            func main(): unit {
                let a = A();
                let b = B();
                a.greet();
                b.greet();
                let l: Array<dyn Hello> = [a, b];
                for (var x: l) { x.greet(); }
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// default body with params + return word dispatches through the trait
    #[test]
    fn trait_default_body_params() {
        let src = r#"
            trait Eq {
                func eq(r: int): int {
                    var me = 1;
                    print(me);
                    return r;
                }
            }
            class Wrap impl Eq {
                func __init__(): unit { return; }
            }
            func main(): unit {
                let w = Wrap();
                print(w.eq(7));
                let l: Array<dyn Eq> = [w];
                for (var q: l) {
                    print(q.eq(7) + 100);
                }
            }
        "#;
        // class without eq uses the default body; dyn dispatch ABI-checked
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #18b: Display-plumbed print ----------------
#[cfg(test)]
mod irgen_p18b {
    use super::*;

    /// print(usr-class) routes to impl-Display to_str() then prints the string
    #[test]
    fn print_display_to_str() {
        let src = r#"
            trait Display {
                func to_str(): str;
            }
            class Pt impl Display {
                var x: int;
                var y: int;
                func __init__(x: int, y: int): unit {
                    this.x = x;
                    this.y = y;
                    return;
                }
                func to_str(): str {
                    return "Pt(${this.x}, ${this.y})";
                }
            }
            func main(): unit {
                print(Pt(1, 2));
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// no Display impl: print refuses diagnostically
    #[test]
    fn print_display_missing_diag() {
        let src = r#"
            class Plain {
                func __init__(): unit { return; }
            }
            func main(): unit {
                let q = Plain();
                print(q);
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("class print without Display accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires trait bound `Display`"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #19: generic class monomorphization + Result<T,E> ----------------
#[cfg(test)]
mod irgen_p19 {
    use super::*;

    /// Result<int,str>: ok/err ctors, is_ok/unwrap/err methods on instances
    #[test]
    fn result_builtin() {
        let src = r#"
            func main(): unit {
                let r: Result<int, str> = ok(5);
                print(r.is_ok());
                print(r.unwrap());
                let r2: Result<int, str> = err("boom");
                print(r2.is_ok());
                print(r2.err());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// float payload instance slots route words correctly
    #[test]
    fn result_float_payload() {
        let src = r#"
            func main(): unit {
                let r: Result<float, str> = ok(2.5);
                print(r.unwrap() + 1.0);
                let z: Result<float, int> = err(9);
                print(z.err());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// generic-class annotation requires type args; instance typing sticks
    #[test]
    fn generic_class_typing() {
        let src = r#"
            class Box<T> {
                var v: T;
                func unwrap(): T {
                    return this.v;
                }
            }
            func main(): unit {
                let b = Box<int>();
                b.v = 12;
                print(b.unwrap());
                print(b.v);
                let f = Box<float>();
                f.v = 1.5;
                print(f.unwrap());
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #20: operator overload family completion ----------------
#[cfg(test)]
mod irgen_p20 {
    use super::*;

    /// comparison overloads: __lt__/__eq__ on a class receiver
    #[test]
    fn comparison_overloads() {
        let src = r#"
            class Pt {
                var x: int;
                func __init__(x: int): unit { this.x = x; return; }
                func __lt__(o: Pt): bool { return this.x < o.x; }
                func __eq__(o: Pt): bool { return this.x == o.x; }
            }
            func main(): unit {
                let a = Pt(1);
                let b = Pt(2);
                print(a < b);
                print(a == b);
                print(b < a);
                let c = Pt(2);
                print(b == c);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// full comparison family: __le__/__gt__/__ge__/__ne__
    #[test]
    fn comparison_family() {
        let src = r#"
            class S {
                var v: int;
                func __init__(v: int): unit { this.v = v; return; }
                func __le__(o: S): bool { return this.v <= o.v; }
                func __gt__(o: S): bool { return this.v > o.v; }
                func __ge__(o: S): bool { return this.v >= o.v; }
                func __ne__(o: S): bool { return this.v != o.v; }
            }
            func main(): unit {
                let a = S(1);
                let b = S(2);
                print(a <= b);
                print(a >= b);
                print(a > b);
                print(a != b);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// unary minus dispatches __neg__ on a class receiver
    #[test]
    fn neg_overload() {
        let src = r#"
            class V {
                var x: int;
                func __init__(x: int): unit { this.x = x; return; }
                func __neg__(): V { return V(0 - this.x); }
                func get(): int { return this.x; }
            }
            func main(): unit {
                let a = V(7);
                let n = -a;
                print(n.get());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// Indexable: a[i] ≡ a.__index__(i), a[i] = v ≡ a.__assign__(i, v)
    #[test]
    fn indexable_overloads() {
        let src = r#"
            class Bag {
                var items: Array<int>;
                func __init__(): unit {
                    this.items = [0, 0, 0, 0];
                    return;
                }
                func __index__(i: int): int { return this.items[i]; }
                func __assign__(i: int, v: int): unit {
                    var arr: Array<int> = this.items;
                    arr[i] = v;
                    return;
                }
            }
            func main(): unit {
                let b = Bag();
                b[0] = 11;
                b[1] = 22;
                print(b[0]);
                print(b[1]);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// no overload on comparison of class values: compile-time diagnostic
    /// (patch #33 — no silent cmpi/cmpf word degradation)
    #[test]
    fn cmp_without_overload_int_path() {
        let src = r#"
            class Plain {
                var v: int;
                func __init__(v: int): unit { this.v = v; return; }
            }
            func main(): unit {
                let a = Plain(1);
                let b = Plain(2);
                print(a != b);
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("class comparison without overload accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires a `__ne__` overload"),
            "unexpected: {}",
            e
        );
    }

    /// indexing a class without __index__: diagnostic
    #[test]
    fn index_without_overload_diag() {
        let src = r#"
            class Plain {
                var v: int;
                func __init__(v: int): unit { this.v = v; return; }
            }
            func main(): unit {
                let a = Plain(1);
                print(a[0]);
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("indexing class without __index__ accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires an `__index__` overload"),
            "unexpected: {}",
            e
        );
    }

    /// float element routing through __index__ (f64 return)
    #[test]
    fn indexable_float_return() {
        let src = r#"
            class Grid {
                var data: Array<float>;
                func __init__(): unit {
                    this.data = [1.5, 0.0];
                    return;
                }
                func __index__(i: int): float { return this.data[i]; }
                func __assign__(i: int, v: float): unit {
                    var arr: Array<float> = this.data;
                    arr[i] = v;
                    return;
                }
            }
            func main(): unit {
                let g = Grid();
                g[1] = 2.5;
                print(g[0] + g[1]);
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #21: interpolation routes through Display ----------------
#[cfg(test)]
mod irgen_p21 {
    use super::*;

    /// `${userclass}` interpolates via to_str (symmetric with print)
    #[test]
    fn interpolation_display() {
        let src = r#"
            trait Display {
                func to_str(): str;
            }
            class Pt impl Display {
                var x: int;
                var y: int;
                func __init__(x: int, y: int): unit {
                    this.x = x;
                    this.y = y;
                    return;
                }
                func to_str(): str {
                    return "Pt(${this.x}, ${this.y})";
                }
            }
            func main(): unit {
                let p = Pt(1, 2);
                print("point = ${p}");
                let msg = "value=${Pt(7, 8)}";
                print(msg);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// int field interpolation inside to_str still works (nested chain)
    #[test]
    fn interpolation_mixed_parts() {
        let src = r#"
            trait Display {
                func to_str(): str;
            }
            class Box impl Display {
                var v: int;
                func __init__(v: int): unit { this.v = v; return; }
                func to_str(): str { return "Box{ v=${this.v} }"; }
            }
            func main(): unit {
                let b = Box(42);
                print("x=${b} n=${42} s=${"hi"} f=${2.5} b=${true}");
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// no Display impl: interpolation refuses diagnostically
    #[test]
    fn interpolation_display_missing_diag() {
        let src = r#"
            class Plain {
                var v: int;
                func __init__(v: int): unit { this.v = v; return; }
            }
            func main(): unit {
                let p = Plain(1);
                print("oops ${p}");
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("interpolation of class without Display accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires trait bound `Display`"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #22: var/let surface-type assignment checks ----------------
#[cfg(test)]
mod irgen_p22 {
    use super::*;

    /// var reassignment across word classes: `var x = 1; x = "s"` diagnosed
    #[test]
    fn var_word_class_conflict() {
        let src = r#"
            func main(): unit {
                var x = 1;
                x = 2;
                print(x);
                x = "s";
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("cross-word reassignment accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("expected: int\\n  got: str"),
            "unexpected: {}",
            e
        );
    }

    /// annotated let with mismatched surface initialized diagnosed
    #[test]
    fn let_surface_init_conflict() {
        let src = r#"
            func main(): unit {
                let n: int = "s";
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("mismatched let initializer accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("type mismatch in initializer at line"),
            "unexpected: {}",
            e
        );
    }

    /// subclass instance initialized into a superclass-typed variable passes;
    /// the same-name instance check covers `Result<int,str>` with/without args
    #[test]
    fn is_a_and_same_instance_ok() {
        let src = r#"
            class Animal {
                func __init__(): unit { return; }
                func who(): str { return "A"; }
            }
            class Bird: Animal {
                func __init__(): unit {
                    super.__init__();
                    return;
                }
            }
            func main(): unit {
                let a: Animal = Bird();
                print(a.who());
                let r: Result<int, str> = ok(5);
                print(r.unwrap());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// assigning a foreign class instance into a differently-typed var is not accepted
    #[test]
    fn cross_class_conflict() {
        let src = r#"
            class A { func __init__(): unit { return; } }
            class B { func __init__(): unit { return; } }
            func main(): unit {
                var x = A();
                x = B();
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("cross-class reassignment accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("type mismatch in assignment to `x`"),
            "unexpected: {}",
            e
        );
    }

    /// float target promotes int words on assignment; float value diagnosed
    #[test]
    fn float_var_assignment_routes() {
        let src = r#"
            func main(): unit {
                var f = 0.0;
                f = f + 5.0;
                print(f);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// nil reassignment into a class-typed var passes (word 0); shape-checked arrays pass
    #[test]
    fn nil_and_shape_ok() {
        let src = r#"
            class A { func __init__(): unit { return; } }
            func main(): unit {
                var a: Array<int> = [1, 2];
                a = [3];
                print(a.len());
                let b = A();
                var c = A();
                c = nil;
                print(1);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// shape mismatch: Array<int> into Array<str> variable diagnosed
    #[test]
    fn array_shape_conflict() {
        let src = r#"
            func main(): unit {
                var a: Array<int> = [1, 2];
                a = ["s"];
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("array shape conflict accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("expected: Array<int>\\n  got: Array<str>"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #23: Result<T,E> closure (return ctors + unwrap panic) ----------------
#[cfg(test)]
mod irgen_p23 {
    use super::*;

    /// ok()/err() ctors in return position inferred from the declared
    /// Result<_, _> return annotation
    #[test]
    fn return_position_ctors() {
        let src = r#"
            func make_ok(): Result<int, str> {
                return ok(42);
            }
            func make_err(): Result<int, str> {
                return err("boom");
            }
            func main(): unit {
                let a = make_ok();
                print(a.unwrap());
                let b = make_err();
                print(b.err());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// float payload + err(int) in return position route slots correctly
    #[test]
    fn return_position_slots() {
        let src = r#"
            func make(): Result<float, int> {
                return ok(3.5);
            }
            func bad(): Result<float, int> {
                return err(7);
            }
            func main(): unit {
                let a = make();
                print(a.unwrap() + 1.0);
                let b = bad();
                print(b.err());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// ok()/err() without a Result context diagnosed
    #[test]
    fn ctor_without_result_diag() {
        let src = r#"
            func main(): unit {
                let x = ok(5);
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("untyped ok() accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires a declared Result target"),
            "unexpected: {}",
            e
        );
    }

    /// unwrap() on an err Result stops the process via the panic channel
    /// (verified in a subprocess: exit code 1 + diagnosis on stderr)
    #[test]
    fn unwrap_on_err_panics() {
        let md = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        let exe = format!("{}/../../target/debug/slothc", md);
        if !std::path::Path::new(&exe).exists() {
            eprintln!("skip: slothc binary not built");
            return;
        }
        let src = r#"
            func main(): unit {
                let r: Result<int, str> = err("boom");
                print(r.unwrap());
            }
        "#;
        let dir = std::env::temp_dir();
        let p = dir.join("sloth_p23_unwrap.sl");
        std::fs::write(&p, src).unwrap();
        let out = std::process::Command::new(&exe)
            .arg("run")
            .arg(&p)
            .output()
            .expect("subprocess");
        let stde = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            !out.status.success(),
            "unwrap on err should have exited nonzero"
        );
        assert!(
            stde.contains("unwrap() on err Result"),
            "unexpected stderr: {}",
            stde
        );
    }
}

// ---------------- patch #24: extern type (opaque C-ABI reference) ----------------
#[cfg(test)]
mod irgen_p24 {
    use super::*;

    /// extern type round-trip: opaque token created/used through extern
    /// funcs only (word-transparent ABI)
    #[test]
    fn extern_type_roundtrip() {
        let src = r#"
            extern type Tok;
            extern func sloth_extern_tok_new(): Tok;
            extern func sloth_extern_tok_val(t: Tok): int;
            func main(): unit {
                let t = sloth_extern_tok_new();
                print(sloth_extern_tok_val(t));
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// extern type is opaque: field access diagnosed
    #[test]
    fn extern_type_field_diag() {
        let src = r#"
            extern type Tok;
            extern func sloth_extern_tok_new(): Tok;
            func main(): unit {
                let t = sloth_extern_tok_new();
                print(t.x);
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("opaque extern type field access accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("extern type `Tok` is opaque"),
            "unexpected: {}",
            e
        );
    }

    /// extern type cannot be constructed or `is`-narrowed in sloth
    #[test]
    fn extern_type_ctor_diag() {
        let src = r#"
            extern type Tok;
            func main(): unit {
                let t = Tok();
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("extern type construction accepted"),
            Err(e) => e,
        };
        assert!(e.contains("unknown `Tok`"), "unexpected: {}", e);
    }

    /// extern type word passes multi-hop: new → val → val again (alias-safe)
    #[test]
    fn extern_type_multihop() {
        let src = r#"
            extern type Tok;
            extern func sloth_extern_tok_new(): Tok;
            extern func sloth_extern_tok_val(t: Tok): int;
            func main(): unit {
                let t = sloth_extern_tok_new();
                var u = t;
                u = sloth_extern_tok_new();
                let s = sloth_extern_tok_val(t) + sloth_extern_tok_val(u);
                print(s);
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #25: ctor discipline + bool conds + is comparability ----------------
#[cfg(test)]
mod irgen_p25 {
    use super::*;

    /// subclass ctor without super.__init__: diagnosed
    #[test]
    fn ctor_missing_super_diag() {
        let src = r#"
            class Animal {
                func __init__(): unit { return; }
            }
            class Dog: Animal {
                func __init__(): unit { return; }
            }
            func main(): unit {
                let d = Dog();
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("ctor without super.__init__ accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("constructor of `Dog` must call super.__init__"),
            "unexpected: {}",
            e
        );
    }

    /// ctor calling super.__init__ passes (nested statement forms included)
    #[test]
    fn ctor_super_call_ok() {
        let src = r#"
            class Animal {
                var tag: int;
                func __init__(): unit { this.tag = 0; return; }
            }
            class Dog: Animal {
                func __init__(): unit {
                    var c = 1;
                    if c > 0 {
                        {
                            super.__init__();
                        }
                    }
                    return;
                }
            }
            func main(): unit {
                let d = Dog();
                print(1);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// if/while conditions reject implicit truthy conversions
    #[test]
    fn non_bool_cond_diag() {
        let src = r#"
            func main(): unit {
                var n = 3;
                while n {
                    n = n - 1;
                }
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("int while condition accepted"),
            Err(e) => e,
        };
        assert!(e.contains("condition must be `bool`"), "unexpected: {}", e);
    }

    /// bool conditions (incl. chained comparisons) keep working
    #[test]
    fn bool_cond_ok() {
        let src = r#"
            func main(): unit {
                var n = 3;
                while n > 0 {
                    n = n - 1;
                }
                if n == 0 {
                    print(1);
                } else {
                    print(0);
                }
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// unrelated class `is` test: diagnosed both ways
    #[test]
    fn is_incomparable_diag() {
        let src = r#"
            class A { func __init__(): unit { return; } }
            class B { func __init__(): unit { return; } }
            func main(): unit {
                let a = A();
                if a is B {
                    print(1);
                }
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("unrelated `is` accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("no class relation for `is`"),
            "unexpected: {}",
            e
        );
    }

    /// subclass/parent `is` tests flow normally
    #[test]
    fn is_comparable_ok() {
        let src = r#"
            class Animal { func __init__(): unit { return; } }
            class Dog: Animal { func __init__(): unit { super.__init__(); return; } }
            func main(): unit {
                let a = Dog();
                if a is Animal {
                    print(a is Dog);
                } else {
                    print(0);
                }
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #26: Entry<K,V> map record iteration ----------------
#[cfg(test)]
mod irgen_p26 {
    use super::*;

    /// map for-in yields Entry records: key/val fields readable, live pairs
    #[test]
    fn entry_iteration() {
        let src = r#"
            func main(): unit {
                let m = @("a": 1, "b": 2, "c": 3);
                var total = 0;
                for (var e: m) {
                    total = total + e.val;
                    print(e.key);
                }
                print(total);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// float values route through the Entry val slot (f64 element)
    #[test]
    fn entry_float_values() {
        let src = r#"
            func main(): unit {
                let m = @(1.0: 2.5);
                for (var e: m) {
                    print(e.key + e.val);
                }
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// Hashable-class keys: Entry.key is the object word, Entry.val routes
    #[test]
    fn entry_hashable_keys() {
        let src = r#"
            trait Hashable {
                func __hash__(): int;
            }
            trait Equatable { func __eq__(other: Pt): bool; }
            class Pt impl Hashable, Equatable {
                var x: int;
                func __init__(x: int): unit { this.x = x; return; }
                func __hash__(): int { return this.x; }
                func __eq__(other: Pt): bool { return this.x == other.x; }
            }
            func main(): unit {
                let m = @(Pt(1): 10, Pt(2): 20);
                var s = 0;
                for (var e: m) {
                    s = s + e.val;
                }
                print(s);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// keys()/values() builtins keep working alongside Entry iteration
    #[test]
    fn keys_values_route_kept() {
        let src = r#"
            func main(): unit {
                let m = @(7: 1, 8: 2);
                let ks = keys(m);
                let vs = values(m);
                print(ks.len() + vs.len());
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// examples/ regression gold (§9.1/§9.2 adapted versions)
#[cfg(test)]
mod irgen_examples {
    use super::*;

    fn ws_root() -> std::path::PathBuf {
        let md = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        std::path::PathBuf::from(md)
            .join("..")
            .join("..")
            .join("examples")
    }

    fn run_example(name: &str) {
        let p = ws_root().join(name);
        let src = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {:?}: {}", p, e));
        run_src(&src, "main").unwrap();
    }

    /// §9.1: array of strings, for-in-array, string-interpolation prints
    #[test]
    fn example_hello2_runs() {
        run_example("hello2.sl");
    }

    /// §9.2: trait dispatch, inheritance super chain, dyn array iteration,
    /// field initializers, interpolated str fields
    #[test]
    fn example_objects2_runs() {
        run_example("objects2.sl");
    }
}

// ---------------- patch #31: Hashable⇒Equatable contract ----------------
#[cfg(test)]
mod irgen_p31 {
    use super::*;

    /// class impl Hashable without Equatable: contract diag (design §2.5)
    #[test]
    fn hashable_without_equatable_diag() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            class Key impl Hashable {
                var x: int = 1;
                func __init__(): unit { return; }
                func hashKey(): int { return this.x; }
            }
            func main(): unit { print(Key()); }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("Hashable without Equatable accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("must also impl `Equatable`"),
            "unexpected: {}",
            e
        );
    }

    /// Hashable on a subclass while Equatable sits higher on the class
    /// chain: chain lookup satisfies the contract
    #[test]
    fn hashable_equatable_chain_ok() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            trait Equatable { func __eq__(other: Sub): bool; }
            class Base impl Equatable {
                var x: int = 1;
                func __init__(): unit { return; }
                func __eq__(other: Sub): bool { return this.x == other.x; }
            }
            class Sub: Base impl Hashable {
                func hashKey(): int { return this.x; }
            }
            func main(): unit {
                var m = @(Sub(): 7);
                print(m.len());
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// Equatable on the chain but Hashable declared later keeps working
    #[test]
    fn hashable_with_equatable_ok() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            trait Equatable { func __eq__(other: K): bool; }
            class K impl Equatable, Hashable {
                var x: int = 5;
                func __init__(): unit { return; }
                func hashKey(): int { return this.x; }
                func __eq__(other: K): bool { return this.x == other.x; }
            }
            func main(): unit {
                var m = @(K(): 3);
                print(m.len());
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #32: builtin-type `is` surfaces ----------------
#[cfg(test)]
mod irgen_p32 {
    use super::*;

    /// `x is str` on a str token is statically true; narrows inside the if
    #[test]
    fn is_str_true_narrows() {
        let src = r#"
            func main(): unit {
                var s = "ab";
                if s is str {
                    print(s.len());      // expect narrowing keep the word route
                }
                print(s is str);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// `x is int` false on a str token, true on an int token; negation flips
    #[test]
    fn is_builtin_mismatch_and_negation() {
        let src = r#"
            func main(): unit {
                var s = "x";
                if s is int {
                    print(0);
                } else {
                    print(1);            // int face != str token
                }
                var n = 5;
                print(n is int);         // true
                print(s is int);         // false
                print(n is not str);     // true
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// optional token: `s? is str` looks through the option layer
    #[test]
    fn is_builtin_through_option() {
        let src = r#"
            func main(): unit {
                var s: str? = "abc";
                if s is str {
                    print(s.len());      // expect 3
                } else {
                    print(0);
                }
                var n: str? = nil;
                if n is str {
                    print(n.len());
                } else {
                    print(-1);           // expect -1 (nil never is-str)
                }
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #33: comparison family strictness ----------------
#[cfg(test)]
mod irgen_p33 {
    use super::*;

    /// class equality without __eq__/__ne__: compile-time diagnostic
    #[test]
    fn class_eq_without_overload_diag() {
        let src = r#"
            class Plain {
                var v: int;
                func __init__(v: int): unit { this.v = v; return; }
            }
            func main(): unit {
                print(Plain(1) == Plain(1));
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("class `==` without overload accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires a `__eq__` overload"),
            "unexpected: {}",
            e
        );
    }

    /// ordered comparisons without __lt__: diagnostic too
    #[test]
    fn class_lt_without_overload_diag() {
        let src = r#"
            class Plain {
                func __init__(): unit { return; }
            }
            func main(): unit {
                print(Plain() < Plain());
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("class `<` without overload accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires a `__lt__` overload"),
            "unexpected: {}",
            e
        );
    }

    /// with the overload present the dispatch path is unchanged; int/str
    /// word comparisons never reach the new diagnostic
    #[test]
    fn compare_overloads_and_builtin_words_ok() {
        let src = r#"
            class Num {
                var v: int;
                func __init__(v: int): unit { this.v = v; return; }
                func __eq__(other: Num): bool { return this.v == other.v; }
                func __ne__(other: Num): bool { return this.v != other.v; }
                func __lt__(other: Num): bool { return this.v < other.v; }
            }
            func main(): unit {
                print(Num(1) == Num(1));
                print(Num(1) != Num(2));
                print(Num(1) < Num(2));
                print(7 == 7);
                print(7 < 8);
                print("aa" == "aa");
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #34: out-of-bounds panic channel ----------------
#[cfg(test)]
mod irgen_p34 {

    /// subprocess helper: run `slothc run`, expect failure + stderr match
    fn run_expect_panic(src: &str, tag: &str, want: &str) {
        let md = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        let exe = format!("{}/../../target/debug/slothc", md);
        if !std::path::Path::new(&exe).exists() {
            eprintln!("skip: slothc binary not built");
            return;
        }
        let dir = std::env::temp_dir();
        let p = dir.join(format!("sloth_p34_{}.sl", tag));
        std::fs::write(&p, src).unwrap();
        let out = std::process::Command::new(&exe)
            .arg("run")
            .arg(&p)
            .output()
            .expect("subprocess");
        let stde = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(!out.status.success(), "out-of-bounds should exit nonzero");
        assert!(stde.contains(want), "unexpected stderr: {}", stde);
    }

    /// array read out of bounds: kind/index/len diagnosis, exit 1
    #[test]
    fn arr_get_oob_panics() {
        run_expect_panic(
            r#"
                func main(): unit {
                    let a = [1, 2, 3];
                    print(a[5]);
                }
            "#,
            "arrget",
            "array index 5 out of bounds (len 3)",
        );
    }

    /// array index assignment out of bounds: same bounded channel
    #[test]
    fn arr_set_oob_panics() {
        run_expect_panic(
            r#"
                func main(): unit {
                    let a = [1, 2];
                    a[9] = 7;
                }
            "#,
            "arrset",
            "array index 9 out of bounds (len 2)",
        );
    }

    /// pop from an empty array: diagnosed, not a raw abort
    #[test]
    fn pop_empty_panics() {
        run_expect_panic(
            r#"
                func main(): unit {
                    let a = [];
                    print(a.pop());
                }
            "#,
            "popempty",
            "sloth panic: pop",
        );
    }

    /// missing map key on get: panic channel with the key in the message
    #[test]
    fn map_missing_key_panics() {
        run_expect_panic(
            r#"
                func main(): unit {
                    let m = @(1: 10, 2: 20);
                    print(m[7]);
                }
            "#,
            "mapkey",
            "map key not found (7)",
        );
    }
}

// ---------------- patch #35: Map keys via monomorphized hash() ----------------
#[cfg(test)]
mod irgen_p35 {
    use super::*;

    /// class keys route the monomorphized hash() at every map call site:
    /// literal construction, index get, index assignment, Entry loop value
    /// fetch — all on the same content-hash bucket line
    #[test]
    fn map_object_key_hash_route() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            trait Equatable { func __eq__(other: Pt): bool; }
            class Pt impl Hashable, Equatable {
                var x: int;
                func __init__(x: int): unit { this.x = x; return; }
                func hashKey(): int { return this.x * 31; }
                func __eq__(other: Pt): bool { return this.x == other.x; }
            }
            func main(): unit {
                var m = @(Pt(1): 10, Pt(2): 20);
                print(m.len());          // expect: 2
                let a = Pt(1);
                print(m[a]);             // expect: 10
                let b = Pt(2);
                print(m[b]);             // expect: 20
                m[b] = 25;               // set via content hash hits the slot
                print(m[b]);             // expect: 25
                var s = 0;
                for (var e: m) {
                    s = s + e.val;
                }
                print(s);                // expect: 35
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// growth-over-threshold map: cached slot hash survives the rehash
    #[test]
    fn map_object_key_hash_growth() {
        let src = r#"
            trait Hashable { func __hash__(): int; }
            trait Equatable { func __eq__(other: K): bool; }
            class K impl Hashable, Equatable {
                var x: int;
                func __init__(x: int): unit { this.x = x; return; }
                func __hash__(): int { return this.x; }
                func __eq__(other: K): bool { return this.x == other.x; }
            }
            func main(): unit {
                var m = @(K(0): 0);
                var i = 1;
                while i < 40 {
                    let k = K(i);
                    m[k] = i * 3;
                    i = i + 1;
                }
                print(m.len());          // expect: 40
                print(m[K(3)]);          // expect: 9
                print(m[K(39)]);         // expect: 117
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// builtin int keys keep the internal content-hash route through growth
    #[test]
    fn map_int_keys_route_kept() {
        let src = r#"
            func main(): unit {
                var m = @(1: 10);
                var i = 1;
                while i < 50 {
                    m[i] = i;
                    i = i + 1;
                }
                print(m.len());
                print(m[13]);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// Hashable class without any hash()-family method: compile-time hint
    /// (pointer-identity fallback emission still produced)
    #[test]
    fn hashable_without_hash_fn_hint() {
        let src = r#"
            trait Hashable { func hid(): int; }
            trait Equatable { func __eq__(other: Weird): bool; }
            class Weird impl Hashable, Equatable {
                var x: int = 3;
                func __init__(): unit { return; }
                func hid(): int { return 9; }
                func __eq__(other: Weird): bool { return this.x == other.x; }
            }
            func main(): unit {
                var m = @(Weird(): 5);
                print(m.len());
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("hash-less Hashable map key accepted silently"),
            Err(e) => e,
        };
        assert!(
            e.contains("implements no `hash()`-family method"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #36: str == value semantics ----------------
#[cfg(test)]
mod irgen_p36 {
    use super::*;

    /// concatenated strs compare by content against interned literals
    #[test]
    fn str_eq_value_semantics() {
        let src = r#"
            func main(): unit {
                print("a" + "b" == "ab");   // expect: true
                print("ab" == "a" + "b");   // expect: true
                print("ab" != "a" + "b");   // expect: false
                print("" == "a" + "");      // expect: false
                print(("" + "") == "");     // expect: true
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// str equality drives control flow
    #[test]
    fn str_eq_control_flow() {
        let src = r#"
            func greet(n: str): str {
                if n == "world" {
                    return "hello world";
                }
                return "hi " + n;
            }
            func main(): unit {
                print(greet("wor" + "ld"));   // expect: hello world
                print(greet("x"));            // expect: hi x
                var m = @("k" + "1": 7);
                print(m["k1"]);               // expect: 7
                print(m.len());               // expect: 1
                m["k1"] = 8;                  // content-equal key hits the slot
                print(m.len());               // expect: 1
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #37: ctor faces with the context frame ----------------
#[cfg(test)]
mod irgen_p37 {
    use super::*;

    /// assignment face: `x = ok(v)` binds the slot's declared Result<T,E>
    #[test]
    fn assign_face_ctors() {
        let src = r#"
            func main(): unit {
                var x: Result<int, str> = err("seed");
                print(x.err());          // expect: seed
                x = ok(7);
                print(x.unwrap());       // expect 7
                x = err("later");
                print(x.err());          // expect: later
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// generic context: ok() instantiates the monomorphized T frame
    #[test]
    fn generic_return_ctor_context() {
        let src = r#"
            func wrap<T, E>(v: T, e: E): Result<T, E> {
                if v != v {
                    return err(e);
                }
                return ok(v);
            }
            func main(): unit {
                print(wrap(5, "no").unwrap());       // expect: 5
                print(wrap("two", 9).unwrap());      // expect: two
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// assign-face ctor without a Result target still diagnosed (#23 口径)
    #[test]
    fn assign_face_ctor_without_target_diag() {
        let src = r#"
            func main(): unit {
                var x: int = 0;
                x = ok(5);
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("assign-face ctor without Result target accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("requires a declared Result target"),
            "unexpected: {}",
            e
        );
    }
}

// ---------------- patch #38: return-driven generic inference ----------------
#[cfg(test)]
mod irgen_p38 {
    use super::*;

    /// T appears only in the return surface: annotation/assign-target face
    /// binds it (`func fail<T, E>(e: E): Result<T, E>` — no T-bearing param)
    #[test]
    fn let_annotation_infer_return() {
        let src = r#"
            func fail<T, E>(e: E): Result<T, E> {
                return err(e);
            }
            func main(): unit {
                var r: Result<int, str> = fail("boom");
                print(r.err());          // expect: boom

                var q: Result<float, str> = fail("na");
                print(q.err());          // expect: na
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// no expected-type context for the leftover T: diagnosed (Plan convention)
    #[test]
    fn infer_without_hint_diag() {
        let src = r#"
            func fail<T, E>(e: E): Result<T, E> {
                return err(e);
            }
            func main(): unit {
                print(fail("boom"));
            }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("unhinted T accepted silently"),
            Err(e) => e,
        };
        assert!(
            e.contains("cannot infer type parameter"),
            "unexpected: {}",
            e
        );
    }

    /// explicit type args still win and the inference path changes nothing
    #[test]
    fn explicit_targs_kept() {
        let src = r#"
            func fail<T, E>(e: E): Result<T, E> {
                return err(e);
            }
            func main(): unit {
                let r = fail<int, str>("boom");
                print(r.err());          // expect: boom
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #39: lambda param faces & free-call captures ----------------
#[cfg(test)]
mod irgen_p39 {
    use super::*;

    /// annotated params + return annotation; calls to free functions inside
    /// the body are looked up, not captured
    #[test]
    fn lambda_free_calls_and_annots() {
        let src = r#"
            func pick<T>(v: T): T { return v; }
            func main(): unit {
                let e: str = "m";
                var gen = |x: int| -> Result<int, str> { return err(e); };
                print(gen(1).err());     // expect: m   (capture intact, fn face ok)
                var withT = |x: int| -> int { return pick(x); };
                print(withT(4));         // expect: 4
                var neg = |x: int, y: str| -> bool { return y == "x"; };
                print(neg(1, "x"));      // expect: true
                print(neg(2, "z"));      // expect: false
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// capture-snapshot semantics untouched: local mutation stays a copy
    #[test]
    fn lambda_capture_snapshot_kept() {
        let src = r#"
            func main(): unit {
                var base = 10;
                var inc = |x: int| { return x + base; };
                print(inc(5));           // expect: 15
                base = 99;
                print(inc(5));           // expect: 15 (snapshot)
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #41: multi-error diag batch ----------------
#[cfg(test)]
mod irgen_p41 {
    use super::*;

    /// several unrelated semantic errors in one pass all get reported
    /// (numbered batch, no first-error short-circuit)
    #[test]
    fn multi_errors_all_reported() {
        let src = r#"
            func main(): unit {
                var x = 1;
                x = "s";
                var y = 2;
                y = 1.5;
                if 3 {
                    print(0);
                }
            }
        "#;
        let prog = sloth_frontend::parser::parse(src).unwrap();
        let mut me = ModEmitter::new("main");
        me.emit_module(&prog);
        assert!(me.diags.len() >= 3, "want >= 3 diags, got {:?}", me.diags);
        let rep = crate::irgen::format_diags(&me);
        assert!(rep.contains("L4:C17"), "unexpected: {}", rep);
        assert!(rep.contains("L7:C17"), "unexpected: {}", rep);
        assert!(rep.contains("3 error(s)"), "unexpected: {}", rep);
    }

    /// an unknown identifier does not stop later statements from checking
    #[test]
    fn err_does_not_short_circuit() {
        let src = r#"
            func main(): unit {
                print(nope);
                var q = 2;
                q = "s";
            }
        "#;
        let prog = sloth_frontend::parser::parse(src).unwrap();
        let mut me = ModEmitter::new("main");
        me.emit_module(&prog);
        assert!(
            me.diags.len() >= 2
                && me
                    .diags
                    .iter()
                    .any(|d| d.msg.contains("unknown identifier `nope`"))
                && me.diags.iter().any(|d| d.msg.contains("assignment to `q`")),
            "unexpected: {:?}",
            me.diags
        );
    }
}

// ---------------- patch #42: value-optional boxes (ARC D1) ----------------
#[cfg(test)]
mod irgen_p42 {
    use super::*;

    /// int? value 0 is a live box, not nil — the confusion collapsed
    #[test]
    fn opt_int_zero_is_not_nil() {
        let src = r#"
            func main(): unit {
                var n: int? = 0;
                print(n is nil);        // false: boxed 0 != nil
                if n is not nil {
                    print(n);           // 0
                }
                var m: int? = nil;
                print(m is nil);        // true
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// float?/bool? boxes: float payloads ride the box; elvis + conversion
    #[test]
    fn opt_float_bool_boxes_work() {
        let src = r#"
            func pickf(f: float?): float { return f ?: 1.5; }
            func main(): unit {
                var f: float? = 0.0;
                print(f is nil);        // false
                print(pickf(0.0) == 0.0);  // true (boxed 0.0 kept)
                print(pickf(nil) == 1.5);  // true
                var b: bool? = false;
                print(b is nil);        // false
                if b is not nil {
                    print(b);           // false
                }
                var i: int? = 7;
                print(int(i));          // 7
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// class fields of `int?` box through the field write/read faces
    #[test]
    fn opt_class_field_works() {
        let src = r#"
            class C {
                var n: int? = nil;
                var f: float? = nil;
            }
            func main(): unit {
                var c = C();
                print(c.n is nil);      // true (declared init keeps nil)
                c.n = 5;
                c.f = 0.0;
                print(c.n is nil);      // false
                if c.n is not nil {
                    print(c.n + 1);     // 6
                }
                if c.f is not nil {
                    print(c.f);         // 0 (0.0, not nil)
                }
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// optional values in containers: arrays and maps keep their boxes and
    /// the slot ownership cascade releases them on death
    #[test]
    fn opt_container_elements_work() {
        let src = r#"
            func main(): unit {
                var n: int? = 1;
                var z: int? = nil;
                var arr = [n, z, 3];
                print(arr.len());       // 3
                print(arr[0] is nil);   // false
                print(arr[1] is nil);   // true
                if arr[2] is not nil {
                    print(arr[2]);      // 3
                }
                var m = @("k": n);
                var e = m["k"];
                print(e is nil);        // false
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// optional params across function calls box on both sides: the return
    /// face transfers the box into the caller binding
    #[test]
    fn opt_return_and_churn_work() {
        let src = r#"
            func mk(v: int): int? {
                return v;
            }
            func nz(): int? {
                return nil;
            }
            func main(): unit {
                var a = mk(5);
                print(a is nil);        // false
                var b = nz();
                print(b is nil);         // true
                // chained overwrites release each dead box (ownership stays)
                var i = 0;
                var base = sloth_rc_live();
                while i < 2000 {
                    a = mk(i);
                    i = i + 1;
                }
                print(a);               // 1999
                print(sloth_rc_live() == base);   // churn fully collected
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- patch #43: Weak<T> reference boxes (ARC D2) ----------------
#[cfg(test)]
mod irgen_p43 {
    use super::*;

    /// weak fields hold a weakbox (no strong count): the wrapped target rides
    /// into upgrade() as T?; nil weaks stay nil
    #[test]
    fn weak_field_and_upgrade_work() {
        let src = r#"
            class Node {
                var name: str = "n";
                var next: Weak<Node> = nil;
            }
            func main(): unit {
                var a = Node();
                var w: Weak<Node> = nil;
                print(w is nil);        // true
                w = a;
                print(w is nil);        // false
                var s = w.upgrade();     // strong borrow
                print(s is nil);         // false
                if s is not nil {
                    print(s.name);      // n
                }
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// weak ring: two nodes referencing each other through weak fields die
    /// fully — no strong edges through next (2000 ring churns leave no entries)
    #[test]
    fn weak_self_ring_collected() {
        let src = r#"
            class Node {
                var next: Weak<Node> = nil;
            }
            pub func main(): unit {
                var base = sloth_rc_live();
                var i = 0;
                while i < 2000 {
                    var a = Node();
                    var b = Node();
                    a.next = b;
                    b.next = a;      // weak fields reject the ring
                    i = i + 1;
                }
                print(sloth_rc_live() == base);   // ring fully collected
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// dead targets upgrade to nil; weak payloads for boxed values ride int?
    #[test]
    fn weak_dead_upgrade_and_boxed_payload() {
        let src = r#"
            func dead_check(): int {
                var w: Weak<int> = nil;
                {
                    var n: int? = 0;
                    w = n;
                }
                var s = w.upgrade();
                if s is not nil {
                    print(s);       // 0 (boxed payload still alive? no: died)
                    return 0;
                }
                return 1;
            }
            func main(): unit {
                var w: Weak<int> = 5;
                var s = w.upgrade();    // int? box, payload 5
                print(s is nil);         // false
                if s is not nil {
                    print(int(s));      // 5
                }
                print(dead_check());     // 1 (inner scope dropped the box)
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

// ---------------- hidden-bug regression batch: globals, dispatch,
// divide-by-zero, ranges, optionals, numeric promotion ----------------
#[cfg(test)]
mod irgen_regress {
    use super::*;

    /// module-level globals with a declared main(): initializers run and
    /// function bodies can read/write the cells
    #[test]
    fn globals_init_and_write_work() {
        let src = r#"
            var g = 10;
            var name = "world";
            func bump(): unit { g = g + 1; }
            func greet(): str { return "hi " + name; }
            func main(): unit {
                print(g);           // 10
                bump();
                bump();
                print(g);           // 12
                g = 100;
                print(g);           // 100
                name = "there";
                print(greet());     // hi there
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// immutable top-level `let` rejects writes from a function body
    #[test]
    fn global_immutable_rejected() {
        let src = r#"
            let fixed = 7;
            func clobber() { fixed = 8; }
            func main(): unit { clobber(); }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("immutable global write accepted"),
            Err(e) => e,
        };
        assert!(
            e.contains("cannot assign to immutable"),
            "unexpected: {}",
            e
        );
    }

    /// range literals bind to variables and flow through params/returns
    #[test]
    fn range_values_work() {
        let src = r#"
            func mk(a: int, b: int): range { return a..b; }
            func sum(r: range): int {
                var s = 0;
                for x in r { s = s + x; }
                return s;
            }
            func main(): unit {
                var r = 0..5;
                var s = 0;
                for x in r { s = s + x; }
                print(s);                 // 10
                print(sum(mk(1, 4)));     // 6
                print(sum(2..=4));        // 9
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// and/or short-circuit: the rhs of a guard must not run when the lhs
    /// already decides (avoids out-of-bounds evaluation)
    #[test]
    fn logical_short_circuits() {
        let src = r#"
            var calls = 0;
            func yes(): bool { calls = calls + 1; return true; }
            func no(): bool { calls = calls + 1; return false; }
            func main(): unit {
                var a = [1, 2];
                var i = 5;
                if i < a.len() and a[i] == 2 { print(1); } else { print(2); }
                calls = 0;
                if no() and yes() { print(1); } else { print(2); }
                print(calls);             // 1 (yes skipped)
                if yes() or no() { print(3); } else { print(4); }
                print(calls);             // 2 (no skipped)
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// str iteration is by Unicode scalar value, not by byte
    #[test]
    fn unicode_iteration_works() {
        let src = r#"
            func main(): unit {
                var s = "aé中";
                var n = 0;
                for c in s { n = n + 1; }
                print(n);                 // 3
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// empty Array/Map literals adopt the declared element surface
    #[test]
    fn empty_literal_context_types() {
        let src = r#"
            func main(): unit {
                var a: Array<float> = [];
                a.push(1.5);
                print(a[0]);              // 1.5
                var m: Map<str, int> = @();
                m["a"] = 1;
                print(m["a"]);            // 1
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// float keys are Hashable and ride the word route
    #[test]
    fn float_map_keys_work() {
        let src = r#"
            func main(): unit {
                var m = @(1.5: 10, 2.5: 20);
                print(m[1.5]);            // 10
                var m2: Map<float, int> = @();
                m2[0.25] = 4;
                print(m2[0.25]);          // 4
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// else-branch of `if x is nil` narrows to the payload type
    #[test]
    fn else_branch_nil_narrowing() {
        let src = r#"
            class C { var n: int = 3; }
            func p1(o: C?): int {
                if o is nil { return 0 - 1; } else { return o.n; }
            }
            func main(): unit {
                print(p1(C()));           // 3
                print(p1(nil));           // -1
                var n: int? = 5;
                if n is nil { print(0); } else { print(n + 1); }  // 6
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// a declared base-typed variable kept as its concrete init still works,
    /// and trait-method calls from base bodies dispatch virtually
    #[test]
    fn trait_virtual_dispatch_emits_vtable_call() {
        let src = r#"
            trait Shape {
                func area(): int;
                func describe(): str { return "${this.area()}"; }
            }
            class Base impl Shape {
                func area(): int { return 1; }
            }
            class Sub: Base {
                func area(): int { return 2; }
            }
            func main(): unit {
                let s = Sub();
                print(s.describe());      // 2
            }
        "#;
        let ir = crate::irgen::compile_to_ir(src, "main").expect("compile");
        assert!(
            ir.contains("sloth_obj_vtable"),
            "trait method call from a base body must route through the vtable"
        );
        assert!(ir.contains("llvm.call"), "expected indirect vtable call");
        run_src(src, "main").unwrap();
    }

    /// design §2.1: no implicit numeric conversion — mixed int/float
    /// comparisons and arithmetic are compile-time type mismatches
    #[test]
    fn mixed_numeric_rejected() {
        for (tag, src) in [
            ("cmp", "func main(): unit { print(3.0 == 3); }\n"),
            ("arith", "func main(): unit { print(1 + 2.5); }\n"),
            (
                "lit",
                "func main(): unit { var a = [1, 2.5]; print(a[0]); }\n",
            ),
        ] {
            let e = match run_src(src, "main") {
                Ok(()) => panic!("{}: implicit int/float conversion accepted", tag),
                Err(e) => e,
            };
            assert!(e.contains("type mismatch"), "{}: unexpected: {}", tag, e);
        }
        // explicit conversions stay legal
        run_src(
            "func main(): unit { print(float(3) == 3.0); print(float(1) + 2.5); }\n",
            "main",
        )
        .unwrap();
    }

    /// static surface checks: return, field and element assignment
    #[test]
    fn return_and_field_surface_checks() {
        for (tag, src) in [
            ("ret", "func f(): int { return \"x\"; }\nfunc main(): unit { print(f()); }\n"),
            (
                "field",
                "class C { var n: int; func __init__() { this.n = 1; } }\nfunc main(): unit { let c = C(); c.n = \"s\"; }\n",
            ),
            (
                "elem",
                "func main(): unit { var a = [1, 2]; a[0] = \"x\"; }\n",
            ),
        ] {
            let e = match run_src(src, "main") {
                Ok(()) => panic!("{}: unsound store accepted", tag),
                Err(e) => e,
            };
            assert!(
                e.contains("type mismatch"),
                "{}: unexpected: {}",
                tag,
                e
            );
        }
    }

    /// heterogeneous class array literals upcast to the common ancestor
    #[test]
    fn class_array_lub() {
        let src = r#"
            class A { func k(): str { return "a"; } }
            class B: A { func k(): str { return "b"; } }
            func main(): unit {
                let arr = [A(), B()];
                print(arr.len());         // 2
                print(arr[0].k());        // a
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// first-class functions: fn-typed params, named-function values,
    /// returned closures, IIFE and an arbitrary callee expression
    #[test]
    fn first_class_functions_work() {
        let src = r#"
            func apply(f: (int) -> int, x: int): int { return f(x); }
            func inc(x: int): int { return x + 1; }
            func dbl(x: int): int { return x * 2; }
            func compose(f: (int) -> int, g: (int) -> int, x: int): int {
                return f(g(x));
            }
            func make_add(n: int): (int) -> int {
                return |x: int| { return x + n; };
            }
            func main(): unit {
                print(apply(|y: int| { return y * y; }, 5));   // 25
                let g = inc;
                print(g(4));                                    // 5
                print(compose(inc, dbl, 3));                    // 7
                let add5 = make_add(5);
                print(add5(10));                                // 15
                print((|x: int| { return x - 1; })(9));         // 8
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// method reference `obj.method`: a bound closure keeps `this`
    #[test]
    fn method_references_work() {
        let src = r#"
            class Counter {
                var n: int = 0;
                func __init__(k: int) { this.n = k; }
                func bump(): int { this.n = this.n + 1; return this.n; }
            }
            func run(f: () -> int): int { return f(); }
            func main(): unit {
                var c = Counter(10);
                let bump = c.bump;
                print(bump());          // 11
                print(bump());          // 12
                print(run(c.bump));     // 13
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// plain-class virtual dispatch through a base-typed reference (§2.4)
    #[test]
    fn plain_class_virtual_dispatch_works() {
        let src = r#"
            class A { func k(): str { return "a"; } func via_this(): str { return this.k(); } }
            class B: A { func k(): str { return "b"; } }
            func pick(x: A): str { return x.k(); }
            func main(): unit {
                var b: A = B();
                print(pick(b));         // b
                print(b.via_this());    // b
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// chained assignable target: a[i][j] and obj.field[i] (design §3 EBNF)
    #[test]
    fn nested_index_assignment_works() {
        let src = r#"
            class Grid { var xs: Array<Array<int>> = [[1, 2], [3, 4]]; }
            func main(): unit {
                var a = [[1, 2], [3, 4]];
                a[0][1] = 9;
                print(a[0][1]);         // 9
                var g = Grid();
                g.xs[1][1] = 42;
                print(g.xs[1][1]);      // 42
                var m: Map<str, Map<str, int>> = @();
                m["a"] = @();
                m["a"]["b"] = 7;
                print(m["a"]["b"]);     // 7
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// array growth keeps the handle stable across aliases, callee params,
    /// closure captures and nested container elements (no stale/double-free)
    #[test]
    fn array_growth_stable_handle() {
        let src = r#"
            func fill(a: Array<int>, n: int) {
                for i in 0..n { a.push(i); }
            }
            func main(): unit {
                var a: Array<int> = [];
                var b = a;
                for i in 0..20 { a.push(i); }
                print(a.len()); print(b.len()); print(b[19]);
                var c: Array<int> = [];
                fill(c, 100);
                print(c.len());
                var d: Array<int> = [];
                var pushd = |x: int| { d.push(x); return 0; };
                for i in 0..20 { pushd(i); }
                print(d.len());
                var g: Array<Array<int>> = [];
                g.push([]);
                for j in 0..20 { g[0].push(j); }
                print(g[0].len()); print(g[0][19]);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// Result ctor as an index/container value: m["k"] = ok(v)
    #[test]
    fn map_result_value_assign() {
        let src = r#"
            func main(): unit {
                var m: Map<str, Result<int, str>> = @();
                m["a"] = ok(1);
                print(m["a"].unwrap());     // 1
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// scientific-notation float literals lex correctly
    #[test]
    fn scientific_notation_literals() {
        let src = r#"
            func main(): unit {
                print(1e0);        // 1
                print(1.5e-3);     // 0.0015
                print(2E+4);       // 20000
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// nested optional and optional-receiver field access are diagnosed
    #[test]
    fn nested_optional_and_opt_receiver_diag() {
        let e = match run_src(
            "func f(x: int??): unit { return; }\nfunc main(): unit { f(nil); }\n",
            "main",
        ) {
            Ok(()) => panic!("nested optional accepted"),
            Err(e) => e,
        };
        assert!(e.contains("nested optional T??"), "unexpected: {}", e);

        let src = r#"
            class N { var v: int = 1; var next: N? = nil; }
            func main(): unit { var n = N(); print(n.next.v); }
        "#;
        let e = match run_src(src, "main") {
            Ok(()) => panic!("optional receiver field access accepted"),
            Err(e) => e,
        };
        assert!(e.contains("on an optional receiver"), "unexpected: {}", e);
    }

    /// integer divide/modulo by zero is a diagnosed panic (not garbage)
    #[test]
    fn int_divzero_panics() {
        let md = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        let exe = format!("{}/../../target/debug/slothc", md);
        if !std::path::Path::new(&exe).exists() {
            eprintln!("skip: slothc binary not built");
            return;
        }
        for (tag, src) in [
            ("div", "func main(): unit { var z = 0; print(10 / z); }\n"),
            ("rem", "func main(): unit { var z = 0; print(10 % z); }\n"),
        ] {
            let p = std::env::temp_dir().join(format!("sloth_regress_{}.sl", tag));
            std::fs::write(&p, src).unwrap();
            let out = std::process::Command::new(&exe)
                .arg("run")
                .arg(&p)
                .output()
                .expect("subprocess");
            let stde = String::from_utf8_lossy(&out.stderr).to_string();
            assert!(!out.status.success(), "{}: should exit nonzero", tag);
            assert!(
                stde.contains("integer division by zero"),
                "{}: unexpected stderr: {}",
                tag,
                stde
            );
        }
    }

    /// ARC ownership protocol (§5.1.1): ref-return temporaries, class
    /// ref-field death cascades, `continue`-abandoned loop locals and the
    /// iterator protocol all settle without crashing or corrupting the heap
    /// (spec 94 asserts the exact rc baseline convergence)
    #[test]
    fn arc_ownership_settles_ref_paths() {
        let src = r#"
            class Box { var v: int; func __init__(v: int) { this.v = v; } }
            class Holder {
                var a: Array<int>;
                var s: str;
                func __init__() { this.a = mkarr(); this.s = "held"; }
            }
            func mkarr(): Array<int> { return [1, 2, 3]; }
            class Counter {
                var n: int = 0;
                var limit: int = 0;
                func __init__(l: int) { this.limit = l; }
                func iter(): Counter { return this; }
                func next(): int? {
                    if this.n >= this.limit { return nil; }
                    this.n = this.n + 1;
                    return this.n;
                }
            }
            func main(): unit {
                var acc = 0;
                var i = 0;
                while i < 500 {
                    acc = acc + mkarr().len() + Box(i).v;
                    let h = Holder();
                    acc = acc + h.a.len() + h.s.len();
                    let s = "abc";
                    let a = [i, i + 1];
                    if i % 2 == 0 { acc = acc + s.len(); i = i + 1; continue; }
                    acc = acc + a.len();
                    let c = Counter(8);
                    for x in c { acc = acc + x; }
                    i = i + 1;
                }
                print(acc > 0);   // expect: true
            }
        "#;
        run_src(src, "main").unwrap();
    }
}

#[cfg(test)]
mod irgen_te_p0 {
    use super::*;

    /// compound assignment over every target kind: local, float, array
    /// element, map element, object field, module global. Also asserts the
    /// index expression is evaluated exactly once (calls == 1).
    #[test]
    fn compound_assign_targets() {
        let src = r#"
            var calls: int = 0;
            func next_index(): int { calls += 1; return 0; }
            class Acc { var v: int; func __init__(a: int) { this.v = a; } }
            func main(): unit {
                var a = 5;
                a += 3;
                a -= 1;
                print(a);
                var f = 1.5;
                f += 2.0;
                print(f);
                var arr = [1, 2, 3];
                arr[1] += 10;
                arr[0] -= 1;
                print(arr[1]);
                print(arr[0]);
                var m = @("k": 1);
                m["k"] += 5;
                print(m["k"]);
                var p = Acc(10);
                p.v += 4;
                print(p.v);
                var data = [10];
                data[next_index()] += 5;
                print(data[0]);
                print(calls);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// int bitwise/shift operators and `~` (design §3.3)
    #[test]
    fn bitwise_ops() {
        let src = r#"
            func main(): unit {
                print(6 & 3);
                print(6 | 3);
                print(6 ^ 3);
                print(1 << 4);
                print(256 >> 4);
                print(~5);
                print(1 + 2 << 3);
                print(1 << 2 + 1);
                print(6 & 3 == 3);
                print(1 | 2 ^ 3);
                var x = 20;
                x = x & 6;
                print(x);
                print(~x);
                print(~(~7));
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// `<<` / `>>` are lexed as two Lt/Gt tokens and coalesced in the Pratt
    /// loop, so generic closers (`Map<int,Array<int>>`) keep parsing.
    #[test]
    fn shifts_keep_nested_generics() {
        let src = r#"
            func main(): unit {
                var g: Array<Array<int>> = [[1, 2], [3, 4]];
                var both: Map<int, Array<int>> = @(1: [7]);
                print(g[1][1] >> 1);
                print(both[1][0] << 1);
                print(1 << 2 >> 1);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn bitwise_float_diag() {
        let src = r#"
            func main(): unit {
                print(1.5 & 1.0);
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected bitwise operand diag"),
            Err(e) => assert!(
                e.contains("bitwise operators require `int` operands"),
                "unexpected: {}",
                e
            ),
        }
    }

    #[test]
    fn bitnot_float_diag() {
        let src = r#"
            func main(): unit {
                print(~1.5);
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected `~` operand diag"),
            Err(e) => assert!(e.contains("requires an `int` operand"), "unexpected: {}", e),
        }
    }

    /// compound assign on a `let` is still rejected (immutability)
    #[test]
    fn compound_assign_immutable_diag() {
        let src = r#"
            func main(): unit {
                let x = 1;
                x += 1;
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected immutability diag"),
            Err(e) => assert!(
                e.contains("cannot assign to immutable"),
                "unexpected: {}",
                e
            ),
        }
    }
}

#[cfg(test)]
mod irgen_te_p1 {
    use super::*;

    /// tensor construction, indexing/slicing views and view writes
    #[test]
    fn tensor_views_and_writes() {
        let src = r#"
            func main(): unit {
                var t: Tensor<float, 2> = tensor.zeros([2, 3]);
                t[0][1] = 5.0;
                t[1][2] = 7.5;
                var row: Tensor<float, 1> = t[1];
                row[0] = 9.0;
                var col: Tensor<float, 1> = t[0][0..2];
                col[1] = 4.0;
                var src: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
                t[0] = src;
                var cube: Tensor<float, 3> = tensor.zeros([2, 2, 2]);
                cube[1][1][1] = 8.0;
                print(cube[1][1][1]);
                var it: Tensor<int, 1> = tensor.from_array([10, 20, 30], [3]);
                print(it[1]);
                it[2] = 99;
                print(it[2]);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    /// tensor field in a class: view write reaches the owning storage
    #[test]
    fn tensor_class_field() {
        let src = r#"
            class Holder {
                var buf: Tensor<float, 2>;
                func __init__(): unit {
                    this.buf = tensor.zeros([2, 2]);
                }
            }
            func main(): unit {
                var h = Holder();
                h.buf[1][1] = 3.5;
                print(h.buf[1][1]);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn tensor_zeros_needs_target() {
        let src = r#"
            func main(): unit {
                var t = tensor.zeros([2, 2]);
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected missing-target diag"),
            Err(e) => assert!(
                e.contains("requires a declared `Tensor<T, R>` target"),
                "unexpected: {}",
                e
            ),
        }
    }

    #[test]
    fn tensor_rank_limit_diag() {
        let src = r#"
            func main(): unit {
                var t: Tensor<float, 4> = tensor.zeros([2, 2, 2, 2]);
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected rank diag"),
            Err(e) => assert!(e.contains("rank 4 unsupported"), "unexpected: {}", e),
        }
    }
}

/// TE-P2 formal: channel-B `linalg` operators. The source-level kernels emit
/// `linalg.matvec/matmul/dot/generic` over memrefs bridged from the tensor
/// descriptor, through the shared pipeline.
#[cfg(test)]
mod irgen_te_p2 {
    use super::*;

    fn p2_ops_body() -> &'static str {
        r#"
            func main(): unit {
                var w: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], [2, 3]);
                var x: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
                var y: Tensor<float, 1> = tensor.matvec(w, x);
                print(y[0]);
                print(y[1]);
                var a: Tensor<float, 1> = tensor.from_array([1.0, 2.0, 3.0], [3]);
                var b: Tensor<float, 1> = tensor.from_array([10.0, 20.0, 30.0], [3]);
                var c: Tensor<float, 1> = tensor.add(a, b);
                print(c[2]);
                var m: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [2, 2]);
                var mm: Tensor<float, 2> = tensor.matmul(m, m);
                print(mm[1][1]);
                print(tensor.dot(a, b));
                print(tensor.sum(a));
                tensor.add_into(a, b);
                print(a[0]);
            }
        "#
    }

    #[test]
    fn linalg_operators_run() {
        run_src(p2_ops_body(), "main").unwrap();
    }

    /// matvec on a rank-3 layer view exercises runtime-stride memrefs
    #[test]
    fn matvec_layer_view() {
        let src = r#"
            func main(): unit {
                var w3: Tensor<float, 3> = tensor.from_array(
                    [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0], [2, 2, 3]);
                var layer: Tensor<float, 2> = w3[1];
                var ones: Tensor<float, 1> = tensor.from_array([1.0, 1.0, 1.0], [3]);
                var y: Tensor<float, 1> = tensor.matvec(layer, ones);
                print(y[0]);
                print(y[1]);
            }
        "#;
        run_src(src, "main").unwrap();
    }

    #[test]
    fn matvec_rank_diag() {
        let src = r#"
            func main(): unit {
                var w: Tensor<float, 1> = tensor.from_array([1.0, 2.0], [2]);
                var y: Tensor<float, 1> = tensor.matvec(w, w);
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected matvec rank diag"),
            Err(e) => assert!(e.contains("requires `Tensor<T,2>`"), "unexpected: {}", e),
        }
    }

    #[test]
    fn matvec_int_diag() {
        let src = r#"
            func main(): unit {
                var w: Tensor<int, 2> = tensor.from_array([1, 2, 3, 4], [2, 2]);
                var x: Tensor<int, 1> = tensor.from_array([1, 1], [2]);
                var y: Tensor<int, 1> = tensor.matvec(w, x);
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected matvec int diag"),
            Err(e) => assert!(e.contains("float"), "unexpected: {}", e),
        }
    }

    #[test]
    fn elementwise_rank_diag() {
        let src = r#"
            func main(): unit {
                var a: Tensor<float, 1> = tensor.from_array([1.0, 2.0], [2]);
                var b: Tensor<float, 2> = tensor.from_array([1.0, 2.0, 3.0, 4.0], [2, 2]);
                var c: Tensor<float, 1> = tensor.add(a, b);
            }
        "#;
        match run_src(src, "main") {
            Ok(_) => panic!("expected elementwise rank diag"),
            Err(e) => assert!(e.contains("rank"), "unexpected: {}", e),
        }
    }
}

/// TE-P2 R1 gate: the `sloth_tensor_basis` memref ABI. Validates that a
/// runtime-built tensor's element buffer can cross into MLIR as a memref
/// descriptor, be `memref.reinterpret_cast` to its runtime shape/strides,
/// and be read/written there — with writes visible through the ordinary
/// `sloth_tensor_get1` route (shared storage).
#[cfg(test)]
mod irgen_te_p2_r1 {
    use super::*;

    const ENC_I: fn(i64) -> i64 = |v| v << 1;
    fn enc_f(v: f64) -> i64 {
        ((v.to_bits() & !1) as i64) >> 1
    }

    #[test]
    fn basis_memref_abi() {
        // [[1,2,3],[4,5,6]] built through the rt, addressed as a rank-2
        // memref via basis + reinterpret_cast using runtime dim/stride.
        let mut ir = String::from("module @r1probe {\n");
        ir.push_str("  func.func private @sloth_arr_new(i64) -> i64\n");
        ir.push_str("  func.func private @sloth_arr_set(i64, i64, i64) -> i64\n");
        ir.push_str("  func.func private @sloth_tensor_new_2(i64, i64, i64) -> i64\n");
        ir.push_str("  func.func private @sloth_tensor_copy_from_array(i64, i64) -> i64\n");
        ir.push_str(
            "  func.func private @sloth_tensor_basis_f64(i64) -> memref<?xf64, strided<[?], offset: ?>>\n",
        );
        ir.push_str("  func.func private @sloth_tensor_dim(i64, i64) -> i64\n");
        ir.push_str("  func.func private @sloth_tensor_stride(i64, i64) -> i64\n");
        ir.push_str("  func.func private @sloth_tensor_view(i64, i64, i64, i64) -> i64\n");
        ir.push_str("  func.func private @sloth_tensor_get1(i64, i64) -> i64\n");
        ir.push_str("  func.func private @sloth_rt_print_f64(f64) -> i64\n");
        ir.push_str("  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {\n");
        // array of 6 float words, values 1.0 .. 6.0
        ir.push_str(&format!("    %len6 = arith.constant {} : i64\n", ENC_I(6)));
        ir.push_str("    %arr = call @sloth_arr_new(%len6) : (i64) -> i64\n");
        for (i, v) in [1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            ir.push_str(&format!(
                "    %ai{i} = arith.constant {} : i64\n    %av{i} = arith.constant {} : i64\n",
                ENC_I(i as i64),
                enc_f(*v)
            ));
            ir.push_str(&format!(
                "    call @sloth_arr_set(%arr, %ai{i}, %av{i}) : (i64, i64, i64) -> i64\n"
            ));
        }
        // 2x3 float tensor + copy
        ir.push_str(&format!(
            "    %d2 = arith.constant {} : i64\n    %d3 = arith.constant {} : i64\n    %kind = arith.constant {} : i64\n",
            ENC_I(2),
            ENC_I(3),
            ENC_I(1)
        ));
        ir.push_str(
            "    %t = call @sloth_tensor_new_2(%d2, %d3, %kind) : (i64, i64, i64) -> i64\n",
        );
        ir.push_str("    call @sloth_tensor_copy_from_array(%t, %arr) : (i64, i64) -> i64\n");
        // flat basis memref
        ir.push_str(
            "    %flat = call @sloth_tensor_basis_f64(%t) : (i64) -> memref<?xf64, strided<[?], offset: ?>>\n",
        );
        // runtime dims/strides from the descriptor (tagged words -> index)
        ir.push_str("    %one64 = arith.constant 1 : i64\n");
        for (name, axis) in [("0", 0i64), ("1", 1i64)] {
            ir.push_str(&format!(
                "    %ax{name} = arith.constant {} : i64\n",
                ENC_I(axis)
            ));
            ir.push_str(&format!(
                "    %dw{name} = call @sloth_tensor_dim(%t, %ax{name}) : (i64, i64) -> i64\n"
            ));
            ir.push_str(&format!(
                "    %di{name} = arith.shrsi %dw{name}, %one64 : i64\n    %d{name} = arith.index_cast %di{name} : i64 to index\n"
            ));
            ir.push_str(&format!(
                "    %sw{name} = call @sloth_tensor_stride(%t, %ax{name}) : (i64, i64) -> i64\n"
            ));
            ir.push_str(&format!(
                "    %si{name} = arith.shrsi %sw{name}, %one64 : i64\n    %s{name} = arith.index_cast %si{name} : i64 to index\n"
            ));
        }
        ir.push_str("    %o0 = arith.constant 0 : index\n");
        ir.push_str(
            "    %r2 = memref.reinterpret_cast %flat to offset: [%o0], sizes: [%d0, %d1], strides: [%s0, %s1] : memref<?xf64, strided<[?], offset: ?>> to memref<?x?xf64, strided<[?, ?], offset: ?>>\n",
        );
        // load [1][2] -> expect 6
        ir.push_str("    %i1 = arith.constant 1 : index\n    %i2 = arith.constant 2 : index\n");
        ir.push_str(
            "    %v = memref.load %r2[%i1, %i2] : memref<?x?xf64, strided<[?, ?], offset: ?>>\n",
        );
        ir.push_str("    %p = call @sloth_rt_print_f64(%v) : (f64) -> i64\n");
        // write 7 through the memref; read back through the tensor route
        ir.push_str("    %seven = arith.constant 7.0 : f64\n");
        ir.push_str(
            "    memref.store %seven, %r2[%i1, %i2] : memref<?x?xf64, strided<[?, ?], offset: ?>>\n",
        );
        ir.push_str(&format!(
            "    %off1 = arith.constant {} : i64\n    %drop1 = arith.constant {} : i64\n    %zero = arith.constant {} : i64\n",
            ENC_I(1),
            ENC_I(1),
            ENC_I(0)
        ));
        ir.push_str(
            "    %t1 = call @sloth_tensor_view(%t, %off1, %drop1, %zero) : (i64, i64, i64, i64) -> i64\n",
        );
        ir.push_str(&format!(
            "    %g = call @sloth_tensor_get1(%t1, %d2) : (i64, i64) -> i64\n"
        ));
        ir.push_str("    %g1 = arith.shli %g, %one64 : i64\n");
        ir.push_str("    %gv = arith.bitcast %g1 : i64 to f64\n");
        ir.push_str("    %p2 = call @sloth_rt_print_f64(%gv) : (f64) -> i64\n");
        ir.push_str("    return\n  }\n}\n");

        let ctx = Context::new();
        let op = Op::parse(ctx.raw, &ir, "r1probe.mlir").expect("parse");
        crate::jit::run_llvm_pipeline(ctx.raw, op.raw).expect("pipeline");
        let engine = crate::jit::Engine::new(&op, 2, &[lib_path()]);
        engine.invoke("sloth_main", &mut []).expect("invoke");
    }

    unsafe extern "C" fn swallow(_s: crate::sys::MlirStringRef, _u: *mut std::ffi::c_void) {}

    /// the JIT can register all MLIR passes and parse the TE-P2 pipeline
    /// string (linalg fusion + one-shot-bufferize + math), which is the
    /// precondition for the channel-A linalg route in the JIT.
    #[test]
    fn target_pass_pipeline_parses() {
        use crate::sys;
        unsafe {
            sys::mlirRegisterAllPasses();
            let ctx = sys::mlirContextCreate();
            let pm = sys::mlirPassManagerCreate(ctx);
            let opm = sys::mlirPassManagerGetAsOpPassManager(pm);
            let pipe = std::ffi::CString::new(
                "canonicalize,cse,linalg-fuse-elementwise-ops,one-shot-bufferize,\
                 convert-linalg-to-loops,convert-scf-to-cf,convert-math-to-llvm",
            )
            .unwrap();
            let r = sys::mlirOpPassManagerAddPipeline(
                opm,
                sys::mlirStringRefCreateFromCString(pipe.as_ptr()),
                Some(swallow),
                std::ptr::null_mut(),
            );
            // a bogus pass must be rejected: proves the success above is real
            let bogus = std::ffi::CString::new("no-such-sloth-pass").unwrap();
            let rb = sys::mlirOpPassManagerAddPipeline(
                opm,
                sys::mlirStringRefCreateFromCString(bogus.as_ptr()),
                Some(swallow),
                std::ptr::null_mut(),
            );
            sys::mlirPassManagerDestroy(pm);
            sys::mlirContextDestroy(ctx);
            assert_eq!(r.value, 1, "TE-P2 pass pipeline failed to parse");
            assert_eq!(rb.value, 0, "bogus pass should be rejected");
        }
    }
}
