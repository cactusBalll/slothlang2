//! Statement lowering incl. control flow and loop forms.

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
    pub(crate) fn walk_body(&mut self, fw: &mut FnWalk, s: &Stmt) {
        match &s.node {
            StmtNode::Block(ss) => {
                fw.push_scope();
                for st in ss {
                    self.walk_stmt(fw, st);
                }
                fw.pop_scope();
            }
            _ => {
                self.walk_stmt(fw, s);
            }
        }
    }
}

impl ModEmitter {
    pub(crate) fn walk_stmt(&mut self, fw: &mut FnWalk, s: &Stmt) {
        match &s.node {
            StmtNode::Expr(e) => {
                let _ = self.emit_expr(fw, e);
                // rc patch B: producer temps unused here die now
                fw.rc_flush();
            }
            StmtNode::Let {
                mutable,
                name,
                ty,
                init,
            } => {
                // typed(s) ok()/err() ctor fast-path: Result init (annotation
                // provides the T/E binding; absent annotation diagnosed)
                let pre: Option<(String, TyId)> = match &init.node {
                    ExprNode::Call { callee, args }
                        if args.len() == 1
                            && matches!(
                                callee.node,
                                ExprNode::Ident(ref id) if id == "ok" || id == "err",
                            ) =>
                    {
                        let inst = ty.as_ref().and_then(|te| {
                            let dt = self.ty_of(te);
                            match self.r.get(dt).clone() {
                                Ty::Named(nm, _) if self.result_insts.contains(&nm) => Some(nm),
                                _ => None,
                            }
                        });
                        match inst {
                            Some(inst) => {
                                let is_ok = matches!(
                                    &callee.node,
                                    ExprNode::Ident(ref id) if id == "ok"
                                );
                                Some(self.emit_result_ctor(fw, &inst, &args[0], is_ok, &init.pos))
                            }
                            None => {
                                self.err(
                                    &init.pos,
                                    format!(
                                        "ctor `{}` requires a declared Result target (let/var with Result<_, _>)",
                                        if Self::init_str2(callee) == "err" { "err" } else { "ok" }
                                    ),
                                );
                                None
                            }
                        }
                    }
                    _ => None,
                };
                let (v, t) = match &pre {
                    Some((w, tt)) => (w.clone(), *tt),
                    None => {
                        // expected-type hint from the declared annotation
                        // (return-driven generic inference, patch #38)
                        let hint = ty.as_ref().map(|te| self.ty_of(te));
                        self.exp_ret.push(hint.unwrap_or(self.r.mk(Ty::Unit)));
                        let out = self.emit_expr(fw, init);
                        self.exp_ret.pop();
                        out
                    }
                };
                // declared `dyn T`: auto-box builtin value types so the dyn
                // surface always holds a real object handle (see dynbox.rs)
                let (v, t) = if let Some(te) = ty {
                    let dt0 = self.ty_of(te);
                    if let Ty::Dyn(tn) = self.r.get(dt0).clone() {
                        if self.value_kind(t).is_some() {
                            if self.value_impls_trait(&tn) {
                                let b = self.emit_dyn_box(fw, &v, t, &tn);
                                (b, dt0)
                            } else {
                                let got = self.surface_name(self.r.get(t));
                                self.err_diff(&s.pos, "initializer", &format!("dyn {}", tn), &got);
                                (v, t)
                            }
                        } else {
                            (v, t)
                        }
                    } else {
                        (v, t)
                    }
                } else {
                    (v, t)
                };
                // declared `dyn T` / trait positions coerce the binding's type
                let t = match ty {
                    Some(te) => {
                        let dt = self.ty_of(te);
                        match self.r.get(dt) {
                            Ty::Dyn(_) => dt,
                            // declared Array<dyn T> coerces a list literal binding
                            Ty::Array(el) => match self.r.get(*el) {
                                Ty::Dyn(_) => match self.r.get(t) {
                                    Ty::Array(_) => dt,
                                    _ => t,
                                },
                                _ => t,
                            },
                            Ty::Named(n, _) if self.traits.contains_key(n.as_str()) => {
                                // only coerce actual object values (class instances)
                                match self.r.get(t) {
                                    Ty::Named(_, _) => self.r.mk(Ty::Dyn(n.clone())),
                                    _ => t,
                                }
                            }
                            _ => t,
                        }
                    }
                    None => t,
                };
                let _ = ty;
                // declared scalar-kind conflict check (MVP: word-family match)
                let (v, t) = match ty {
                    Some(te) => {
                        let dt = self.ty_of(te);
                        let df = self.is_float(dt);
                        let vf = self.is_float(t);

                        if df && !vf {
                            // promote the int word to an f64 word
                            (self.int_to_f64_word(fw, &v, t), dt)
                        } else if !df && vf && self.opt_inner(dt).is_none()
                            && !matches!(self.r.get(dt), Ty::Any)
                        {
                            self.err_diff(
                                &s.pos,
                                "initializer",
                                "non-float surface",
                                &self.surface_name(self.r.get(t)),
                            );
                            (v, t)
                        } else {
                            // declared word-surface check (patch #22):
                            // cross-kind i64-word declarations (int/str/bool/
                            // class/array/map) conflict structurally
                            let dts = self.r.get(dt).clone();
                            let vts = self.r.get(t).clone();
                            if !self.surface_compat(&dts, &vts) {
                                self.err_diff(
                                    &s.pos,
                                    "initializer",
                                    &self.surface_name(&dts),
                                    &self.surface_name(&vts),
                                );
                            }
                            // fixed-width integer binding: coerce the word to
                            // the declared surface (and record it)
                            if self.is_int_like(dt) && self.is_int_like(t) {
                                let vc = self.coerce_int_word(fw, &v, dt);
                                (vc, dt)
                            } else {
                                (v, t)
                            }
                        }
                    }
                    None => (v, t),
                };
                // patch 42: declared value-optional face (`int?` etc) boxes
                // the payload word (nil/Unit inits stay the raw nil word);
                // the binding surface records the Opt type itself
                let (v, t) = if ty.is_some() {
                    let dtr42 = ty.clone().unwrap();
                    let dt = self.ty_of(&dtr42);
                    if self.opt_inner(dt).is_some()
                        || self.weak_inner(dt).is_some()
                        || matches!(self.r.get(dt).clone(), Ty::Any)
                    {
                        // value-optional: box the payload word; `any`: box into
                        // a runtime-typed cell
                        let (vc, _tc2) = self.coerce_word_to(fw, &v, t, dt);
                        (vc, dt)
                    } else if matches!(self.r.get(dt).clone(), Ty::Opt(_)) {
                        // reference-optional: the handle word is already the
                        // representation, just record the Opt surface so
                        // `is nil` narrowing / later assigns see it
                        (v, dt)
                    } else {
                        (v, t)
                    }
                } else {
                    (v, t)
                };
                let fl = self.is_float(t);
                // rc patch B: declare + retain (slot ownership; nil/unknown
                // words are rt no-ops), then flush the temp's producer +1.
                // patch 42: transferred call results bind raw.
                let a = fw.declare(name, t, fl, *mutable);
                let xfer0 = fw.rc_take_xfer(&v);
                if self.is_ref(t) && !fl {
                    let (rv, xf) = if xfer0 {
                        (v.clone(), true)
                    } else {
                        (self.emit_retain(fw, &v), false)
                    };
                    let _ = xf;
                    fw.assign(name, &rv, fl);
                    fw.track_slot(&a);
                    fw.scope_decls
                        .last_mut()
                        .unwrap()
                        .insert(name.to_string(), a);
                } else {
                    fw.assign(name, &v, fl);
                }
                fw.rc_flush();
            }
            StmtNode::Assign { target, value } => {
                // declared surface of the assignee (drives value inference and
                // the Result-ctor fast path); for index targets it is the
                // container's element/value type, so `m["k"] = @()` and
                // `m["k"] = ok(v)` see the right Result/Map frame
                let target_ty = self.assign_target_type(fw, target);
                // assign-face Result ctor fast-path (patch #37): `x = ok(v)`
                // binds to the target's declared Result<T,E> frame
                let mut pre: Option<(String, TyId)> = None;
                if let ExprNode::Call { callee, args } = &value.node {
                    if args.len() == 1
                        && matches!(
                            callee.node,
                            ExprNode::Ident(ref id) if id == "ok" || id == "err",
                        )
                    {
                        let oname = if Self::init_str2(callee) == "err" {
                            "err"
                        } else {
                            "ok"
                        };
                        match target_ty.map(|dt| self.r.get(dt).clone()) {
                            Some(Ty::Named(nm, _)) if self.result_insts.contains(&nm) => {
                                let is_ok = matches!(
                                    callee.node,
                                    ExprNode::Ident(ref id) if id == "ok",
                                );
                                pre = Some(
                                    self.emit_result_ctor(fw, &nm, &args[0], is_ok, &value.pos),
                                );
                            }
                            _ => {
                                self.err(
                                    &value.pos,
                                    format!(
                                        "ctor `{}` requires a declared Result target (assignment target is not Result<_, _>)",
                                        oname
                                    ),
                                );
                            }
                        }
                    }
                }
                let (mut v, vty) = match pre {
                    Some((w, tt)) => (w, tt),
                    None => {
                        let hint = target_ty.unwrap_or_else(|| self.r.mk(Ty::Unit));
                        self.exp_ret.push(hint);
                        let out = self.emit_expr(fw, value);
                        self.exp_ret.pop();
                        out
                    }
                };
                // super.x = v: store into an inherited field slot of this
                if let (Some(PathSeg::Name(h)), Some(PathSeg::Name(f))) =
                    (target.first(), target.last())
                {
                    if *h == "super" && target.len() == 2 {
                        match fw.cur_cls.clone() {
                            Some(cur) => {
                                let idx = self.field_index(&cur, f);
                                let (rv, _t) = self.emit_expr(
                                    fw,
                                    &Expr {
                                        pos: s.pos.clone(),
                                        node: ExprNode::This,
                                    },
                                );
                                let zi = fw.v();
                                fw.op(&format!(
                                    "    {} = arith.constant {} : i64",
                                    zi,
                                    enc_i_lit(idx as i64)
                                ));
                                let fty = self.field_type(&cur, f);
                                let mut v = v;
                                let mut vty = vty;
                                if self.is_int_like(fty) && self.is_int_like(vty) {
                                    v = self.coerce_int_word(fw, &v, fty);
                                    vty = fty;
                                } else {
                                    self.check_field_surface(&s.pos, f, fty, vty);
                                }
                                self.op_set_field(fw, &rv, &zi, &v, vty, fty, s.pos.clone());
                                fw.rc_flush();
                                return;
                            }
                            None => {
                                self.err(&s.pos, "super.x assignment outside method".to_string())
                            }
                        }
                    }
                }
                // object-field target: [name, field] where head is a local receiver
                if target.len() >= 2 {
                    if let (Some(PathSeg::Name(h)), Some(PathSeg::Name(f))) =
                        (target.first(), target.last())
                    {
                        if let Some((at, rty)) = fw.lookup(&h.clone()) {
                            if let Ty::Named(c, _) = self.r.get(rty).clone() {
                                // receiver word: alloca stores the object pointer word
                                let z = fw.v();
                                let recv = fw.v();
                                let _ = rty;
                                fw.op(&format!("    {} = arith.constant 0 : index", z));
                                fw.op(&format!(
                                    "    {} = memref.load {}[{}] : memref<1xi64>",
                                    recv, at, z
                                ));
                                let idx = self.field_index(&c, &f.clone());
                                let zi = fw.v();
                                fw.op(&format!(
                                    "    {} = arith.constant {} : i64",
                                    zi,
                                    enc_i_lit(idx as i64)
                                ));
                                // f64 field route: int words promote; float->i64 rejects
                                let fty = self.field_type(&c, f);
                                let mut vc = v.clone();
                                let mut vct = vty;
                                let dfo = !self.is_float(fty)
                                    && matches!(self.opt_inner(fty), Some((_, true)));
                                if (self.is_float(fty) || dfo) && !self.is_float(vty) {
                                    vc = self.int_to_f64_word(fw, &v, vty);
                                    vct = self.r.mk(Ty::F64);
                                } else if !dfo && !self.is_float(fty) && self.is_float(vty) {
                                    self.err_diff(
                                        &s.pos,
                                        &format!("field assignment `{}`", f),
                                        "non-float surface",
                                        "float",
                                    );
                                } else if !dfo {
                                    if self.is_int_like(fty) && self.is_int_like(vct) {
                                        vc = self.coerce_int_word(fw, &vc, fty);
                                        vct = fty;
                                    } else {
                                        self.check_field_surface(&s.pos, f, fty, vct);
                                    }
                                }
                                self.op_set_field(fw, &recv, &zi, &vc, vct, fty, s.pos.clone());
                                fw.rc_flush();
                                return;
                            }
                        }
                    }
                }
                match target.last() {
                    Some(PathSeg::Name(n)) => {
                        if fw.imm_of(n) {
                            self.err(
                                &s.pos,
                                format!("cannot assign to immutable `{}` (declared with `let`)", n),
                            );
                        }
                        match fw.lookup(n) {
                            Some((_, dt)) => {
                                self.check_named_assign(fw, n, dt, &v, vty, &s.pos);
                            }
                            None => {
                                let cur_mod = self.cur_mod.clone();
                                let own = self.globals.get(n).cloned();
                                let foreign =
                                    self.fglobals.get(&format!("{}.{}", cur_mod, n)).cloned();
                                // while emitting a foreign body, its own global
                                // wins over a same-named local one
                                let picked = if cur_mod != self.name {
                                    foreign.or(own)
                                } else {
                                    own.or(foreign)
                                };
                                match picked {
                                    Some((gsym, gt, gmut)) => {
                                        if !gmut {
                                            self.err(
                                                &s.pos,
                                                format!(
                                                    "cannot assign to immutable `{}` (declared with `let`)",
                                                    n
                                                ),
                                            );
                                        }
                                        self.check_global_assign(fw, n, &gsym, gt, &v, vty, &s.pos);
                                    }
                                    None => {
                                        fw.assign(n, &v, false);
                                    }
                                }
                            }
                        }
                    }
                    Some(PathSeg::Index(ix)) => {
                        // a[i] = v / m[k] = v / a[i][j] = v / h.f[i] = v
                        // (design §3 EBNF assignable: any `[expr]` chain)
                        let nseg = target.len();
                        let mut cont: Option<(String, TyId)> = None;
                        match &target[..nseg - 1] {
                            [PathSeg::Name(h)] => match fw.lookup(&h.clone()) {
                                Some((aa, at)) => {
                                    let z = fw.v();
                                    fw.op(&format!("    {} = arith.constant 0 : index", z));
                                    let av = fw.v();
                                    fw.op(&format!(
                                        "    {} = memref.load {}[{}] : memref<1xi64>",
                                        av, aa, z
                                    ));
                                    cont = Some((av, at));
                                }
                                None => self.err(&s.pos, format!("unknown array `{}`", h)),
                            },
                            _ => cont = self.eval_assign_container(fw, &target[..nseg - 1], &s.pos),
                        }
                        if let Some((av, at)) = cont {
                            let ats = self.r.get(at).clone();
                            // tensor extension TE-P1: `t[a..b] = src` assigns
                            // through a keep-rank view (copy_into)
                            let mut handled = false;
                            if let Ty::Tensor(elem, rank) = ats.clone() {
                                if matches!(ix.node, ExprNode::Range { .. }) {
                                    let (view, _vt) =
                                        self.emit_tensor_index(fw, &av, elem, rank, ix, &s.pos);
                                    let ok = match self.r.get(vty).clone() {
                                        Ty::Tensor(ve, vr) => {
                                            vr == rank
                                                && self.surface_compat(
                                                    self.r.get(elem),
                                                    self.r.get(ve),
                                                )
                                        }
                                        _ => false,
                                    };
                                    if ok {
                                        self.emit_tensor_copy_into(fw, &view, &v);
                                    } else {
                                        self.err(
                                            &s.pos,
                                            "tensor slice assignment expects a matching tensor"
                                                .to_string(),
                                        );
                                    }
                                    handled = true;
                                }
                            }
                            if handled {
                                fw.rc_flush();
                                return;
                            }
                            let (iv, _it) = self.emit_expr(fw, ix);
                            match ats {
                                Ty::Array(el) => {
                                    // tag migration: single word route;
                                    // int words promote for float slots;
                                    // cross-kind stores diagnose
                                    if self.opt_inner(el).is_some() || self.weak_inner(el).is_some()
                                    {
                                        let (vc, _tc) = self.coerce_word_to(fw, &v, vty, el);
                                        v = vc;
                                    } else if self.is_float(el) && !self.is_float(vty) {
                                        v = self.int_to_f64_word(fw, &v, vty);
                                    } else if !self.is_float(el) && self.is_float(vty) {
                                        self.err_diff(
                                            &s.pos,
                                            "array element assignment",
                                            "non-float surface",
                                            "float",
                                        );
                                    } else if !matches!(self.r.get(vty).clone(), Ty::Unit) {
                                        let els = self.r.get(el).clone();
                                        let vts = self.r.get(vty).clone();
                                        if !self.surface_compat(&els, &vts) {
                                            let en = self.surface_name(&els);
                                            let vn = self.surface_name(&vts);
                                            self.err_diff(
                                                &s.pos,
                                                "array element assignment",
                                                &en,
                                                &vn,
                                            );
                                        }
                                    }
                                    if self.is_int_like(el) && self.is_int_like(vty) {
                                        v = self.coerce_int_word(fw, &v, el);
                                    }
                                    // rc patch B: release the old elem
                                    // (scalar/nil words no-op in rt),
                                    // retain the new one if ref-typed
                                    if self.is_ref(el) {
                                        let old = fw.v();
                                        fw.op(&format!(
                                                    "    {} = func.call @sloth_arr_get({}, {}) : (i64, i64) -> i64",
                                                    old, av, iv
                                                ));
                                        self.emit_release(fw, &old);
                                        let rv2 = self.emit_retain(fw, &v);
                                        v = rv2;
                                    }
                                    fw.op(&format!(
                                                "    func.call @sloth_arr_set({}, {}, {}) : (i64, i64, i64) -> i64",
                                                av, iv, v
                                            ));
                                }
                                Ty::Map(k, v2) => {
                                    let kkind = matches!(self.r.get(k), Ty::Str);
                                    let vf = self.is_float(v2);
                                    // object keys: monomorphized hash()
                                    // routed into the call (patch #35)
                                    let mut use_h: Option<String> = None;
                                    if let Ty::Named(kcls, _) = self.r.get(k).clone() {
                                        if self.class_ids.contains_key(&kcls) {
                                            match self.find_map_key_hash(&kcls) {
                                                Some((hmname, defcls, hfd)) => {
                                                    let oargv =
                                                        vec![(iv.clone(), self.r.mk(Ty::I64))];
                                                    let osig = vec!["i64".to_string()];
                                                    let (hv, _ht) = self.emit_method_call(
                                                        fw, &defcls, &hmname, &hfd, false, &oargv,
                                                        &osig, &s.pos,
                                                    );
                                                    use_h = Some(hv);
                                                }
                                                None => {
                                                    self.err(
                                                            &s.pos,
                                                            format!(
                                                                "map key `{}` implements no `hash()`-family method — keyed by pointer identity (Hashable surface needs `hash()`/`hashKey()`/`__hash__()`)",
                                                                kcls
                                                            ),
                                                        );
                                                }
                                            }
                                        }
                                    }
                                    if self.opt_inner(v2).is_some()
                                        || self.weak_inner(v2).is_some()
                                        || matches!(self.r.get(v2), Ty::Dyn(_) | Ty::Int(_))
                                    {
                                        let (vc, _tc) = self.coerce_word_to(fw, &v, vty, v2);
                                        v = vc;
                                    } else if vf && !self.is_float(vty) {
                                        // int word -> f64 word
                                        v = self.int_to_f64_word(fw, &v, vty);
                                    } else if !vf && self.is_float(vty) {
                                        self.err_diff(
                                            &s.pos,
                                            "map value assignment",
                                            "non-float surface",
                                            "float",
                                        );
                                    } else if !matches!(self.r.get(vty).clone(), Ty::Unit) {
                                        let v2s = self.r.get(v2).clone();
                                        let vts = self.r.get(vty).clone();
                                        if !self.surface_compat(&v2s, &vts) {
                                            let en = self.surface_name(&v2s);
                                            let vn = self.surface_name(&vts);
                                            self.err_diff(&s.pos, "map value assignment", &en, &vn);
                                        }
                                    }
                                    // rc patch B: the map slot owns its
                                    // key (str/object) and value copy;
                                    // scalar/nil words no-op in rt.
                                    // Overwrite-time release of evicted
                                    // old pairs lands in patch C.
                                    let kty = self.r.get(k).clone();
                                    let kref = matches!(kty, Ty::Str | Ty::Named(_, _));
                                    if kref {
                                        self.emit_retain(fw, &iv);
                                    }
                                    let vref = !vf && self.is_ref(v2);
                                    if vref {
                                        let rv2 = self.emit_retain(fw, &v);
                                        v = rv2;
                                    }
                                    match use_h {
                                        Some(hv) => {
                                            fw.op(&format!(
                                                        "    func.call @sloth_map_set_h({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
                                                        av, iv, hv, v
                                                    ));
                                        }
                                        None => {
                                            let sym = if kkind {
                                                "sloth_map_str_set"
                                            } else {
                                                "sloth_map_set"
                                            };
                                            fw.op(&format!(
                                                "    func.call @{}({}, {}, {}) : (i64, i64, i64) -> i64",
                                                sym, av, iv, v
                                            ));
                                        }
                                    }
                                }
                                Ty::Named(cn, _) => {
                                    // Indexable overload: a[i] = v ≡ a.__assign__(i, v)
                                    match self.find_method(cn.as_str(), "__assign__") {
                                        Some((defcls, fd)) => {
                                            let mut vc = v.clone();
                                            let retf = fd.params.len() >= 3 && {
                                                let pt = fd.params[2].ty.clone();
                                                pt.as_ref()
                                                    .map(|t| {
                                                        let dt = self.ty_of(t);
                                                        self.is_float(dt)
                                                    })
                                                    .unwrap_or(false)
                                            };
                                            if retf && !self.is_float(vty) {
                                                vc = self.int_to_f64_word(fw, &v, vty);
                                            }
                                            let oargv = vec![
                                                (av.clone(), at),
                                                (iv.clone(), self.r.mk(Ty::I64)),
                                                (vc, vty),
                                            ];
                                            let osig = vec![
                                                mlir_word_ty(at, &self.r),
                                                "i64".to_string(),
                                                mlir_word_ty(vty, &self.r),
                                            ];
                                            self.emit_method_call(
                                                fw,
                                                &defcls,
                                                "__assign__",
                                                &fd,
                                                false,
                                                &oargv,
                                                &osig,
                                                &s.pos,
                                            );
                                        }
                                        None => {
                                            self.err(
                                                        &s.pos,
                                                        format!("class `{}` requires an `__assign__` overload for index assignment", cn),
                                                    );
                                        }
                                    }
                                }
                                // tensor extension TE-P1: rank-1 target is a
                                // scalar store; rank>1 target is a view that
                                // receives an element-wise copy (D1/D2)
                                Ty::Tensor(elem, rank) if rank == 1 => {
                                    if self.is_float(elem) && !self.is_float(vty) {
                                        v = self.int_to_f64_word(fw, &v, vty);
                                    } else if !self.is_float(elem) && self.is_float(vty) {
                                        self.err_diff(
                                            &s.pos,
                                            "tensor element assignment",
                                            "non-float surface",
                                            "float",
                                        );
                                    } else if !matches!(self.r.get(vty).clone(), Ty::Unit) {
                                        let es = self.r.get(elem).clone();
                                        let vs = self.r.get(vty).clone();
                                        if !self.surface_compat(&es, &vs) {
                                            let en = self.surface_name(&es);
                                            let vn = self.surface_name(&vs);
                                            self.err_diff(
                                                &s.pos,
                                                "tensor element assignment",
                                                &en,
                                                &vn,
                                            );
                                        }
                                    }
                                    self.emit_tensor_set1(fw, &av, &iv, &v);
                                }
                                Ty::Tensor(elem, rank) if rank > 1 => {
                                    let vtyc = self.r.get(vty).clone();
                                    let elem_ok = match &vtyc {
                                        Ty::Tensor(ve, vr) => {
                                            *vr == rank - 1
                                                && self.surface_compat(
                                                    self.r.get(elem),
                                                    self.r.get(*ve),
                                                )
                                        }
                                        _ => false,
                                    };
                                    if elem_ok {
                                        let (view, _vt) = self.emit_tensor_view_drop(
                                            fw, &av, elem, rank, &iv, &s.pos,
                                        );
                                        self.emit_tensor_copy_into(fw, &view, &v);
                                    } else {
                                        self.err_diff(
                                            &s.pos,
                                            "tensor view assignment",
                                            &format!(
                                                "Tensor<{},{}>",
                                                sloth_frontend::ty::ty_name(self.r.get(elem)),
                                                rank - 1
                                            ),
                                            &self.surface_name(&vtyc),
                                        );
                                    }
                                }
                                _ => {
                                    self.err(&s.pos, "index assignment on non-array".to_string());
                                    return;
                                }
                            }
                        }
                    }
                    _ => {
                        self.err(&s.pos, "unsupported assignment target".to_string());
                    }
                }
                // rc patch B: producer temps of this statement settle here
                fw.rc_flush();
            }
            StmtNode::AssignOp { target, op, value } => {
                // `t op= v` desugars to `t = t op v`, with every index
                // sub-expression evaluated exactly once (spilled into a fresh
                // immutable local before the read/write pair) so side-effecting
                // indices are not run twice.
                let mut t2: Vec<PathSeg> = Vec::with_capacity(target.len());
                for seg in target {
                    match seg {
                        PathSeg::Name(n) => t2.push(PathSeg::Name(n.clone())),
                        PathSeg::Index(ix) => {
                            let (w, ty) = self.emit_expr(fw, ix);
                            let nm = self.spill_temp(fw, &w, ty);
                            t2.push(PathSeg::Index(Expr {
                                pos: ix.pos.clone(),
                                node: ExprNode::Ident(nm),
                            }));
                        }
                    }
                }
                let rhs = Expr {
                    pos: s.pos.clone(),
                    node: ExprNode::Arith {
                        op: *op,
                        lhs: Box::new(path_to_expr(&t2, &s.pos)),
                        rhs: Box::new(value.clone()),
                    },
                };
                let desugared = Stmt {
                    pos: s.pos.clone(),
                    node: StmtNode::Assign {
                        target: t2,
                        value: rhs,
                    },
                };
                self.walk_stmt(fw, &desugared);
            }
            StmtNode::Return(None) => {
                self.emit_ret_flag_store(fw);
                // rc patch B: unsettled producer temps die before the jump
                fw.rc_flush();
                self.jump_to_ret(fw);
            }
            StmtNode::Return(Some(e)) => {
                self.walk_return_value(fw, e);
            }
            StmtNode::While { cond, body } => {
                self.walk_while(fw, cond, body, &s.pos);
            }
            StmtNode::If { cond, then_, else_ } => {
                self.walk_if(fw, cond, then_, else_.as_deref(), &s.pos);
            }
            StmtNode::For { var, iter, body } => {
                self.walk_for(fw, var, iter, body, &s.pos);
            }
            StmtNode::Break => {
                self.walk_break(fw, &s.pos);
            }
            StmtNode::Continue => {
                self.walk_continue(fw, &s.pos);
            }
            StmtNode::Block(_) => {
                self.walk_body(fw, s);
            }
        }
    }
}

