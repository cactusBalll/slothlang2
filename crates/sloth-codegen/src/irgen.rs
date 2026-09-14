//! End-to-end codegen: typed-AST walk -> textual MLIR.
//! One pass does name resolution + type inference + emission; diagnostics
//! are collected instead of aborting on the first error.

use crate::sys;
use sloth_frontend::ast::*;
use sloth_frontend::ty::{Diag, Reg, Ty, TyId};
use std::cell::Cell;
use std::collections::HashMap;
use std::cell::RefCell;

/// Intermediate program form after the frontend parse. Kept identical to
/// Parser's AST but enriched with per-node type annotations injected by the
/// checker half of this pass.
pub struct Mod {
    pub prog: Program,
}

#[derive(Clone, Copy)]
pub struct EmitCtx {
    pub mod_name: String,
}

/// Function emission state.
pub struct FnEmitter {
    pub body: String,
    pub prologue: String,
    pub vcount: usize,
    /// var name -> alloca ptr (as %v token)
    pub vars: Vec<HashMap<String, String>>,
    pub ret: TyId,
    /// self class for this method (used by `this`)
    pub this_ty: Option<TyId>,
}

impl FnEmitter {
    pub fn v(&mut self) -> String {
        let id = self.vcount;
        self.vcount += 1;
        format!("%v{}", id)
    }
}

/// Whole-module emitter: holds registries and symbol tables.
pub struct ModEmitter {
    pub r: Reg,
    /// module qualified name (e.g. "main" or imported path key)
    pub name: String,
    /// function signatures incl. generics
    pub funcs: HashMap<String, FuncSig>,
    /// class layouts
    pub classes: HashMap<String, ClassInfo>,
    /// trait surface
    pub traits: HashMap<String, TraitInfo>,
    /// global bindings (name -> binding info)
    pub globals: Vec<(String, TyId, bool)>,
    pub diags: Vec<Diag>,
    /// emitted pieces to concatenate
    pub out: String,
    pub strpool: Vec<String>,
}

#[derive(Clone)]
pub struct FuncSig {
    pub name: String,
    pub type_params: Vec<String>,
    pub bounds: HashMap<String, Vec<String>>,
    pub params: Vec<(String, TyId)>,
    pub variadic_elem: Option<TyId>,
    pub ret: TyId,
    /// monomorphized body cache key -> already emitted?
    pub emitted_bodies: std::cell::RefCell<HashMap<String, bool>>,
}

pub struct ClassInfo {
    pub name: String,
    pub type_params: Vec<String>,
    pub name: String,
    pub fields: Vec<(String, TyId)>,
    pub methods: RefCell<HashMap<String, FuncSig>>,
    pub superclass: Option<String>,
    pub impls: Vec<String>,
}

pub struct TraitInfo {
    pub methods: HashMap<String, (Vec<TyId>, TyId)>,
}

// ---------- type helpers ----------

impl ModEmitter {
    fn err_at(&mut self, line: usize, col: usize, msg: String) {
        self.diags.push(Diag { line, col, msg });
    }

    pub fn prim(&mut self, p: Prim) -> TyId {
        let t = match p {
            Prim::Bool => Ty::Bool,
            Prim::Int => Ty::I64,
            Prim::Float => Ty::F64,
            Prim::Str => Ty::Str,
            Prim::Range => Ty::Range,
        };
        self.r.mk(t)
    }

    pub fn arr(&mut self, el: TyId) -> TyId {
        self.r.mk(Ty::Array(el))
    }

    pub fn map(&mut self, k: TyId, v: TyId) -> TyId {
        self.r.mk(Ty::Map(k, v))
    }

    pub fn fn_ty(&mut self, params: Vec<TyId>, ret: TyId) -> TyId {
        self.r.mk(Ty::Fn(sloth_frontend::ty::FnTy { params, ret }))
    }

    pub fn opt(&mut self, inner: TyId) -> TyId {
        self.r.mk(Ty::Opt(inner))
    }
}

