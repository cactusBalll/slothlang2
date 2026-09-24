//! End-to-end codegen: untyped AST -> textual MLIR (func/arith/cf/memref).
//! One pass does name resolution + type inference + emission. Diagnostics are
//! collected rather than aborting. Lowering uses plain `cf` basic blocks built
//! with SSA env captured at emission time (locals live in memref allocas).
//!
//! Split by responsibility:
//! - `state`: emitter state (`ModEmitter`, class registry)
//! - `fnwalk`: per-function CFG emission state machine
//! - `tybind`: type registry, assignability and trait-bound checks
//! - `collect`: pass 1 — collect symbols, plan functions, vtable slots
//! - `func`: pass 2 — emit a single function body
//! - `stmt` / `expr`: statement / expression lowering
//! - `lambda`: snapshot-capturing closures
//! - `class`: objects, methods, vtables, dynamic dispatch
//! - `module`: module-level emission, globals, imports, runtime decls
//! - `util`: mangling and small IR-text helpers

#[allow(unused_imports)]
use sloth_frontend::ast::*;
#[allow(unused_imports)]
use sloth_frontend::lexer::Pos;
#[allow(unused_imports)]
use sloth_frontend::ty::{Reg, Ty, TyId};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

mod anydesc;
mod class;
mod closure;
mod collect;
mod dynbox;
mod expr;
mod fiber;
mod fnwalk;
mod func;
mod lambda;
mod module;
mod state;
mod stmt;
mod tensor;
mod thread;
mod tybind;
mod util;

pub use module::{obj_rt_decls, rt_decls};
pub use state::{ClassInfo, ModEmitter};
pub use util::WW;

#[allow(unused_imports)]
pub(crate) use class::*;
#[allow(unused_imports)]
pub(crate) use collect::*;
#[allow(unused_imports)]
pub(crate) use dynbox::*;
#[allow(unused_imports)]
pub(crate) use expr::*;
#[allow(unused_imports)]
pub(crate) use fiber::*;
#[allow(unused_imports)]
pub(crate) use fnwalk::*;
#[allow(unused_imports)]
pub(crate) use func::*;
#[allow(unused_imports)]
pub(crate) use lambda::*;
#[allow(unused_imports)]
pub(crate) use module::*;
#[allow(unused_imports)]
pub(crate) use state::*;
#[allow(unused_imports)]
pub(crate) use stmt::*;
#[allow(unused_imports)]
pub(crate) use tensor::*;
#[allow(unused_imports)]
pub(crate) use thread::*;
#[allow(unused_imports)]
pub(crate) use tybind::*;
#[allow(unused_imports)]
pub(crate) use util::*;

pub fn compile_to_ir(src: &str, mod_name: &str) -> Result<String, String> {
    let prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    let mut me = ModEmitter::new(mod_name);
    me.emit_module(&prog);
    if !me.diags.is_empty() {
        return Err(format_diags(&me));
    }
    let ir = ModEmitter::take_ir(&mut me);
    // lower sloth.* → standard dialects so `slothc ir`/AOT only expose
    // dialects that external tools (mlir-opt) understand
    crate::dialect::lower_text(&ir, &format!("{}.mlir", mod_name))
}
pub fn compile_multimod(root_src: &str, base_dir: &std::path::Path) -> Result<String, String> {
    let mut stack: Vec<std::path::PathBuf> = Vec::new();
    let mut done: HashSet<std::path::PathBuf> = HashSet::new();
    let (root, mods) = resolve_program(root_src, base_dir, &mut stack, &mut done)?;
    let mut me = ModEmitter::new("main");
    // order-independent devirtualization: index overrides across every module
    // (imports emit method bodies before root classes are registered)
    {
        let mut progs: Vec<&Program> = mods.iter().map(|(_, p, _, _)| p).collect();
        progs.push(&root);
        me.register_override_index(&progs);
    }
    for (mname, mut prog, alias, token) in mods {
        // reserved `__sloth_*` gate: run on the raw module before any prelude
        // (which itself legitimately uses reserved symbols) is injected
        me.check_reserved_module(&prog, token);
        // imported modules get the same `print` prelude so bare calls resolve
        crate::irgen::inject_print_prelude(&mut prog.decls);
        // and the reserved runtime ABI declarations so stdlib bodies resolve
        crate::irgen::inject_abi_prelude(&mut prog.decls);
        // `s.chars()` bodies in an imported module emit before the root is
        // collected, so the `StrChars` class must land in the first module
        // that needs it (the shared emitter registers it exactly once)
        me.inject_strchars_prelude(&mut prog.decls);
        me.register_import(&mname, alias.as_deref(), &prog);
    }
    // hmm: root module runs under @sloth_main through emit_module
    me.emit_module(&root);
    if !me.diags.is_empty() {
        return Err(format_diags(&me));
    }
    let ir = ModEmitter::take_ir(&mut me);
    crate::dialect::lower_text(&ir, "main.mlir")
}
/// dev-tree stdlib root: `<repo>/lib` (design D5 search order item 4)
const DEV_LIB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../lib");