impl ModEmitter {
    /// declared surface of an assignment target: the variable/field type for a
    /// plain name, or the container's element/value type for an index target.
    /// Used both for value inference hints and the Result-ctor fast path.
    pub(crate) fn assign_target_type(&mut self, fw: &FnWalk, target: &[PathSeg]) -> Option<TyId> {
        match target.last()? {
            PathSeg::Name(n) => {
                if target.len() == 1 {
                    fw.lookup(&n.clone()).map(|x| x.1)
                } else {
                    self.assign_container_type(fw, &target[..target.len() - 1])
                        .and_then(|ct| match self.r.get(ct).clone() {
                            // `a.f` / `a.f.g` — field of an object surface
                            Ty::Named(c, _) => Some(self.field_type(&c, n)),
                            _ => None,
                        })
                }
            }
            PathSeg::Index(_) => {
                let ct = self.assign_container_type(fw, &target[..target.len() - 1])?;
                match self.r.get(ct).clone() {
                    Ty::Array(el) => Some(el),
                    Ty::Map(_, v) => Some(v),
                    Ty::Tensor(e, r) if r > 1 => Some(self.r.mk(Ty::Tensor(e, r - 1))),
                    Ty::Tensor(e, _) => Some(e),
                    _ => None,
                }
            }
        }
    }

