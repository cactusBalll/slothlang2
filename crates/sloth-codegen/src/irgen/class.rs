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
        let (mut v, vt) = self.emit_expr(fw, arg);
        // slot route: float field promotes int words; refuse float into ints
        let slot = if is_ok { fvy } else { fey };
        if self.is_float(slot) && !self.is_float(vt) {
            let cv = fw.v();
            fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
            v = cv;
        } else if !self.is_float(slot) && self.is_float(vt) {
            self.err(
                pos,
                "type mismatch: Result slot is a word but a float value was passed".to_string(),
            );
        }
        // object + default zero fields
        let (obj, _ot) = self.emit_new_obj(fw, inst, &Vec::new(), &Vec::new(), pos);
        // ok flag & payload / err pair
        let zi = fw.v();
        let zc = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zi));

        let okv = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            okv,
            if is_ok { 1 } else { 0 }
        ));
        let okidx = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            okidx,
            self.field_index(inst, "ok")
        ));
        self.op_set_field(fw, &obj, &okidx, &okv, fok, pos.clone());
        if is_ok {
            let vidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                vidx,
                self.field_index(inst, "v")
            ));
            self.op_set_field(fw, &obj, &vidx, &v, fvy, pos.clone());
            // zero the err slot by its word spelling
            let ez = fw.v();
            if self.is_float(fey) {
                fw.op(&format!("    {} = arith.constant 0.0 : f64", ez));
            } else {
                fw.op(&format!("    {} = arith.constant 0 : i64", ez));
            }
            let eidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                eidx,
                self.field_index(inst, "e")
            ));
            self.op_set_field(fw, &obj, &eidx, &ez, fey, pos.clone());
        } else {
            // zero the v slot
            let vz = fw.v();
            if self.is_float(fvy) {
                fw.op(&format!("    {} = arith.constant 0.0 : f64", vz));
            } else {
                fw.op(&format!("    {} = arith.constant 0 : i64", vz));
            }
            let vidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                vidx,
                self.field_index(inst, "v")
            ));
            self.op_set_field(fw, &obj, &vidx, &vz, fvy, pos.clone());
            let eidx = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                eidx,
                self.field_index(inst, "e")
            ));
            self.op_set_field(fw, &obj, &eidx, &v, fey, pos.clone());
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
    /// class ctor: named `sloth_main_<Cls>_cls`
    pub(crate) fn class_ctor_name(&self, cls: &str) -> String {
        format!("{}_{}", self.name, cls)
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
        fw.op(&format!("    {} = arith.constant {} : i64", ids, clsid));
        let cid = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_cls_info({}, {}) : (i64, i64) -> i64",
            cid, z, ids
        ));
        let nfw = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", nfw, nf));
        let r2 = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_new({}, {}) : (i64, i64) -> i64",
            r2, cid, nfw
        ));
        self.emit_vt_build(fw, clsname, &r2);
        // field initializers run before __init__
        {
            // field decls need the original program AST: find via decl map
            let defs: Vec<(
                String,
                Option<sloth_frontend::ast::Expr>,
                sloth_frontend::ast::Type,
            )> = {
                match self.class_defs.get(clsname) {
                    Some((_, cdef)) => cdef
                        .fields
                        .iter()
                        .map(|fd| (fd.name.clone(), fd.init.clone(), fd.ty.clone()))
                        .collect(),
                    None => Vec::new(),
                }
            };
            for (fname, iopt, fty) in defs {
                if let Some(ix) = &iopt {
                    let idx = self.field_index(clsname, &fname);
                    let (mut iv, iit) = self.emit_expr(fw, ix);
                    // float field route: int init words get promoted first
                    let ftt = self.ty_of(&fty);
                    let ftf = self.is_float(ftt);
                    if ftf && !self.is_float(iit) {
                        let cv = fw.v();
                        fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, iv));
                        iv = cv;
                    }
                    let zi = fw.v();
                    fw.op(&format!("    {} = arith.constant {} : i64", zi, idx));
                    let ft2 = self.ty_of(&fty);
                    self.op_set_field(fw, &r2, &zi, &iv, ft2, ix.pos.clone());
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
            // ctor args words: int values sitofp-promote to f64 params
            let mut argvals: Vec<String> = Vec::new();
            for ((v, t), (_n, pt, pfl)) in argv.iter().zip(plan.params.iter().skip(1)) {
                if *pfl && !self.is_float(*t) {
                    let cv = fw.v();
                    fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                    argvals.push(cv);
                } else {
                    argvals.push(v.clone());
                }
            }
            let vals = [r2.clone()]
                .iter()
                .cloned()
                .chain(argvals.iter().cloned())
                .collect::<Vec<_>>()
                .join(", ");
            let tys = plan
                .params
                .iter()
                .map(|(_n, t, fl)| {
                    if self.is_float(*t) {
                        "f64".to_string()
                    } else {
                        "i64".to_string()
                    }
                })
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
        let vals: Vec<String> = argv.iter().map(|x| x.0.clone()).collect();
        let tys = sigargs.join(", ");
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
        (r, plan.ret)
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
    /// attach the class vtable to a fresh object (header word 1). Slot value =
    /// raw fn pointer (llvm.mlir.addressof + llvm.ptrtoint) of the resolved
    /// method emitting (llvm.func-marked).
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
        let ncap = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            ncap, self.vt_cap
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
            fw.op(&format!("    {} = arith.constant {} : i64", slotc, slot));
            fw.op(&format!(
                "    call @sloth_vt_set({}, {}, {}) : (i64, i64, i64) -> i64",
                vt, slotc, fp
            ));
        }
        fw.op(&format!(
            "    call @sloth_obj_set_vtable({}, {}) : (i64, i64) -> i64",
            obj, vt
        ));
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
        let mut tys: Vec<String> = vec!["i64".to_string()];
        for sp in &ms.params {
            tys.push(if self.sig_word_float(sp.ty.clone()) {
                "f64".to_string()
            } else {
                "i64".to_string()
            });
        }
        // result slot: keeps SSA dominance across the two branches
        let resslot: Option<(String, bool)> = if ret_unit {
            None
        } else {
            let a = fw.v();
            let mty = if ret_flt {
                "memref<1xf64>"
            } else {
                "memref<1xi64>"
            };
            fw.op(&format!("    {} = memref.alloca() : {}", a, mty));
            Some((a, ret_flt))
        };
        // object header word 1 -> class vtable; slot -> fn ptr
        let vt = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_vtable({}) : (i64) -> i64",
            vt, recv
        ));
        let slotc = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", slotc, slot));
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
        fw.op(&format!("    {} = arith.extsi {} : i1 to i64", ce, cc));
        let lbl_call = fw.newlabel("dc");
        let lbl_panic = fw.newlabel("dp");
        let lbl_end = fw.newlabel("de");
        fw.cjump(&ce, &lbl_call, &lbl_panic);
        // resolved: call through the slot pointer
        fw.label(&lbl_call);
        let vp = fw.v();
        fw.op(&format!(
            "    {} = llvm.inttoptr {} : i64 to !llvm.ptr",
            vp, fp
        ));
        let mut vals: Vec<String> = Vec::new();
        vals.extend(argv.iter().map(|x| x.0.clone()));
        let sig = tys.join(", ");
        let ret_ty_txt = if ret_flt {
            "f64".to_string()
        } else {
            "i64".to_string()
        };
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
            if let Some((slot2, fl)) = &resslot {
                let zi = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", zi));
                let mty = if *fl {
                    "memref<1xf64>"
                } else {
                    "memref<1xi64>"
                };
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : {}",
                    rv, slot2, zi, mty
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
        if let Some((slot2, fl)) = &resslot {
            let zi = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            if *fl {
                let zf = fw.v();
                fw.op(&format!("    {} = arith.constant 0.0 : f64", zf));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xf64>",
                    zf, slot2, zi
                ));
            } else {
                let z2 = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z2));
                fw.op(&format!(
                    "    memref.store {}, {}[{}] : memref<1xi64>",
                    z2, slot2, zi
                ));
            }
        }
        fw.jump(&lbl_end);
        fw.label(&lbl_end);
        if let Some((slot2, fl)) = resslot {
            let zi = fw.v();
            let v = fw.v();
            let mty = if fl { "memref<1xf64>" } else { "memref<1xi64>" };
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            fw.op(&format!(
                "    {} = memref.load {}[{}] : {}",
                v, slot2, zi, mty
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
    /// typed field store (float fields use the f64 rt routine)
    pub(crate) fn op_set_field(
        &mut self,
        fw: &mut FnWalk,
        obj: &str,
        idx: &str,
        v: &str,
        ft: TyId,
        pos: Pos,
    ) {
        let _ = pos;
        if self.is_float(ft) {
            fw.op(&format!(
                "    call @sloth_obj_set_field_f64({}, {}, {}) : (i64, i64, f64) -> i64",
                obj, idx, v
            ));
        } else {
            fw.op(&format!(
                "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                obj, idx, v
            ));
        }
    }
}
