//! Type registry: `ty_of`, surface/assignability & trait-bound checks.

#[allow(unused_imports)]
use crate::irgen::*;
#[allow(unused_imports)]
use sloth_frontend::ast::*;
#[allow(unused_imports)]
use sloth_frontend::lexer::{Pos, StrPart};
#[allow(unused_imports)]
use sloth_frontend::ty::{Diag, FnTy, LamMeta, Reg, Ty, TyId};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

impl ModEmitter {
    /// type of a syntactic type expression
    pub(crate) fn ty_of(&mut self, t: &Type) -> TyId {
        match t {
            Type::Unit => self.r.mk(Ty::Unit),
            Type::Optional(i) => {
                let ni = self.ty_of(i);
                self.r.mk(Ty::Opt(ni))
            }
            Type::Simple(st) => self.ty_of_simple(st),
        }
    }
}

impl ModEmitter {
    pub(crate) fn ty_of_simple(&mut self, st: &SimpleType) -> TyId {
        use sloth_frontend::ty::FnTy;
        match st {
            SimpleType::Bool => self.r.mk(Ty::Bool),
            SimpleType::Int => self.r.mk(Ty::I64),
            SimpleType::FixedInt(k) => self.r.mk(Ty::Int(*k)),
            SimpleType::Float => self.r.mk(Ty::F64),
            SimpleType::Str => self.r.mk(Ty::Str),
            SimpleType::Range => self.r.mk(Ty::Range),
            SimpleType::Any => self.r.mk(Ty::Any),
            SimpleType::Array(e) => {
                let ne = self.ty_of(e);
                self.r.mk(Ty::Array(ne))
            }
            SimpleType::Map(k, v) => {
                let nk = self.ty_of(k);
                let nv = self.ty_of(v);
                self.r.mk(Ty::Map(nk, nv))
            }
            // tensor extension TE-P1: element + static rank (validated by the
            // parser: only `float`/`int` elements are accepted)
            SimpleType::Tensor(el, rank) => {
                let ne = self.ty_of(el);
                self.r.mk(Ty::Tensor(ne, *rank))
            }
            SimpleType::Fn(f) => {
                let ps: Vec<TyId> = f.params.iter().map(|p| self.ty_of(p)).collect();
                let nr = self.ty_of(&f.ret);
                self.r.mk(Ty::Fn(FnTy {
                    params: ps,
                    ret: nr,
                    lam: None,
                }))
            }
            SimpleType::Dyn(t) => self.r.mk(Ty::Dyn(t.clone())),
            SimpleType::Named(n, args) => {
                let a: Vec<TyId> = args.iter().map(|t| self.ty_of(t)).collect();
                self.ty_named(n, a)
            }
            SimpleType::Ident(n) => {
                // generic type-param reference in a generic instance being emitted
                if let Some(frame) = self.tp_subst.last() {
                    if let Some(t) = frame.get(n) {
                        return *t;
                    }
                }
                let a: Vec<TyId> = Vec::new();
                self.ty_named(n, a)
            }
        }
    }
}

