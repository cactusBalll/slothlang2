//! Interned type registry with arena, substitution, and diagnostics.

use std::collections::BTreeMap;
use std::fmt::Write as _;

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
    Fn(FnTy),
    Named(String, Vec<TyId>), // user class w/ optional ty args
    Opt(TyId),
    Dyn(String),
    /// type param placeholder (during monomorphization substitution)
    Tp(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnTy {
    pub params: Vec<TyId>,
    pub ret: TyId,
}

#[derive(Debug, Clone)]
pub struct Diag {
    pub line: usize,
    pub col: usize,
    pub msg: String,
}

impl Diag {
    pub fn at(file: &crate::ast::Pos2, msg: String) -> Diag {
        Diag { line: file.line, col: file.col, msg }
    }
}

#[derive(Default)]
pub struct Reg {
    pub types: Vec<Ty>,
    /// cache: key = Ty debug key → TyId, for structural reuse
    cache: BTreeMap<String, TyId>,
}

impl Reg {
    pub fn new() -> Reg {
        Reg { types: Vec::new(), cache: BTreeMap::new() }
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
    pub fn subst(&self, id: TyId, map: &std::collections::HashMap<String, TyId>) -> TyId {
        match self.get(id) {
            Ty::Tp(n) => *map.get(n).unwrap_or(&id),
            Ty::Array(e) => self.mk(Ty::Array(self.subst(*e, map))),
            Ty::Map(k, v) => self.mk(Ty::Map(self.subst(*k, map), self.subst(*v, map))),
            Ty::Fn(f) => {
                let ps: Vec<TyId> = f.params.iter().map(|p| self.subst(*p, map)).collect();
                self.mk(Ty::Fn(FnTy { params: ps, ret: self.subst(f.ret, map) }))
            }
            Ty::Named(n, args) => {
                let a2: Vec<TyId> = args.iter().map(|a| self.subst(*a, map)).collect();
                self.mk(Ty::Named(n.clone(), a2))
            }
            Ty::Opt(e) => self.mk(Ty::Opt(self.subst(*e, map))),
            other => {
                let nt: Ty = match self.get(id).clone() {
                    Ty::Unit => Ty::Unit,
                    Ty::Bool => Ty::Bool,
                    Ty::I64 => Ty::I64,
                    Ty::F64 => Ty::F64,
                    Ty::Str => Ty::Str,
                    Ty::Range => Ty::Range,
                    Ty::Map(..) => unreachable!(),
                    Ty::Array(..) | Ty::Fn(..) | Ty::Named(..) | Ty::Opt(..) => unreachable!(),
                    Ty::Dyn(n) => Ty::Dyn(n),
                    Ty::Tp(n) => Ty::Tp(n),
                };
                let _ = nt;
                self.mk(other.clone())
            }
        }
    }
}


/// Printable structural name of a Ty (also backend cache key).
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
        Ty::Fn(_f) => "fn".to_string(),
        Ty::Named(n, _a) => n.clone(),
        Ty::Opt(_e) => "opt".to_string(),
        Ty::Dyn(n) => format!("dyn:{}", n),
        Ty::Tp(n) => format!("?tp:{}", n),
    }
}
