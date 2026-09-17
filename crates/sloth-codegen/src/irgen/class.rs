//! Object model: layout, ctors, method calls, vtables, dyn dispatch.

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
    /// Result ctor synth for `let r: Result<T,E> = ok(v) / err(e)`
    pub(crate) fn emit_result_ctor(
        &mut self,
        fw: &mut FnWalk,
        inst: &str,
        arg: &Expr,
        is_ok: bool,
        pos: &Pos,
    ) -> (String, TyId) {
        let ity = self.r.mk(Ty::Named(inst.to_string(), vec![]));
        let ci = match self.classes.get(inst).cloned() {
            Some(c) => c,
            None => return (String::new(), self.r.mk(Ty::Unit)),
        };
        let fvy = ci
            .fields
            .iter()
            .find(|f| f.0 == "v")
            .map(|f| f.1)
            .unwrap_or_else(|| self.r.mk(Ty::Unit));
        let fey = ci
            .fields
            .iter()
            .find(|f| f.0 == "e")
            .map(|f| f.1)
            .unwrap_or_else(|| self.r.mk(Ty::Unit));
        let fok = ci
            .fields
            .iter()
            .find(|f| f.0 == "ok")
            .map(|f| f.1)
            .unwrap_or_else(|| self.r.mk(Ty::Unit));
        let (mut v, vt) = self.emit_expr(fw, arg); // slot route: float field promotes int words; refuse float into ints
        let slot = if is_ok { fvy } else { fey };
        if self.is_float(slot) != self.is_float(vt) {
            // design §2.1: no implicit numeric conversion on Result payloads
            let an = sloth_frontend::ty::ty_name(self.r.get(slot));
            let bn = sloth_frontend::ty::ty_name(self.r.get(vt));
            self.err_diff(pos, "Result payload", &an, &bn);
            if self.is_float(slot) {
                v = iw_to_f64_word(fw, &v);
            }
        }
        // object + default zero fields
        let (obj, _ot) = self.emit_new_obj(fw, inst, &Vec::new(), &Vec::new(), pos);
        // ok flag & payload / err pair
        let zi = fw.v();
        let _zc = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zi));

        let okv = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            okv,
            enc_i_lit(if is_ok { 1 } else { 0 })
        ));
        let okidx = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            okidx,
            enc_i_lit(self.field_index(inst, "ok") as i64)
        ));
        self.op_set_field(fw, &obj, &okidx, &okv, fok, fok, pos.clone());
        if is_ok {
            let vidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                vidx,
                enc_i_lit(self.field_index(inst, "v") as i64)
            ));
            self.op_set_field(fw, &obj, &vidx, &v, vt, fvy, pos.clone());
            // zero the err slot by its word spelling
            let ez = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", ez));
            let eidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                eidx,
                enc_i_lit(self.field_index(inst, "e") as i64)
            ));
            self.op_set_field(fw, &obj, &eidx, &ez, fey, fey, pos.clone());
        } else {
            // zero the v slot
            let vz = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", vz));
            let vidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                vidx,
                enc_i_lit(self.field_index(inst, "v") as i64)
            ));
            self.op_set_field(fw, &obj, &vidx, &vz, fey, fvy, pos.clone());
            let eidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                eidx,
                enc_i_lit(self.field_index(inst, "e") as i64)
            ));
            self.op_set_field(fw, &obj, &eidx, &v, vt, fey, pos.clone());
        }
        (obj, ity)
    }
}

