//! Pass 1 — semantic analysis (name resolution + type inference + checking).
//!
//! This module is the explicit first pass of the compiler. It runs before
//! `irgen` (Pass 2) and owns every user-facing diagnostic. `slothc check`
//! invokes only this pass; `ir`/`run`/`build` run it and then emit.
//!
//! Analysis produces a **type side table** keyed by `(frame, NodeId)`:
//! `Expr`/`Stmt` nodes carry a stable id (`sloth_frontend::ast::assign_ids`),
//! and every expression's inferred `TyId` is recorded as the analysis walk
//! visits it. Pass 2 is handed this table and consults it as the authoritative
//! expression type (falling back to its own inference only on a miss), so
//! type checking lives exclusively in Pass 1.
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

/// Pass 1 for a single module: analyse, returning the type side table.
pub fn analyze_program(prog: &Program, mod_name: &str) -> Result<TypeTable, String> {
    let mut me = ModEmitter::new(mod_name);
    me.emit_module(prog);
    if me.diags.is_empty() {
        Ok(me.type_table)
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

/// Pass 1 for a multi-module program, returning the type side table.
pub fn analyze_multimod(root_src: &str, base_dir: &std::path::Path) -> Result<TypeTable, String> {
    crate::irgen::analyze_multimod_table(root_src, base_dir)
}
