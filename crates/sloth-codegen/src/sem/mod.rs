//! Pass 1 — semantic analysis (name resolution + type inference + checking).
//!
//! This module is the explicit first pass of the compiler. It runs before
//! `irgen` (Pass 2) and owns every user-facing diagnostic. `slothc check`
//! invokes only this pass; `ir`/`run`/`build` run it and then emit.
//!
//! Analysis produces two products, bundled as [`SemOutput`]:
//!
//! * a **type side table** keyed by `(frame, NodeId)`: `Expr`/`Stmt` nodes
//!   carry a stable id (`sloth_frontend::ast::assign_ids`), and every
//!   expression's inferred `TyId` is recorded as the analysis walk visits it.
//!   Pass 2 consumes it as the authoritative expression type (the table is
//!   complete across the whole program, so Pass 2 contributes no expression
//!   types of its own), so type checking lives in Pass 1.
//! * a **monomorphization plan** (A1): the complete set of generic-function and
//!   generic-class instances, discovered by walking the program to a fixed
//!   point. Pass 2 emits exactly that set instead of re-deriving it.
//!
//! The analysis walk currently reuses the shared inference engine; splitting
//! the body-level inference into a distinct typed-AST traversal is staged.

pub(crate) mod collect;
pub(crate) mod types;

use crate::irgen::{format_diags, ModEmitter};
use sloth_frontend::ast::Program;
use sloth_frontend::ty::TyId;
use std::collections::HashMap;

/// `(frame, NodeId) -> TyId`, produced by Pass 1 and consumed by Pass 2.
pub type TypeTable = HashMap<(String, u32), TyId>;

/// The complete product of one Pass-1 run: the expression type side table and
/// the eager-monomorphization plan.
pub struct SemOutput {
    pub type_table: TypeTable,
    pub(crate) mono: crate::mono::MonoPlan,
}

/// Pass 1 for a single module: analyse, returning the sem products.
pub fn analyze_program(prog: &Program, mod_name: &str) -> Result<SemOutput, String> {
    let mut me = ModEmitter::new(mod_name);
    me.emit_module(prog);
    if me.diags.is_empty() {
        let type_table = std::mem::take(&mut me.type_table);
        Ok(SemOutput {
            type_table,
            mono: me.take_mono_plan(),
        })
    } else {
        Err(format_diags(&me))
    }
}

/// Pass 1 for a single module given as source text (assigns NodeIds first).
pub fn check_src(src: &str, mod_name: &str) -> Result<(), String> {
    let mut prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    sloth_frontend::ast::assign_ids(&mut prog);
    check_program(&prog, mod_name)
}

/// Pass 1 for a single already-parsed, id-assigned module.
pub fn check_program(prog: &Program, mod_name: &str) -> Result<(), String> {
    analyze_program(prog, mod_name).map(|_| ())
}

/// Pass 1 for a multi-module program (imports resolved from `base_dir`).
pub fn check_multimod(root_src: &str, base_dir: &std::path::Path) -> Result<(), String> {
    analyze_multimod(root_src, base_dir).map(|_| ())
}

/// Pass 1 for a multi-module program, returning the sem products.
pub fn analyze_multimod(root_src: &str, base_dir: &std::path::Path) -> Result<SemOutput, String> {
    crate::irgen::analyze_multimod_table(root_src, base_dir)
}