impl ModEmitter {
    /// field offsets in words: idx counted from slot 2 (slot 0: cls info, 1: unused?id)
    pub(crate) fn field_index(&self, clsname: &str, field: &str) -> usize {
        // layout: base-class fields first; materialize the chain base-first
        let mut chain: Vec<String> = Vec::new();
        let mut cur = Some(clsname.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    chain.push(c.clone());
                    cur = ci.superclass.clone();
                }
                None => break,
            }
        }
        chain.reverse();
        let mut out: usize = 0;
        for c in chain {
            let ci = match self.classes.get(&c) {
                Some(c2) => c2,
                None => break,
            };
            if let Some(pos) = ci.fields.iter().position(|(n, _t, _m)| n == field) {
                return out + pos;
            }
            out += ci.fields.len();
        }
        out
    }

    /// does the class (or an ancestor) declare a field with this name?
    pub(crate) fn has_field(&self, clsname: &str, field: &str) -> bool {
        let mut cur = Some(clsname.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    if ci.fields.iter().any(|(n, _, _)| n == field) {
                        return true;
                    }
                    cur = ci.superclass.clone();
                }
                None => break,
            }
        }
        false
    }

    /// field type by name following the base-class-first chain (inherited
    /// fields keep their declared surface type; a name miss falls to i64)
    pub(crate) fn field_type(&mut self, clsname: &str, field: &str) -> TyId {
        let mut cur = Some(clsname.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    if let Some((_, t, _)) = ci.fields.iter().find(|(n, _, _)| n == field) {
                        return *t;
                    }
                    cur = ci.superclass.clone();
                }
                None => break,
            }
        }
        self.r.mk(Ty::I64)
    }
}

impl ModEmitter {
    /// is `sub` the same class as `sup` or a (transitive) descendant of it?
    pub(crate) fn is_descendant_of(&self, sup: &str, sub: &str) -> bool {
        let mut cur = Some(sub.to_string());
        while let Some(c) = cur {
            if c == sup {
                return true;
            }
            cur = self.classes.get(&c).and_then(|ci| ci.superclass.clone());
        }
        false
    }

    /// design §2.4: calling a method through a base-class reference must
    /// dispatch on the runtime class. Emits a class-id switch over every
    /// loaded concrete subclass that overrides `mname`. Returns None when no
    /// subclass overrides it (the caller keeps the direct call).
    pub(crate) fn emit_class_virtual_call(
        &mut self,
        fw: &mut FnWalk,
        base_cls: &str,
        mname: &str,
        recv: &str,
        argv: &Vec<(String, TyId)>,
        sigargs: &Vec<String>,
        pos: &Pos,
    ) -> Option<(String, TyId)> {
        let (basedefcls, basedef) = self.find_method(base_cls, mname)?;
        let mut branches: Vec<(String, String, FuncDef)> = Vec::new();
        let names: Vec<String> = self.classes.keys().cloned().collect();
        for cname in names {
            if !self.is_descendant_of(base_cls, &cname) {
                continue;
            }
            if let Some((dc, fd)) = self.find_method(&cname, mname) {
                if dc != basedefcls {
                    branches.push((cname, dc, fd));
                }
            }
        }
        if branches.is_empty() {
            return None;
        }
        let ret = self.plan_for_class(mname, &basedefcls, &basedef).ret;
        let unit = self.is_unit(ret);
        let resslot: Option<String> = if unit {
            None
        } else {
            let a = fw.v();
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", a));
            Some(a)
        };
        let cid = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_cls_id({}) : (i64) -> i64",
            cid, recv
        ));
        let lbl_def = fw.newlabel("cv");
        let lbl_end = fw.newlabel("cv");
        let labels: Vec<String> = branches.iter().map(|_| fw.newlabel("cv")).collect();
        // class-id switch splits the CFG mid-expression: preserve producers
        // of the enclosing expression across the merge
        let saved_dangling = std::mem::take(&mut fw.dangling);
        for (i, (cname, _, _)) in branches.iter().enumerate() {
            let idw = fw.v();
            let cidn = *self.class_ids.get(cname).unwrap_or(&0);
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                idw,
                enc_i_lit(cidn)
            ));
            let c = fw.v();
            fw.op(&format!(
                "    {} = arith.cmpi eq, {}, {} : i64",
                c, cid, idw
            ));
            let ce = fw.v();
            fw.op(&format!("    {} = arith.extui {} : i1 to i64", ce, c));
            let nxt = if i + 1 < branches.len() {
                fw.newlabel("cv")
            } else {
                lbl_def.clone()
            };
            fw.cjump(&ce, &labels[i], &nxt);
            if i + 1 < branches.len() {
                fw.label(&nxt);
            }
        }
        // default: the base implementation
        fw.label(&lbl_def);
        let (rv, _) =
            self.emit_method_call(fw, &basedefcls, mname, &basedef, false, argv, sigargs, pos);
        if let Some(slot) = &resslot {
            let zi = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                rv, slot, zi
            ));
        }
        fw.jump(&lbl_end);
        for (i, (_, dc, fd)) in branches.iter().enumerate() {
            fw.label(&labels[i]);
            let (rv, _) = self.emit_method_call(fw, dc, mname, fd, false, argv, sigargs, pos);
            if let Some(slot) = &resslot {
                let zi = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", zi));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xi64>",
                    rv, slot, zi
                ));
            }
            fw.jump(&lbl_end);
        }
        fw.label(&lbl_end);
        fw.dangling = saved_dangling;
        if let Some(slot) = resslot {
            let zi = fw.v();
            let v = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            fw.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                v, slot, zi
            ));
            return Some((v, ret));
        }
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        Some((z, ret))
    }
}

