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
mod coerce;
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
mod util;

pub use module::{obj_rt_decls, rt_decls};
pub use state::{ClassInfo, ModEmitter};
pub use util::WW;

/// function plan (semantic surface): defined by the `sem` pass, consumed by the
/// emitter for call/ctor ABI spellings
pub(crate) use crate::sem::collect::FuncPlan;

#[allow(unused_imports)]
pub(crate) use class::*;
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
pub(crate) use util::*;

pub fn compile_to_ir(src: &str, mod_name: &str) -> Result<String, String> {
    let mut prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    sloth_frontend::ast::assign_ids(&mut prog);
    // Pass 1: semantic analysis (inference + checking); owns diagnostics and
    // produces the NodeId type side table + the monomorphization plan.
    let sem = crate::sem::analyze_program(&prog, mod_name)?;
    // Pass 2: emission (diagnostics suppressed — Pass 1 already validated).
    let mut me = ModEmitter::new(mod_name);
    me.check_mode = false;
    // adopt Pass 1's type registry, frozen instance set, and typed tables (A1/A3)
    me.seed_mono_plan(sem.mono);
    me.seed_typed(sem.types);
    me.emit_module(&prog);
    if !me.diags.is_empty() {
        return Err(format_diags(&me));
    }
    let ir = ModEmitter::take_ir(&mut me);
    // lower sloth.* → standard dialects so `slothc ir`/AOT only expose
    // dialects that external tools (mlir-opt) understand
    crate::dialect::lower_text(&ir, &format!("{}.mlir", mod_name))
}
/// build a `ModEmitter` with the root + imported modules registered and the
/// root module emitted. `check_mode` selects Pass 1 (analysis) vs Pass 2
/// (emission); the module assembly itself is shared by both passes.
fn build_multimod(
    root: &Program,
    mods: &[(String, Program, Option<String>, bool)],
    mod_name: &str,
    check_mode: bool,
    sem: Option<(crate::mono::MonoPlan, crate::sem::TypedTables)>,
) -> ModEmitter {
    let mut me = ModEmitter::new(mod_name);
    me.check_mode = check_mode;
    // Pass 2 adopts Pass 1's type registry + frozen instance set + typed side
    // tables *before* anything is emitted, so `TyId`s from the plan resolve
    // during import typing and every expression lookup can hit.
    if let Some((plan, typed)) = sem {
        me.seed_mono_plan(plan);
        me.seed_typed(typed);
    }
    // order-independent devirtualization: index overrides across every module
    // (imports emit method bodies before root classes are registered)
    {
        let mut progs: Vec<&Program> = mods.iter().map(|(_, p, _, _)| p).collect();
        progs.push(root);
        me.register_override_index(&progs);
    }
    for (mname, prog, alias, token) in mods {
        // alias-only entry recording a duplicate import's alias (bug M7)
        if prog.decls.is_empty() && prog.imports.is_empty() {
            if let Some(a) = alias {
                me.register_module_alias(mname, a);
            }
            continue;
        }
        // reserved `__sloth_*` gate: run on the raw module before any prelude
        // (which itself legitimately uses reserved symbols) is injected
        me.check_reserved_module(prog, *token);
        // imports get their own copy: the shared emitter registers each module
        // exactly once, and both passes must observe the same prelude-injected
        // surface
        let mut p2 = Program {
            imports: prog.imports.clone(),
            decls: prog.decls.clone(),
            stmts: prog.stmts.clone(),
        };
        // imported modules get the same `print` prelude so bare calls resolve
        inject_print_prelude(&mut p2.decls);
        // and the reserved runtime ABI declarations so stdlib bodies resolve
        inject_abi_prelude(&mut p2.decls);
        // a module carrying `import "__sloth";` also gets the container/core
        // prelude so the reserved container symbols resolve inside it (bug M6);
        // fixed symbols dedup against the root's copy
        if *token {
            me.inject_container_prelude(&mut p2.decls);
        }
        // `s.chars()` bodies in an imported module emit before the root is
        // collected, so the `StrChars` class must land in the first module
        // that needs it (the shared emitter registers it exactly once)
        me.inject_strchars_prelude(&mut p2.decls);
        me.register_import(mname, alias.as_deref(), &p2);
    }
    // hmm: root module runs under @sloth_main through emit_module
    me.emit_module(root);
    me
}

