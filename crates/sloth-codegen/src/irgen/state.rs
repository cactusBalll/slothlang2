//! Emitter state: `ModEmitter` tables + class registry structs.

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use sloth_frontend::ast::*;
#[allow(unused_imports)]
use sloth_frontend::lexer::{Pos, StrPart};
#[allow(unused_imports)]
use sloth_frontend::ty::{Diag, FnTy, LamMeta, Reg, Ty, TyId};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

pub struct ModEmitter {
    pub r: Reg,
    /// module name (file stem, e.g. "main")
    pub name: String,
    /// top-level function signatures: qualified name -> sig
    pub funcs: HashMap<String, FuncDef>,
    /// class layouts per simple name (same-module scope; cross-module via mangle)
    pub classes: HashMap<String, ClassInfo>,
    /// trait surface: simple name -> methods
    pub traits: HashMap<String, Vec<MethodSig>>,
    /// module-level globals: name -> (mangled symbol, type id, mutable)
    pub globals: HashMap<String, (String, TyId, bool)>,
    pub diags: Vec<Diag>,
    /// module text to parse (html-safe)
    pub out: String,
    /// compile-time string-literal pool for `llvm.mlir.global` output (kept
    /// for the globals emitter; the current lowerer builds literals through
    /// `__sloth_str_push`/`__sloth_str_finish`, so this is normally empty)
    pub strpool: Vec<String>,
    /// classes emitted: name -> id (assigned in collect order)
    pub class_ids: HashMap<String, i64>,
    /// mangled function names already emitted
    pub emitted_names: Vec<String>,
    /// module name used for mangling the next emit (settable for foreign imports)
    pub cur_mod: String,
    /// foreign module surface: simple name -> (mangled symbol, ret ty)
    pub cross_funcs: HashMap<String, (String, TyId)>,
    /// foreign function defs (same keys as `cross_funcs`): lets a qualified
    /// call to an imported *generic* function be monomorphized at the caller
    pub foreign_func_defs: HashMap<String, (String, FuncDef)>,
    /// satisfied import paths (file canonical)
    pub imported_paths: Vec<String>,
    /// raw class defs (for field-initializer emission at ctor time)
    pub class_defs: HashMap<String, (String, ClassDef)>,
    /// non-pub symbols exported by imported modules; access = diagnostic
    pub hidden: HashSet<String>,
    /// foreign globals: "mod.name" or "alias.name" -> (mangled symbol, ty, mutable)
    pub fglobals: HashMap<String, (String, TyId, bool)>,
    /// global symbol declarations to prepend to the module IR
    pub global_decls: Vec<String>,
    /// global symbols already declared (dedupe)
    pub declared_syms: std::collections::HashSet<String>,
    /// imported modules whose init func runs before @sloth_main body
    pub init_mods: Vec<String>,
    /// import aliases: alias -> module name
    pub mod_alias: HashMap<String, String>,
    /// foreign classes imported (ids 100+; offset starts here)
    pub foreign_cls: std::collections::HashSet<String>,
    /// module that owns a class (own modules use `name`; foreign classes owned mod)
    pub cls_mod: HashMap<String, String>,
    /// lambda function counter (unique symbols per lambda site)
    pub lamcount: usize,
    /// generic-class instances registered during typing: (inst name, T-frame)
    pub pending_insts: Vec<(String, HashMap<String, TyId>)>,
    /// eager-monomorphization worklist for generic *function* instances,
    /// in discovery order; emitted to a fixpoint by `emit_module`
    pub(crate) pending_fn_insts: Vec<crate::mono::FnInst>,
    /// dedupe for `pending_fn_insts` by mangled symbol
    pub(crate) pending_fn_seen: HashSet<String>,
    /// dedupe for `pending_insts` by instance name (seeded from the Pass-1 plan)
    pub(crate) pending_cls_seen: HashSet<String>,
    /// next fresh native class id for synthesized instances
    pub native_cls_id: i64,
    /// Result<T,E> instances (builtins `ok(v)`/`err(e)` target them)
    pub result_insts: std::collections::HashSet<String>,
    /// `extern type` declared opaque surfaces (no ctor/fields/methods)
    pub extern_types: std::collections::HashSet<String>,
    /// generic base T-frame per registered instance (plan-time T resolution)
    pub class_frames: HashMap<String, HashMap<String, TyId>>,
    /// devirt/inline observation counters (patch #18c; SLOTH_STATS=1 prints)
    pub stat_dcalls: usize,
    pub stat_dyncalls: usize,
    /// class-method virtual calls (class-id chain pre-VD-P1; vtable after)
    pub stat_cvcalls: usize,
    pub stat_ginsts: usize,
    /// expected-type hint stack for return-driven inference (patch #38):
    /// let/var annotation & assignment target surface pushed around emitting
    /// the init/target value
    pub(crate) exp_ret: Vec<TyId>,
    pub stat_extdecls: usize,
    pub stat_vtbuilds: usize,
    /// registration order of classes (deterministic dyn-dispatch chain)
    pub class_order: Vec<String>,
    /// vtable slot assignment: (trait, method) -> (index, ret-float, ret-unit)
    pub vt_slots: HashMap<(String, String), usize>,
    /// per-cls vtable builders already emitted (patch #28 global cache)
    pub vt_built: std::collections::HashSet<String>,
    /// methods emitted inside llvm.func (vtable-addressable)
    pub llvm_method: std::collections::HashSet<(String, String)>,
    /// method names overridden somewhere in the whole program (all modules).
    /// Pre-computed before emission so the devirtualization decision does not
    /// depend on emission order (root subclasses are registered after imports).
    pub(crate) known_override_methods: HashSet<String>,
    /// object bodies carry a fixed vtable capacity (total slots)
    pub vt_cap: usize,
    /// active type-param substitution for the generic instance being emitted
    /// (stacked for nesting; ty positions resolve T against the top frame)
    pub(crate) tp_subst: Vec<HashMap<String, TyId>>,
    /// function-value trampolines: cache key -> bridge symbol. A closure is
    /// a 2-word object { tagged fnptr, env }; the bridge has the uniform
    /// `(i64 env, i64 args...) -> i64` ABI that any `Ty::Fn` call site uses.
    pub bridges: HashMap<String, String>,
    /// auto-boxed builtin value-type method bridges: (kind, method, arity) -> sym
    pub(crate) builtin_bridges: HashMap<String, String>,
    /// mangled instance name forced for the next plan_func/emit_func
    pub(crate) tp_mangled: Vec<String>,
    /// runtime `typeid` registry: canonical type key -> integer id (monomorphic
    /// non-class reference types; classes/dyn use `ObjInfo.cls_id` instead)
    pub(crate) type_ids: HashMap<String, i64>,
    /// next fresh `typeid` constant (starts at `TYPEID_BASE`)
    pub(crate) next_type_id: i64,
    /// display name per concrete class (incl. generic instances) for type_name
    pub(crate) cls_display: HashMap<String, String>,
    /// byte blobs backing `type_name` results (pooled, emitted as LLVM globals)
    pub(crate) tyname_pool: Vec<String>,
    /// byte blob -> global symbol (dedupe)
    pub(crate) tyname_syms: HashMap<String, String>,
    /// `any` structural type descriptors, in emission order
    pub(crate) anydescs: Vec<AnyDesc>,
    /// structural type key -> descriptor symbol (dedupe)
    pub(crate) anydesc_syms: HashMap<String, String>,
    /// runtime symbols already declared by `rt_decls`/`obj_rt_decls`: an
    /// `extern func` for one of these must not re-emit a declaration
    pub(crate) predeclared: HashSet<String>,
    /// self-hosted container prelude functions: emitted under their raw name
    /// (no module mangling) so the hardcoded `@__sloth_arr_*`/`@__sloth_map_*`
    /// call sites in this emitter resolve to them
    pub(crate) fixed_syms: HashSet<String>,
    /// top-level functions whose address is taken (`fn_addr`): emitted as
    /// `llvm.func` so `llvm.mlir.addressof` is legal
    pub(crate) addressable: HashSet<String>,
    /// Pass selector. `true` = semantic analysis pass (`sem`): infer + check
    /// and collect diagnostics. `false` = code-emission pass (`irgen`):
    /// diagnostics are suppressed because Pass 1 owns them.
    pub(crate) check_mode: bool,
    /// current function/instance frame key (mangled symbol or module marker):
    /// scopes the type side table so a generic body's nodes keep one entry per
    /// monomorphic instance
    pub(crate) cur_frame: String,
    /// Pass 1 (sem) type side table: `(frame, node id) -> TyId`. Pass 2 reads
    /// it as the authoritative expression type; it is complete over the whole
    /// program, so Pass 2 contributes no expression types (a miss is a bug and
    /// is surfaced in debug builds).
    pub(crate) type_table: HashMap<(String, u32), TyId>,
    /// A3 typed plan: resolved type-argument substitution per generic call
    /// site, keyed by `(frame, line, col)`. Pass 1 fills it; Pass 2 replays it
    /// instead of re-running shape/return-type inference.
    pub(crate) inst_sites: HashMap<(String, usize, usize), HashMap<String, TyId>>,
    /// A3 arithmetic plan: unified integer surface of integer arithmetic /
    /// comparison nodes, keyed by `(frame, NodeId)`.
    pub(crate) int_ops: HashMap<(String, u32), TyId>,
    /// A3 container-hint plan: store-face coercion target of a list/map
    /// literal, keyed by `(frame, NodeId)`; `None` = sem decided no
    /// store-face coercion applies. Pass 2 replays it instead of re-reading
    /// the expected-type hint stack.
    pub(crate) store_faces: HashMap<(String, u32), Option<TyId>>,
    /// A3 call-argument plan: per-argument coercion target of a call site,
    /// keyed by `(frame, line, col)`; `None` entry = the word binds as-is.
    /// Pass 1 records it in `coerce_args_to_params`, Pass 2 replays it.
    pub(crate) arg_coercions: HashMap<crate::sem::SiteKey, Vec<Option<TyId>>>,
    /// A3 call-argument plan: post-coercion argument surfaces of a call site
    /// (they drive the emitted call signature), keyed like `arg_coercions`.
    pub(crate) arg_faces: HashMap<crate::sem::SiteKey, Vec<TyId>>,
    /// payload `Y` of a fiber entry lambda about to be emitted: consumed by
    /// `emit_func_env` so `fiber.yield` in the body can be type-checked
    pub(crate) pending_fiber_payload: Option<TyId>,
    /// imported non-`pub` class names (bug M4): unqualified construction from
    /// another module must be rejected
    pub(crate) hidden_classes: HashSet<String>,
}