impl ModEmitter {
    /// walk the superclass chain up from `cls`, returning
    /// (defining-class name, method def) for the first decl of `name`
    pub(crate) fn find_method(&self, cls: &str, name: &str) -> Option<(String, FuncDef)> {
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            if let Some(ci) = self.classes.get(&c) {
                if let Some((_, f)) = ci.methods.iter().find(|(n, _)| n == name) {
                    return Some((c, f.clone()));
                }
                cur = ci.superclass.clone();
                continue;
            }
            break;
        }
        None
    }
}

pub(crate) fn words_for_cls(me: &ModEmitter, clsname: &str) -> usize {
    let mut n = 0usize;
    let mut cur = Some(clsname.to_string());
    while let Some(c) = cur {
        match me.classes.get(&c) {
            Some(ci) => {
                n += ci.fields.len();
                cur = ci.superclass.clone();
                continue;
            }
            None => break,
        }
    }
    n + 2
}

impl ModEmitter {
    /// per-class rc field mask: bit i (base-class-first layout) = field i is
    /// a refcounted word; fields are the flat chain of the superclass chain
    pub(crate) fn class_refmask(&self, clsname: &str) -> (i64, i64) {
        let mut chain: Vec<String> = Vec::new();
        let mut cur = Some(clsname.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    chain.push(c.clone());
                    cur = ci.superclass.clone();
                }
                None => break,
            }
        }
        chain.reverse();
        let (mut mask, mut bit, mut nf) = (0i64, 0i64, 0i64);
        for c in chain {
            let ci = match self.classes.get(&c) {
                Some(c2) => c2,
                None => break,
            };
            for (_fn2, ft, _mv) in &ci.fields {
                if !self.is_float(*ft) && self.is_ref(*ft) {
                    mask |= 1 << bit;
                }
                bit += 1;
                nf += 1;
            }
        }
        (mask, nf)
    }
}