impl ModEmitter {
    pub(crate) fn ty_named(&mut self, n: &str, a: Vec<TyId>) -> TyId {
        match n {
            "int" | "i64" | "int64" => self.r.mk(Ty::I64),
            n if sloth_frontend::ty::IntKind::from_name(n).is_some() => self
                .r
                .mk(Ty::Int(sloth_frontend::ty::IntKind::from_name(n).unwrap())),
            "float" | "f64" => self.r.mk(Ty::F64),
            "bool" => self.r.mk(Ty::Bool),
            "str" => self.r.mk(Ty::Str),
            "range" => self.r.mk(Ty::Range),
            "any" => self.r.mk(Ty::Any),
            "Array" => {
                let el = match a.into_iter().next() {
                    Some(e) => e,
                    None => self.r.mk(Ty::Unit),
                };
                self.r.mk(Ty::Array(el))
            }
            "Map" => {
                let mut it = a.into_iter();
                let k = it.next().unwrap_or_else(|| self.r.mk(Ty::Unit));
                let v = it.next().unwrap_or_else(|| self.r.mk(Ty::Unit));
                self.r.mk(Ty::Map(k, v))
            }
            _ => {
                // trait name as a type position: an interface reference
                if a.is_empty() && self.traits.contains_key(n) {
                    return self.r.mk(Ty::Dyn(n.to_string()));
                }
                // Weak<T> reference module (patch 43): the weakbox handle set
                if n == "Weak" {
                    if let Some(e0) = a.into_iter().next() {
                        return self.r.mk(Ty::Weak(e0));
                    }
                    let u0 = self.r.mk(Ty::Unit);
                    return self.r.mk(Ty::Weak(u0));
                }
                // Fiber<Y> coroutine handle (CE): payload type Y
                if n == "Fiber" {
                    if let Some(e0) = a.into_iter().next() {
                        return self.r.mk(Ty::Fiber(e0));
                    }
                    let u0 = self.r.mk(Ty::Unit);
                    return self.r.mk(Ty::Fiber(u0));
                }
                // TH builtin surfaces: JoinHandle<R> / Channel<T> / Mutex /
                // AtomicInt (no source declaration required)
                if n == "JoinHandle" {
                    if let Some(e0) = a.into_iter().next() {
                        return self.r.mk(Ty::JoinHandle(e0));
                    }
                    let u0 = self.r.mk(Ty::Unit);
                    return self.r.mk(Ty::JoinHandle(u0));
                }
                if n == "Channel" {
                    if let Some(e0) = a.into_iter().next() {
                        return self.r.mk(Ty::Channel(e0));
                    }
                    let u0 = self.r.mk(Ty::Unit);
                    return self.r.mk(Ty::Channel(u0));
                }
                if n == "Mutex" && a.is_empty() {
                    return self.r.mk(Ty::Mutex);
                }
                if n == "AtomicInt" && a.is_empty() {
                    return self.r.mk(Ty::AtomicInt);
                }
                // generic class instance: C<A1,A2> -> monomorphic C_<A>_...
                if !a.is_empty() {
                    if let Some((_, cdef)) = self.class_defs.get(n).cloned() {
                        if !cdef.type_params.is_empty() {
                            return self.declare_class_inst(n, &a, &sloth_frontend::ast::eof_pos());
                        }
                    }
                }
                self.r.mk(Ty::Named(n.to_string(), a))
            }
        }
    }
}

impl ModEmitter {
    /// register (or fetch) the monomorphic instance of generic class `n`
    /// with text args `a`; fields are typed under the substitution frame
    pub(crate) fn declare_class_inst(&mut self, n: &str, a: &[TyId], pos: &Pos) -> TyId {
        let inst = format!("{}{}", n, mangle_t(a, &self.r));
        if self.class_ids.contains_key(&inst) {
            return self.r.mk(Ty::Named(inst.clone(), a.to_vec()));
        }
        let (defmod, cdef) = match self.class_defs.get(n).cloned() {
            Some(x) => x,
            None => return self.r.mk(Ty::Named(n.to_string(), a.to_vec())),
        };
        // generic-class type-param bounds (§2.3): the same check the generic
        // *function* path applies (bug B12/OOP-12)
        for (tp, ty) in cdef.type_params.iter().zip(a.iter()) {
            if let Some(bound) = &tp.bound {
                if !self.satisfies_bound(*ty, bound) {
                    self.err(
                        pos,
                        format!(
                            "type argument `{}` does not satisfy trait bound `{}`",
                            sloth_frontend::ty::ty_name(self.r.get(*ty)),
                            bound
                        ),
                    );
                }
            }
        }
        let mut frame: HashMap<String, TyId> = HashMap::new();
        for (tp, ty) in cdef.type_params.iter().zip(a.iter()) {
            frame.insert(tp.name.clone(), *ty);
        }
        let fields: Vec<(String, TyId, bool)> = {
            self.tp_subst.push(frame.clone());
            let f = cdef
                .fields
                .iter()
                .map(|fd| (fd.name.clone(), self.ty_of(&fd.ty), fd.mutable))
                .collect();
            self.tp_subst.pop();
            f
        };
        if !self.class_order.contains(&inst) {
            self.class_order.push(inst.clone());
        }
        self.native_cls_id += 1;
        let mut nid = self.native_cls_id;
        while self.class_ids.values().any(|&v| v == nid) {
            nid += 1;
        }
        self.class_ids.insert(inst.clone(), nid);
        let disp = if a.is_empty() {
            n.to_string()
        } else {
            let args: Vec<String> = a.iter().map(|x| self.pretty_ty(*x)).collect();
            format!("{}<{}>", n, args.join(", "))
        };
        self.cls_display.insert(inst.clone(), disp);
        self.cls_mod.insert(inst.clone(), defmod.clone());
        self.class_defs
            .insert(inst.clone(), (defmod.clone(), cdef.clone()));
        let meth: Vec<(String, FuncDef)> = cdef
            .methods
            .iter()
            .map(|m| (m.name.clone(), m.fd.clone()))
            .collect();
        self.classes.insert(
            inst.clone(),
            ClassInfo {
                name: inst.clone(),
                fields,
                methods: meth,
                superclass: cdef.superclass.clone(),
                impls: cdef.impls.clone(),
            },
        );
        self.register_class_vt_surface(&inst);
        if n == "Result" {
            self.result_insts.insert(inst.clone());
        }
        self.class_frames.insert(inst.clone(), frame.clone());
        // eager worklist (A1): every instance is emitted exactly once, whether
        // discovered during this pass's walk or loaded from the Pass-1 plan.
        if self.pending_cls_seen.insert(inst.clone()) {
            self.pending_insts.push((inst.clone(), frame));
        }
        self.r.mk(Ty::Named(inst.clone(), a.to_vec()))
    }
}