    /// materialize `w:ty` into a fresh immutable local slot and return the
    /// synthetic binding name. Mirrors the `let` ownership rule (retain, or
    /// take a transferred call result, so the slot owns the word) and registers
    /// it for scope-exit release. Used by the compound-assign desugaring to
    /// evaluate index sub-expressions exactly once.
    pub(crate) fn spill_temp(&mut self, fw: &mut FnWalk, w: &str, ty: TyId) -> String {
        let nm = fw.v();
        let fl = self.is_float(ty);
        let a = fw.declare(&nm, ty, fl, false);
        if self.is_ref(ty) && !fl {
            let rv = if fw.rc_take_xfer(w) {
                w.to_string()
            } else {
                self.emit_retain(fw, w)
            };
            fw.assign(&nm, &rv, fl);
            fw.track_slot(&a);
            fw.scope_decls.last_mut().unwrap().insert(nm.clone(), a);
        } else {
            fw.assign(&nm, w, fl);
        }
        nm
    }

    /// evaluate the container denoted by an assignment prefix into a word:
    /// a local, an object field, or a nested container element (design §3
    /// EBNF `assignable` allows arbitrarily chained `.` / `[]`). Used by the
    /// index-assignment path for `a[i][j] = v` / `h.f[i] = v` / `m[k1][k2] = v`.
    pub(crate) fn eval_assign_container(
        &mut self,
        fw: &mut FnWalk,
        prefix: &[PathSeg],
        pos: &Pos,
    ) -> Option<(String, TyId)> {
        match prefix {
            [] => None,
            [PathSeg::Name(h)] => match fw.lookup(&h.clone()) {
                Some((aa, at)) => {
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", z));
                    let av = fw.v();
                    fw.op(&format!(
                        "    {} = memref.load {}[{}] : memref<1xi64>",
                        av, aa, z
                    ));
                    Some((av, at))
                }
                None => {
                    self.err(pos, format!("unknown `{}`", h));
                    None
                }
            },
            _ => {
                let (parent, last) = prefix.split_at(prefix.len() - 1);
                let (pw, pt) = self.eval_assign_container(fw, parent, pos)?;
                match &last[0] {
                    PathSeg::Name(f) => {
                        if let Ty::Named(c, _) = self.r.get(pt).clone() {
                            let fi = self.field_index(&c, f);
                            let zi = fw.v();
                            fw.op(&format!(
                                "    {} = arith.constant {} : i64",
                                zi,
                                enc_i_lit(fi as i64)
                            ));
                            let fv = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                fv, pw, zi
                            ));
                            Some((fv, self.field_type(&c, f)))
                        } else {
                            self.err(pos, "nested assignment base must be an object".to_string());
                            None
                        }
                    }
                    PathSeg::Index(ix) => {
                        let (iv, _) = self.emit_expr(fw, ix);
                        match self.r.get(pt).clone() {
                            Ty::Array(el) => {
                                let inner = fw.v();
                                fw.op(&format!(
                                    "    {} = func.call @sloth_arr_get({}, {}) : (i64, i64) -> i64",
                                    inner, pw, iv
                                ));
                                Some((inner, el))
                            }
                            Ty::Map(k, v) => {
                                let kkind = matches!(self.r.get(k), Ty::Str);
                                let sym = if kkind {
                                    "sloth_map_str_get"
                                } else {
                                    "sloth_map_get"
                                };
                                let inner = fw.v();
                                fw.op(&format!(
                                    "    {} = func.call @{}({}, {}) : (i64, i64) -> i64",
                                    inner, sym, pw, iv
                                ));
                                Some((inner, v))
                            }
                            // tensor extension TE-P1: a mid-path index on a
                            // rank>1 tensor is a shared-storage view
                            Ty::Tensor(el, rank) if rank > 1 => {
                                let (view, vt) =
                                    self.emit_tensor_view_drop(fw, &pw, el, rank, &iv, pos);
                                Some((view, vt))
                            }
                            _ => {
                                self.err(
                                    pos,
                                    "nested index assignment base is not a container".to_string(),
                                );
                                None
                            }
                        }
                    }
                }
            }
        }
    }

    /// type of the container expression denoted by an assignment prefix
    /// (a local, an object field, or a nested container element)
    pub(crate) fn assign_container_type(
        &mut self,
        fw: &FnWalk,
        prefix: &[PathSeg],
    ) -> Option<TyId> {
        match prefix {
            [PathSeg::Name(h)] => fw.lookup(&h.clone()).map(|x| x.1),
            [PathSeg::Name(h), PathSeg::Name(f)] => match fw.lookup(&h.clone()) {
                Some((_a, t)) => match self.r.get(t).clone() {
                    Ty::Named(c, _) => Some(self.field_type(&c, f)),
                    _ => None,
                },
                None => None,
            },
            [PathSeg::Name(h), PathSeg::Index(_)] => match fw.lookup(&h.clone()) {
                Some((_a, t)) => match self.r.get(t).clone() {
                    Ty::Array(el) => Some(el),
                    Ty::Map(_, v) => Some(v),
                    Ty::Tensor(e, r) if r > 1 => Some(self.r.mk(Ty::Tensor(e, r - 1))),
                    _ => None,
                },
                None => None,
            },
            _ => None,
        }
    }
}

