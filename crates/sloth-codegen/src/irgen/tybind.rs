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
                // Weak<T> reference module (patch 43): the weakbox handle set
                if n == "Weak" {
                    if let Some(e0) = a.into_iter().next() {
                        return self.r.mk(Ty::Weak(e0));
                    }
                    let u0 = self.r.mk(Ty::Unit);
                    return self.r.mk(Ty::Weak(u0));
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
                Ty::I64 | Ty::Bool => Some((e, false)),
                _ => None,
            },
            _ => None,
        }
    }
    pub(crate) fn is_opt_val(&self, t: TyId) -> bool {
        self.opt_inner(t).is_some()
    }
    pub fn is_ref(&self, t: TyId) -> bool {
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
                    | Ty::Range
            ),
        }
    }
}

impl ModEmitter {
    // ---------------- rc machinery (ARC migration, patch B) ----------------

    /// emit `sloth_rc_release(h)` (nil and untracked words are rt no-ops)
    pub(crate) fn emit_release(&mut self, fw: &mut FnWalk, h: &str) {
        fw.op(&format!("    call @sloth_rc_release({}) : (i64) -> i64", h));
    }

    /// emit `sloth_rc_retain(h)` (value-preserving)
    pub(crate) fn emit_retain(&mut self, fw: &mut FnWalk, h: &str) -> String {
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_rc_retain({}) : (i64) -> i64",
            r, h
        ));
        r
    }

    /// load the current word stored in a slot alloca (i64 route; ref words
    /// never live in float slots)
    pub(crate) fn load_slot(&mut self, fw: &mut FnWalk, a: &str) -> String {
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        let w = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            w, a, z
        ));
        w
    }

    /// assignment to a declared name: release the overload word first, then
    /// store (unconditional — rt no-ops for non-ref/nil words)
    #[allow(dead_code)]
    pub(crate) fn rc_assign_slot(&mut self, fw: &mut FnWalk, a: &str) {
        let old = self.load_slot(fw, a);
        self.emit_release(fw, &old);
    }

    /// declare bookkeeping (call at fw.declare sites): a ref-typed local's
    /// alloca joins this scope's release set
    #[allow(dead_code)]
    pub(crate) fn declare_rc(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        t: TyId,
        fl: bool,
        mutable: bool,
    ) -> String {
        let a = fw.declare(name, t, fl, mutable);
        if self.is_ref(t) {
            fw.scope_decls
                .last_mut()
                .unwrap()
                .insert(name.to_string(), a.clone());
        }
        a
    }

    /// count a freshly created handle as a statement-dangling temp: the
    /// producer owns it; released once after the enclosing statement ends
    pub(crate) fn dangling_producer(&mut self, fw: &mut FnWalk, h: &str, t: TyId) {
        if self.is_ref(t) {
            fw.dangling.push(h.to_string());
        }
    }

    // -------- value-optional box coercions (patch 42) --------

    /// wrap a produced word into its value-optional surface (`int?` etc):
    /// a nil/Unit word passes through as nil (0); a bare scalar is boxed
    /// (`sloth_box_new[_f64]`), an already-opt word passes through (idempotent)
    pub(crate) fn coerce_into_opt(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
        to: TyId,
    ) -> (String, TyId) {
        let (_inner, fli) = match self.opt_inner(to) {
            Some(x) => x,
            None => return (v.to_string(), from),
        };
        let froms = self.r.get(from).clone();
        if matches!(froms, Ty::Unit) || from == to {
            return (v.to_string(), to);
        }
        if matches!(froms, Ty::I64 | Ty::Bool) {
            // int/bool word boxes as-is; into a float? surface promote first
            let payload = if fli {
                iw_to_f64_word(fw, v)
            } else {
                v.to_string()
            };
            let r = fw.v();
            fw.op(&format!(
                "    {} = call @sloth_box_new({}) : (i64) -> i64",
                r, payload
            ));
            self.dangling_producer(fw, &r, to);
            return (r, to);
        }
        if froms == Ty::F64 {
            if fli {
                // f64 word boxes as-is (the box holds the encoded word)
                let r = fw.v();
                fw.op(&format!(
                    "    {} = call @sloth_box_new({}) : (i64) -> i64",
                    r, v
                ));
                self.dangling_producer(fw, &r, to);
                return (r, to);
            }
            // float word into int?/bool?: word-view fallback, no box
            return (v.to_string(), to);
        }
        if matches!(froms, Ty::Opt(_)) {
            // already-boxed word of another inner family: unwrap, promote,
            // rebox into the target family
            let (p, pt) = self.unwrap_opt_word(fw, v, from);
            let (r, _t2) = self.coerce_into_opt(fw, &p, pt, to);
            return (r, to);
        }
        // word-view fallback (cross optional families / incompatible words)
        (v.to_string(), to)
    }

    /// read an optional word as its inner payload (nil reads as 0/0.0 —
    /// unwrap-or-0 semantics keeps the historical word view behavior)
    pub(crate) fn unwrap_opt_word(&mut self, fw: &mut FnWalk, v: &str, t: TyId) -> (String, TyId) {
        let (inner, fli) = match self.opt_inner(t) {
            Some(x) => x,
            None => return (v.to_string(), t),
        };
        let _ = fli;
        // tag migration: the box holds one tagged payload word
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_box_get({}) : (i64) -> i64",
            r, v
        ));
        (r, inner)
    }

    /// caller-side coercion of args to Opt(值型) params (patch 42): bare
    /// scalars box, opt/nil words pass through; other pairs unchanged;
    /// Weak(值型)-typed params (patch 43) wrap their targets too
    pub(crate) fn coerce_args_to_params(
        &mut self,
        fw: &mut FnWalk,
        argv: &[(String, TyId)],
        params: &[(String, TyId, bool)],
    ) -> Vec<String> {
        let n = argv.len().min(params.len());
        (0..argv.len())
            .map(|i| {
                if i < n
                    && (self.opt_inner(params[i].1).is_some()
                        || self.weak_inner(params[i].1).is_some())
                {
                    self.coerce_word_to(fw, &argv[i].0, argv[i].1, params[i].1)
                        .0
                } else {
                    argv[i].0.clone()
                }
            })
            .collect()
    }

    /// bind a word into a declared surface (patch 42/43 entry): value
    /// optionals box up, Weak targets wrap in a weak box, else as-is
    pub(crate) fn coerce_word_to(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
        to: TyId,
    ) -> (String, TyId) {
        if self.opt_inner(to).is_some() {
            return self.coerce_into_opt(fw, v, from, to);
        }
        if self.weak_inner(to).is_some() {
            return self.coerce_into_weak(fw, v, from, to);
        }
        (v.to_string(), from)
    }

    /// wrap a produced word into a Weak<T> surface (patch 43): a weakbox
    /// (rc-tracked, malloc'd) holding the (possibly boxed) target; nil
    /// passes through as word 0; already-weak words ride along
    pub(crate) fn coerce_into_weak(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
        to: TyId,
    ) -> (String, TyId) {
        let inner = match self.weak_inner(to) {
            Some(e) => e,
            None => return (v.to_string(), from),
        };
        let froms = self.r.get(from).clone();
        if matches!(froms, Ty::Unit) || froms == Ty::Weak(inner) {
            return (v.to_string(), to);
        }
        let check = self.r.mk(Ty::Opt(inner));
        let (targ, _tt) = self.coerce_word_to(fw, v, from, check);
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_weak_new({}) : (i64) -> i64",
            r, targ
        ));
        self.dangling_producer(fw, &r, to);
        (r, to)
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
            // value-optional (boxed) and Weak surfaces are coercible store
            // faces (patch 42/43): wrap at bind time
            (Ty::Weak(..), _) => true,
            (Ty::Dyn(_), Ty::Named(..)) => true,
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
        // value-optional surfaces (patch 42): wrap bare scalars into boxes,
        // keep nil (Unit) / already-opt words as they are, then the common
        // i64 store path below releases the old and retains the new owner
        let mut v = v.to_string();
        let mut vty = vty;
        if self.opt_inner(dt).is_some() || self.weak_inner(dt).is_some() {
            let (vc, vtc) = self.coerce_word_to(fw, &v, vty, dt);
            v = vc;
            vty = vtc;
        }
        if self.is_float(dt) {
            if self.is_float(vty) {
                fw.assign(name, &v, true);
            } else {
                // int word -> f64 word (slot storage is always the word plane)
                let cv = iw_to_f64_word(fw, &v);
                fw.assign(name, &cv, true);
            }
            return;
        }
        if self.is_float(vty) && self.opt_inner(dt).is_none() {
            let dtn = sloth_frontend::ty::ty_name(self.r.get(dt));
            self.err_diff(pos, &format!("assignment to `{}`", name), "float", &dtn);
            // keep IR parseable: store with the value's own float spelling
            fw.assign(name, &v, true);
            return;
        }
        let dts = self.r.get(dt).clone();
        let vts = self.r.get(vty).clone();
        if !self.surface_compat(&dts, &vts) {
            let dtn = self.surface_name(&dts);
            let vtn = self.surface_name(&vts);
            self.err_diff(pos, &format!("assignment to `{}`", name), &dtn, &vtn);
        }
        // rc patch B: release the overwritten word, retain the new owner's
        // copy (nil/untracked = rt no-ops). Loop variables are BORROWS of
        // container elements (patch C): their slot owns no count.
        // patch 42: transferred call-result words already carry their +1 —
        // bind them raw instead of retaining a second count
        let xferred = fw.rc_take_xfer(&v);
        match fw.lookup(name) {
            Some((a, _)) if !fw.loopvars.contains(&name.to_string()) => {
                let old = self.load_slot(fw, &a);
                self.emit_release(fw, &old);
                if xferred {
                    fw.assign(name, &v, false);
                } else {
                    let rv = self.emit_retain(fw, &v);
                    fw.assign(name, &rv, false);
                }
            }
            _ => {
                fw.assign(name, &v, false);
            }
        }
    }

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

    /// store a tagged word into a module-level global cell
    pub(crate) fn store_global(&self, fw: &mut FnWalk, gsym: &str, dt: TyId, val: &str) {
        let mty = memref_cell_ty(self, dt);
        let g = fw.v();
        fw.op(&format!(
            "    {} = memref.get_global @{} : {}",
            g, gsym, mty
        ));
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        fw.op(&format!("    memref.store {}, {}[{}] : {}", val, g, z, mty));
    }

    /// plain-name assignment to a module-level global cell: mirrors
    /// `check_named_assign` (coercion + surface check + rc overwrite) but
    /// stores through `memref.get_global` instead of a local slot.
    pub(crate) fn check_global_assign(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        gsym: &str,
        dt: TyId,
        v: &str,
        vty: TyId,
        pos: &Pos,
    ) {
        let mut v = v.to_string();
        let mut vty = vty;
        if self.opt_inner(dt).is_some() || self.weak_inner(dt).is_some() {
            let (vc, vtc) = self.coerce_word_to(fw, &v, vty, dt);
            v = vc;
            vty = vtc;
        }
        if self.is_float(dt) {
            let cv = if self.is_float(vty) {
                v.clone()
            } else {
                iw_to_f64_word(fw, &v)
            };
            self.store_global(fw, gsym, dt, &cv);
            return;
        }
        if self.is_float(vty) && self.opt_inner(dt).is_none() {
            let dtn = sloth_frontend::ty::ty_name(self.r.get(dt));
            self.err_diff(pos, &format!("assignment to `{}`", name), "float", &dtn);
            self.store_global(fw, gsym, dt, &v);
            return;
        }
        // unannotated global (`dt == Unit`) stays word-lenient
        let dts = self.r.get(dt).clone();
        if !matches!(dts, Ty::Unit) {
            let vts = self.r.get(vty).clone();
            if !self.surface_compat(&dts, &vts) {
                let dtn = self.surface_name(&dts);
                let vtn = self.surface_name(&vts);
                self.err_diff(pos, &format!("assignment to `{}`", name), &dtn, &vtn);
            }
        }
        // rc: read old, retain new (unless ownership transferred), release old,
        // then store. Retain-before-release keeps `g = g` self-assignment safe.
        let (old, _) = self.emit_global_read(fw, gsym, dt);
        let stored = if fw.rc_take_xfer(&v) {
            v.clone()
        } else {
            self.emit_retain(fw, &v)
        };
        self.emit_release(fw, &old);
        self.store_global(fw, gsym, dt, &stored);
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
            "int" | "i64" | "float" | "f64" | "str" | "bool" | "range"
        )
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