impl ModEmitter {
    pub fn is_float(&self, t: TyId) -> bool {
        matches!(self.r.get(t), Ty::F64)
    }
    pub fn is_str(&self, t: TyId) -> bool {
        matches!(self.r.get(t), Ty::Str)
    }
    /// integer surface info `(bits, signed)`; `int` is `(64, true)`
    pub fn int_info(&self, t: TyId) -> Option<(u32, bool)> {
        sloth_frontend::ty::int_info(self.r.get(t))
    }
    pub fn is_int_like(&self, t: TyId) -> bool {
        self.int_info(t).is_some()
    }
    pub fn is_unsigned_int(&self, t: TyId) -> bool {
        matches!(self.int_info(t), Some((_, false)))
    }
    /// common integer surface for a binary op: equal surfaces unify; an `int`
    /// literal adopts the other operand's fixed-width surface
    pub(crate) fn unify_int(&self, at: TyId, lit_a: bool, bt: TyId, lit_b: bool) -> Option<TyId> {
        if at == bt {
            return Some(at);
        }
        if self.is_int_like(at) && self.is_int_like(bt) {
            if lit_b && matches!(self.r.get(at), Ty::Int(_)) {
                return Some(at);
            }
            if lit_a && matches!(self.r.get(bt), Ty::Int(_)) {
                return Some(bt);
            }
        }
        None
    }
    pub fn is_unit(&self, t: TyId) -> bool {
        matches!(self.r.get(t), Ty::Unit)
    }
    /// Weak<T> face (patch 43): the weak box handle word set
    pub(crate) fn weak_inner(&self, t: TyId) -> Option<TyId> {
        match self.r.get(t).clone() {
            Ty::Weak(e) => Some(e),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn is_weak(&self, t: TyId) -> bool {
        self.weak_inner(t).is_some()
    }
    /// value-optional box face (patch 42): Some((inner,
    /// inner_is_float)) for the boxed `int? / float? / bool?` surfaces;
    /// reference optionals (str?/class?) stay word-view handle-or-0
    pub(crate) fn opt_inner(&self, t: TyId) -> Option<(TyId, bool)> {
        match self.r.get(t).clone() {
            Ty::Opt(e) => match self.r.get(e).clone() {
                Ty::F64 => Some((e, true)),
                Ty::I64 | Ty::Bool | Ty::Int(_) => Some((e, false)),
                _ => None,
            },
            _ => None,
        }
    }
    /// Send marker (design §4.2, first cut): a value is shareable across a
    /// thread boundary unless it contains a thread-confined `Fiber<Y>`.
    /// Shallow structural walk (class internals are not traversed — see
    /// design risk #1).
    pub(crate) fn send_ok(&self, t: TyId, depth: usize) -> bool {
        if depth > 8 {
            return true;
        }
        match self.r.get(t).clone() {
            Ty::Fiber(_) => false,
            Ty::Array(e) | Ty::Opt(e) | Ty::Weak(e) | Ty::JoinHandle(e) | Ty::Channel(e) => {
                self.send_ok(e, depth + 1)
            }
            Ty::Map(k, v) => self.send_ok(k, depth + 1) && self.send_ok(v, depth + 1),
            _ => true,
        }
    }

    pub fn is_ref(&self, t: TyId) -> bool {
        // `extern type` handles are opaque C pointers: never rc-managed
        if let Ty::Named(n, _) = self.r.get(t) {
            if self.extern_types.contains(n) {
                return false;
            }
        }
        match self.r.get(t) {
            // optionals are ref-shaped when their payload is (value optionals
            // ride an rc box; reference optionals are the bare handle)
            Ty::Opt(e) => self.is_ref(*e) || self.opt_inner(t).is_some(),
            _ => matches!(
                self.r.get(t),
                Ty::Str
                    | Ty::Array(_)
                    | Ty::Map(..)
                    | Ty::Tensor(..)
                    | Ty::Fn(_)
                    | Ty::Named(_, _)
                    | Ty::Dyn(_)
                    | Ty::Weak(_)
                    | Ty::Fiber(_)
                    | Ty::JoinHandle(_)
                    | Ty::Channel(_)
                    | Ty::Mutex
                    | Ty::AtomicInt
                    | Ty::Range
                    | Ty::Any
            ),
        }
    }

    /// can a value of this surface hold `nil`? Per design §2.1 only `T?` can,
    /// but `dyn` accepts `nil`, `Weak<T>` yields a 0 handle from a nil target,
    /// `Unit` is the `nil` literal / unknown surface, and a generic `Tp`
    /// placeholder is not yet resolved — all stay nil-testable.
    pub(crate) fn nil_capable(&self, t: TyId) -> bool {
        match self.r.get(t) {
            Ty::Opt(_) | Ty::Unit | Ty::Tp(_) | Ty::Dyn(_) | Ty::Weak(_) | Ty::Any => true,
            // an unresolved generic type parameter surfaces as a `Named` that
            // is not a declared class/trait/extern type (generic bodies are
            // emitted once as a template before monomorphization)
            Ty::Named(n, _) => {
                !self.classes.contains_key(n)
                    && !self.traits.contains_key(n)
                    && !self.extern_types.contains(n)
            }
            _ => false,
        }
    }
}

impl ModEmitter {
    /// map-key family tag used by map-literal family checking and key
    /// accumulation: distinct hashable families must not mix in one literal
    /// (book ch13 §13.1 lists `int`/`float`/`bool`/`str`/`range` as distinct).
    /// `Unit` is an already-reported error and stays neutral.
    pub(crate) fn map_key_family(&self, t: TyId) -> String {
        if matches!(self.r.get(t), Ty::Unit) {
            return "?".to_string();
        }
        if matches!(self.r.get(t), Ty::Bool) {
            return "bool".to_string();
        }
        if self.is_str(t) {
            return "str".to_string();
        }
        if self.is_float(t) {
            return "float".to_string();
        }
        if self.is_int_like(t) {
            return "int".to_string();
        }
        if matches!(self.r.get(t), Ty::Range) {
            return "range".to_string();
        }
        if matches!(self.r.get(t), Ty::Named(_, _)) {
            return "class".to_string();
        }
        format!("t{:?}", self.r.get(t))
    }

    /// structural surface compatibility (patch #22): equal-by-interning,
    /// nil (word 0) into anything, Opt target lenient (word view), dyn
    /// target accepts concrete class instances, element-wise arrays/maps.
    pub(crate) fn surface_compat(&self, a: &Ty, b: &Ty) -> bool {
        if a == b {
            return true;
        }
        match (a, b) {
            (_, Ty::Unit) => true,
            // `any` is the top type: every source surface coerces into it
            (Ty::Any, _) => true,
            (Ty::Opt(..), _) => true,
            // value-optional (boxed) and Weak surfaces are coercible store
            // faces (patch 42/43): wrap at bind time
            (Ty::Weak(..), _) => true,
            // `dyn T` is a nominal surface: only a class that declares `impl T`
            // (directly or through its chain) is accepted (design §19.4)
            (Ty::Dyn(t), Ty::Named(c, _)) => self.impl_chain_has(c, t),
            // builtin value types auto-box into a dyn surface when the trait
            // is predefined (or has no methods to satisfy)
            (Ty::Dyn(t), Ty::I64)
            | (Ty::Dyn(t), Ty::Int(_))
            | (Ty::Dyn(t), Ty::F64)
            | (Ty::Dyn(t), Ty::Bool) => self.value_impls_trait(t),
            // integer surfaces are mutually assignable (widening/truncation is
            // inserted at the store face by `coerce_int_word`)
            (Ty::I64, Ty::Int(_)) | (Ty::Int(_), Ty::I64) => true,
            (Ty::Int(_), Ty::Int(_)) => true,
            // function surfaces compare structurally (lambda metadata — the
            // closure frame symbol — must not defeat compatibility)
            (Ty::Fn(x), Ty::Fn(y)) => {
                x.params.len() == y.params.len()
                    && x.params
                        .iter()
                        .zip(y.params.iter())
                        .all(|(p, q)| self.surface_compat(self.r.get(*p), self.r.get(*q)))
                    && self.surface_compat(self.r.get(x.ret), self.r.get(y.ret))
            }
            (Ty::Array(x), Ty::Array(y)) => self.surface_compat(self.r.get(*x), self.r.get(*y)),
            (Ty::Fiber(x), Ty::Fiber(y)) => self.surface_compat(self.r.get(*x), self.r.get(*y)),
            (Ty::JoinHandle(x), Ty::JoinHandle(y)) => {
                self.surface_compat(self.r.get(*x), self.r.get(*y))
            }
            (Ty::Channel(x), Ty::Channel(y)) => self.surface_compat(self.r.get(*x), self.r.get(*y)),
            // tensor surfaces require element AND rank to match exactly
            (Ty::Tensor(x, rx), Ty::Tensor(y, ry)) => {
                rx == ry && self.surface_compat(self.r.get(*x), self.r.get(*y))
            }
            (Ty::Map(k1, v1), Ty::Map(k2, v2)) => {
                self.surface_compat(self.r.get(*k1), self.r.get(*k2))
                    && self.surface_compat(self.r.get(*v1), self.r.get(*v2))
            }
            (Ty::Named(p, _), Ty::Named(c, _)) => {
                // same mangled instance name covers Result_int_str([args] vs []),
                // else subclass-targets-superclass chain
                if p == c {
                    true
                } else {
                    self.class_chain_has(c, p)
                }
            }
            _ => false,
        }
    }
}

impl ModEmitter {
    /// nearest common ancestor of two class names (identity counts)
    pub(crate) fn common_ancestor(&self, a: &str, b: &str) -> Option<String> {
        let mut chain_a: Vec<String> = Vec::new();
        let mut cur = Some(a.to_string());
        while let Some(c) = cur {
            chain_a.push(c.clone());
            cur = self.classes.get(&c).and_then(|ci| ci.superclass.clone());
        }
        let mut cur = Some(b.to_string());
        while let Some(c) = cur {
            if chain_a.contains(&c) {
                return Some(c);
            }
            cur = self.classes.get(&c).and_then(|ci| ci.superclass.clone());
        }
        None
    }

    /// least upper bound of a list of class-valued element types (None when
    /// any element is not a class or no common ancestor exists)
    pub(crate) fn named_lub(&self, ets: &[TyId]) -> Option<String> {
        let mut acc: Option<String> = None;
        for t in ets {
            let n = match self.r.get(*t).clone() {
                Ty::Named(n, _) => n,
                _ => return None,
            };
            acc = Some(match acc {
                None => n,
                Some(p) => self.common_ancestor(&p, &n)?,
            });
        }
        acc
    }
}

impl ModEmitter {
    /// does the superclass chain of `cls` include `base`? (is-a ranking)
    pub(crate) fn class_chain_has(&self, cls: &str, target: &str) -> bool {
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            if c == target {
                return true;
            }
            match self.classes.get(&c) {
                Some(ci) => cur = ci.superclass.clone(),
                None => return false,
            }
        }
        false
    }
}

impl ModEmitter {
    /// fully recursive readable type name for the `type_name` builtin
    pub(crate) fn pretty_ty(&self, t: TyId) -> String {
        match self.r.get(t) {
            Ty::Unit => "unit".to_string(),
            Ty::Bool => "bool".to_string(),
            Ty::I64 => "int".to_string(),
            Ty::Int(k) => k.name().to_string(),
            Ty::F64 => "float".to_string(),
            Ty::Str => "str".to_string(),
            Ty::Range => "range".to_string(),
            Ty::Array(e) => format!("Array<{}>", self.pretty_ty(*e)),
            Ty::Map(k, v) => format!("Map<{}, {}>", self.pretty_ty(*k), self.pretty_ty(*v)),
            Ty::Tensor(e, r) => format!("Tensor<{}, {}>", self.pretty_ty(*e), r),
            Ty::Fn(f) => {
                let ps: Vec<String> = f.params.iter().map(|p| self.pretty_ty(*p)).collect();
                format!("({}) -> {}", ps.join(", "), self.pretty_ty(f.ret))
            }
            Ty::Named(n, a) => {
                // prefer the registered display surface (`type_name` uses it too)
                // so `Box<int>` is not rendered as the mangled `Box_int` (H4)
                if let Some(d) = self.cls_display.get(n) {
                    if !a.is_empty() && !d.contains('<') {
                        let args: Vec<String> = a.iter().map(|x| self.pretty_ty(*x)).collect();
                        return format!("{}<{}>", d, args.join(", "));
                    }
                    return d.clone();
                }
                if a.is_empty() {
                    n.clone()
                } else {
                    let args: Vec<String> = a.iter().map(|x| self.pretty_ty(*x)).collect();
                    format!("{}<{}>", n, args.join(", "))
                }
            }
            Ty::Opt(e) => format!("{}?", self.pretty_ty(*e)),
            Ty::Weak(e) => format!("Weak<{}>", self.pretty_ty(*e)),
            Ty::Fiber(e) => format!("Fiber<{}>", self.pretty_ty(*e)),
            Ty::JoinHandle(e) => format!("JoinHandle<{}>", self.pretty_ty(*e)),
            Ty::Channel(e) => format!("Channel<{}>", self.pretty_ty(*e)),
            Ty::Mutex => "Mutex".to_string(),
            Ty::AtomicInt => "AtomicInt".to_string(),
            Ty::Any => "any".to_string(),
            Ty::Dyn(n) => format!("dyn {}", n),
            Ty::Tp(n) => n.clone(),
        }
    }