impl ModEmitter {
    /// allocate object with GC; fields set after ctor body
    pub(crate) fn emit_new_obj(
        &mut self,
        fw: &mut FnWalk,
        clsname: &str,
        argv: &Vec<(String, TyId)>,
        _sigargs: &Vec<String>,
        pos: &Pos,
    ) -> (String, TyId) {
        let nf = words_for_cls(self, clsname);
        if !self.classes.contains_key(clsname) {
            self.err(pos, format!("unknown class `{}`", clsname));
            return (String::new(), self.r.mk(Ty::Unit));
        }
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        let clsid = *self.class_ids.get(clsname).unwrap_or(&0);
        let ids = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            ids,
            enc_i_lit(clsid)
        ));
        let cid = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_cls_info({}, {}) : (i64, i64) -> i64",
            cid, z, ids
        ));
        // rc migration patch C: crate the class's refcounted-field mask so
        // the rt cascade releases ref fields on instance death
        {
            let (mask, nft) = self.class_refmask(clsname);
            let mv = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", mv, mask));
            let nfv = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", nfv, nft));
            fw.op(&format!(
                "    call @sloth_cls_refmask({}, {}, {}) : (i64, i64, i64) -> i64",
                cid, mv, nfv
            ));
        }
        let nfw = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            nfw,
            enc_i_lit(nf as i64)
        ));
        let r2 = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_new({}, {}) : (i64, i64) -> i64",
            r2, cid, nfw
        ));
        // rc patch B: fresh instance = producer temp
        let ot2 = self.r.mk(Ty::Named(clsname.to_string(), vec![]));
        self.dangling_producer(fw, &r2, ot2);
        self.emit_vt_build(fw, clsname, &r2);
        // field initializers run before __init__
        {
            // field decls need the original program AST: find via decl map.
            // The whole ancestor chain runs base-first so inherited field
            // defaults are present on every subclass instance.
            let defs: Vec<(
                String,
                Option<sloth_frontend::ast::Expr>,
                sloth_frontend::ast::Type,
            )> = {
                let mut chain: Vec<String> = Vec::new();
                let mut cur = Some(clsname.to_string());
                while let Some(c) = cur {
                    cur = self.classes.get(&c).and_then(|ci| ci.superclass.clone());
                    chain.push(c);
                }
                chain.reverse();
                let mut out = Vec::new();
                for c in chain {
                    if let Some((_, cdef)) = self.class_defs.get(&c) {
                        for fd in &cdef.fields {
                            out.push((fd.name.clone(), fd.init.clone(), fd.ty.clone()));
                        }
                    }
                }
                out
            };
            for (fname, iopt, fty) in defs {
                if let Some(ix) = &iopt {
                    let idx = self.field_index(clsname, &fname);
                    let (mut iv, iit) = self.emit_expr(fw, ix);
                    // design §2.1: no implicit numeric conversion on field
                    // initializers (int vs float is a compile error)
                    let ftt = self.ty_of(&fty);
                    let ftf = self.is_float(ftt);
                    if ftf != self.is_float(iit) {
                        let an = sloth_frontend::ty::ty_name(self.r.get(ftt));
                        let bn = sloth_frontend::ty::ty_name(self.r.get(iit));
                        self.err_diff(&ix.pos, "field initializer", &an, &bn);
                        if ftf {
                            iv = iw_to_f64_word(fw, &iv);
                        }
                    }
                    let zi = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        zi,
                        enc_i_lit(idx as i64)
                    ));
                    let ft2 = self.ty_of(&fty);
                    self.op_set_field(fw, &r2, &zi, &iv, iit, ft2, ix.pos.clone());
                }
            }
        }
        let fdinit = self.find_method(clsname, "__init__");
        if let Some((defcls, fd)) = fdinit {
            let saved_mod = self.cur_mod.clone();
            self.cur_mod = self
                .cls_mod
                .get(&defcls)
                .cloned()
                .unwrap_or_else(|| self.name.clone());
            let plan = self.plan_mangled("__init__", Some(&defcls), &fd, None);
            self.cur_mod = saved_mod;
            // ctor args: every word-plane param is one tagged i64 word
            let argvals: Vec<String> = argv
                .iter()
                .zip(plan.params.iter().skip(1))
                .map(|((v, _t), _p)| v.clone())
                .collect();
            let vals = [r2.clone()]
                .iter()
                .cloned()
                .chain(argvals.iter().cloned())
                .collect::<Vec<_>>()
                .join(", ");
            let tys = plan
                .params
                .iter()
                .map(|_p| "i64".to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let rt = mlir_ret_ty(self, plan.ret);
            if self.is_unit(plan.ret) {
                fw.op(&format!(
                    "    call @{}({}) : ({}) -> ()",
                    plan.mangled, vals, tys
                ));
            } else {
                let rr = fw.v();
                fw.op(&format!(
                    "    {} = call @{}({}) : ({}) -> {}",
                    rr, plan.mangled, vals, tys, rt
                ));
            }
        }
        (r2, self.r.mk(Ty::Named(clsname.to_string(), vec![])))
    }
}