impl ModEmitter {
    /// store default return value into ret slots (no explicit value given)
    pub(crate) fn emit_ret_flag_store(&mut self, fw: &mut FnWalk) {
        let zi = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", zi));
        if !self.is_unit(fw.ret) {
            // word plane: every return slot holds one tagged i64 word (0 is
            // the nil/zero spell for both int and f64)
            let zret = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", zret));
            let fzi = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : index", fzi));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                zret, fw.ret_alloca, fzi
            ));
        }
        let st = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : i64", st));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            st, fw.ret_flag, zi
        ));
    }
}

impl ModEmitter {
    pub(crate) fn jump_to_ret(&mut self, fw: &mut FnWalk) {
        let l = fw.end_label.clone();
        fw.jump(&l);
    }
}

impl ModEmitter {
    pub(crate) fn walk_return_value(&mut self, fw: &mut FnWalk, e: &Expr) {
        // typed ok()/err() ctor fast-path in return position (patch #23):
        // the T/E binding comes from the function's declared Result<_ of,_>
        // return annotation
        let mut pre: Option<(String, TyId)> = None;
        if let ExprNode::Call { callee, args } = &e.node {
            if args.len() == 1
                && matches!(
                    callee.node,
                    ExprNode::Ident(ref id) if id == "ok" || id == "err",
                )
            {
                let inst = match self.r.get(fw.ret).clone() {
                    Ty::Named(nm, _) if self.result_insts.contains(&nm) => Some(nm),
                    _ => None,
                };
                match inst {
                    Some(inst) => {
                        let is_ok = matches!(
                            &callee.node,
                            ExprNode::Ident(ref id) if id == "ok"
                        );
                        pre = Some(self.emit_result_ctor(fw, &inst, &args[0], is_ok, &e.pos));
                    }
                    None => {
                        self.err(
                            &e.pos,
                            format!(
                                "ctor `{}` requires a declared Result target (Result<_, _> return annotation)",
                                if Self::init_str2(callee) == "err" { "err" } else { "ok" }
                            ),
                        );
                    }
                }
            }
        }
        let (v, t) = match pre {
            Some((w, tt)) => (w, tt),
            None => self.emit_expr(fw, e),
        };
        // patch 42: value-optional return surfaces box bare scalars; nil
        // word (0) passes through as nil. `dyn T` returns auto-box builtin
        // values the same way.
        let (mut v, mut t) =
            if self.opt_inner(fw.ret).is_some()
                || matches!(self.r.get(fw.ret), Ty::Dyn(_) | Ty::Int(_))
            {
                let (vc, tc) = self.coerce_word_to(fw, &v, t, fw.ret);
                (vc, tc)
            } else {
                (v, t)
            };
        // declared-return surface check (design §2.2 static typing): int
        // words promote into float returns; word-family conflicts diagnose.
        // Unit returns and unannotated bodies stay lenient.
        if !self.is_unit(fw.ret)
            && self.opt_inner(fw.ret).is_none()
            && self.weak_inner(fw.ret).is_none()
        {
            if self.is_float(fw.ret) {
                if !self.is_float(t) && !matches!(self.r.get(t).clone(), Ty::Unit) {
                    v = self.int_to_f64_word(fw, &v, t);
                    t = self.r.mk(Ty::F64);
                }
            } else if self.is_float(t) {
                self.err_diff(&e.pos, "return value", "non-float surface", "float");
            } else if !matches!(self.r.get(t).clone(), Ty::Unit) {
                let rts = self.r.get(fw.ret).clone();
                let vts = self.r.get(t).clone();
                if !self.surface_compat(&rts, &vts) {
                    let rtn = self.surface_name(&rts);
                    let vtn = self.surface_name(&vts);
                    self.err_diff(&e.pos, "return value", &rtn, &vtn);
                }
            }
        }
        let fl = self.is_float(t);
        // §5.1.1 rule 4: a ref-typed return ALWAYS hands the caller an owned
        // (+1) word. A pending producer or transferred call result already
        // carries the +1 (transfer it); a borrowed slot/field/param does not,
        // so materialize a retained +1 here. Non-ref returns just drop
        // producers. This must run before the store so the slot holds the
        // owned word.
        if !fl && self.is_ref(fw.ret) {
            if !fw.rc_take_xfer(&v) && !fw.rc_consume(&v) {
                v = self.emit_retain(fw, &v);
            }
        } else if !fl {
            fw.rc_consume(&v);
        }
        // the frame is abandoned: settle owned locals the normal scope-exit
        // path would have released
        fw.rc_release_scope_slots();
        let zi = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", zi));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            v, fw.ret_alloca, zi
        ));
        let st = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : i64", st));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            st, fw.ret_flag, zi
        ));
        fw.rc_flush();
        self.jump_to_ret(fw);
    }
}