    /// diagnostic-friendly surface name: composites spill their element kinds
    pub(crate) fn surface_name(&self, t: &Ty) -> String {
        match t {
            Ty::Array(e) => {
                format!("Array<{}>", sloth_frontend::ty::ty_name(self.r.get(*e)))
            }
            Ty::Tensor(e, r) => {
                format!(
                    "Tensor<{}, {}>",
                    sloth_frontend::ty::ty_name(self.r.get(*e)),
                    r
                )
            }
            Ty::Map(k, v) => {
                format!(
                    "Map<{}, {}>",
                    sloth_frontend::ty::ty_name(self.r.get(*k)),
                    sloth_frontend::ty::ty_name(self.r.get(*v))
                )
            }
            Ty::Fiber(e) => {
                format!("Fiber<{}>", sloth_frontend::ty::ty_name(self.r.get(*e)))
            }
            Ty::JoinHandle(e) => {
                format!(
                    "JoinHandle<{}>",
                    sloth_frontend::ty::ty_name(self.r.get(*e))
                )
            }
            Ty::Channel(e) => {
                format!("Channel<{}>", sloth_frontend::ty::ty_name(self.r.get(*e)))
            }
            other => sloth_frontend::ty::ty_name(other),
        }
    }
}

impl ModEmitter {
    /// structural surface check for object-field assignment (float routes are
    /// handled by the callers before this is reached; Unit value = unknown)
    pub(crate) fn check_field_surface(&mut self, pos: &Pos, fname: &str, fty: TyId, vty: TyId) {
        if self.is_float(fty) || self.is_float(vty) {
            return;
        }
        let vts = self.r.get(vty).clone();
        if matches!(vts, Ty::Unit) {
            return;
        }
        let fts = self.r.get(fty).clone();
        if !self.surface_compat(&fts, &vts) {
            let ftn = self.surface_name(&fts);
            let vtn = self.surface_name(&vts);
            self.err_diff(pos, &format!("field assignment `{}`", fname), &ftn, &vtn);
        }
    }
}

impl ModEmitter {
    /// does a class chain (cls + superclasses) implement trait `tr`?
    pub(crate) fn impl_chain_has(&self, cls: &str, tr: &str) -> bool {
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    if ci.impls.iter().any(|x| x == tr) {
                        return true;
                    }
                    cur = ci.superclass.clone();
                }
                None => return false,
            }
        }
        false
    }
}

