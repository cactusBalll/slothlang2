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
        // rc migration patch C: frames own ref-typed captures — the mask
        // makes the death cascade release each capture's count
        {
            let mut lam_mask = 0i64;
            for (j, _cn) in caps.iter().enumerate() {
                match fw.lookup(&_cn.clone()) {
                    Some((_, t)) => {
                        if self.is_ref(t) {
                            lam_mask |= 1 << j;
                        }
                    }
                    None => {}
                };
            }
            let mvc = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", mvc, lam_mask));
            let nfc = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", nfc, ncap));
            fw.op(&format!(
                "    call @sloth_cls_refmask({}, {}, {}) : (i64, i64, i64) -> i64",
                ci, mvc, nfc
            ));
        }
        let nf = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            nf,
            enc_i_lit(ncap as i64)
        ));
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
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                zi,
                enc_i_lit(j as i64)
            ));
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
            .map(|c| {
                let ty = match fw.lookup(c) {
                    Some((_a, t)) => syn_ty_of(self, t),
                    None => None,
                };
                Param {
                    name: c.clone(),
                    ty,
                }
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
        // first-class value: wrap the capture frame behind a uniform-ABI
        // bridge, yielding a 2-word closure object
        let bkey = format!("lam:{}:{}", sym, ncap);
        let bridge = self.fn_bridge(
            &bkey,
            &sym,
            ncap,
            l.params.len(),
            crate::irgen::closure::BR_LAMBDA,
            false,
            self.is_unit(ret),
        );
        let fp = self.emit_fnptr_word(fw, &bridge);
        let cbox = self.emit_closure_box(fw, &fp, &frame);
        let ft = self.r.mk(Ty::Fn(FnTy {
            params: pty,
            ret,
            lam: Some(LamMeta {
                sym: bridge,
                caps: Vec::new(),
            }),
        }));
        // fresh box = producer temp (released at stmt close unless stored)
        self.dangling_producer(fw, &cbox, ft);
        (cbox, ft)
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

/// reconstruct a syntactic Type for a capture's resolved surface (patch 44)
fn syn_ty_of(me: &ModEmitter, t: TyId) -> Option<Type> {
    use sloth_frontend::ast::*;
    use sloth_frontend::ty::Ty;
    match me.r.get(t).clone() {
        Ty::Bool => Some(Type::prim(Prim::Bool)),
        Ty::I64 => Some(Type::prim(Prim::Int)),
        Ty::F64 => Some(Type::prim(Prim::Float)),
        Ty::Str => Some(Type::prim(Prim::Str)),
        Ty::Range => Some(Type::prim(Prim::Range)),
        Ty::Named(n, args) => {
            let ats: Vec<Type> = args
                .iter()
                .map(|x| syn_ty_of(me, *x).unwrap_or(Type::Simple(SimpleType::Ident("_".into()))))
                .collect();
            Some(Type::Simple(SimpleType::Named(n.clone(), ats)))
        }
        Ty::Opt(e) => syn_ty_of(me, e).map(|x| Type::Optional(Box::new(x))),
        Ty::Weak(e) => {
            let et = syn_ty_of(me, e)?;
            let ea = match et {
                Type::Simple(s) => s,
                other => SimpleType::Ident(plain_ty_name(&other)),
            };
            Some(Type::Simple(SimpleType::Named(
                "Weak".into(),
                vec![Type::Simple(ea)],
            )))
        }
        Ty::Array(e) => syn_ty_of(me, e).map(|x| Type::Simple(SimpleType::Array(Box::new(x)))),
        Ty::Map(k, v) => {
            let kt = syn_ty_of(me, k)?;
            let vt = syn_ty_of(me, v)?;
            Some(Type::Simple(SimpleType::Map(Box::new(kt), Box::new(vt))))
        }
        _ => None,
    }
}

fn plain_ty_name(t: &Type) -> String {
    use sloth_frontend::ast::*;
    match t {
        Type::Unit => "unit".into(),
        Type::Optional(x) => format!("{}?", plain_ty_name(x)),
        Type::Simple(s) => match s {
            SimpleType::Bool => "bool".into(),
            SimpleType::Int => "int".into(),
            SimpleType::Float => "float".into(),
            SimpleType::Str => "str".into(),
            SimpleType::Range => "range".into(),
            SimpleType::Array(x) => format!("Array<{}>", plain_ty_name(x)),
            SimpleType::Map(k, v) => {
                format!("Map<{}, {}>", plain_ty_name(k), plain_ty_name(v))
            }
            SimpleType::Named(n, _) => n.clone(),
            SimpleType::Ident(n) => n.clone(),
            SimpleType::Dyn(d) => format!("dyn:{}", d),
            SimpleType::Tensor(e, r) => format!("Tensor<{}, {}>", plain_ty_name(e), r),
            SimpleType::Fn(_) => "fn".into(),
        },
    }
}