impl ModEmitter {
    /// flow-typing MVP: `x is C` / `x is not nil` narrows x inside then-branch
    /// by shadowing its slot binding with the narrowed type
    pub(crate) fn narrow_pattern(
        &mut self,
        fw: &mut FnWalk,
        cond: &Expr,
    ) -> Option<(String, TyId)> {
        match &cond.node {
            ExprNode::Is {
                negated: false,
                lhs,
                rhs,
            } => {
                let x = match &lhs.node {
                    ExprNode::Ident(n) => n.clone(),
                    _ => return None,
                };
                match &rhs.node {
                    ExprNode::Ident(cn) if self.class_ids.contains_key(cn.as_str()) => {
                        let nty = self.r.mk(Ty::Named(cn.to_string(), Vec::new()));
                        Some((x, nty))
                    }
                    // builtin `is` on an optional: look through the option
                    // layer and narrow to the payload family when it matches
                    ExprNode::Ident(cn) if Self::is_builtin_type_name(cn) => {
                        let cur = fw.lookup(&x).map(|(_a, t)| self.r.get(t).clone())?;
                        // `dyn T` value narrowed by a builtin test: the branch
                        // sees the value family (walk_narrowed unboxes)
                        if matches!(cur, Ty::Dyn(_)) {
                            return Self::builtin_name_ty(cn).map(|t| (x.clone(), self.r.mk(t)));
                        }
                        // `any` narrowed by a primitive test (walk_narrowed
                        // materialises the boxed payload)
                        if matches!(cur, Ty::Any) {
                            return Self::builtin_name_ty(cn).map(|t| (x.clone(), self.r.mk(t)));
                        }
                        let mut wt = cur;
                        let mut had_opt = false;
                        while let Ty::Opt(inner) = wt {
                            had_opt = true;
                            wt = self.r.get(inner).clone();
                        }
                        if !had_opt {
                            return None;
                        }
                        let hit = Self::builtin_name_ty(cn)
                            .map(|t| t == wt)
                            .unwrap_or(false);
                        if hit {
                            let ni = self.r.mk(wt);
                            Some((x, ni))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            ExprNode::Is {
                negated: true,
                lhs,
                rhs,
            } if matches!(&lhs.node, ExprNode::Ident(_)) => {
                if !matches!(&rhs.node, ExprNode::Nil) {
                    return None;
                }
                let x = match &lhs.node {
                    ExprNode::Ident(n) => n.clone(),
                    _ => return None,
                };
                let narrowed = match fw.lookup(&x).map(|(_a, t)| self.r.get(t).clone()) {
                    Some(Ty::Opt(e)) => e,
                    _ => return None,
                };
                Some((x, narrowed))
            }
            _ => None,
        }
    }

    /// narrowing valid on the FALSE branch of `cond`:
    /// `if x is nil {} else {}` ⇒ x is not nil; `if x is not C {} else {}` ⇒ x is C
    pub(crate) fn narrow_pattern_false(
        &mut self,
        fw: &mut FnWalk,
        cond: &Expr,
    ) -> Option<(String, TyId)> {
        let (negated, lhs, rhs) = match &cond.node {
            ExprNode::Is { negated, lhs, rhs } => (*negated, lhs, rhs),
            _ => return None,
        };
        let x = match &lhs.node {
            ExprNode::Ident(n) => n.clone(),
            _ => return None,
        };
        match (negated, &rhs.node) {
            (false, ExprNode::Nil) => {
                // x is nil is false ⇒ x is not nil
                match fw.lookup(&x).map(|(_a, t)| self.r.get(t).clone()) {
                    Some(Ty::Opt(e)) => Some((x, e)),
                    _ => None,
                }
            }
            (true, ExprNode::Ident(cn)) if self.class_ids.contains_key(cn.as_str()) => {
                // x is not C is false ⇒ x is C
                let nty = self.r.mk(Ty::Named(cn.to_string(), Vec::new()));
                Some((x, nty))
            }
            (true, ExprNode::Ident(cn)) if Self::is_builtin_type_name(cn) => {
                // x is not <primitive> is false ⇒ x is that primitive
                let cur = fw.lookup(&x).map(|(_a, t)| self.r.get(t).clone())?;
                if !matches!(cur, Ty::Any) {
                    return None;
                }
                return Self::builtin_name_ty(cn).map(|t| (x.clone(), self.r.mk(t)));
            }
            _ => None,
        }
    }

    /// emit `body` with `x` shadow-narrowed to `nty` (value-optional payloads
    /// are unboxed into a fresh scalar shadow slot)
    pub(crate) fn walk_narrowed(&mut self, fw: &mut FnWalk, x: &str, nty: TyId, body: &Stmt) {
        // `any` source: the box's payload word is the narrowed value (a borrow
        // of the boxed value — the `any` slot keeps its strong reference)
        if let Some((a, cur)) = fw.lookup(x).map(|(a, t)| (a, self.r.get(t).clone())) {
            if matches!(cur, Ty::Any) {
                let zz = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", zz));
                let w = fw.v();
                fw.op(&format!(
                    "    {} = memref.load {}[{}] : memref<1xi64>",
                    w, a, zz
                ));
                let p = fw.v();
                fw.op(&format!(
                    "    {} = func.call @sloth_any_word({}) : (i64) -> i64",
                    p, w
                ));
                fw.push_scope();
                let sa = fw.v();
                fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", sa));
                let zz2 = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", zz2));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xi64>",
                    p, sa, zz2
                ));
                fw.scopes
                    .last_mut()
                    .unwrap()
                    .insert(x.to_string(), (sa, nty));
                self.walk_body(fw, body);
                fw.pop_scope();
                return;
            }
        }
        if matches!(
            self.r.get(nty).clone(),
            Ty::I64 | Ty::Int(_) | Ty::F64 | Ty::Bool
        ) {
            let slot = fw.lookup(x).map(|(a, t)| (a, self.r.get(t).clone()));
            if let Some((a, cur)) = slot {
                let w = {
                    let zz = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", zz));
                    let w2 = fw.v();
                    fw.op(&format!(
                        "    {} = memref.load {}[{}] : memref<1xi64>",
                        w2, a, zz
                    ));
                    w2
                };
                // a `dyn T` word is a box: unbox its payload; otherwise the
                // optional payload route applies
                let (u, _ut) = if matches!(cur, Ty::Dyn(_)) {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @sloth_dyn_unbox({}) : (i64) -> i64",
                        r, w
                    ));
                    (r, nty)
                } else {
                    let ot = self.r.mk(Ty::Opt(nty));
                    self.unwrap_opt_word(fw, &w, ot)
                };
                fw.push_scope();
                let sa = fw.v();
                fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", sa));
                let zz2 = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", zz2));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xi64>",
                    u, sa, zz2
                ));
                fw.scopes
                    .last_mut()
                    .unwrap()
                    .insert(x.to_string(), (sa, nty));
                self.walk_body(fw, body);
                fw.pop_scope();
                return;
            }
        }
        let slot = fw.lookup(x).map(|(a, _)| a);
        if let Some(a) = slot {
            fw.push_scope();
            fw.scopes
                .last_mut()
                .unwrap()
                .insert(x.to_string(), (a, nty));
            self.walk_body(fw, body);
            fw.pop_scope();
        } else {
            self.walk_body(fw, body);
        }
    }
}

