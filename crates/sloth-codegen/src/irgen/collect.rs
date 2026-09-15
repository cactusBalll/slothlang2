//! Pass 1: symbol collection, fn planning, impl checks, vtable slots.

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
    pub(crate) fn plan_mangled(
        &mut self,
        name: &str,
        cls: Option<&str>,
        f: &FuncDef,
        variadic: Option<&Variadic>,
    ) -> FuncPlan {
        let mangled = mangle(&self.cur_mod.clone(), cls, name);
        let plan = self.plan_func(name, cls, f, variadic);
        FuncPlan {
            mangled,
            params: plan.params,
            ret: plan.ret,
        }
    }
}

impl ModEmitter {
    pub fn collect(&mut self, prog: &Program) {
        // traits first: type positions may resolve `T`/`dyn T` while collecting
        for d in &prog.decls {
            if let DeclNode::Trait(t) = &d.node {
                self.traits.insert(d.name.clone(), t.methods.clone());
            }
        }
        let mut class_id = 0i64;
        for d in &prog.decls {
            match &d.node {
                DeclNode::Func(f) => {
                    self.funcs.insert(d.name.clone(), (**f).clone());
                }
                DeclNode::Var { ty, .. } => {
                    let t = match ty {
                        Some(t) => self.ty_of(t),
                        None => self.r.mk(Ty::Unit),
                    };
                    let sym = self.declare_global(&self.name.clone(), &d.name, t);
                    self.globals
                        .insert(d.name.clone(), (sym, t, d.kind == DeclKind::Var));
                }
                DeclNode::Class(c) => {
                    self.class_ids.insert(d.name.clone(), class_id);
                    if !self.class_order.contains(&d.name) {
                        self.class_order.push(d.name.clone());
                    }
                    class_id += 1;
                    let fields = c
                        .fields
                        .iter()
                        .map(|fd| (fd.name.clone(), self.ty_of(&fd.ty), fd.mutable))
                        .collect();
                    let meth: Vec<(String, FuncDef)> = c
                        .methods
                        .iter()
                        .map(|m| (m.name.clone(), m.fd.clone()))
                        .collect();
                    self.class_defs
                        .insert(d.name.clone(), (self.cur_mod.clone(), (**c).clone()));
                    self.classes.insert(
                        d.name.clone(),
                        ClassInfo {
                            name: d.name.clone(),
                            fields,
                            methods: meth,
                            superclass: c.superclass.clone(),
                            impls: c.impls.clone(),
                        },
                    );
                }
                DeclNode::Trait(t) => {
                    let _ = t;
                    // already registered in the pre-pass
                }
                DeclNode::ExternType => {
                    self.extern_types.insert(d.name.clone());
                }
            }
        }
        // validate `impl` surfaces after all classes are collected;
        // subclass inheritance transitively carries the trait surface
        for d in &prog.decls {
            if let DeclNode::Class(c) = &d.node {
                let mut eff: Vec<String> = Vec::new();
                let mut cur = Some(d.name.clone());
                while let Some(pn) = cur {
                    match self.classes.get(&pn) {
                        Some(ci) => {
                            for t in &ci.impls {
                                if !eff.contains(t) {
                                    eff.push(t.clone());
                                }
                            }
                            cur = ci.superclass.clone();
                        }
                        None => break,
                    }
                }
                self.check_impls(&d.name, &eff, &d.pos);
            }
        }
        // class ctor discipline (patch #25): a subclass ctor must invoke
        // super.__init__ (anywhere in its body)
        for d in &prog.decls {
            if let DeclNode::Class(c) = &d.node {
                if c.superclass.is_none() {
                    continue;
                }
                let init = self
                    .classes
                    .get(&d.name)
                    .and_then(|ci| ci.methods.iter().find(|(n, _)| n == "__init__"))
                    .map(|(_, fd)| fd.clone());
                let has = init
                    .as_ref()
                    .map(|fd| stmt_has_super_init(&fd.body))
                    .unwrap_or(true); // no declared ctor: inherited surface, not enforced
                if !has {
                    self.err(
                        &d.pos,
                        "constructor of `{}` must call super.__init__"
                            .to_string()
                            .replace("{}", &d.name),
                    );
                }
            }
        }
    }
}