/// one compiler-emitted `any` type descriptor record
#[derive(Clone)]
pub(crate) struct AnyDesc {
    pub sym: String,
    pub kind: i64,
    pub flags: i64,
    pub type_id: i64,
    pub name: Option<(String, usize)>,
    pub elem: Option<String>,
    pub key: Option<String>,
    pub val: Option<String>,
    /// display wrapper spec (resolved to an `llvm.func` at emission)
    pub disp: Option<DispSpec>,
    pub cls_id: i64,
    pub rank: i64,
}

/// how an object/dyn value renders itself to a `str`
#[derive(Clone)]
pub(crate) enum DispSpec {
    /// trait vtable dispatch: read slot `slot` from the receiver's vtable
    Dyn { slot: usize },
}

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub fields: Vec<(String, TyId, bool)>, // (name, ty, mutable)
    pub methods: Vec<(String, FuncDef)>,
    pub superclass: Option<String>,
    pub impls: Vec<String>,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum IdxKind {
    /// array word backed by __sloth_arr_len + __sloth_arr_get(_f64)
    Arr,
    /// string: __sloth_str_len + __sloth_str_char returns Str words
    StrChar,
}

impl ModEmitter {
    pub fn new(name: &str) -> ModEmitter {
        let mut predeclared: HashSet<String> = HashSet::new();
        for src in [super::rt_decls(), super::obj_rt_decls()] {
            for line in src.lines() {
                if let Some(i) = line.find('@') {
                    let rest = &line[i + 1..];
                    let nm: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !nm.is_empty() {
                        predeclared.insert(nm);
                    }
                }
            }
        }
        ModEmitter {
            r: Reg::new(),
            name: name.to_string(),
            funcs: HashMap::new(),
            classes: HashMap::new(),
            traits: HashMap::new(),
            globals: HashMap::new(),
            diags: Vec::new(),
            out: String::new(),
            strpool: Vec::new(),
            class_ids: HashMap::new(),
            emitted_names: Vec::new(),
            cur_mod: name.to_string(),
            cross_funcs: HashMap::new(),
            foreign_func_defs: HashMap::new(),
            imported_paths: Vec::new(),
            class_defs: HashMap::new(),
            hidden: HashSet::new(),
            fglobals: HashMap::new(),
            global_decls: Vec::new(),
            declared_syms: std::collections::HashSet::new(),
            init_mods: Vec::new(),
            mod_alias: HashMap::new(),
            foreign_cls: std::collections::HashSet::new(),
            cls_mod: HashMap::new(),
            lamcount: 0,
            stat_dcalls: 0,
            stat_dyncalls: 0,
            stat_cvcalls: 0,
            stat_ginsts: 0,
            exp_ret: Vec::new(),
            stat_extdecls: 0,
            stat_vtbuilds: 0,
            pending_insts: Vec::new(),
            pending_fn_insts: Vec::new(),
            pending_fn_seen: HashSet::new(),
            pending_cls_seen: HashSet::new(),
            native_cls_id: 0,
            result_insts: std::collections::HashSet::new(),
            extern_types: std::collections::HashSet::new(),
            class_frames: HashMap::new(),
            class_order: Vec::new(),
            vt_slots: HashMap::new(),
            vt_built: std::collections::HashSet::new(),
            llvm_method: std::collections::HashSet::new(),
            known_override_methods: std::collections::HashSet::new(),
            vt_cap: 0,
            tp_subst: Vec::new(),
            tp_mangled: Vec::new(),
            bridges: HashMap::new(),
            builtin_bridges: HashMap::new(),
            type_ids: HashMap::new(),
            next_type_id: TYPEID_BASE,
            cls_display: HashMap::new(),
            tyname_pool: Vec::new(),
            tyname_syms: HashMap::new(),
            anydescs: Vec::new(),
            anydesc_syms: HashMap::new(),
            predeclared,
            fixed_syms: HashSet::new(),
            addressable: HashSet::new(),
            check_mode: true,
            cur_frame: String::new(),
            type_table: HashMap::new(),
            inst_sites: HashMap::new(),
            int_ops: HashMap::new(),
            store_faces: HashMap::new(),
            arg_coercions: HashMap::new(),
            arg_faces: HashMap::new(),
            pending_fiber_payload: None,
            hidden_classes: HashSet::new(),
        }
    }
}