impl ModEmitter {
    /// call the ctor to build the object body, then return it
    pub(crate) fn emit_method_call(
        &mut self,
        fw: &mut FnWalk,
        cls: &str,
        mname: &str,
        m: &FuncDef,
        is_super: bool,
        argv: &Vec<(String, TyId)>,
        sigargs: &Vec<String>,
        pos: &Pos,
    ) -> (String, TyId) {
        // ctor: emit the class ctor wrapper (allocates then runs __init__)
        // (skipped for super.__init__: that runs as a plain method on this)
        if mname == "__init__" && !is_super {
            return self.emit_new_obj(fw, cls, argv, sigargs, pos);
        }
        self.stat_dcalls += 1;
        // direct method dispatch: obj.method(args) => sloth_<mod>_Cls__method(this, args...)
        let saved_mod = self.cur_mod.clone();
        // instance frames: generic-class instance methods resolve T via their base
        let instf = self.class_frames.get(cls).cloned();
        if let Some(fr) = instf.clone() {
            self.tp_subst.push(fr);
        }
        self.cur_mod = self
            .cls_mod
            .get(cls)
            .cloned()
            .unwrap_or_else(|| self.name.clone());
        let plan = self.plan_mangled(mname, Some(cls), m, None);
        self.cur_mod = saved_mod;
        if instf.is_some() {
            self.tp_subst.pop();
        }
        let is_ll = self
            .llvm_method
            .contains(&(cls.to_string(), mname.to_string()));
        let vals: Vec<String> = {
            // patch 42: caller-side Opt-param coercion before the emission;
            // the ABI surface follows the PLAN spelling (boxes ride i64)
            let vals2 = self.coerce_args_to_params(fw, argv, &plan.params);
            let _sigs: Vec<String> = match plan.params.len().cmp(&argv.len()) {
                std::cmp::Ordering::Greater => vec![],
                _ => vec![],
            };
            let _ = _sigs;
            vals2
        };
        let tys: String = plan
            .params
            .iter()
            .take(vals.len())
            .map(|_p| "i64".to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let _ = sigargs;
        let ret = mlir_ret_ty(self, plan.ret);
        let callkw = if is_ll { "llvm.call" } else { "call" };
        if self.is_unit(plan.ret) {
            fw.op(&format!(
                "    {} @{}({}) : ({}) -> ()",
                callkw,
                plan.mangled,
                vals.join(", "),
                tys
            ));
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, plan.ret);
        }
        let r = fw.v();
        fw.op(&format!(
            "    {} = {} @{}({}) : ({}) -> {}",
            r,
            callkw,
            plan.mangled,
            vals.join(", "),
            tys,
            ret
        ));
        if self.is_ref(plan.ret) {
            fw.rc_mark_xfer(&r);
        }
        (r, plan.ret)
    }
}

impl ModEmitter {
    /// if `method` is a trait method on a trait implemented (transitively) by
    /// `cls`, return the owning trait + its vtable slot. Used to route
    /// `this.method()` / base-typed `obj.method()` through the vtable so an
    /// override in a subclass is honored (virtual dispatch, design §2.4).
    pub(crate) fn trait_slot_for(&self, cls: &str, method: &str) -> Option<(String, usize)> {
        let mut impls: Vec<String> = Vec::new();
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    for t in &ci.impls {
                        if !impls.contains(t) {
                            impls.push(t.clone());
                        }
                    }
                    cur = ci.superclass.clone();
                }
                None => break,
            }
        }
        for tr in &impls {
            if let Some(ms) = self.traits.get(tr) {
                if ms.iter().any(|m| m.name == method) {
                    if let Some(slot) = self.vt_slots.get(&(tr.clone(), method.to_string())) {
                        return Some((tr.clone(), *slot));
                    }
                }
            }
        }
        None
    }
}