impl ModEmitter {
    pub(crate) fn walk_if(
        &mut self,
        fw: &mut FnWalk,
        cond: &Expr,
        then_: &Stmt,
        else_: Option<&Stmt>,
        pos: &Pos,
    ) {
        if !fw.noterm() {
            self.err(pos, "unreachable code".to_string());
            return;
        }
        let (c, ct) = self.emit_expr(fw, cond);
        // conditional surfaces take no implicit truthy conversion (§2.1):
        // only a real bool tests
        if self.r.get(ct) != &Ty::Bool {
            let tn = sloth_frontend::ty::ty_name(self.r.get(ct));
            self.err_cond_bool(pos, &tn);
        }
        let (narrow, narrow_else) = if self.diags.is_empty() {
            (
                self.narrow_pattern(fw, cond),
                self.narrow_pattern_false(fw, cond),
            )
        } else {
            (None, None)
        };
        let thlab = fw.newlabel("t");
        let ellab = fw.newlabel("e");
        let endlab = fw.newlabel("fi");
        fw.cjump(&c, &thlab, &ellab);
        fw.label(&thlab);
        match narrow {
            Some((x, nty)) => self.walk_narrowed(fw, &x, nty, then_),
            None => self.walk_body(fw, then_),
        }
        fw.jump(&endlab);
        fw.label(&ellab);
        if let Some(e2) = else_ {
            match narrow_else {
                Some((x, nty)) => self.walk_narrowed(fw, &x, nty, e2),
                None => self.walk_body(fw, e2),
            }
        }
        fw.jump(&endlab);
        fw.label(&endlab);
    }
}

impl ModEmitter {
    pub(crate) fn walk_while(&mut self, fw: &mut FnWalk, cond: &Expr, body: &Stmt, pos: &Pos) {
        if !fw.noterm() {
            self.err(pos, "unreachable code".to_string());
            return;
        }
        let head = fw.newlabel("wh");
        let doo = fw.newlabel("do");
        let done = fw.newlabel("wd");
        fw.jump(&head);
        fw.label(&head);
        let (c, ct) = self.emit_expr(fw, cond);
        if self.r.get(ct) != &Ty::Bool {
            let tn = sloth_frontend::ty::ty_name(self.r.get(ct));
            self.err_cond_bool(pos, &tn);
        }
        fw.cjump(&c, &doo, &done);
        fw.label(&doo);
        fw.loop_bases.push(fw.scope_decls.len());
        fw.loops.push((done.clone(), head.clone()));
        self.walk_body(fw, body);
        fw.loops.pop();
        fw.loop_bases.pop();
        fw.jump(&head);
        fw.label(&done);
    }
}

impl ModEmitter {
    pub(crate) fn walk_for(
        &mut self,
        fw: &mut FnWalk,
        var: &str,
        iter: &Expr,
        body: &Stmt,
        pos: &Pos,
    ) {
        if !fw.noterm() {
            self.err(pos, "unreachable code".to_string());
            return;
        }
        match &iter.node {
            ExprNode::Range {
                low,
                high,
                inclusive,
            } => {
                let (lo, _lt) = self.emit_expr(fw, low);
                let (hi0, _ht) = self.emit_expr(fw, high);
                let hi = if !*inclusive {
                    hi0.clone()
                } else {
                    // both bounds are encoded words: +1 == +enc(1)
                    let one = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        one,
                        enc_i_lit(1)
                    ));
                    let h = fw.v();
                    fw.op(&format!("    {} = arith.addi {}, {} : i64", h, hi0, one));
                    h
                };
                self.emit_range_loop(fw, var, body, &lo, &hi);
            }
            _ => {
                // array/map iteration: for x in arr|map { ... } with a slotted counter
                let (mav, at) = self.emit_expr(fw, iter);
                // the iterable may be an owned producer (array/map/str literal
                // or a ref-returning call: `for x in mkarr()`). Detach it from
                // the enclosing statement flush so the loop-head cjump does not
                // free it mid-iteration; the loop owns and releases it at exit.
                let consumed = fw.rc_consume(&mav);
                let taken = fw.rc_take_xfer(&mav);
                let iter_owned = consumed || taken;
                let ats = self.r.get(at).clone();
                match &ats {
                    Ty::Array(e) => {
                        self.emit_index_loop(fw, var, body, mav, *e, IdxKind::Arr, pos, iter_owned);
                        return;
                    }
                    // map iteration: for-in yields Entry<K,V> records (§3.5)
                    Ty::Map(k, _v) => {
                        self.emit_entry_loop(fw, var, body, mav.clone(), *k, *_v, pos, iter_owned);
                        return;
                    }
                    // str iteration: per-char 1-byte strings
                    Ty::Str => {
                        {
                            let et = self.r.mk(Ty::Str);
                            self.emit_index_loop(
                                fw,
                                var,
                                body,
                                mav,
                                et,
                                IdxKind::StrChar,
                                pos,
                                iter_owned,
                            );
                        }
                        return;
                    }
                    // iterator protocol: iterator object (or iter())/next() -> Opt<el>
                    Ty::Named(_c, _) => {
                        self.emit_proto_loop(
                            fw,
                            var,
                            body,
                            mav.clone(),
                            at.clone(),
                            pos,
                            iter_owned,
                        );
                        return;
                    }
                    // first-class range value: unbox {lo, hi} and run the loop
                    Ty::Range => {
                        let lo = fw.v();
                        fw.op(&format!(
                            "    {} = func.call @sloth_range_lo({}) : (i64) -> i64",
                            lo, mav
                        ));
                        let hi = fw.v();
                        fw.op(&format!(
                            "    {} = func.call @sloth_range_hi({}) : (i64) -> i64",
                            hi, mav
                        ));
                        self.emit_range_loop(fw, var, body, &lo, &hi);
                        if iter_owned {
                            self.emit_release(fw, &mav);
                        }
                        return;
                    }
                    _ => {
                        self.err(pos, "for-iteration requires a range, array, map, string, array-backed or iterator value".to_string());
                        return;
                    }
                };
            }
        }
    }
}