impl ModEmitter {
    /// Capture the Pass-1 typed side tables (A3).
    pub(crate) fn take_typed_tables(&mut self) -> crate::sem::TypedTables {
        crate::sem::TypedTables {
            types: std::mem::take(&mut self.type_table),
            inst_sites: std::mem::take(&mut self.inst_sites),
            int_ops: std::mem::take(&mut self.int_ops),
            store_faces: std::mem::take(&mut self.store_faces),
            arg_coercions: std::mem::take(&mut self.arg_coercions),
            arg_faces: std::mem::take(&mut self.arg_faces),
        }
    }

    /// Load the Pass-1 typed side tables into Pass 2 (A3): expression result
    /// types and generic-call resolutions.
    pub(crate) fn seed_typed(&mut self, t: crate::sem::TypedTables) {
        self.type_table = t.types;
        self.inst_sites = t.inst_sites;
        self.int_ops = t.int_ops;
        self.store_faces = t.store_faces;
        self.arg_coercions = t.arg_coercions;
        self.arg_faces = t.arg_faces;
    }

    /// A3 (container-hint family): the store face `sem` chose for the
    /// container literal `id`. `Some(face)` = every element/value is coerced
    /// to `face` at the store; `None` = sem decided no store-face coercion
    /// applies. Only consultable in Pass 2 (Pass 1 derives it itself).
    pub(crate) fn planned_store_face(&self, id: u32) -> Option<Option<TyId>> {
        if self.check_mode || id == 0 {
            return None;
        }
        self.store_faces.get(&(self.cur_frame.clone(), id)).copied()
    }