impl ModEmitter {
    /// predefined trait surface: builtin kinds satisfy these without declares
    pub(crate) fn is_predef_trait(bound: &str) -> bool {
        matches!(bound, "Hashable" | "Equatable" | "Comparable" | "Display")
    }

    /// builtin type surface names usable as `is` right side (patch #32)
    pub(crate) fn is_builtin_type_name(n: &str) -> bool {
        matches!(
            n,
            "int" | "i64" | "int64" | "float" | "f64" | "str" | "bool" | "range"
        ) || sloth_frontend::ty::IntKind::from_name(n).is_some()
    }

    /// resolved `Ty` for a builtin type surface name (used by `is` narrowing)
    pub(crate) fn builtin_name_ty(n: &str) -> Option<Ty> {
        Some(match n {
            "int" | "i64" | "int64" => Ty::I64,
            "float" | "f64" => Ty::F64,
            "bool" => Ty::Bool,
            "str" => Ty::Str,
            "range" => Ty::Range,
            other => Ty::Int(sloth_frontend::ty::IntKind::from_name(other)?),
        })
    }

    /// map-key hash method (patch #35): the class chain's `hash()`/`hashKey`/
    /// `__hash__` (0 params, int return) per the Hashable contract
    pub(crate) fn find_map_key_hash(&mut self, cls: &str) -> Option<(String, String, FuncDef)> {
        for m in ["hash", "hashKey", "__hash__"] {
            if let Some((defcls, fd)) = self.find_method(cls, m) {
                if fd.params.is_empty()
                    && fd
                        .ret
                        .as_ref()
                        .map(|t| matches!(t, Type::Simple(SimpleType::Int)))
                        .unwrap_or(false)
                {
                    return Some((m.to_string(), defcls, fd));
                }
            }
        }
        None
    }
}

