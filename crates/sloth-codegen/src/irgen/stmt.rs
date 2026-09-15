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
                    None => self.emit_expr(fw, init),
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
                            // promote the int word to f64 spelling
                            let cv = fw.v();
                            fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                            (cv, dt)
                        } else if !df && vf {
                            self.err(
                                &s.pos,
                                "type mismatch: initializer is float but declared type is not"
                                    .to_string(),
                            );
                            (v, t)
                        } else {
                            // declared word-surface check (patch #22):
                            // cross-kind i64-word declarations (int/str/bool/
                            // class/array/map) conflict structurally
                            let dts = self.r.get(dt).clone();
                            let vts = self.r.get(t).clone();
                            if !self.surface_compat(&dts, &vts) {
                                let dtn = self.surface_name(&dts);
                                let vtn = self.surface_name(&vts);
                                self.err(
                                    &s.pos,
                                    format!(
                                        "type mismatch: initializer is `{}` but declared type is `{}`",
                                        vtn, dtn
                                    ),
                                );
                            }
                            (v, t)
                        }
                    }
                    None => (v, t),
                };
                let fl = self.is_float(t);
                fw.declare(name, t, fl, *mutable);
                fw.assign(name, &v, fl);
            }
            StmtNode::Assign { target, value } => {
                // assign-face Result ctor fast-path (patch #37): `x = ok(v)`
                // binds to the target's declared Result<T,E> frame
                let mut pre: Option<(String, TyId)> = None;
                if let (Some(PathSeg::Name(h)), ExprNode::Call { callee, args }) =
                    (target.first(), &value.node)
                {
                    if args.len() == 1
                        && matches!(
                            callee.node,
                            ExprNode::Ident(ref id) if id == "ok" || id == "err",
                        )
                    {
                        match fw.lookup(&h.clone()) {
                            Some((_, dt)) => {
                                match self.r.get(dt).clone() {
                                    Ty::Named(nm, _) if self.result_insts.contains(&nm) => {
                                        let is_ok = matches!(
                                            callee.node,
                                            ExprNode::Ident(ref id) if id == "ok",
                                        );
                                        pre = Some(self.emit_result_ctor(
                                            fw,
                                            &nm,
                                            &args[0],
                                            is_ok,
                                            &value.pos,
                                        ));
                                    }
                                    _ => {
                                        self.err(
                                            &value.pos,
                                            format!(
                                                "ctor `{}` requires a declared Result target (assignment target `{}` is not Result<_, _>)",
                                                if Self::init_str2(callee) == "err" { "err" } else { "ok" },
                                                h
                                            ),
                                        );
                                    }
                                }
                            }
                            None => {
                                self.err(
                                    &value.pos,
                                    format!(
                                        "ctor `{}` requires a declared Result target (assignment to unknown `{}`)",
                                        if Self::init_str2(callee) == "err" { "err" } else { "ok" },
                                        h
                                    ),
                                );
                            }
                        }
                    }
                }
                let (mut v, vty) = match pre {
                    Some((w, tt)) => (w, tt),
                    None => self.emit_expr(fw, value),
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
                                fw.op(&format!("    {} = arith.constant {} : i64", zi, idx));
                                fw.op(&format!(
                                    "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                                    rv, zi, v
                                ));
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
                            if let Ty::Named(c, _) = self.r.get(rty) {
                                // receiver word: alloca stores the object pointer word
                                let z = fw.v();
                                let recv = fw.v();
                                let mty = if self.is_float(rty) {
                                    "memref<1xf64>"
                                } else {
                                    "memref<1xi64>"
                                };
                                fw.op(&format!("    {} = arith.constant 0 : index", z));
                                fw.op(&format!(
                                    "    {} = memref.load {}[{}] : {}",
                                    recv, at, z, mty
                                ));
                                let idx = self.field_index(&c, &f.clone());
                                let zi = fw.v();
                                fw.op(&format!("    {} = arith.constant {} : i64", zi, idx));
                                // f64 field route: int words promote; float->i64 rejects
                                let fty = self
                                    .classes
                                    .get(c)
                                    .and_then(|ci| ci.fields.iter().find(|fd| fd.0 == *f))
                                    .map(|fd| fd.1)
                                    .unwrap_or_else(|| self.r.mk(Ty::I64));
                                let mut vc = v.clone();
                                if self.is_float(fty) && !self.is_float(vty) {
                                    let cv = fw.v();
                                    fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                                    vc = cv;
                                } else if !self.is_float(fty) && self.is_float(vty) {
                                    self.err(
                                        &s.pos,
                                        "type mismatch: cannot assign float to non-float field"
                                            .to_string(),
                                    );
                                }
                                self.op_set_field(fw, &recv, &zi, &vc, fty, s.pos.clone());
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
                                fw.assign(n, &v, false);
                            }
                        }
                    }
                    Some(PathSeg::Index(ix)) => {
                        // a[i] = v / m[k] = v (single-index MVP)
                        if target.len() != 2 {
                            self.err(&s.pos, "nested index assignment unsupported".to_string());
                            return;
                        }
                        if let Some(PathSeg::Name(h)) = target.first() {
                            match fw.lookup(h) {
                                Some((aa, at)) => {
                                    let ats = self.r.get(at).clone();
                                    let z = fw.v();
                                    fw.op(&format!("    {} = arith.constant 0 : index", z));
                                    let av = fw.v();
                                    fw.op(&format!(
                                        "    {} = memref.load {}[{}] : memref<1xi64>",
                                        av, aa, z
                                    ));
                                    let (iv, _it) = self.emit_expr(fw, ix);
                                    match ats {
                                        Ty::Array(el) => {
                                            if self.is_float(el) {
                                                if !self.is_float(vty) {
                                                    let cv = fw.v();
                                                    fw.op(&format!(
                                                        "    {} = arith.sitofp {} : i64 to f64",
                                                        cv, v
                                                    ));
                                                    v = cv;
                                                }
                                                fw.op(&format!(
                                                    "    call @sloth_arr_set_f64({}, {}, {}) : (i64, i64, f64) -> i64",
                                                    av, iv, v
                                                ));
                                            } else {
                                                fw.op(&format!(
                                                    "    call @sloth_arr_set({}, {}, {}) : (i64, i64, i64) -> i64",
                                                    av, iv, v
                                                ));
                                            }
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
                                                            fw,
                                                            &defcls,
                                                            &hmname,
                                                            &hfd,
                                                            false,
                                                            &oargv,
                                                            &osig,
                                                            &s.pos,
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
                                            let vsig = if vf { "f64" } else { "i64" };
                                            if vf && !self.is_float(vty) {
                                                let cv = fw.v();
                                                fw.op(&format!(
                                                    "    {} = arith.sitofp {} : i64 to f64",
                                                    cv, v
                                                ));
                                                v = cv;
                                            }
                                            match use_h {
                                                Some(hv) => {
                                                    let sym = if vf {
                                                        "sloth_map_set_h_f64"
                                                    } else {
                                                        "sloth_map_set_h"
                                                    };
                                                    fw.op(&format!(
                                                        "    call @{}({}, {}, {}, {}) : (i64, i64, i64, {}) -> i64",
                                                        sym, av, iv, hv, v, vsig
                                                    ));
                                                }
                                                None => {
                                                    let sym = match (kkind, vf) {
                                                        (true, true) => "sloth_map_str_set_f64",
                                                        (true, false) => "sloth_map_str_set",
                                                        (false, true) => "sloth_map_set_f64",
                                                        (false, false) => "sloth_map_set",
                                                    };
                                                    fw.op(&format!(
                                                        "    call @{}({}, {}, {}) : (i64, i64, {}) -> i64",
                                                        sym, av, iv, v, vsig
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
                                                        let cv = fw.v();
                                                        fw.op(&format!(
                                                            "    {} = arith.sitofp {} : i64 to f64",
                                                            cv, v
                                                        ));
                                                        vc = cv;
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
                                        _ => {
                                            self.err(
                                                &s.pos,
                                                "index assignment on non-array".to_string(),
                                            );
                                            return;
                                        }
                                    }
                                }
                                None => {
                                    self.err(&s.pos, format!("unknown array `{}`", h));
                                }
                            }
                        }
                    }
                    _ => {
                        self.err(&s.pos, "unsupported assignment target".to_string());
                    }
                }
            }
            StmtNode::Return(None) => {
                self.emit_ret_flag_store(fw);
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
    /// store default return value into ret slots (no explicit value given)
    pub(crate) fn emit_ret_flag_store(&mut self, fw: &mut FnWalk) {
        let zi = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zi));
        if !self.is_unit(fw.ret) {
            let zret = fw.v();
            let fl = self.is_float(fw.ret);
            fw.op(&format!("    {} = arith.constant 0 : i64", zret));
            let mz = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", mz));
            if fl {
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0.0 : f64", z));
                let fzi = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", fzi));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xf64>",
                    z, fw.ret_alloca, fzi
                ));
            } else {
                let fzi = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", fzi));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xi64>",
                    zret, fw.ret_alloca, fzi
                ));
            }
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
        let fl = self.is_float(t);
        let zi = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zi));
        if fl {
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xf64>",
                v, fw.ret_alloca, zi
            ));
        } else {
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                v, fw.ret_alloca, zi
            ));
        }
        let st = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : i64", st));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            st, fw.ret_flag, zi
        ));
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
            self.err(
                pos,
                format!(
                    "condition must be `bool` (no implicit truthy conversion from `{}`)",
                    tn
                ),
            );
        }
        let narrow = if self.diags.is_empty() {
            self.narrow_pattern(fw, cond)
        } else {
            None
        };
        let thlab = fw.newlabel("t");
        let ellab = fw.newlabel("e");
        let endlab = fw.newlabel("fi");
        fw.cjump(&c, &thlab, &ellab);
        fw.label(&thlab);
        match narrow {
            Some((x, nty)) => {
                let slot = fw.lookup(&x).map(|(a, _)| a);
                if let Some(a) = slot {
                    fw.push_scope();
                    fw.scopes.last_mut().unwrap().insert(x.clone(), (a, nty));
                    self.walk_body(fw, then_);
                    fw.pop_scope();
                } else {
                    self.walk_body(fw, then_);
                }
            }
            None => self.walk_body(fw, then_),
        }
        fw.jump(&endlab);
        fw.label(&ellab);
        if let Some(e2) = else_ {
            self.walk_body(fw, e2);
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
            self.err(
                pos,
                format!(
                    "condition must be `bool` (no implicit truthy conversion from `{}`)",
                    tn
                ),
            );
        }
        fw.cjump(&c, &doo, &done);
        fw.label(&doo);
        fw.loops.push((done.clone(), head.clone()));
        self.walk_body(fw, body);
        fw.loops.pop();
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
                    let one = fw.v();
                    fw.op(&format!("    {} = arith.constant 1 : i64", one));
                    let h = fw.v();
                    fw.op(&format!("    {} = arith.addi {}, {} : i64", h, hi0, one));
                    h
                };
                // slot for range bound; slot for index
                fw.push_scope();
                let islot = fw.v();
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
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
                fw.op(&format!("    {} = arith.extsi {} : i1 to i64", c1, c));
                fw.cjump(&c1, &doo, &done);
                fw.label(&doo);
                // continue lands on the increment, not the head test
                let cont = fw.newlabel("fc");
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
                self.walk_body(fw, body);
                fw.loops.pop();
                fw.label_br(&cont);
                // idx += 1
                let one2 = fw.v();
                fw.op(&format!("    {} = arith.constant 1 : i64", one2));
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
            _ => {
                // array/map iteration: for x in arr|map { ... } with a slotted counter
                let (mav, at) = self.emit_expr(fw, iter);
                let ats = self.r.get(at).clone();
                match &ats {
                    Ty::Array(e) => {
                        self.emit_index_loop(fw, var, body, mav, *e, IdxKind::Arr, pos);
                        return;
                    }
                    // map iteration: for-in yields Entry<K,V> records (§3.5)
                    Ty::Map(k, _v) => {
                        self.emit_entry_loop(fw, var, body, mav.clone(), *k, *_v, pos);
                        return;
                    }
                    // str iteration: per-char 1-byte strings
                    Ty::Str => {
                        {
                            let et = self.r.mk(Ty::Str);
                            self.emit_index_loop(fw, var, body, mav, et, IdxKind::StrChar, pos);
                        }
                        return;
                    }
                    // iterator protocol: iterator object (or iter())/next() -> Opt<el>
                    Ty::Named(c, _) => {
                        self.emit_proto_loop(fw, var, body, mav.clone(), at.clone(), pos);
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
            "    {} = call @sloth_map_keys({}) : (i64) -> i64",
            ks, mav
        ));
        let lenv = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_arr_len({}) : (i64) -> i64",
            lenv, ks
        ));
        fw.push_scope();
        let islot = fw.v();
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
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
        fw.op(&format!("    {} = arith.extsi {} : i1 to i64", c1, c));
        fw.cjump(&c1, &doo, &done);
        fw.label(&doo);
        // continue lands on the increment, not the head test
        let cont = fw.newlabel("mc");
        fw.loops.push((done.clone(), cont.clone()));
        // key word: keys array (word route covers int/str/Hashable keys)
        let kw = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_arr_get({}, {}) : (i64, i64) -> i64",
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
                            fw,
                            &defcls,
                            &hmname,
                            &hfd,
                            false,
                            &oargv,
                            &osig,
                            pos,
                        );
                        let getfn = if vf {
                            "sloth_map_get_h_f64"
                        } else {
                            "sloth_map_get_h"
                        };
                        let vw = fw.v();
                        fw.op(&format!(
                            "    {} = call @{}({}, {}, {}) : (i64, i64, i64) -> {}",
                            vw,
                            getfn,
                            mav,
                            kw,
                            hv,
                            if vf { "f64" } else { "i64" }
                        ));
                        vw
                    }
                    None => {
                        // diag already reported at literal/lookup sites; keep
                        // the legacy pointer-identity fetch
                        let vw = fw.v();
                        fw.op(&format!(
                            "    {} = call @{}({}, {}) : {}",
                            vw,
                            if vf { "sloth_map_get_f64" } else { "sloth_map_get" },
                            mav,
                            kw,
                            if vf { "(i64, i64) -> f64" } else { "(i64, i64) -> i64" }
                        ));
                        vw
                    }
                }
            } else {
                let vw = fw.v();
                fw.op(&format!(
                    "    {} = call @{}({}, {}) : {}",
                    vw,
                    if vf { "sloth_map_get_f64" } else { "sloth_map_get" },
                    mav,
                    kw,
                    if vf { "(i64, i64) -> f64" } else { "(i64, i64) -> i64" }
                ));
                vw
            }
        } else {
            let vw = fw.v();
            fw.op(&format!(
                "    {} = call @{}({}, {}) : {}",
                vw,
                if vf { "sloth_map_str_get_f64" } else { "sloth_map_str_get" },
                mav,
                kw,
                if vf { "(i64, i64) -> f64" } else { "(i64, i64) -> i64" }
            ));
            vw
        };
        // build the Entry record: plain object + fields (no user ctor)
        let (obj, _ot) = self.emit_new_obj(fw, &ename, &Vec::new(), &Vec::new(), &pos);
        let ki = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", ki, kidxf));
        self.op_set_field(fw, &obj, &ki, &kw, k, pos.clone());
        let vi = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", vi, vidxf));
        self.op_set_field(fw, &obj, &vi, &vw, v2, pos.clone());
        let vs = fw.v();
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            obj, vs, z
        ));
        fw.scopes
            .last_mut()
            .unwrap()
            .insert(var.to_string(), (vs, et));
        self.walk_body(fw, body);
        fw.loops.pop();
        fw.label_br(&cont);
        let one2 = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : i64", one2));
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
    pub(crate) fn emit_index_loop(
        &mut self,
        fw: &mut FnWalk,
        var: &str,
        body: &Stmt,
        arr: String,
        el: TyId,
        kind: IdxKind,
        pos: &Pos,
    ) {
        let (countfn, getfn, getty) = match kind {
            IdxKind::Arr => {
                if self.is_float(el) {
                    ("sloth_arr_len", "sloth_arr_get_f64", "(i64, i64) -> f64")
                } else {
                    ("sloth_arr_len", "sloth_arr_get", "(i64, i64) -> i64")
                }
            }
            IdxKind::StrChar => ("sloth_str_len", "sloth_str_char", "(i64, i64) -> i64"),
        };
        let lenv = fw.v();
        fw.op(&format!(
            "    {} = call @{}({}) : (i64) -> i64",
            lenv, countfn, arr
        ));
        fw.push_scope();
        let islot = fw.v();
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
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
        fw.op(&format!("    {} = arith.extsi {} : i1 to i64", c1, c));
        fw.cjump(&c1, &doo, &done);
        fw.label(&doo);
        // continue lands on the increment, not the head test
        let cont = fw.newlabel("ic");
        fw.loops.push((done.clone(), cont.clone()));
        // loop var = seq[i]
        let gtv = fw.v();
        fw.op(&format!(
            "    {} = call @{}({}, {}) : {}",
            gtv, getfn, arr, iv, getty
        ));
        let gety = if kind == IdxKind::StrChar {
            self.r.mk(Ty::Str)
        } else {
            el
        };
        let vs = fw.v();
        if self.is_float(gety) {
            fw.op(&format!("    {} = memref.alloca() : memref<1xf64>", vs));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xf64>",
                gtv, vs, z
            ));
        } else {
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                gtv, vs, z
            ));
        }
        fw.scopes
            .last_mut()
            .unwrap()
            .insert(var.to_string(), (vs, gety));
        self.walk_body(fw, body);
        fw.loops.pop();
        fw.label_br(&cont);
        // idx += 1
        let one2 = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : i64", one2));
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
            itv = recv;
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
        if self.is_float(el) {
            self.err(
                &pos,
                "iterator element type float unsupported (MVP)".to_string(),
            );
            return;
        }
        // iterator slot storage
        let islot = fw.v();
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", islot));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            itv, islot, z
        ));
        fw.push_scope();
        let head = fw.newlabel("if");
        let doo = fw.newlabel("ib");
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
        let nc = fw.v();
        let znil = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", znil));
        fw.op(&format!(
            "    {} = arith.cmpi eq, {}, {} : i64",
            nc, ov, znil
        ));
        let nc1 = fw.v();
        fw.op(&format!("    {} = arith.extsi {} : i1 to i64", nc1, nc));
        fw.cjump(&nc1, &done, &doo);
        fw.label(&doo);
        fw.loops.push((done.clone(), head.clone()));
        // loop var = unwrap(next())
        let vs = fw.v();
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            ov, vs, z
        ));
        fw.scopes
            .last_mut()
            .unwrap()
            .insert(var.to_string(), (vs, el));
        self.walk_body(fw, body);
        fw.loops.pop();
        fw.jump(&head);
        fw.label(&done);
        fw.pop_scope();
    }
}

impl ModEmitter {
    pub(crate) fn walk_break(&mut self, fw: &mut FnWalk, pos: &Pos) {
        match fw.loops.last() {
            Some((b, _c)) => {
                let t = b.clone();
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
                fw.jump(&t);
            }
            None => {
                self.err(pos, "continue outside loop".to_string());
            }
        }
    }
}