impl ModEmitter {
    /// bound-driven integer loop shared by range literals and first-class
    /// range values. `lo`/`hi` are tagged words; `hi` is the exclusive upper
    /// bound (inclusive ranges normalize it before calling).
    pub(crate) fn emit_range_loop(
        &mut self,
        fw: &mut FnWalk,
        var: &str,
        body: &Stmt,
        lo: &str,
        hi: &str,
    ) {
        let lo = lo.to_string();
        let hi = hi.to_string();
        // slot for range bound; slot for index
        fw.push_scope();
        let islot = fw.v();
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", islot));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            lo, islot, z
        ));
        let head = fw.newlabel("fr");
        let doo = fw.newlabel("fb");
        let done = fw.newlabel("fd");
        fw.jump(&head);
        fw.label(&head);
        let iv = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            iv, islot, z
        ));
        let c = fw.v();
        fw.op(&format!("    {} = arith.cmpi slt, {}, {} : i64", c, iv, hi));
        let c1 = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", c1, c));
        fw.cjump(&c1, &doo, &done);
        fw.label(&doo);
        // continue lands on the increment, not the head test
        let cont = fw.newlabel("fc");
        fw.loop_bases.push(fw.scope_decls.len());
        fw.loops.push((done.clone(), cont.clone()));
        // bind loop var
        let vs = fw.v();
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            iv, vs, z
        ));
        fw.scopes
            .last_mut()
            .unwrap()
            .insert(var.to_string(), (vs, self.r.mk(Ty::I64)));
        fw.loopvars.push(var.to_string());
        self.walk_body(fw, body);
        fw.loopvars.pop();
        fw.loops.pop();
        fw.loop_bases.pop();
        fw.label_br(&cont);
        // idx += 1 (encoded word: the loop var holds a tagged int)
        let one2 = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            one2,
            enc_i_lit(1)
        ));
        let nx = fw.v();
        fw.op(&format!("    {} = arith.addi {}, {} : i64", nx, iv, one2));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            nx, islot, z
        ));
        fw.jump(&head);
        fw.label(&done);
        fw.pop_scope();
    }
}

impl ModEmitter {
    /// map for-in: per iteration build an Entry<K,V> record object with the
    /// live key/value pair; loop var binds the Entry object (§3.5)
    pub(crate) fn emit_entry_loop(
        &mut self,
        fw: &mut FnWalk,
        var: &str,
        body: &Stmt,
        mav: String,
        k: TyId,
        v2: TyId,
        pos: &Pos,
        iter_owned: bool,
    ) {
        let is_str = self.is_str(k);
        let vf = self.is_float(v2);
        let et = self.declare_class_inst("Entry", &[k, v2]);
        let ename = match self.r.get(et) {
            Ty::Named(n, _) => n.clone(),
            _ => "Entry".to_string(),
        };
        let kidxf = self.field_index(&ename, "key");
        let vidxf = self.field_index(&ename, "val");
        // keys snapshot array (same as keys() route)
        let ks = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_map_keys({}) : (i64) -> i64",
            ks, mav
        ));
        let lenv = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_arr_len({}) : (i64) -> i64",
            lenv, ks
        ));
        fw.push_scope();
        let islot = fw.v();
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", islot));
        let zi2 = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zi2));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            zi2, islot, z
        ));
        let head = fw.newlabel("me");
        let doo = fw.newlabel("mb");
        let done = fw.newlabel("md");
        fw.jump(&head);
        fw.label(&head);
        let iv = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            iv, islot, z
        ));
        let c = fw.v();
        fw.op(&format!(
            "    {} = arith.cmpi slt, {}, {} : i64",
            c, iv, lenv
        ));
        let c1 = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", c1, c));
        fw.cjump(&c1, &doo, &done);
        fw.label(&doo);
        // continue lands on the increment, not the head test
        let cont = fw.newlabel("mc");
        // break must still drop the current Entry (cont is skipped); route it
        // through a cleanup block that reloads the entry slot
        let brk = fw.newlabel("mx");
        fw.loop_bases.push(fw.scope_decls.len());
        fw.loops.push((brk.clone(), cont.clone()));
        // key word: keys array (word route covers int/str/Hashable keys)
        let kw = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_arr_get({}, {}) : (i64, i64) -> i64",
            kw, ks, iv
        ));
        // value word: same map-read family as indexing (kkind routed by table);
        // object keys fetch through the cached-hash _h call (patch #35)
        let vw = if !is_str {
            if let Ty::Named(kcls, _) = self.r.get(k).clone() {
                match self.find_map_key_hash(&kcls) {
                    Some((hmname, defcls, hfd)) => {
                        let oargv = vec![(kw.clone(), k)];
                        let osig = vec![mlir_word_ty(k, &self.r)];
                        let (hv, _ht) = self.emit_method_call(
                            fw, &defcls, &hmname, &hfd, false, &oargv, &osig, pos,
                        );
                        let _ = vf;
                        let vw = fw.v();
                        fw.op(&format!(
                            "    {} = func.call @sloth_map_get_h({}, {}, {}) : (i64, i64, i64) -> i64",
                            vw, mav, kw, hv
                        ));
                        vw
                    }
                    None => {
                        // diag already reported at literal/lookup sites; keep
                        // the legacy pointer-identity fetch
                        let vw = fw.v();
                        fw.op(&format!(
                            "    {} = func.call @sloth_map_get({}, {}) : (i64, i64) -> i64",
                            vw, mav, kw
                        ));
                        vw
                    }
                }
            } else {
                let vw = fw.v();
                fw.op(&format!(
                    "    {} = func.call @sloth_map_get({}, {}) : (i64, i64) -> i64",
                    vw, mav, kw
                ));
                vw
            }
        } else {
            let vw = fw.v();
            fw.op(&format!(
                "    {} = func.call @sloth_map_str_get({}, {}) : (i64, i64) -> i64",
                vw, mav, kw
            ));
            vw
        };
        // build the Entry record: plain object + fields (no user ctor)
        let (obj, _ot) = self.emit_new_obj(fw, &ename, &Vec::new(), &Vec::new(), &pos);
        // the per-iteration Entry is a fresh producer owned by the loop var
        // (a borrow view): cancel its statement-dangling slot and release it
        // at the iteration continuation below
        fw.rc_consume(&obj);
        let ki = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            ki,
            enc_i_lit(kidxf as i64)
        ));
        self.op_set_field(fw, &obj, &ki, &kw, k, k, pos.clone());
        let vi = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            vi,
            enc_i_lit(vidxf as i64)
        ));
        self.op_set_field(fw, &obj, &vi, &vw, v2, v2, pos.clone());
        let vs = fw.v();
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            obj, vs, z
        ));
        fw.scopes
            .last_mut()
            .unwrap()
            .insert(var.to_string(), (vs.clone(), et));
        // rc patch C: proto-for vars are borrows too
        fw.loopvars.push(var.to_string());
        self.walk_body(fw, body);
        fw.loopvars.pop();
        fw.loops.pop();
        fw.loop_bases.pop();
        fw.label_br(&cont);
        // the Entry built above is owned by the loop var; drop its +1 at the
        // end of each iteration (fallthrough + continue)
        self.emit_release(fw, &obj);
        // counter is an encoded word: +1 == +enc(1)
        let one2 = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            one2,
            enc_i_lit(1)
        ));
        let nx = fw.v();
        fw.op(&format!("    {} = arith.addi {}, {} : i64", nx, iv, one2));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            nx, islot, z
        ));
        fw.jump(&head);
        // break cleanup: drop the Entry the abandoned iteration still owns
        fw.label(&brk);
        let bv = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            bv, vs, z
        ));
        self.emit_release(fw, &bv);
        fw.jump(&done);
        fw.label(&done);
        // the keys snapshot array is a fresh producer owned by the loop
        // (break lands here too, so it is released exactly once)
        self.emit_release(fw, &ks);
        // a producer iterable (`for (e: mkmap())`) is owned by the loop
        if iter_owned {
            self.emit_release(fw, &mav);
        }
        fw.pop_scope();
    }
}

