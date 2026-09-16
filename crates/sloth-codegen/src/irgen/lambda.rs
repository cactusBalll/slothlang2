//! Snapshot-capturing lambda closure construction.

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
    /// snapshot-capturing lambda: value = frame obj; symbol = clo function
    pub(crate) fn emit_lambda(&mut self, fw: &mut FnWalk, l: &Lambda, pos: &Pos) -> (String, TyId) {
        self.lamcount += 1;
        let lname = format!("lam{}", self.lamcount);
        let caps = lambda_caps(self, l);
        let ncap = caps.len();
        let ret = match &l.ret {
            Some(t) => self.ty_of(t),
            None => self.r.mk(Ty::I64),
        };
        // frame allocation through the object runtime
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        let cid = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", cid));
        let ci = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_cls_info({}, {}) : (i64, i64) -> i64",
            ci, z, cid
        ));
        let nf = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", nf, ncap));
        let frame = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_new({}, {}) : (i64, i64) -> i64",
            frame, ci, nf
        ));
        // rc patch B: fresh frame = producer temp (released at stmt close
        // unless stored)
        {
            let ftv = self.r.mk(Ty::Fn(FnTy {
                params: Vec::new(),
                ret,
                lam: None,
            }));
            self.dangling_producer(fw, &frame, ftv);
        }
        // snapshot each captured variable into the frame words
        for (j, cn) in caps.iter().enumerate() {
            let (cv, ct) = match fw.lookup(cn) {
                Some((a, t)) => {
                    let zz = fw.v();
                    let mty = memref_cell_ty(self, t);
                    fw.op(&format!("    {} = arith.constant 0 : i64", zz));
                    let vv = fw.v();
                    fw.op(&format!("    {} = memref.load {}[{}] : {}", vv, a, zz, mty));
                    (vv, t)
                }
                None => {
                    self.err(pos, format!("lambda captures unknown `{}`", cn));
                    (String::new(), self.r.mk(Ty::Unit))
                }
            };
            let zi = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", zi, j));
            // rc patch B: the frame owns ref-typed captures
            if self.is_ref(ct) {
                let rcv = self.emit_retain(fw, &cv);
                fw.op(&format!(
                    "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                    frame, zi, rcv
                ));
            } else {
                fw.op(&format!(
                    "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                    frame, zi, cv
                ));
            }
        }
        // construct the closured function body
        let mut params: Vec<Param> = caps
            .iter()
            .map(|c| Param {
                name: c.clone(),
                ty: None,
            })
            .collect();
        params.extend(l.params.iter().cloned());
        let fd = FuncDef {
            type_params: Vec::new(),
            params,
            variadic: None,
            ret: Some(l.ret.clone().unwrap_or_else(|| Type::prim(Prim::Int))),
            body: l.body.clone(),
            is_extern: false,
        };
        let sym = self.emit_func(&lname, None, &fd, None, false);
        let pty: Vec<TyId> = l
            .params
            .iter()
            .map(|p| match &p.ty {
                Some(t) => self.ty_of(t),
                None => self.r.mk(Ty::I64),
            })
            .collect();
        let ft = self.r.mk(Ty::Fn(FnTy {
            params: pty,
            ret,
            lam: Some(LamMeta { sym, caps }),
        }));
        (frame, ft)
    }
}

pub(crate) fn lambda_caps(me: &ModEmitter, l: &Lambda) -> Vec<String> {
    let mut used: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut decls: std::collections::HashSet<String> = std::collections::HashSet::new();
    for p in &l.params {
        decls.insert(p.name.clone());
    }
    let mut capfn = |n: &String| {
        if seen.insert(n.clone()) {
            used.push(n.clone());
        }
    };
    walk_ids_stmt(&l.body, &mut capfn, &mut decls);
    let mut out: Vec<String> = Vec::new();
    let mut got: std::collections::HashSet<String> = std::collections::HashSet::new();
    for u in used {
        // only true identifiers count as captures: not a declared call target
        // (builtin face or module funcs) and not already bound (patch #39)
        let callee_like = matches!(
            u.as_str(),
            "ok" | "err"
                | "print"
                | "len"
                | "keys"
                | "values"
                | "str"
                | "int"
                | "float"
                | "bool"
                | "range"
        ) || me.funcs.contains_key(&u)
            || me.cross_funcs.contains_key(&u)
            || me.class_defs.contains_key(&u);
        if !decls.contains(&u)
            && !me.globals.contains_key(&u)
            && !callee_like
            && got.insert(u.clone())
        {
            out.push(u);
        }
    }
    out
}