/// resolve an import path (D5): importer dir → `$SLOTH_STDLIB` → `<exe>/../lib`
/// → dev-tree `<repo>/lib`, so `import "sloth/tensor.slt"` works in-tree and
/// once installed.
fn find_import(dir: &std::path::Path, rel: &str) -> Option<std::path::PathBuf> {
    let cand = dir.join(rel);
    if cand.is_file() {
        return Some(cand);
    }
    if let Ok(root) = std::env::var("SLOTH_STDLIB") {
        let p = std::path::Path::new(&root).join(rel);
        if p.is_file() {
            return Some(p);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(d) = exe.parent() {
            let p = d.join("../lib").join(rel);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    let p = std::path::Path::new(DEV_LIB).join(rel);
    if p.is_file() {
        return Some(p);
    }
    None
}

fn resolve_program(
    src: &str,
    dir: &std::path::Path,
    stack: &mut Vec<std::path::PathBuf>,
    done: &mut HashSet<std::path::PathBuf>,
) -> Result<(Program, Vec<(String, Program, Option<String>, bool)>), String> {
    let prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    let mut mods: Vec<(String, Program, Option<String>, bool)> = Vec::new();
    for imp in &prog.imports {
        // pseudo-import: capability token for the reserved `__sloth_*` ABI
        // surface — no file is resolved for it.
        if imp.path == crate::irgen::SLOTH_IMPORT {
            continue;
        }
        let pb = match find_import(dir, &imp.path) {
            Some(p) => p,
            None => {
                return Err(format!(
                    "cannot resolve import {:?} (searched {:?}, $SLOTH_STDLIB, <exe>/../lib, {})",
                    imp.path, dir, DEV_LIB
                ));
            }
        };
        let pb2 = match std::fs::canonicalize(&pb) {
            Ok(p) => p,
            Err(_) => pb.clone(),
        };
        if stack.iter().any(|x| x == &pb2) {
            let join = stack
                .iter()
                .filter_map(|x| x.file_name().map(|f| f.to_string_lossy().to_string()))
                .collect::<Vec<String>>()
                .join(" -> ");
            return Err(format!(
                "circular import: {} -> {}",
                join,
                pb2.file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| "self".to_string())
            ));
        }
        if done.contains(&pb2) {
            continue;
        }
        let src2 = std::fs::read_to_string(&pb2).map_err(|e| format!("read {:?}: {}", pb2, e))?;
        let dir2 = pb2
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        stack.push(pb2.clone());
        let (_p2, mut m2) = resolve_program(&src2, &dir2, stack, done)?;
        stack.pop();
        done.insert(pb2.clone());
        let stem = pb2
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "mod".to_string());
        // capture the capability token before `prog2_of` drops the import list
        let token = _p2
            .imports
            .iter()
            .any(|i| i.path == crate::irgen::SLOTH_IMPORT);
        // dependencies first: a module's body may call the imports of its own
        // imports, so `register_import` must see them registered already
        mods.append(&mut m2);
        mods.push((stem, prog2_of(&_p2), imp.alias.clone(), token));
    }
    Ok((prog, mods))
}
fn prog2_of(prog: &Program) -> Program {
    Program {
        imports: Vec::new(),
        decls: prog.decls.clone(),
        stmts: Vec::new(),
    }
}
fn visible_of(d: &Decl) -> bool {
    d.visible
}

/// full-compilation diagnostic batch report (patch #41): numbered with
/// line/col tags instead of a Debug dump; multi-error scenarios surface
/// every collected diag of the whole pass
pub fn format_diags(me: &ModEmitter) -> String {
    let mut s = String::from("codegen diags:\n");
    for d in &me.diags {
        s.push_str(&format!("  [L{}:C{}] {}\n", d.line, d.col, d.msg));
        s.push('\n');
    }
    s.push_str(&format!("  ({} error(s))", me.diags.len()));
    s
}