impl ModEmitter {
    pub(crate) fn emit_index_loop(
        &mut self,
        fw: &mut FnWalk,
        var: &str,
        body: &Stmt,
        arr: String,
        el: TyId,
        kind: IdxKind,
        _pos: &Pos,
        iter_owned: bool,
    ) {
        let (countfn, getfn, getty) = match kind {
            // tag migration: one word route (float elements ride the word)
            IdxKind::Arr => ("sloth_arr_len", "sloth_arr_get", "(i64, i64) -> i64"),
            IdxKind::StrChar => ("sloth_str_clen", "sloth_str_char", "(i64, i64) -> i64"),
        };
        let lenv = fw.v();
        fw.op(&format!(
            "    {} = func.call @{}({}) : (i64) -> i64",
            lenv, countfn, arr
        ));
        fw.push_scope();
        let islot = fw.v();
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", islot));
        let zi2 = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zi2));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            zi2, islot, z
        ));
        let head = fw.newlabel("af");
        let doo = fw.newlabel("ab");
        let done = fw.newlabel("ae");
        fw.jump(&head);
        fw.label(&head);
        let iv = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            iv, islot, z
        ));
        let c = fw.v();
        fw.op(&format!(
            "    {} = arith.cmpi slt, {}, {} : i64",
            c, iv, lenv
        ));
        let c1 = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", c1, c));
        fw.cjump(&c1, &doo, &done);
        fw.label(&doo);
        // continue lands on the increment, not the head test
        let cont = fw.newlabel("ic");
        // break must still settle the current element for string iteration
        // (a fresh char str is owned; array elements are borrows)
        let brk = fw.newlabel("ix");
        fw.loop_bases.push(fw.scope_decls.len());
        fw.loops.push((brk.clone(), cont.clone()));
        // loop var = seq[i]
        let gtv = fw.v();
        fw.op(&format!(
            "    {} = func.call @{}({}, {}) : {}",
            gtv, getfn, arr, iv, getty
        ));
        let gety = if kind == IdxKind::StrChar {
            self.r.mk(Ty::Str)
        } else {
            el
        };
        let _ = gety;
        let vs = fw.v();
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            gtv, vs, z
        ));
        fw.scopes
            .last_mut()
            .unwrap()
            .insert(var.to_string(), (vs.clone(), gety));
        self.walk_body(fw, body);
        fw.loops.pop();
        fw.loop_bases.pop();
        fw.label_br(&cont);
        // str iteration produces a fresh char str per element (§5.1.1
        // rule 7): the loop var holds a borrow view, so the producer's +1 is
        // released at the end of each iteration (fallthrough + continue)
        if kind == IdxKind::StrChar {
            self.emit_release(fw, &gtv);
        }
        // idx += 1 (encoded word)
        let one2 = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            one2,
            enc_i_lit(1)
        ));
        let nx = fw.v();
        fw.op(&format!("    {} = arith.addi {}, {} : i64", nx, iv, one2));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            nx, islot, z
        ));
        fw.jump(&head);
        // break cleanup: settle the abandoned iteration's owned char
        fw.label(&brk);
        if kind == IdxKind::StrChar {
            let bv = fw.v();
            fw.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                bv, vs, z
            ));
            self.emit_release(fw, &bv);
        }
        fw.jump(&done);
        fw.label(&done);
        // a producer iterable (`for x in mkarr()` / `for c in mkstr()`) is
        // owned by the loop and released once at exit
        if iter_owned {
            self.emit_release(fw, &arr);
        }
        fw.pop_scope();
    }

    /// iterator protocol: iterator object (or it.iter())/next() -> Opt<el>
    /// (worded MVP: Opt as i64 word, 0 = nil; float elements rejected)
    fn emit_proto_loop(
        &mut self,
        fw: &mut FnWalk,
        var: &str,
        body: &Stmt,
        recv: String,
        recvty: TyId,
        pos: &Pos,
        recv_owned: bool,
    ) {
        // normalize: value itself an iterator, or expose iter() first
        let cls = match self.r.get(recvty) {
            Ty::Named(c, _) => c.clone(),
            _ => {
                self.err(
                    &pos,
                    "for-iteration requires a range, array, map or iterator value".to_string(),
                );
                return;
            }
        };
        let (itv, itty);
        let has_iter = self.find_method(&cls, "iter").is_some();
        if let Some((defcls, fd)) = self.find_method(&cls, "iter") {
            let (v, t) = self.emit_method_call(
                fw,
                &defcls,
                "iter",
                &fd,
                false,
                &vec![(recv.clone(), recvty.clone())],
                &vec!["i64".to_string()],
                &pos,
            );
            if let Ty::Named(c, _) = self.r.get(t) {
                itv = v;
                itty = t;
                _ = c;
            } else {
                self.err(&pos, "iter() must return an iterator object".to_string());
                return;
            }
        } else {
            itv = recv.clone();
            itty = recvty;
        }
        let icls = match self.r.get(itty) {
            Ty::Named(c, _) => c.clone(),
            _ => unreachable!(),
        };
        let nfd = self.find_method(&icls, "next");
        if nfd.is_none() {
            self.err(
                &pos,
                format!("class `{}` has no `next()` (iterator protocol)", icls),
            );
            return;
        }
        let (ndefcls, nfn) = nfd.unwrap();
        let nplan = self.plan_for_class("next", &ndefcls, &nfn);
        let el = match self.r.get(nplan.ret) {
            Ty::Opt(e) => *e,
            _ => {
                self.err(
                    &pos,
                    format!("iterator `next()` must return Opt (class `{}`)", icls),
                );
                return;
            }
        };
        // patch 42: boxed value optional elements (float payloads included)
        // unwrap through the box; ref-shaped elements keep the borrow view
        let el_boxed = self.opt_inner(nplan.ret).is_some();
        // iterator slot storage
        let islot = fw.v();
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", islot));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            itv, islot, z
        ));
        // §5.1.1 rule 5/8: the iterator obtained from `iter()` is an owned
        // temp (a borrowed iterator has no +1 and is left alone). Take its
        // transfer mark before the loop so the in-loop cjump does not free it,
        // and hand the slot to the scope so every loop exit releases it once.
        // When the receiver itself was an owned producer (`for x in mk()`)
        // and an iter() bridge produced a distinct owned iterator, the
        // receiver keeps its own +1 and is released separately at loop exit.
        let itv_xfer = fw.rc_take_xfer(&itv);
        let it_owned = itv_xfer || (recv_owned && !has_iter);
        fw.push_scope();
        if it_owned {
            fw.track_slot(&islot);
            fw.scope_decls
                .last_mut()
                .unwrap()
                .insert("__iter".to_string(), islot.clone());
        }
        let recv_release = recv_owned && has_iter;
        let head = fw.newlabel("if");
        let doo = fw.newlabel("ib");
        let cont = fw.newlabel("ic");
        let brk = fw.newlabel("ix");
        let done = fw.newlabel("ie");
        fw.jump(&head);
        fw.label(&head);
        let itw = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            itw, islot, z
        ));
        let (ov, _ot) = self.emit_method_call(
            fw,
            &ndefcls,
            "next",
            &nfn,
            false,
            &vec![(itw, itty.clone())],
            &vec!["i64".to_string()],
            &pos,
        );
        // `next()` is an owned temp: take it before the nil-test cjump so the
        // current element survives until the end of the iteration
        let ov_owned = fw.rc_take_xfer(&ov);
        let nc = fw.v();
        let znil = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", znil));
        fw.op(&format!(
            "    {} = arith.cmpi eq, {}, {} : i64",
            nc, ov, znil
        ));
        let nc1 = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", nc1, nc));
        fw.cjump(&nc1, &done, &doo);
        fw.label(&doo);
        fw.loop_bases.push(fw.scope_decls.len());
        // break must still drop the current non-boxed ref element (owned
        // next() result); boxed payloads were already dropped above
        fw.loops.push((brk.clone(), cont.clone()));
        // loop var = unwrap(next())
        let vs = fw.v();
        if el_boxed {
            // boxed payload: unwrap the copy and let the consumed box die
            let (u, _ut) = self.unwrap_opt_word(fw, &ov, nplan.ret);
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                u, vs, z
            ));
            // the consumed box is an owned temp; drop its +1 now
            if ov_owned {
                self.emit_release(fw, &ov);
            }
        } else {
            // tag migration: the iterator word (handle or nil) is the value
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                ov, vs, z
            ));
        }
        fw.scopes
            .last_mut()
            .unwrap()
            .insert(var.to_string(), (vs.clone(), el));
        self.walk_body(fw, body);
        fw.loops.pop();
        fw.loop_bases.pop();
        // iteration continuation: reached by fallthrough and by `continue`;
        // a ref-shaped element that stayed a borrow view is dropped here
        fw.label_br(&cont);
        if !el_boxed && ov_owned {
            self.emit_release(fw, &ov);
        }
        fw.jump(&head);
        // break cleanup: drop the owned element the abandoned iteration holds
        fw.label(&brk);
        if !el_boxed && ov_owned {
            let bv = fw.v();
            fw.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                bv, vs, z
            ));
            self.emit_release(fw, &bv);
        }
        fw.jump(&done);
        fw.label(&done);
        if recv_release {
            self.emit_release(fw, &recv);
        }
        fw.pop_scope();
    }
}

impl ModEmitter {
    pub(crate) fn walk_break(&mut self, fw: &mut FnWalk, pos: &Pos) {
        match fw.loops.last() {
            Some((b, _c)) => {
                let t = b.clone();
                // settle the loop-body locals this jump abandons
                fw.rc_release_loop_body();
                fw.jump(&t);
            }
            None => {
                self.err(pos, "break outside loop".to_string());
            }
        }
    }
}

impl ModEmitter {
    pub(crate) fn walk_continue(&mut self, fw: &mut FnWalk, pos: &Pos) {
        match fw.loops.last() {
            Some((_b, c)) => {
                let t = c.clone();
                // settle the loop-body locals this jump abandons
                fw.rc_release_loop_body();
                fw.jump(&t);
            }
            None => {
                self.err(pos, "continue outside loop".to_string());
            }
        }
    }
}