impl ModEmitter {
    /// scalar (stack-allocated) vs GC-heap reference types in MLIR ABI words
    pub fn is_scalar(&self, id: TyId) -> bool {
        matches!(
            self.r.get(id),
            Ty::Unit | Ty::Bool | Ty::I64 | Ty::F64 | Ty::Range | Ty::Tp(_)
        )
    }
    pub fn is_float(&self, id: TyId) -> bool {
        matches!(self.r.get(id), Ty::F64)
    }
    pub fn is_unit(&self, id: TyId) -> bool {
        matches!(self.r.get(id), Ty::Unit)
    }
    pub fn is_str(&self, id: TyId) -> bool {
        matches!(self.r.get(id), Ty::Str)
    }
}

// ---------- symbol collection ----------

impl ModEmitter {
    pub fn collect(&mut self, prog: Program) {
        let n = prog.decls.len();
        self.prog = prog;
        let _ = self.prog.decls.len();
        self.prog.decls.iter().for_each(|d| {
            self.collect_decl(d);
        });
    }
}

pub struct PrivateMarker;

impl ModEmitter {
    fn fresh_str(&mut self, s: &str) -> String {
        // intern into module pool; reuse existing
        for (i, x) in self.strpool.iter().enumerate() {
            if x == s {
                return format!("@str_{}", i);
            }
        }
        self.strpool.push(s.to_string());
        let i = self.strpool.len() - 1;
        format!("@str_{}", i)
    }
}

impl ModEmitter {
    fn collect_decl(&mut self, d: &Decl) {
        match &d.node {
            DeclNode::Func(f) => {
                let (params, vt) = self.collect_params(&f.params, &f.variadic);
                let ret = match &f.ret {
                    Some(t) => self.ty_of(t),
                    None => self.r.mk(Ty::Unit),
                };
                let mut bounds: HashMap<String, Vec<String>> = HashMap::new();
                for p in &f.type_params {
                    if let Some(b) = &p.bound {
                        bounds.insert(p.name.clone(), vec![b.clone()]);
                    }
                }
                let key = format!("{}_{}", self.name, d.name);
                let sig = FuncSig {
                    name: key.clone(),
                    type_params: f.type_params.iter().map(|t| t.name.clone()).collect(),
                    bounds,
                    params,
                    variadic_elem: vt.as_ref().map(|v| self.ty_of(&v.elem)),
                    ret,
                    emitted_bodies: RefCell::new(HashMap::new()),
                };
                self.funcs.insert(key, sig);
            }
            _ => {}
        }
    }
}

impl ModEmitter {
    fn collect_params(&self, ps: &[Param], v: &Option<Variadic>) -> (Vec<(String, TyId)>, Option<Variadic>) {
        let mut out: Vec<(String, TyId)> = Vec::new();
        for p in ps {
            let t = match &p.ty {
                Some(t) => self.ty_of(t),
                None => self.r.mk(Ty::Unit),
            };
            out.push((p.name.clone(), t));
        }
        (out, v.clone())
    }
}

impl ModEmitter {
    /// map a syntactic Type to an interned TyId (pass 1 uses Tp for unknown idents)
    fn ty_of(&self, t: &Type) -> TyId {
        match t {
            Type::Unit => self.r.mk(Ty::Unit),
            Type::Simple(st) => self.ty_of_simple(st),
            Type::Optional(inner) => self.r.mk(Ty::Opt(self.ty_of(inner))),
        }
    }

    fn ty_of_ident_as(&self, name: &str, args: Vec<TyId>) -> TyId {
        // pass-1: named class or scalar alias resolved by collect_decl
        // int/float/bool/str/range are context keywords for types (already
        // matched by ty_base); anything else here is a user type
        match name {
            "int" | "i64" => self.r.mk(Ty::I64),
            "float" | "f64" => self.r.mk(Ty::F64),
            "bool" => self.r.mk(Ty::Bool),
            "str" => self.r.mk(Ty::Str),
            "range" => self.r.mk(Ty::Range),
            "Array" => match args.into_iter().next() {
                Some(el) => self.r.mk(Ty::Array(el)),
                None => self.r.mk(Ty::Array(self.r.mk(Ty::Unit))),
            },
            "Map" => {
                let mut it = args.into_iter();
                let k = it.next().unwrap_or_else(|| self.r.mk(Ty::Unit));
                let v = it.next().unwrap_or_else(|| self.r.mk(Ty::Unit));
                self.r.mk(Ty::Map(k, v))
            }
            other => {
                if other == "dyn" {
                    self.r.mk(Ty::Dyn("Object".to_string()))
                } else {
                    self.r.mk(Ty::Named(name.to_string(), args))
                }
            }
        }
    }
}