impl ModEmitter {
    /// resolve a method plan against a class, honoring its owning module
    pub(crate) fn plan_for_class(&mut self, mname: &str, cls: &str, m: &FuncDef) -> FuncPlan {
        let saved_mod = self.cur_mod.clone();
        self.cur_mod = self
            .cls_mod
            .get(&cls.to_string())
            .cloned()
            .unwrap_or_else(|| self.name.clone());
        let plan = self.plan_mangled(mname, Some(cls), m, None);
        self.cur_mod = saved_mod;
        plan
    }
}

impl ModEmitter {
    /// stable slot capacity: cover every declared trait surface; called
    /// after registration/collect and before any emission
    pub(crate) fn finalize_vt(&mut self) {
        let surfaces: Vec<(String, Vec<String>)> = self
            .traits
            .iter()
            .map(|(t, ms)| (t.clone(), ms.iter().map(|m| m.name.clone()).collect()))
            .collect();
        for (t, ms) in surfaces {
            for m in ms {
                self.vt_slot(&t, &m);
            }
        }
        self.vt_cap = self.vt_slots.len();
    }
}

impl ModEmitter {
    /// attach the class vtable to a fresh object (header word 1). The class
    /// vtable is built once per class by a lazy builder func and cached in a
    /// mutable global cell (patch #28: per-cls global vtable cache); object
    /// creation just fetches the cached pointer.
    pub(crate) fn emit_vt_build(&mut self, fw: &mut FnWalk, clsname: &str, obj: &str) {
        // effective impls: union over the superclass chain
        let mut impls: Vec<String> = Vec::new();
        let mut cur = Some(clsname.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    for t in &ci.impls {
                        if !impls.contains(t) {
                            impls.push(t.clone());
                        }
                    }
                    cur = ci.superclass.clone();
                }
                None => break,
            }
        }
        if impls.is_empty() || self.vt_cap == 0 {
            return;
        }
        let builder = self.emit_vt_builder(clsname, &impls);
        let vt = fw.v();
        fw.op(&format!("    {} = call @{}() : () -> i64", vt, builder));
        fw.op(&format!(
            "    call @sloth_obj_set_vtable({}, {}) : (i64, i64) -> i64",
            obj, vt
        ));
    }

    /// per-cls vtable builder `@sloth_vtb_<mod>_<cls>() -> i64`: builds the
    /// class vtable (slot -> raw fn pointer of the resolved llvm.func method)
    /// on first sight, keeps it in a mutable global cell, and returns it.
    /// Emits once per class; later objects reuse the cached pointer.
    fn emit_vt_builder(&mut self, clsname: &str, impls: &Vec<String>) -> String {
        let fname = format!("sloth_vtb_{}_{}", self.name, clsname);
        if self.vt_built.contains(&fname) {
            return fname;
        }
        self.vt_built.insert(fname.clone());
        self.stat_vtbuilds += 1;
        let ity = self.r.mk(Ty::I64);
        let modname = self.name.clone();
        let gsym = self.declare_global(&modname, &format!("vtb_{}", clsname), ity);
        let mut fw = fresh_walk(self);
        let g = fw.v();
        fw.op(&format!(
            "    {} = memref.get_global @{} : memref<1xi64>",
            g, gsym
        ));
        let zi = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", zi));
        let cached = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            cached, g, zi
        ));
        let zz = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zz));
        let cc = fw.v();
        fw.op(&format!(
            "    {} = arith.cmpi eq, {}, {} : i64",
            cc, cached, zz
        ));
        let cce = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", cce, cc));
        let lbl_build = fw.newlabel("vtb");
        let lbl_done = fw.newlabel("vtb");
        fw.cjump(&cce, &lbl_build, &lbl_done);
        // miss: build the class vtable once
        fw.label(&lbl_build);
        let ncap = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            ncap,
            enc_i_lit(self.vt_cap as i64)
        ));
        let vt = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_vt_new({}) : (i64) -> i64",
            vt, ncap
        ));
        let mut slots: Vec<(usize, String, String)> = self
            .vt_slots
            .iter()
            .map(|((t, m), &s)| (s, t.clone(), m.clone()))
            .collect();
        slots.sort_by_key(|x| x.0);
        for (slot, tr, m) in slots {
            if !impls.contains(&tr) {
                continue;
            }
            let (defcls, fd) = match self.find_method(clsname, &m) {
                Some(x) => x,
                None => continue,
            };
            let plan = self.plan_for_class(&m, &defcls, &fd);
            let fpa = fw.v();
            fw.op(&format!(
                "    {} = llvm.mlir.addressof @{} : !llvm.ptr",
                fpa, plan.mangled
            ));
            let fp = fw.v();
            fw.op(&format!(
                "    {} = llvm.ptrtoint {} : !llvm.ptr to i64",
                fp, fpa
            ));
            let slotc = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                slotc,
                enc_i_lit(slot as i64)
            ));
            // tagged fn-pointer word (bit0 set); dyn dispatch clears it back
            let one = fw.v();
            fw.op(&format!("    {} = arith.constant 1 : i64", one));
            let fpw = fw.v();
            fw.op(&format!("    {} = arith.ori {}, {} : i64", fpw, fp, one));
            fw.op(&format!(
                "    call @sloth_vt_set({}, {}, {}) : (i64, i64, i64) -> i64",
                vt, slotc, fpw
            ));
        }
        // cache the pointer for later objects, then merge
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            vt, g, zi
        ));
        fw.jump(&lbl_done);
        // hit: return the cached pointer
        fw.label(&lbl_done);
        let res = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            res, g, zi
        ));
        fw.op(&format!("    return {} : i64", res));
        self.out.push_str(&format!(
            "  func.func private @{}() -> i64 {{\n{}  }}\n",
            fname, fw.cur
        ));
        fname
    }
}