impl ModEmitter {
    /// does a type satisfy the trait bound? (builtin kinds cover predefined
    /// traits; user classes need the impl chain; Opt looks through)
    pub(crate) fn satisfies_bound(&self, t: TyId, bound: &str) -> bool {
        match self.r.get(t).clone() {
            Ty::I64
            | Ty::Int(_)
            | Ty::F64
            | Ty::Str
            | Ty::Bool
            | Ty::Range
            | Ty::Array(_)
            | Ty::Map(_, _) => Self::is_predef_trait(bound),
            Ty::Named(cls, _) => self.impl_chain_has(&cls, bound),
            Ty::Opt(e) => self.satisfies_bound(e, bound),
            // a `dyn T` value carries exactly trait T's surface
            Ty::Dyn(t) => t == bound,
            _ => false,
        }
    }
}

impl ModEmitter {
    /// float word? for a syntactic param/ret type (None / unit -> i64)
    pub(crate) fn sig_word_float(&self, t: Option<Type>) -> bool {
        match t {
            Some(ty) => !matches!(ty, Type::Unit) && matches!(ty, Type::Simple(SimpleType::Float)),
            None => false,
        }
    }
}

impl ModEmitter {
    /// build one fresh Array<T> word from already-coerced words (variadic pack)
    /// shape walker: declared param ty -> matchable Ty, holes as Named(T)
    pub(crate) fn shape_of(&mut self, pt: &Type, tnames: &[String]) -> TyId {
        match pt {
            Type::Simple(SimpleType::Ident(n)) if tnames.contains(n) => {
                self.r.mk(Ty::Named(n.to_string(), Vec::new()))
            }
            Type::Simple(SimpleType::Array(el)) if matches!(el.as_ref(), Type::Simple(SimpleType::Ident(n)) if tnames.contains(n)) => {
                match el.as_ref() {
                    Type::Simple(SimpleType::Ident(n)) => {
                        let tn = n.to_string();
                        let hole = self.r.mk(Ty::Named(tn, Vec::new()));
                        self.r.mk(Ty::Array(hole))
                    }
                    _ => unreachable!(),
                }
            }
            Type::Simple(SimpleType::Map(k, v)) => {
                let pk = self.shape_of(k, tnames);
                let pv = self.shape_of(v, tnames);
                self.r.mk(Ty::Map(pk, pv))
            }
            Type::Optional(inner) => self.shape_of(inner, tnames),
            other => self.ty_of(other),
        }
    }
}

