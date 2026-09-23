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
            "    {} = func.call @sloth_cls_info({}, {}) : (i64, i64) -> i64",
            ci, z, cid
        ));
        // per-frame reference mask: a captured field is a ref exactly when
        // its surface type is (base-first == capture order)
        let mut cmask: i64 = 0;
        for (j, cn) in caps.iter().enumerate() {
            if let Some((_a, ct)) = fw.lookup(cn) {
                if !self.is_float(ct) && self.is_ref(ct) {
                    cmask |= 1 << j;
                }
            }
        }
        {
            let mvc = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                mvc,
                enc_i_lit(cmask)
            ));
            let nvc = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                nvc,
                enc_i_lit(ncap as i64)
            ));
            fw.op(&format!(
                "    func.call @sloth_cls_refmask({}, {}, {}) : (i64, i64, i64) -> i64",
                ci, mvc, nvc
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
            "    {} = func.call @sloth_obj_new({}, {}) : (i64, i64) -> i64",
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
                    fw.op(&format!("    {} = arith.constant 0 : index", zz));
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
                    "    func.call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                    frame, zi, rcv
                ));
            } else {
                fw.op(&format!(
                    "    func.call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
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
        // a lambda body is its own function: it must not inherit the enclosing
        // generic instance's mangled name (`tp_mangled`), or every instantiation
        // would share one body whose ARC depends on the concrete type args
        let saved_tpm = self.tp_mangled.pop();
        let sym = self.emit_func(&lname, None, &fd, None, false);
        if let Some(m) = saved_tpm {
            self.tp_mangled.push(m);
        }
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
                | "fiber"
                | "tensor"
                | "thread"
                | "channel"
                | "mutex"
                | "atomic"
                | "true"
                | "false"
        ) || me.funcs.contains_key(&u)
            || me.cross_funcs.contains_key(&u)
            || me.class_defs.contains_key(&u)
            // qualified cross-module callees: `event.sleep(...)`, `io.bytes_new(...)`
            // — the module qualifier is not a captured variable
            || me.init_mods.contains(&u)
            || me.mod_alias.contains_key(&u);
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
        Ty::Int(k) => Some(Type::Simple(SimpleType::FixedInt(k))),
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
        Ty::Fiber(e) => {
            let et = syn_ty_of(me, e)?;
            let ea = match et {
                Type::Simple(s) => s,
                other => SimpleType::Ident(plain_ty_name(&other)),
            };
            Some(Type::Simple(SimpleType::Named(
                "Fiber".into(),
                vec![Type::Simple(ea)],
            )))
        }
        Ty::JoinHandle(e) => {
            let et = syn_ty_of(me, e)?;
            Some(Type::Simple(SimpleType::Named(
                "JoinHandle".into(),
                vec![et],
            )))
        }
        Ty::Channel(e) => {
            let et = syn_ty_of(me, e)?;
            Some(Type::Simple(SimpleType::Named("Channel".into(), vec![et])))
        }
        Ty::Mutex => Some(Type::Simple(SimpleType::Ident("Mutex".into()))),
        Ty::AtomicInt => Some(Type::Simple(SimpleType::Ident("AtomicInt".into()))),
        Ty::Tensor(e, rank) => {
            let el = match me.r.get(e) {
                Ty::F64 => Type::prim(Prim::Float),
                Ty::I64 => Type::prim(Prim::Int),
                _ => return None,
            };
            Some(Type::Simple(SimpleType::Tensor(Box::new(el), rank)))
        }
        Ty::Dyn(n) => Some(Type::Simple(SimpleType::Dyn(n))),
        Ty::Fn(ft) => {
            // captured function value: keep the surface a callable function so
            // the generated lambda body can call the capture indirectly
            let params: Vec<Type> = ft
                .params
                .iter()
                .map(|p| syn_ty_of(me, *p).unwrap_or(Type::Simple(SimpleType::Ident("_".into()))))
                .collect();
            let ret = syn_ty_of(me, ft.ret).unwrap_or(Type::Unit);
            Some(Type::Simple(SimpleType::Fn(Box::new(FnType {
                params,
                ret,
            }))))
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
            SimpleType::FixedInt(k) => k.name().into(),
            SimpleType::Float => "float".into(),
            SimpleType::Str => "str".into(),
            SimpleType::Range => "range".into(),
            SimpleType::Any => "any".into(),
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
