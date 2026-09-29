//! Typed side tables (A3): everything `irgen` (Pass 2) needs in order to lower
//! a program **without re-inferring anything**. The `sem` pass owns and
//! produces these tables; the emitter only consumes them.
//!
//! This is the seam for the staged typed-AST migration: today it carries the
//! expression result types, the resolved type arguments of every generic call
//! site, and the coercion plans the branch logic used to derive on the fly
//! (integer widths, container store faces, call-site argument coercions, and
//! the expected-type-hint decisions: literal `ok()/err()` ctors and tensor
//! target shapes); the remaining branch logic will move into a dedicated
//! `sem` walk next.

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
    /// `(frame, NodeId) -> unified integer surface` for integer arithmetic and
    /// comparison nodes. Pass 2 replays the width chosen by `unify_int` rather
    /// than re-deriving it (A3, arithmetic family).
    pub int_ops: HashMap<(String, u32), TyId>,
    /// `(frame, NodeId) -> store face` of a container literal: the declared
    /// element (list) / value (map) surface every slot is coerced to at the
    /// store face (`Array<Weak<T>>` / `Map<_, int8>` / `Array<dyn D>` …).
    /// The value is `Some` exactly when Pass 1 applied the hint-driven
    /// store-face coercion and `None` when it decided no coercion applies, so
    /// Pass 2 never re-reads the expected-type hint to decide it (A3,
    /// container-hint family).
    pub store_faces: HashMap<(String, u32), Option<TyId>>,
    /// call site -> per-argument coercion target (`Some(param surface)` when
    /// the argument word is boxed/wrapped/narrowed into the parameter,
    /// `None` when it binds as-is). `sem` decides it once in Pass 1; Pass 2
    /// replays it instead of re-deriving the gate from the parameter
    /// surfaces (A3, call-argument family).
    pub arg_coercions: HashMap<SiteKey, Vec<Option<TyId>>>,
    /// call site -> post-coercion argument surfaces. These drive the call
    /// signature (a boxed `float?` argument rides `i64`, not `f64`), so Pass
    /// 2 replays them rather than re-deriving them from the parameter list
    /// (A3, call-argument family).
    pub arg_faces: HashMap<SiteKey, Vec<TyId>>,
    /// container-literal element node -> resolved `Result` instance. Pass 1
    /// resolves `ok(v)`/`err(e)` elements against the declared element/value
    /// surface (the expected-type hint) and freezes the instance here; Pass 2
    /// replays the ctor instead of re-reading the hint (A3, expected-hint
    /// family).
    pub literal_ctors: HashMap<(String, u32), String>,
    /// `tensor.zeros` / `tensor.from_array` call site -> resolved
    /// `(element, rank)` of the declared `Tensor<T, R>` target. `None` records
    /// that Pass 1 found no tensor hint (diagnosed there); Pass 2 replays the
    /// decision instead of re-reading the hint (A3, expected-hint family).
    pub tensor_shapes: HashMap<SiteKey, Option<(TyId, u32)>>,
}
