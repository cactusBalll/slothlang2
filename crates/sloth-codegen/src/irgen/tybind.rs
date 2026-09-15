//! Type registry: `ty_of`, surface/assignability & trait-bound checks.

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
            SimpleType::Float => self.r.mk(Ty::F64),
            SimpleType::Str => self.r.mk(Ty::Str),
            SimpleType::Range => self.r.mk(Ty::Range),
            SimpleType::Array(e) => {
                let ne = self.ty_of(e);
                self.r.mk(Ty::Array(ne))
            }
            SimpleType::Map(k, v) => {
                let nk = self.ty_of(k);
                let nv = self.ty_of(v);
                self.r.mk(Ty::Map(nk, nv))
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
            other => {
                let _ = other;
                self.r.mk(Ty::Unit)
            }
        }
    }
}

impl ModEmitter {
    pub(crate) fn ty_named(&mut self, n: &str, a: Vec<TyId>) -> TyId {
        match n {
            "int" | "i64" => self.r.mk(Ty::I64),
            "float" | "f64" => self.r.mk(Ty::F64),
            "bool" => self.r.mk(Ty::Bool),
            "str" => self.r.mk(Ty::Str),
            "range" => self.r.mk(Ty::Range),
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
                // generic class instance: C<A1,A2> -> monomorphic C_<A>_...
                if !a.is_empty() {
                    if let Some((_, cdef)) = self.class_defs.get(n).cloned() {
                        if !cdef.type_params.is_empty() {
                            return self.declare_class_inst(n, &a);
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
    pub(crate) fn declare_class_inst(&mut self, n: &str, a: &[TyId]) -> TyId {
        let inst = format!("{}{}", n, mangle_t(a, &self.r));
        if self.class_ids.contains_key(&inst) {
            return self.r.mk(Ty::Named(inst.clone(), a.to_vec()));
        }
        let (defmod, cdef) = match self.class_defs.get(n).cloned() {
            Some(x) => x,
            None => return self.r.mk(Ty::Named(n.to_string(), a.to_vec())),
        };
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
        if n == "Result" {
            self.result_insts.insert(inst.clone());
        }
        self.class_frames.insert(inst.clone(), frame.clone());
        self.pending_insts.push((inst.clone(), frame));
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
    pub fn is_unit(&self, t: TyId) -> bool {
        matches!(self.r.get(t), Ty::Unit)
    }
    pub fn is_ref(&self, t: TyId) -> bool {
        matches!(
            self.r.get(t),
            Ty::Str | Ty::Array(_) | Ty::Map(..) | Ty::Fn(_) | Ty::Named(_, _) | Ty::Dyn(_)
        )
    }
}

impl ModEmitter {
    /// structural surface compatibility (patch #22): equal-by-interning,
    /// nil (word 0) into anything, Opt target lenient (word view), dyn
    /// target accepts concrete class instances, element-wise arrays/maps.
    pub(crate) fn surface_compat(&self, a: &Ty, b: &Ty) -> bool {
        if a == b {
            return true;
        }
        match (a, b) {
            (_, Ty::Unit) => true,
            (Ty::Opt(..), _) => true,
            (Ty::Dyn(_), Ty::Named(..)) => true,
            (Ty::Array(x), Ty::Array(y)) => self.surface_compat(self.r.get(*x), self.r.get(*y)),
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
    /// diagnostic-friendly surface name: composites spill their element kinds
    pub(crate) fn surface_name(&self, t: &Ty) -> String {
        match t {
            Ty::Array(e) => {
                format!("Array<{}>", sloth_frontend::ty::ty_name(self.r.get(*e)))
            }
            Ty::Map(k, v) => {
                format!(
                    "Map<{}, {}>",
                    sloth_frontend::ty::ty_name(self.r.get(*k)),
                    sloth_frontend::ty::ty_name(self.r.get(*v))
                )
            }
            other => sloth_frontend::ty::ty_name(other),
        }
    }
}

impl ModEmitter {
    /// plain-name assignment checked against the declared/inferred surface
    /// type recorded at declare time (patch #22): float target promotes int
    /// words; float value into non-float target diagnosed; structurally
    /// different i64-word surfaces (int/str/bool/class/array/map) diagnosed;
    /// nil (word 0) accepted into any non-float target.
    pub(crate) fn check_named_assign(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        dt: TyId,
        v: &str,
        vty: TyId,
        pos: &Pos,
    ) {
        if self.is_float(dt) {
            if self.is_float(vty) {
                fw.assign(name, v, true);
            } else {
                let cv = fw.v();
                fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                fw.assign(name, &cv, true);
            }
            return;
        }
        if self.is_float(vty) {
            let dtn = sloth_frontend::ty::ty_name(self.r.get(dt));
            self.err(
                pos,
                format!(
                    "type mismatch: cannot assign `float` to `{}` (`{}`)",
                    dtn, name
                ),
            );
            // keep IR parseable: store with the value's own float spelling
            fw.assign(name, v, true);
            return;
        }
        let dts = self.r.get(dt).clone();
        let vts = self.r.get(vty).clone();
        if self.surface_compat(&dts, &vts) {
            fw.assign(name, v, false);
        } else {
            let dtn = self.surface_name(&dts);
            let vtn = self.surface_name(&vts);
            self.err(
                pos,
                format!(
                    "type mismatch: cannot assign `{}` to `{}` variable `{}`",
                    vtn, dtn, name
                ),
            );
            fw.assign(name, v, false);
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
}

impl ModEmitter {
    /// does a type satisfy the trait bound? (builtin kinds cover predefined
    /// traits; user classes need the impl chain; Opt looks through)
    pub(crate) fn satisfies_bound(&self, t: TyId, bound: &str) -> bool {
        match self.r.get(t).clone() {
            Ty::I64 | Ty::F64 | Ty::Str | Ty::Bool | Ty::Range | Ty::Array(_) | Ty::Map(_, _) => {
                Self::is_predef_trait(bound)
            }
            Ty::Named(cls, _) => self.impl_chain_has(&cls, bound),
            Ty::Opt(e) => self.satisfies_bound(e, bound),
            _ => false,
        }
    }
}

impl ModEmitter {
    /// typed-bound lint in builtin positions with a clearer message context
    pub(crate) fn satisfies_bound_check(&mut self, pos: &Pos, t: &TyId, bound: &str, ctx: &str) {
        let ok = self.satisfies_bound(*t, bound);
        if !ok {
            self.err(
                pos,
                format!(
                    "`{}` requires trait bound `{}` (`{}` does not satisfy it)",
                    ctx,
                    bound,
                    sloth_frontend::ty::ty_name(self.r.get(*t)),
                ),
            );
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
            (Ty::Opt(pe), Ty::Opt(ae)) => self.unify_tp(tnames, pe, ae, map),
            (Ty::Map(pk, pv), Ty::Map(ak, av)) => {
                self.unify_tp(tnames, pk, ak, map);
                self.unify_tp(tnames, pv, av, map);
            }
            _ => {}
        }
    }
}