fn resolve_multimod(
    root_src: &str,
    base_dir: &std::path::Path,
) -> Result<(Program, Vec<(String, Program, Option<String>, bool)>), String> {
    let mut stack: Vec<std::path::PathBuf> = Vec::new();
    let mut done: HashSet<std::path::PathBuf> = HashSet::new();
    let mut stems: HashMap<String, std::path::PathBuf> = HashMap::new();
    let (mut root, mut mods) =
        resolve_program(root_src, base_dir, &mut stack, &mut done, &mut stems)?;
    // stable NodeIds for the type side table (assigned once, shared by both
    // passes because the resolved Programs are reused)
    sloth_frontend::ast::assign_ids(&mut root);
    for (_, p, _, _) in mods.iter_mut() {
        sloth_frontend::ast::assign_ids(p);
    }
    Ok((root, mods))
}

/// Pass 1 for a multi-module program: resolve imports, assemble, analyse,
/// returning the sem products for Pass 2.
pub fn analyze_multimod_table(
    root_src: &str,
    base_dir: &std::path::Path,
) -> Result<crate::sem::SemOutput, String> {
    let (root, mods) = resolve_multimod(root_src, base_dir)?;
    let mut me = build_multimod(&root, &mods, "main", true, None);
    if me.diags.is_empty() {
        Ok(crate::sem::SemOutput {
            types: me.take_typed_tables(),
            mono: me.take_mono_plan(),
        })
    } else {
        Err(format_diags(&me))
    }
}

/// Pass 1 for a multi-module program (diagnostics only).
pub fn analyze_multimod(root_src: &str, base_dir: &std::path::Path) -> Result<(), String> {
    analyze_multimod_table(root_src, base_dir).map(|_| ())
}

pub fn compile_multimod(root_src: &str, base_dir: &std::path::Path) -> Result<String, String> {
    let (root, mods) = resolve_multimod(root_src, base_dir)?;
    // Pass 1
    let sem = {
        let mut me = build_multimod(&root, &mods, "main", true, None);
        if !me.diags.is_empty() {
            return Err(format_diags(&me));
        }
        crate::sem::SemOutput {
            types: me.take_typed_tables(),
            mono: me.take_mono_plan(),
        }
    };
    // Pass 2: hand Pass 1's side table + plan to the assembler so the emitter
    // consumes them from the very first lookup.
    let mut me = build_multimod(&root, &mods, "main", false, Some((sem.mono, sem.types)));
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

/// FNV-1a over the canonical path: disambiguates modules that share a file
/// stem but are distinct files (bug M8).
fn path_hash(p: &std::path::Path) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    for b in p.to_string_lossy().as_bytes() {
        h ^= *b as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

fn resolve_program(
    src: &str,
    dir: &std::path::Path,
    stack: &mut Vec<std::path::PathBuf>,
    done: &mut HashSet<std::path::PathBuf>,
    stems: &mut HashMap<String, std::path::PathBuf>,
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
            // the module body is already registered once; still record a second
            // alias so `import "x" as L2;` after `import "x" as L1;` resolves
            // (bug M7) via an alias-only entry
            if let Some(a) = &imp.alias {
                let mname = stems
                    .iter()
                    .find(|(_, p)| **p == pb2)
                    .map(|(k, _)| k.clone())
                    .unwrap_or_else(|| {
                        pb2.file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "mod".to_string())
                    });
                mods.push((
                    mname,
                    Program {
                        imports: Vec::new(),
                        decls: Vec::new(),
                        stmts: Vec::new(),
                    },
                    Some(a.clone()),
                    false,
                ));
            }
            continue;
        }
        let src2 = std::fs::read_to_string(&pb2).map_err(|e| format!("read {:?}: {}", pb2, e))?;
        let dir2 = pb2
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        stack.push(pb2.clone());
        let (_p2, mut m2) = resolve_program(&src2, &dir2, stack, done, stems)?;
        stack.pop();
        done.insert(pb2.clone());
        let mut stem = pb2
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "mod".to_string());
        // distinct files that share a basename must not mangle to the same
        // symbols (`sloth_lib__ginit`); disambiguate the second occurrence with
        // a path hash (bug M8)
        if let Some(prev) = stems.get(&stem) {
            if prev != &pb2 {
                stem = format!("{}_{:08x}", stem, path_hash(&pb2));
            }
        }
        stems.insert(stem.clone(), pb2.clone());
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