impl ModEmitter {
    /// check a class's declared traits: known + every method satisfied
    /// by the class chain with the same arity (this param excluded).
    /// Also assigns global vtable slots, marks slot-resolved methods
    /// for llvm.func emission, and checks the call ABI (word kinds).
    pub(crate) fn check_impls(&mut self, cls: &str, impls: &[String], pos: &Pos) {
        for tr in impls {
            let sigs = match self.traits.get(tr) {
                Some(s) => s.clone(),
                None => {
                    self.err(pos, format!("unknown trait `{}` in impl", tr));
                    continue;
                }
            };
            for m in &sigs {
                self.vt_slot(tr, &m.name);
                match self.find_method(cls, &m.name) {
                    Some((defcls, fd)) => {
                        if fd.params.len() != m.params.len() {
                            let want = m.params.len();
                            let got = fd.params.len();
                            self.err(
                                pos,
                                format!(
                                    "trait `{}` method `{}` arity: want {}, `{}`.{} has {}",
                                    tr, m.name, want, defcls, m.name, got
                                ),
                            );
                        }
                        // ABI check: word kinds must match the trait signature
                        for (i, sp) in m.params.iter().enumerate() {
                            let sfl = self.sig_word_float(sp.ty.clone());
                            let ft = match fd.params.get(i) {
                                Some(p) => match &p.ty {
                                    Some(t) => {
                                        let it = self.ty_of(t);
                                        self.is_float(it)
                                    }
                                    None => false,
                                },
                                None => false,
                            };
                            if sfl != ft {
                                self.err(
                                    pos,
                                    format!(
                                        "trait `{}` method `{}` param {}: ABI word mismatch",
                                        tr,
                                        m.name,
                                        i + 1
                                    ),
                                );
                            }
                        }
                        let sret = self.sig_word_float(Some(m.ret.clone()));
                        let fret = match &fd.ret {
                            Some(t) => {
                                let it = self.ty_of(t);
                                self.is_float(it)
                            }
                            None => false,
                        };
                        if sret != fret {
                            self.err(
                                pos,
                                format!(
                                    "trait `{}` method `{}` return: ABI word mismatch",
                                    tr, m.name
                                ),
                            );
                        }
                        self.llvm_method.insert((defcls.clone(), m.name.clone()));
                    }
                    None => {
                        // default body: synthesize the method on the class
                        if let Some(dbody) = &m.body {
                            let fd = FuncDef {
                                type_params: Vec::new(),
                                params: m.params.clone(),
                                variadic: None,
                                ret: Some(m.ret.clone()),
                                body: dbody.clone(),
                                is_extern: false,
                            };
                            let ci = match self.classes.get_mut(cls) {
                                Some(c) => c,
                                None => {
                                    self.err(
                                        pos,
                                        format!("trait `{}` impl on missing class `{}`", tr, cls),
                                    );
                                    continue;
                                }
                            };
                            ci.methods.push((m.name.clone(), fd));
                            // synthesized body must be vtable-addressable
                            self.llvm_method.insert((cls.to_string(), m.name.clone()));
                            _ = dbody;
                            continue;
                        }
                        self.err(
                            pos,
                            format!(
                                "trait `{}` method `{}` not implemented by `{}`",
                                tr, m.name, cls
                            ),
                        );
                    }
                }
            }
        }
    }
}

impl ModEmitter {
    /// (trait, method) -> slot; allocates on first sight
    pub(crate) fn vt_slot(&mut self, tr: &str, m: &str) -> usize {
        let key = (tr.to_string(), m.to_string());
        if let Some(&s) = self.vt_slots.get(&key) {
            return s;
        }
        let s = self.vt_slots.len();
        self.vt_slots.insert(key, s);
        s
    }
}

pub(crate) struct FuncPlan {
    pub(crate) mangled: String,
    pub(crate) params: Vec<(String, TyId, bool)>, // name, ty, is_float
    pub(crate) ret: TyId,
}

impl ModEmitter {
    pub(crate) fn plan_func(
        &mut self,
        name: &str,
        cls: Option<&str>,
        f: &FuncDef,
        variadic: Option<&Variadic>,
    ) -> FuncPlan {
        let mut params: Vec<(String, TyId, bool)> = Vec::new();
        if let Some(c) = cls {
            let nilcls = self.r.mk(Ty::Named(c.to_string(), vec![]));
            params.push(("this".to_string(), nilcls, false));
        }
        for p in &f.params {
            let t = match &p.ty {
                Some(t) => self.ty_of(t),
                None => self.r.mk(Ty::Unit),
            };
            params.push((p.name.clone(), t, self.is_float(t)));
        }
        if let Some(v) = variadic {
            let t = self.ty_of(&v.elem);
            // variadic packs into Array<T>; treated as one ref param
            params.push((v.name.clone(), self.r.mk(Ty::Array(t)), false));
        }
        let ret = match &f.ret {
            Some(t) => self.ty_of(t),
            None => self.r.mk(Ty::Unit),
        };
        let mangled = if let Some(m) = self.tp_mangled.last() {
            m.clone()
        } else {
            mangle(&self.cur_mod.clone(), cls, name)
        };
        FuncPlan {
            mangled,
            params,
            ret,
        }
    }
}