impl ModEmitter {
    fn ty_of_simple(&self, st: &SimpleType) -> TyId {
        match st {
            SimpleType::Bool => self.r.mk(Ty::Bool),
            SimpleType::Int => self.r.mk(Ty::I64),
            SimpleType::Float => self.r.mk(Ty::F64),
            SimpleType::Str => self.r.mk(Ty::Str),
            SimpleType::Range => self.r.mk(Ty::Range),
            SimpleType::Array(el) => self.r.mk(Ty::Array(self.ty_of(el))),
            SimpleType::Map(k, v) => self.r.mk(Ty::Map(self.ty_of(k), self.ty_of(v))),
            SimpleType::Fn(f) => {
                let ps: Vec<TyId> = f.params.iter().map(|p| self.ty_of(p)).collect();
                self.r.mk(Ty::Fn(sloth_frontend::ty::FnTy { params: ps, ret: self.ty_of(&f.ret) }))
            }
            SimpleType::Named(n, args) => {
                let a: Vec<TyId> = args.iter().map(|t| self.ty_of(t)).collect();
                self.ty_of_ident_as(n, a)
            }
            SimpleType::Ident(n) => {
                let a: Vec<TyId> = Vec::new();
                self.ty_of_ident_as(n, a)
            }
            SimpleType::Dyn(tr) => self.r.mk(Ty::Dyn(tr.clone())),
        }
    }
}

// ---------- per-function body emission ----------

#[derive(Clone, Copy)]
pub struct EmitEnv<'m> {
    pub me: &'m RefCell<ModEmitter>,
}

/// One function's body emission context.
pub struct FnWalk {
    pub body: String,
    pub prologue: String,
    pub vcount: usize,
    /// name -> alloca ptr token (SSA value string "%vN") for re-assignment
    pub vars_stack: Vec<HashMap<String, (String, TyId, bool)>>,
    /// current return type
    pub ret: TyId,
    pub this_ty: Option<TyId>,
    /// loop nesting; >0 means break/continue valid
    pub loop_depth: usize,
}


// ---------- per-function body emission (clean) ----------

pub struct FnWalk {
    pub body: String,
    pub prologue: String,
    pub vcount: usize,
    pub vars_stack: Vec<HashMap<String, (String, TyId, bool)>>,
    pub ret: TyId,
    pub this_ty: Option<TyId>,
    pub loop_depth: usize,
}