impl ModEmitter {
    pub(crate) fn unify_tp(
        &mut self,
        tnames: &[TypeParam],
        pat: TyId,
        act: TyId,
        map: &mut std::collections::HashMap<String, TyId>,
    ) {
        let pt = self.r.get(pat).clone();
        let atc = self.r.get(act).clone();
        match (pt, atc) {
            (Ty::Named(n, a), _) if a.is_empty() && tnames.iter().any(|p| p.name == n) => {
                map.entry(n).or_insert(act);
            }
            (Ty::Array(pe), Ty::Array(ae)) => self.unify_tp(tnames, pe, ae, map),
            (Ty::Fn(pf), Ty::Fn(af)) => {
                // unify a function-typed parameter: `(T) -> R` binds T from the
                // parameter types and R from the return type
                for (p, a) in pf.params.iter().zip(af.params.iter()) {
                    self.unify_tp(tnames, *p, *a, map);
                }
                self.unify_tp(tnames, pf.ret, af.ret, map);
            }
            (Ty::Fiber(pe), Ty::Fiber(ae)) => self.unify_tp(tnames, pe, ae, map),
            (Ty::Opt(pe), Ty::Opt(ae)) => self.unify_tp(tnames, pe, ae, map),
            (Ty::Map(pk, pv), Ty::Map(ak, av)) => {
                self.unify_tp(tnames, pk, ak, map);
                self.unify_tp(tnames, pv, av, map);
            }
            _ => {}
        }
    }
}

