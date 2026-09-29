//! Monomorphization product shared across the two compilation passes (A1).
//!
//! Pass 1 (`sem`) walks the whole program to a fixpoint and discovers every
//! monomorphic instance of a generic function or generic class — including
//! instances only reachable *through* the body of another instance. It freezes
//! that complete set, plus the type registry the instances were interned
//! against, into a [`MonoPlan`]. Pass 2 (`irgen`) loads the plan instead of
//! re-deriving it, so the two passes agree on the instance set by construction
//! (and, because the registry is carried over, the stored `TyId`s stay valid).

use sloth_frontend::ast::FuncDef;
use sloth_frontend::ty::{Reg, TyId};
use std::collections::HashMap;

/// one monomorphic generic-function instance
#[derive(Clone)]
pub(crate) struct FnInst {
    /// module namespace active at the call site (drives mangling and the
    /// `cur_mod` the body is emitted under)
    pub module: String,
    /// generic base name (as passed to `emit_func`)
    pub name: String,
    /// the generic definition
    pub fd: FuncDef,
    /// T-frame: type-param name -> concrete `TyId`
    pub frame: HashMap<String, TyId>,
    /// monomorphic symbol
    pub mangled: String,
}

/// Pass-1 output: the complete instance set and the type registry backing the
/// `TyId`s it stores.
#[derive(Clone, Default)]
pub(crate) struct MonoPlan {
    /// Pass-1 type registry; Pass 2 starts from this so every `TyId` recorded
    /// in the plan (or in the type side table) resolves correctly regardless of
    /// the order Pass 2 interns its own types.
    pub reg: Reg,
    /// generic-function instances, in discovery order
    pub fns: Vec<FnInst>,
    /// generic-class instances `(mangled name, T-frame)`, in discovery order
    pub cls: Vec<(String, HashMap<String, TyId>)>,
}