    /// A3: freeze the store face decided for container literal `id` while
    /// walking it in Pass 1 (no-op for id-less synthetic literals).
    pub(crate) fn record_store_face(&mut self, id: u32, face: Option<TyId>) {
        if self.check_mode && id != 0 {
            self.store_faces.insert((self.cur_frame.clone(), id), face);
        }
    }

    /// A3 (call-argument family): the per-argument coercion plan `sem`
    /// recorded for the call site at `pos`; `None` means Pass 1 has no plan
    /// for it (Pass 1 itself, or a miss — reported by the caller).
    pub(crate) fn planned_arg_coercions(&self, pos: &Pos) -> Option<Vec<Option<TyId>>> {
        if self.check_mode {
            return None;
        }
        self.arg_coercions.get(&self.site_key(pos)).cloned()
    }

    /// A3: freeze the per-argument coercion decision for the call site at
    /// `pos` (Pass 1 only). A site may be reached twice in one pass (a
    /// vtable attempt falling back to a direct call): the first plan wins,
    /// and a *conflicting* second plan means the two routes disagree about
    /// the parameter surfaces — a bug, not a tie to break.
    pub(crate) fn record_arg_coercions(&mut self, pos: &Pos, plan: Vec<Option<TyId>>) {
        if !self.check_mode {
            return;
        }
        let key = self.site_key(pos);
        match self.arg_coercions.get(&key) {
            Some(prev) => debug_assert!(
                prev == &plan,
                "conflicting arg coercion plan at {:?}: {:?} vs {:?}",
                key,
                prev,
                plan
            ),
            None => {
                self.arg_coercions.insert(key, plan);
            }
        }
    }