impl FnWalk {
    pub fn v(&mut self) -> String {
        self.vcount += 1;
        format!("%v{}", self.vcount - 1)
    }
    pub fn push_scope(&mut self) {
        self.vars_stack.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        self.vars_stack.pop();
    }
    pub fn lookup(&self, name: &str) -> Option<(String, TyId, bool)> {
        for sc in self.vars_stack.iter().rev() {
            if let Some(v) = sc.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
    pub fn declare(&mut self, name: &str, t: TyId, mutable: bool) -> String {
        let p = self.v();
        self.prologue.push_str(match is_float_layout(t) {
            true => format!("  {} = memref.alloca() : memref<1xf64>\n", p),
            false => format!("  {} = memref.alloca() : memref<1xi64>\n", p),
        });
        self.vars_stack
            .last_mut()
            .unwrap()
            .insert(name.to_string(), (p.clone(), t, mutable));
        p
    }
}

fn is_float_layout(t: TyId) -> bool {
    false // placeholder; real check consults ModEmitter in emit path
}

impl ModEmitter {
    /// emit one statement inside the current scf region
    pub fn walk_stmt(&mut self, fw: &mut FnWalk, s: &Stmt) {
        match &s.node {
            StmtNode::Expr(e) => {
                let _ = self.emit_expr(fw, e);
            }
            _ => {}
        }
    }

    pub fn walk_block(&mut self, fw: &mut FnWalk, ss: &Vec<Stmt>) {
        fw.push_scope();
        for s in ss {
            self.walk_stmt(fw, s);
        }
        fw.pop_scope();
    }
}

impl ModEmitter {
    /// evaluate to SSA token; returns token + inferred TyId
    pub fn emit_expr(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Int(v) => {
                let t = self.prim(Prim::Int);
                let r = fw.v();
                fw.body.push_str(&format!(
                    "  {} = arith.constant {} : i64\n",
                    r, v
                ));
                (r, t)
            }
            other => {
                let _ = other;
                (String::new(), self.r.mk(Ty::Unit))
            }
        }
    }
}

impl ModEmitter {
    pub fn emit_int(&mut self, fw: &mut FnWalk, v: i64) -> String {
        let r = fw.v();
        fw.body.push_str(&format!("  {} = arith.constant {} : i64\n", r, v));
        r
    }
    pub fn emit_float(&mut self, fw: &mut FnWalk, v: f64) -> String {
        let r = fw.v();
        fw.body.push_str(&format!("  {} = arith.constant {} : f64\n", r, v));
        r
    }
    pub fn emit_bool(&mut self, fw: &mut FnWalk, v: bool) -> String {
        let r = fw.v();
        let b = if v { 1 } else { 0 };
        fw.body.push_str(&format!("  {} = arith.constant {} : i1\n", r, b));
        r
    }
}

impl ModEmitter {
    pub fn emit_arith_i64(&mut self, fw: &mut FnWalk, op: &ArithOp, a: &str, b: &str) -> String {
        let ao = match op {
            ArithOp::Add => "arith.addi",
            ArithOp::Sub => "arith.subi",
            ArithOp::Mul => "arith.muli",
            ArithOp::Div => "arith.divsi",
            ArithOp::Mod => "arith.remsi",
        };
        let r = fw.v();
        fw.body.push_str(&format!(
            "  {} = {} {}, {} : i64\n",
            r, ao, a, b
        ));
        r
    }
    pub fn emit_arith_f64(&mut self, fw: &mut FnWalk, op: &ArithOp, a: &str, b: &str) -> String {
        let r = fw.v();
        let ao = match op {
            ArithOp::Add => "arith.addf",
            ArithOp::Sub => "arith.subf",
            ArithOp::Mul => "arith.mulf",
            ArithOp::Div => "arith.divf",
            ArithOp::Mod => "arith.remf",
        };
        fw.body.push_str(&format!("  {} = {} {}, {} : f64\n", r, ao, a, b));
        r
    }
    pub fn emit_cmp_i(&mut self, fw: &mut FnWalk, pred: &str, a: &str, b: &str) -> String {
        let r = fw.v();
        fw.body.push_str(&format!("  {} = arith.cmpi {}, {}, {} : i64\n", r, pred, a, b));
        r
    }
    pub fn emit_cmp_f(&mut self, fw: &mut FnWalk, pred: &str, a: &str, b: &str) -> String {
        let r = fw.v();
        fw.body.push_str(&format!("  {} = arith.cmpf {}, {}, {} : f64\n", r, pred, a, b));
        r
    }
    pub fn emit_not(&mut self, fw: &mut FnWalk, v: &str) -> String {
        let r = fw.v();
        let z = self.emit_bool(fw, false);
        fw.body.push_str(&format!("  {} = arith.xori {}, {} : i1\n", r, v, z));
        r
    }
}

// ---------- expression emission (core kinds) ----------

impl ModEmitter {
    pub fn emit_expr2(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Int(v) => (self.emit_int(fw, *v), self.prim(Prim::Int)),
            ExprNode::Float(v) => (self.emit_float(fw, *v), self.prim(Prim::Float)),
            ExprNode::Bool(v) => (self.emit_bool(fw, *v), self.prim(Prim::Bool)),
            ExprNode::Nil => (self.emit_int(fw, 0), self.r.mk(Ty::Unit)),
            ExprNode::Ident(name) => {
                if let Some((tok, t, _m)) = fw.lookup(name) {
                    // load from alloca: result = memref.load
                    let r = fw.v();
                    let b = fw.v();
                    fw.body.push_str(&format!("  {} = arith.constant 0 : i64\n", b));
                    fw.body.push_str(&format!("  {} = memref.load {}[{}] : memref<1xi64>\n", r, tok, b));
                    (r, t)
                } else {
                    self.err_at(1, 0, format!("unknown ident: {}", name));
                    (String::new(), self.r.mk(Ty::Unit))
                }
            }
            other => {
                let _ = other;
                unrecognized_expr()
            }
        }
    }

    fn fresh_string(&mut self) -> () {}
}

fn unrecognized_expr() -> (String, sloth_frontend::ty::TyId) {
    (String::new(), sloth_frontend::ty::TyId(0))
}