impl ModEmitter {
    /// `dyn T` receiver: vtable dispatch. Object word 1 holds the class
    /// vt pointer; slot = (trait, method) index; slot value = raw fn-pointer
    /// word of the llvm.func-emitted resolved method.
    pub(crate) fn emit_dyn_call(
        &mut self,
        fw: &mut FnWalk,
        tname: &str,
        mname: &str,
        recv: &str,
        argv: &[(String, TyId)],
        _sigargs: &Vec<String>,
        pos: &Pos,
    ) -> (String, TyId) {
        self.stat_dyncalls += 1;
        // dispatch ABI from the trait method signature
        let ms = match self
            .traits
            .get(tname)
            .and_then(|ts| ts.iter().find(|x| x.name == mname))
            .cloned()
        {
            Some(ms) => ms,
            None => {
                self.err(
                    pos,
                    format!("unknown trait `{}` for method `{}`", tname, mname),
                );
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                return (z, self.r.mk(Ty::Unit));
            }
        };
        let ret_flt = self.sig_word_float(Some(ms.ret.clone()));
        let ret_unit = matches!(ms.ret, Type::Unit);
        let slot = self.vt_slot(tname, mname);
        // word plane: every param/return is one tagged i64 word
        let mut tys: Vec<String> = vec!["i64".to_string()];
        for _sp in &ms.params {
            tys.push("i64".to_string());
        }
        // result slot: keeps SSA dominance across the two branches
        let resslot: Option<(String, bool)> = if ret_unit {
            None
        } else {
            let a = fw.v();
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", a));
            Some((a, ret_flt))
        };
        // object header word 1 -> class vtable; slot -> fn ptr
        let vt = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_vtable({}) : (i64) -> i64",
            vt, recv
        ));
        let slotc = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            slotc,
            enc_i_lit(slot as i64)
        ));
        let fp = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_vt_get({}, {}) : (i64, i64) -> i64",
            fp, vt, slotc
        ));
        let zero = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zero));
        let cc = fw.v();
        fw.op(&format!(
            "    {} = arith.cmpi ne, {}, {} : i64",
            cc, fp, zero
        ));
        let ce = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", ce, cc));
        let lbl_call = fw.newlabel("dc");
        let lbl_panic = fw.newlabel("dp");
        let lbl_end = fw.newlabel("de");
        // A vtable call splits the CFG mid-expression: producers from the
        // ENCLOSING expression dominate the merge and must survive the
        // branch (releasing them at `cjump` would free live operands, e.g.
        // the lhs of `"x" + obj.name()`). Producers created inside the
        // branches do not exist, so none are dropped.
        let saved_dangling = std::mem::take(&mut fw.dangling);
        fw.cjump(&ce, &lbl_call, &lbl_panic);
        // resolved: call through the slot pointer
        fw.label(&lbl_call);
        // clear the tag bit: slots hold tagged fn-pointer words
        let fm2 = fw.v();
        fw.op(&format!("    {} = arith.constant -2 : i64", fm2));
        let fpr = fw.v();
        fw.op(&format!("    {} = arith.andi {}, {} : i64", fpr, fp, fm2));
        let vp = fw.v();
        fw.op(&format!(
            "    {} = llvm.inttoptr {} : i64 to !llvm.ptr",
            vp, fpr
        ));
        let mut vals: Vec<String> = Vec::new();
        vals.extend(argv.iter().map(|x| x.0.clone()));
        let sig = tys.join(", ");
        let ret_ty_txt = "i64".to_string();
        if ret_unit {
            fw.op(&format!(
                "    llvm.call {}({}) : !llvm.ptr, ({}) -> ()",
                vp,
                vals.join(", "),
                sig
            ));
        } else {
            let rv = fw.v();
            fw.op(&format!(
                "    {} = llvm.call {}({}) : !llvm.ptr, ({}) -> {}",
                rv,
                vp,
                vals.join(", "),
                sig,
                ret_ty_txt
            ));
            if let Some((slot2, _fl)) = &resslot {
                let zi = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", zi));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xi64>",
                    rv, slot2, zi
                ));
            }
        }
        fw.jump(&lbl_end);
        fw.label(&lbl_panic);
        let pv = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", pv));
        let pz = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_panic_noimpl({}) : (i64) -> i64",
            pz, pv
        ));
        if let Some((slot2, _fl)) = &resslot {
            let zi = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            let z2 = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z2));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                z2, slot2, zi
            ));
        }
        fw.jump(&lbl_end);
        fw.label(&lbl_end);
        // enclosing-expression producers resume ownership at the merge
        fw.dangling = saved_dangling;
        if let Some((slot2, fl)) = resslot {
            let zi = fw.v();
            let v = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            fw.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                v, slot2, zi
            ));
            // surface type comes from the trait signature (str/fn keep identity)
            let tret = if fl {
                self.r.mk(Ty::F64)
            } else {
                self.ty_of(&ms.ret)
            };
            return (v, tret);
        }
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }
}

impl ModEmitter {
    /// typed field store (float fields use the f64 rt routine); rc patch B:
    /// the old field word is released BEFORE the overwrite (unconditional —
    /// rt no-ops nil/untracked/scalars), the new value is retained (slot
    /// ownership: the object now owns its field copy)
    pub(crate) fn op_set_field(
        &mut self,
        fw: &mut FnWalk,
        obj: &str,
        idx: &str,
        v: &str,
        vt: TyId,
        ft: TyId,
        pos: Pos,
    ) {
        let _ = pos;
        // patch 42: value-optional fields box bare scalar stores (nil /
        // already-opt words pass through untouched)
        let (v, _vt2) = if self.opt_inner(ft).is_some() || self.weak_inner(ft).is_some() {
            self.coerce_word_to(fw, v, vt, ft)
        } else {
            (v.to_string(), vt)
        };
        let _ = _vt2;
        // tag migration: one word route — release the overwritten word and
        // retain the new one; rt no-ops on non-ref/nil words, so the owner
        // bookkeeping is unconditional
        let old = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
            old, obj, idx
        ));
        self.emit_release(fw, &old);
        let rv = self.emit_retain(fw, &v);
        fw.op(&format!(
            "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
            obj, idx, rv
        ));
    }
}
