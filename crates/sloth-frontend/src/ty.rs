//! Interned type registry with arena, substitution, and diagnostics.

use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TyId(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Unit,
    Bool,
    I64,
    F64,
    Str,
    Range,
    Array(TyId),
    Map(TyId, TyId),
    /// tensor extension TE-P1: element type + static rank. Rank is not a
    /// substitution key (independent channel; never enters `tp_subst`).
    Tensor(TyId, u32),
    Fn(FnTy),
    Named(String, Vec<TyId>), // user class w/ optional ty args
    Opt(TyId),
    /// Weak<T> reference box (ARC patch D2): holds a weak reference to a
    /// (possibly boxed) target; upgrade() yields T?
    Weak(TyId),
    /// stackful coroutine handle (CE): single payload type Y shared by the
    /// resume/yield channel
    Fiber(TyId),
    Dyn(String),
    /// type param placeholder (during monomorphization substitution)
    Tp(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnTy {
    pub params: Vec<TyId>,
    pub ret: TyId,
    /// lambda frame metadata (None for named function types)
    pub lam: Option<LamMeta>,
}

/// metadata attached to lambda values (frames capture by snapshot)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LamMeta {
    pub sym: String,
    pub caps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Diag {
    pub line: usize,
    pub col: usize,
    pub msg: String,
}

#[derive(Default)]
pub struct Reg {
    pub types: Vec<Ty>,
    /// cache: key = Ty debug key → TyId, for structural reuse
    cache: BTreeMap<String, TyId>,
}

impl Reg {
    pub fn new() -> Reg {
        Reg {
            types: Vec::new(),
            cache: BTreeMap::new(),
        }
    }

    pub fn mk(&mut self, t: Ty) -> TyId {
        let key = fmt_ty(&t);
        if let Some(id) = self.cache.get(&key) {
            return *id;
        }
        let id = TyId(self.types.len() as u32);
        self.types.push(t);
        self.cache.insert(key, id);
        id
    }

    pub fn get(&self, id: TyId) -> &Ty {
        &self.types[id.0 as usize]
    }

    /// structural substitution of type params `Tp(name)` by `rep`
    pub fn subst(&mut self, id: TyId, map: &std::collections::HashMap<String, TyId>) -> TyId {
        match self.get(id).clone() {
            Ty::Tp(n) => *map.get(&n).unwrap_or(&id),
            Ty::Array(e) => {
                let ne = self.subst(e, map);
                self.mk(Ty::Array(ne))
            }
            Ty::Map(k, v) => {
                let nk = self.subst(k, map);
                let nv = self.subst(v, map);
                self.mk(Ty::Map(nk, nv))
            }
            Ty::Tensor(e, rank) => {
                let ne = self.subst(e, map);
                self.mk(Ty::Tensor(ne, rank))
            }
            Ty::Fn(f) => {
                let params = FnTy {
                    params: f.params.clone(),
                    ret: f.ret,
                    lam: None,
                };
                let ps: Vec<TyId> = params.params.iter().map(|p| self.subst(*p, map)).collect();
                let nr = self.subst(params.ret, map);
                self.mk(Ty::Fn(FnTy {
                    params: ps,
                    ret: nr,
                    lam: None,
                }))
            }
            Ty::Named(n, args) => {
                let a2: Vec<TyId> = args.iter().map(|a| self.subst(*a, map)).collect();
                self.mk(Ty::Named(n.clone(), a2))
            }
            Ty::Opt(e) => {
                let ne = self.subst(e, map);
                self.mk(Ty::Opt(ne))
            }
            Ty::Weak(e) => {
                let ne = self.subst(e, map);
                self.mk(Ty::Weak(ne))
            }
            Ty::Fiber(e) => {
                let ne = self.subst(e, map);
                self.mk(Ty::Fiber(ne))
            }
            other => self.mk(other.clone()),
        }
    }
}

/// Printable structural name of a Ty (also backend cache key).
fn fmt_ty(t: &Ty) -> String {
    match t {
        Ty::Unit => "unit".to_string(),
        Ty::Bool => "bool".to_string(),
        Ty::I64 => "i64".to_string(),
        Ty::F64 => "f64".to_string(),
        Ty::Str => "str".to_string(),
        Ty::Range => "range".to_string(),
        Ty::Array(e) => format!("arr:{}", e.0),
        Ty::Map(k, v) => format!("map:{}:{}", k.0, v.0),
        Ty::Tensor(e, rank) => format!("tensor:{}:{}", e.0, rank),
        Ty::Fn(f) => {
            let ps: Vec<String> = f.params.iter().map(|p| p.0.to_string()).collect();
            match &f.lam {
                Some(l) => format!("fn:{}:{}:lam{}", ps.join(","), f.ret.0, l.sym),
                None => format!("fn:{}:{}", ps.join(","), f.ret.0),
            }
        }
        Ty::Named(n, a) => {
            let args: Vec<String> = a.iter().map(|x| x.0.to_string()).collect();
            format!("named:{}:{}", n, args.join(","))
        }
        Ty::Opt(e) => format!("opt:{}", e.0),
        Ty::Weak(e) => format!("weak:{}", e.0),
        Ty::Fiber(e) => format!("fiber:{}", e.0),
        Ty::Dyn(n) => format!("dyn:{}", n),
        Ty::Tp(n) => format!("tp:{}", n),
    }
}

pub fn ty_name(t: &Ty) -> String {
    match t {
        Ty::Unit => "unit".to_string(),
        Ty::Bool => "bool".to_string(),
        Ty::I64 => "int".to_string(),
        Ty::F64 => "float".to_string(),
        Ty::Str => "str".to_string(),
        Ty::Range => "range".to_string(),
        Ty::Array(_e) => "arr".to_string(),
        Ty::Map(..) => "map".to_string(),
        Ty::Tensor(_e, _r) => "tensor".to_string(),
        Ty::Fn(_f) => "fn".to_string(),
        Ty::Named(n, _a) => n.clone(),
        Ty::Opt(_e) => "opt".to_string(),
        Ty::Weak(_e) => "weak".to_string(),
        Ty::Fiber(_e) => "fiber".to_string(),
        Ty::Dyn(n) => format!("dyn:{}", n),
        Ty::Tp(n) => format!("?tp:{}", n),
    }
}
