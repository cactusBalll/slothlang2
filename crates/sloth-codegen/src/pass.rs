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
        std::fs::write(d.join("lib.mm.sl"), "pub func twofold(a: int) -> int {\n    return a + a;\n}").unwrap();
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
        let src = "import \"pub.mm.sl\" as q;\nprint(q.triple(q.n));\nvar b = q.Box(2);\nprint(b.v);\n";
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
        std::fs::write(d.join("b.sl"), "import \"a.sl\";\nfunc bx() -> int {\n    return 1;\n}\n").unwrap();
        std::fs::write(d.join("a.sl"), "import \"b.sl\";\npub func ax() -> int {\n    return bx();\n}\n").unwrap();
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
            var f: Array<float> = [1, 2.5, 3];
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
            f2.push(2);
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
        assert!(e.contains("cannot assign to immutable"), "unexpected: {}", e);
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
        for src in [
            "var x: int = 2.5;\nprint(x);\n",
            "var y: int = 1.0;\n",
        ] {
            let e = match run_src(src, "main") {
                Ok(()) => panic!("kind mismatch accepted: {:?}", src),
                Err(e) => e,
            };
            assert!(e.contains("initializer is float"), "unexpected: {}", e);
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
            for (var k: m) {
                acc = acc + k + m[k];
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
            print(stats(1, 2.5, 3));
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
        assert!(e.contains("variadic argument is float"), "unexpected: {}", e);
    }
}

// ---------------- patch #12: pipe |> ----------------
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
        assert!(e.contains("pipe rhs must be a function or call"), "unexpected: {}", e);
    }
}

// ---------------- patch #13: is narrowing + int()/float() ----------------
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
        assert!(e.contains("unknown identifier `x`") || e.contains("cannot infer"), "unexpected: {}", e);
    }

    /// variadic x generic stays unsupported (MVP shape) with a diagnostic
    #[test]
    fn generic_variadic_diag() {
        let src = "func f<T>(xs...: Array<T>): unit { print(1); }\nfunc main(): unit { f(1, 2); }\n";
        let e = match run_src(src, "main") {
            Ok(()) => panic!("generic variadic accepted"),
            Err(e) => e,
        };
        assert!(e.contains("generic variadic unsupported"), "unexpected: {}", e);
    }
}

// ---------------- patch #15: operator overloads + trait bounds ----------------
mod irgen_p15 {
    use super::*;

    /// Vec2 + Vec2 dispatches __add__ (new-object return + f64 fields worded)
    #[test]
    fn operator_overload_add() {
        let src = r#"
            class Vec2 {
                var x: float = 0;
                var y: float = 0;
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
                var a = Vec2(1, 2);
                var b = Vec2(3, 4);
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
            "unexpected: {}", e
        );
    }

    /// Hashable-keyed maps: object keys route through pointer identity
    #[test]
    fn map_object_keys_work() {
        let src = r#"
            trait Hashable { func hashKey(): int; }
            class Point impl Hashable {
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
            class Point impl Hashable {
                var x: int = 42;
                func __init__(): unit { return; }
                func hashKey(): int { return 7; }
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
            "unexpected: {}", e
        );
    }
}

// ---------------- patch #16: Iterator/Iterable protocol ----------------
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
        assert!(
            e.contains("has no `next()`"),
            "unexpected: {}", e
        );
    }
}

// ---------------- patch #17: extern func (C ABI) ----------------
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
            "unexpected: {}", e
        );
    }
}

// ---------------- patch #18a: trait default method bodies ----------------
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
            "unexpected: {}", e
        );
    }
}

// ---------------- patch #19: generic class monomorphization + Result<T,E> ----------------
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

    /// no overload on comparison: numeric fallback keeps old behavior (int words)
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
        run_src(src, "main").unwrap();
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
            "unexpected: {}", e
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
            "unexpected: {}", e
        );
    }
}

// ---------------- patch #22: var/let surface-type assignment checks ----------------
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
        assert!(e.contains("cannot assign `str` to `int`"), "unexpected: {}", e);
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
            e.contains("initializer is `str` but declared type is `int`"),
            "unexpected: {}", e
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
        assert!(e.contains("type mismatch: cannot assign"), "unexpected: {}", e);
    }

    /// float target promotes int words on assignment; float value diagnosed
    #[test]
    fn float_var_assignment_routes() {
        let src = r#"
            func main(): unit {
                var f = 0.0;
                f = f + 5;
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
            e.contains("cannot assign `Array<str>` to `Array<int>`"),
            "unexpected: {}", e
        );
    }
}

// examples/ regression gold (§9.1/§9.2 adapted versions)
mod irgen_examples {
    use super::*;

    fn ws_root() -> std::path::PathBuf {
        let md = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        std::path::PathBuf::from(md).join("..").join("..").join("examples")
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