impl ModEmitter {
    /// declared return surface as a matchable pie shape with T holes
    /// (patch #38 return-driven inference)
    pub(crate) fn shape_of_retched(&mut self, ret: Option<Type>, tnames: &[String]) -> TyId {
        match ret {
            Some(t) => self.shape_of(&t, tnames),
            None => self.r.mk(Ty::Unit),
        }
    }
}

impl ModEmitter {
    /// a trait name in a *type* position is not a declared class: the surface
    /// must be spelled `dyn Trait` (book ch19 §19.4). Recurses through composite
    /// type expressions so `Array<Animal>` / `(Animal) -> unit` are caught too.
    pub(crate) fn check_trait_type(&mut self, t: &Type, pos: &Pos) {
        match t {
            Type::Unit => {}
            Type::Optional(i) => self.check_trait_type(i, pos),
            Type::Simple(s) => match s {
                SimpleType::Ident(n) | SimpleType::Named(n, _) if self.traits.contains_key(n) => {
                    self.err(
                        pos,
                        format!("trait `{}` cannot be used as a type; use `dyn {}`", n, n),
                    );
                    // still recurse into any type arguments
                    if let SimpleType::Named(_, args) = s {
                        for a in args {
                            self.check_trait_type(a, pos);
                        }
                    }
                }
                SimpleType::Named(_, args) => {
                    for a in args {
                        self.check_trait_type(a, pos);
                    }
                }
                SimpleType::Array(e) | SimpleType::Tensor(e, _) => self.check_trait_type(e, pos),
                SimpleType::Map(k, v) => {
                    self.check_trait_type(k, pos);
                    self.check_trait_type(v, pos);
                }
                SimpleType::Fn(f) => {
                    for p in &f.params {
                        self.check_trait_type(p, pos);
                    }
                    self.check_trait_type(&f.ret, pos);
                }
                SimpleType::Dyn(_) => {}
                _ => {}
            },
        }
    }
}

impl ModEmitter {
    /// structured expected/got diff diagnostic (patch #40): replaces the
    /// word-face one-liners across assignment/initializer/field faces
    pub(crate) fn err_diff(&mut self, pos: &Pos, ctx: &str, expected: &str, got: &str) {
        self.err(
            pos,
            format!(
                "type mismatch in {} at line {}\n  expected: {}\n  got: {}",
                ctx, pos.line, expected, got
            ),
        );
    }

    /// condition face: no implicit truthy conversion (structured diff too)
    pub(crate) fn err_cond_bool(&mut self, pos: &Pos, got: &str) {
        self.err(
            pos,
            format!(
                "type mismatch in condition at line {}\n  expected: bool\n  got: {}\n  (condition must be `bool`; no implicit truthy conversion)",
                pos.line, got
            ),
        );
    }
}