    /// A3: the post-coercion argument surfaces `sem` recorded for the call
    /// site at `pos` (Pass 2 only).
    pub(crate) fn planned_arg_faces(&self, pos: &Pos) -> Option<Vec<TyId>> {
        if self.check_mode {
            return None;
        }
        self.arg_faces.get(&self.site_key(pos)).cloned()
    }

    /// A3: freeze the post-coercion argument surfaces of the call site at
    /// `pos` (Pass 1 only; see [`Self::record_arg_coercions`] for the
    /// write-once rule).
    pub(crate) fn record_arg_faces(&mut self, pos: &Pos, faces: Vec<TyId>) {
        if !self.check_mode {
            return;
        }
        let key = self.site_key(pos);
        match self.arg_faces.get(&key) {
            Some(prev) => debug_assert!(
                prev == &faces,
                "conflicting arg faces at {:?}: {:?} vs {:?}",
                key,
                prev,
                faces
            ),
            None => {
                self.arg_faces.insert(key, faces);
            }
        }
    }

    /// A3: unified integer surface of an integer arithmetic/comparison node.
    /// Pass 1 computes it with `unify_int` and records it; Pass 2 replays the
    /// recorded value instead of re-deriving the width.
    pub(crate) fn planned_int_width(
        &mut self,
        id: u32,
        at: TyId,
        lit_a: bool,
        bt: TyId,
        lit_b: bool,
    ) -> Option<TyId> {
        let key = (self.cur_frame.clone(), id);
        if !self.check_mode && id != 0 {
            if let Some(&t) = self.int_ops.get(&key) {
                return Some(t);
            }
        }
        let t = self.unify_int(at, lit_a, bt, lit_b);
        if self.check_mode && id != 0 {
            if let Some(t) = t {
                self.int_ops.insert(key, t);
            }
        }
        t
    }

    /// key of a call site under the frame currently being emitted
    pub(crate) fn site_key(&self, pos: &Pos) -> crate::sem::SiteKey {
        (self.cur_frame.clone(), pos.line, pos.col)
    }

    /// A3: replay the Pass-1 expression type of `id` in Pass 2 (no-op in Pass 1
    /// and for synthetic id-less nodes).
    pub(crate) fn planned_expr_ty(&self, id: u32) -> Option<TyId> {
        if self.check_mode || id == 0 {
            return None;
        }
        self.type_table.get(&(self.cur_frame.clone(), id)).copied()
    }

    /// Capture the Pass-1 monomorphization product (A1): the complete instance
    /// set plus the type registry its `TyId`s live in.
    pub(crate) fn take_mono_plan(&self) -> crate::mono::MonoPlan {
        crate::mono::MonoPlan {
            reg: self.r.clone(),
            fns: self.pending_fn_insts.clone(),
            cls: self.pending_insts.clone(),
        }
    }

    /// Load the Pass-1 monomorphization plan into Pass 2 (A1).
    ///
    /// The type registry is adopted wholesale so every `TyId` recorded in the
    /// plan — and in the `sem` type side table — resolves correctly even if
    /// Pass 2 happens to intern a different subset of types. Both frozen
    /// instance sets are queued for eager emission; the Pass-2 walk re-registers
    /// the class instances deterministically (same names, ids and frames), and
    /// the `pending_*_seen` dedupe keeps the worklists free of duplicates.
    pub(crate) fn seed_mono_plan(&mut self, plan: crate::mono::MonoPlan) {
        self.r = plan.reg;
        for inst in &plan.fns {
            if self.pending_fn_seen.insert(inst.mangled.clone()) {
                self.pending_fn_insts.push(inst.clone());
            }
        }
        for (inst, frame) in &plan.cls {
            if self.pending_cls_seen.insert(inst.clone()) {
                self.pending_insts.push((inst.clone(), frame.clone()));
            }
        }
    }
}

impl ModEmitter {
    pub(crate) fn err(&mut self, pos: &Pos, msg: String) {
        // Pass 2 (emission) must not re-report: Pass 1 already validated the
        // program. Keeping the code path but dropping the diagnostic makes the
        // check/emit boundary explicit without a second code path.
        if !self.check_mode {
            return;
        }
        self.diags.push(Diag {
            line: pos.line,
            col: pos.col,
            msg,
        });
    }
}
