//! Typed side tables (A3): everything `irgen` (Pass 2) needs in order to lower
//! a program **without re-inferring anything**. The `sem` pass owns and
//! produces these tables; the emitter only consumes them.
//!
//! This is the seam for the staged typed-AST migration: today it carries the
//! expression result types and the resolved type arguments of every generic
//! call site; branch-level coercion/width plans will move here next.

use sloth_frontend::ty::TyId;
use std::collections::HashMap;

/// `(frame, NodeId) -> TyId`: the inferred type of every expression node.
///
/// * `frame` scopes the table so a generic body's nodes keep one entry per
///   monomorphic instance (the mangled symbol the body is emitted under).
/// * `NodeId` is assigned once by `sloth_frontend::ast::assign_ids` and shared
///   by both passes.
pub type TypeTable = HashMap<(String, u32), TyId>;

/// A generic call site, identified by the emitting frame plus its source
/// position (unique within a frame's walk).
pub type SiteKey = (String, usize, usize);

/// Pass-1 typed products consumed by Pass 2.
#[derive(Default, Clone)]
pub struct TypedTables {
    /// expression type of every `Expr` node
    pub types: TypeTable,
    /// resolved T-substitution (`T` -> concrete `TyId`) of every generic call
    /// site. Pass 2 replays this instead of re-running shape unification and
    /// return-type inference.
    pub inst_sites: HashMap<SiteKey, HashMap<String, TyId>>,
}
