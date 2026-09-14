//! End-to-end codegen: untyped AST -> textual MLIR (func/arith/cf/memref).
//! One pass does name resolution + type inference + emission. Diagnostics are
//! collected rather than aborting. Lowering uses plain `cf` basic blocks built
//! with SSA env captured at emission time (locals live in memref allocas).

use crate::sys;
use sloth_frontend::ast::*;
use sloth_frontend::lexer::{Pos, StrPart};
use sloth_frontend::ty::{Diag, Reg, Ty, TyId};
use std::collections::HashMap;

/// Layout width of an object word (bytes); objects: [cls_info ptr, k/V...]
// runtime ABI: pointers flow as i64 words
pub const WW: usize = 8;

fn words_scalar(t: &Ty) -> usize {
    match t {
        Ty::Unit => 0,
        _ => 1,
    }
}

// ---------------- module emitter ----------------

pub struct ModEmitter {
    pub r: Reg,
    /// module name (file stem, e.g. "main")
    pub name: String,
    /// top-level function signatures: qualified name -> sig
    pub funcs: HashMap<String, FuncDef>,
    /// class layouts per simple name (same-module scope; cross-module via mangle)
    pub classes: HashMap<String, ClassInfo>,
    /// trait surface: simple name -> methods
    pub traits: HashMap<String, Vec<MethodSig>>,
    /// module-level globals: name -> (mangled symbol, type id, mutable)
    pub globals: HashMap<String, (String, TyId, bool)>,
    pub diags: Vec<Diag>,
    /// module text to parse (html-safe)
    pub out: String,
    /// string literals emitted as llvm.mlir.global; interned by runtime
    pub strpool: Vec<String>,
    /// classes emitted: name -> id (assigned in collect order)
    pub class_ids: HashMap<String, i64>,
    /// mangled function names already emitted
    pub emitted_names: Vec<String>,
    /// module name used for mangling the next emit (settable for foreign imports)
    pub cur_mod: String,
    /// foreign module surface: simple name -> (mangled symbol, ret ty)
    pub cross_funcs: HashMap<String, (String, TyId)>,
    /// satisfied import paths (file canonical)
    pub imported_paths: Vec<String>,
    /// foreign globals: "mod.name" or "alias.name" -> (mangled global symbol, ty)
    pub fglobals: HashMap<String, (String, TyId)>,
    /// global symbol declarations to prepend to the module IR
    pub global_decls: Vec<String>,
    /// global symbols already declared (dedupe)
    pub declared_syms: std::collections::HashSet<String>,
    /// imported modules whose init func runs before @sloth_main body
    pub init_mods: Vec<String>,
    /// import aliases: alias -> module name
    pub mod_alias: HashMap<String, String>,
}

pub struct ClassInfo {
    pub name: String,
    pub fields: Vec<(String, TyId, bool)>, // (name, ty, mutable)
    pub methods: Vec<(String, FuncDef)>,
    pub superclass: Option<String>,
    pub impls: Vec<String>,
}

impl ModEmitter {
    pub fn new(name: &str) -> ModEmitter {
        ModEmitter {
            r: Reg::new(),
            name: name.to_string(),
            funcs: HashMap::new(),
            classes: HashMap::new(),
            traits: HashMap::new(),
            globals: HashMap::new(),
            diags: Vec::new(),
            out: String::new(),
            strpool: Vec::new(),
            class_ids: HashMap::new(),
            emitted_names: Vec::new(),
            cur_mod: name.to_string(),
            cross_funcs: HashMap::new(),
            imported_paths: Vec::new(),
            fglobals: HashMap::new(),
            global_decls: Vec::new(),
            declared_syms: std::collections::HashSet::new(),
            init_mods: Vec::new(),
            mod_alias: HashMap::new(),
        }
    }

    /// adopt the surface of a foreign module (emits its funcs/classes under that
    /// module's name and registers them for cross-module calls)
    pub fn register_import(&mut self, mname: &str, alias: Option<&str>, prog: &Program) {
        self.cur_mod = mname.to_string();
        if let Some(a) = alias {
            self.mod_alias.insert(a.to_string(), mname.to_string());
        }
        let qname = alias.unwrap_or(mname).to_string();
        for d in &prog.decls {
            match &d.node {
                DeclNode::Func(f) => {
                    let mangled = mangle(mname, None, &d.name);
                    let plan = self.plan_func(&d.name, None, f, None);
                    self.cross_funcs.insert(d.name.clone(), (mangled.clone(), plan.ret));
                    self.cross_funcs
                        .insert(format!("{}.{}", qname, d.name), (mangled, plan.ret));
                    self.emit_func(&d.name, None, f, None, false);
                }
                DeclNode::Class(c) => {
                    for m in &c.methods {
                        self.emit_func(&m.name, Some(&d.name), &m.fd, None, false);
                    }
                }
                _ => {}
            }
        }
        // foreign global cells (declared lazily; inits run in modinit)
        for d in &prog.decls {
            if let DeclNode::Var { ty, init } = &d.node {
                let t = match ty {
                    Some(t) => self.ty_of(t),
                    None => self.r.mk(Ty::Unit),
                };
                let sym = self.declare_global(mname, &d.name, t);
                self.fglobals.insert(format!("{}.{}", qname, d.name), (sym.clone(), t));
                self.fglobals.insert(format!("{}.{}", mname, d.name), (sym.clone(), t));
            }
        }
        // module init func: runs this module's var inits at startup
        let mut gbody = String::new();
        let mut fw = fresh_walk(self);
        for d in &prog.decls {
            if let DeclNode::Var { init, .. } = &d.node {
                let key = format!("{}.{}", qname, d.name);
                let (sym, t) = match self.fglobals.get(&key).cloned() {
                    Some(x) => x,
                    None => continue,
                };
                let (v, _vt) = self.emit_expr(&mut fw, &init);
                let mty = memref_cell_ty(self, t);
                let g = fw.v();
                fw.op(&format!("    {} = memref.get_global @{} : {}", g, sym, mty));
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", z));
                fw.op(&format!("    memref.store {}, {}[{}] : {}", v, g, z, mty));
            }
        }
        gbody.push_str(&fw.cur);
        self.out.push_str(&format!(
            "  func.func @sloth_{}__ginit() -> () {{\n{}    return\n  }}\n",
            mname, gbody
        ));
        self.init_mods.push(mname.to_string());
        self.cur_mod = self.name.clone();
    }

    fn err(&mut self, pos: &Pos, msg: String) {
        self.diags.push(Diag { line: pos.line, col: pos.col, msg });
    }
}

// ---------------- function emitter ----------------

/// state machine for emitting a single function body's CFG
struct FnWalk {
    /// current (open) basic block text
    cur: String,
    vcount: usize,
    scopes: Vec<HashMap<String, (String, TyId)>>,
    /// loop label stack for break/continue: (break_target, continue_target)
    loops: Vec<(String, String)>,
    ret: TyId,
    ret_alloca: String,
    ret_flag: String,
    /// running count of emitter basic blocks
    bb: usize,
    /// true if current block already ends with terminator
    term: bool,
    /// shared return/end block label for this function
    end_label: String,
}

impl FnWalk {
    fn v(&mut self) -> String {
        let n = self.vcount;
        self.vcount += 1;
        format!("%v{}", n)
    }
    fn z(&mut self) -> String {
        let n = self.vcount;
        self.vcount += 1;
        format!("%{}", n)
    }
}

impl FnWalk {
    /// emit instruction line(s) into current block
    fn op(&mut self, s: &str) {
        self.cur.push_str(s);
        self.cur.push('\n');
    }
    fn noterm(&self) -> bool {
        !self.term
    }
    /// unconditional jump `cf.br ^L` (closes current block)
    fn jump(&mut self, t: &str) {
        if self.noterm() {
            self.op(&format!("    cf.br {}", t));
        }
        self.term = true;
    }
    /// conditional jump
    fn cjump(&mut self, c: &str, t: &str, f: &str) {
        let c1 = self.v();
        self.op(&format!("    {} = arith.trunci {} : i64 to i1", c1, c));
        self.op(&format!("    cf.cond_br {}, {}, {}", c1, t, f));
        self.term = true;
    }
    /// start a new labelled block
    fn label(&mut self, name: &str) {
        self.op(&format!("  {}:", name));
        self.term = false;
    }
    /// fresh label name
    fn newlabel(&mut self, p: &str) -> String {
        self.bb += 1;
        format!("^{}_{}", p, self.bb)
    }
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    fn declare(&mut self, name: &str, t: TyId, fl: bool) -> String {
        let a = self.v();
        let mty = if fl { "memref<1xf64>" } else { "memref<1xi64>" };
        self.op(&format!("    {} = memref.alloca() : {}", a, mty));
        self.scopes.last_mut().unwrap().insert(name.to_string(), (a.clone(), t));
        a
    }
    fn lookup(&self, name: &str) -> Option<(String, TyId)> {
        for sc in self.scopes.iter().rev() {
            if let Some(v) = sc.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
    fn assign(&mut self, name: &str, val: &str, fl: bool) {
        if let Some((a, _t)) = self.lookup(name) {
            let z = self.v();
            self.op(&format!("    {} = arith.constant 0 : i64", z));
            if fl {
                self.op(&format!("    memref.store {}, {}[{}] : memref<1xf64>", val, a, z));
            } else {
                self.op(&format!("    memref.store {}, {}[{}] : memref<1xi64>", val, a, z));
            }
        }
    }
}

// ---------------- pass 1: collect & type registry ----------------

impl ModEmitter {
    /// type of a syntactic type expression
    fn ty_of(&mut self, t: &Type) -> TyId {
        match t {
            Type::Unit => self.r.mk(Ty::Unit),
            Type::Optional(i) => {
                let ni = self.ty_of(i);
                self.r.mk(Ty::Opt(ni))
            }
            Type::Simple(st) => self.ty_of_simple(st),
        }
    }

    fn ty_of_simple(&mut self, st: &SimpleType) -> TyId {
        use sloth_frontend::ty::FnTy;
        match st {
            SimpleType::Bool => self.r.mk(Ty::Bool),
            SimpleType::Int => self.r.mk(Ty::I64),
            SimpleType::Float => self.r.mk(Ty::F64),
            SimpleType::Str => self.r.mk(Ty::Str),
            SimpleType::Range => self.r.mk(Ty::Range),
            SimpleType::Array(e) => {
                let ne = self.ty_of(e);
                self.r.mk(Ty::Array(ne))
            }
            SimpleType::Map(k, v) => {
                let nk = self.ty_of(k);
                let nv = self.ty_of(v);
                self.r.mk(Ty::Map(nk, nv))
            }
            SimpleType::Fn(f) => {
                let ps: Vec<TyId> = f
                    .params
                    .iter()
                    .map(|p| self.ty_of(p))
                    .collect();
                let nr = self.ty_of(&f.ret);
                self.r.mk(Ty::Fn(FnTy { params: ps, ret: nr }))
            }
            SimpleType::Dyn(t) => self.r.mk(Ty::Dyn(t.clone())),
            SimpleType::Named(n, args) => {
                let a: Vec<TyId> = args.iter().map(|t| self.ty_of(t)).collect();
                self.ty_named(n, a)
            }
            SimpleType::Ident(n) => {
                let a: Vec<TyId> = Vec::new();
                self.ty_named(n, a)
            }
            other => {
                let _ = other;
                self.r.mk(Ty::Unit)
            }
        }
    }

    fn ty_named(&mut self, n: &str, a: Vec<TyId>) -> TyId {
        match n {
            "int" | "i64" => self.r.mk(Ty::I64),
            "float" | "f64" => self.r.mk(Ty::F64),
            "bool" => self.r.mk(Ty::Bool),
            "str" => self.r.mk(Ty::Str),
            "range" => self.r.mk(Ty::Range),
            "Array" => {
                let el = match a.into_iter().next() {
                    Some(e) => e,
                    None => self.r.mk(Ty::Unit),
                };
                self.r.mk(Ty::Array(el))
            }
            "Map" => {
                let mut it = a.into_iter();
                let k = it.next().unwrap_or_else(|| self.r.mk(Ty::Unit));
                let v = it.next().unwrap_or_else(|| self.r.mk(Ty::Unit));
                self.r.mk(Ty::Map(k, v))
            }
            _ => self.r.mk(Ty::Named(n.to_string(), a)),
        }
    }


    pub fn is_float(&self, t: TyId) -> bool {
        matches!(self.r.get(t), Ty::F64)
    }
    pub fn is_str(&self, t: TyId) -> bool {
        matches!(self.r.get(t), Ty::Str)
    }
    pub fn is_unit(&self, t: TyId) -> bool {
        matches!(self.r.get(t), Ty::Unit)
    }
    pub fn is_ref(&self, t: TyId) -> bool {
        matches!(
            self.r.get(t),
            Ty::Str | Ty::Array(_) | Ty::Map(..) | Ty::Fn(_) | Ty::Named(_, _) | Ty::Dyn(_)
        )
    }
}

/// mangle: module_scope_name for top-level, class method _Class_method
fn mangle(mod_name: &str, cls: Option<&str>, name: &str) -> String {
    match cls {
        Some(c) => format!("sloth_{}_{}__{}", mod_name, c, name),
        None => format!("sloth_{}__{}", mod_name, name),
    }
}

fn mangle_t(args: &[TyId], r: &Reg) -> String {
    let mut s = String::new();
    for a in args {
        s.push('_');
        s.push_str(&sloth_frontend::ty::ty_name(r.get(*a)).replace(':', "_"));
    }
    s
}

impl ModEmitter {
    fn plan_mangled(&mut self, name: &str, cls: Option<&str>, f: &FuncDef, variadic: Option<&Variadic>) -> FuncPlan {
        let mangled = mangle(&self.cur_mod.clone(), cls, name);
        let plan = self.plan_func(name, cls, f, variadic);
        FuncPlan { mangled, params: plan.params, ret: plan.ret }
    }

    pub fn collect(&mut self, prog: &Program) {
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
                    self.globals.insert(d.name.clone(), (sym, t, d.kind == DeclKind::Var));
                }
                DeclNode::Class(c) => {
                    self.class_ids.insert(d.name.clone(), class_id);
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
                    self.traits.insert(d.name.clone(), t.methods.clone());
                }
            }
        }
    }

}


// ---------------- pass 2: statement emission ----------------

struct FuncPlan {
    mangled: String,
    params: Vec<(String, TyId, bool)>, // name, ty, is_float
    ret: TyId,
}

impl ModEmitter {
    fn plan_func(&mut self, name: &str, cls: Option<&str>, f: &FuncDef, variadic: Option<&Variadic>) -> FuncPlan {
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
        FuncPlan { mangled: mangle(&self.cur_mod.clone(), cls, name).into(), params, ret }
    }
}

impl ModEmitter {
    /// emit one function; returns mangled symbol name
    fn emit_func(
        &mut self,
        name: &str,
        cls: Option<&str>,
        f: &FuncDef,
        variadic: Option<&Variadic>,
        entry: bool,
    ) -> String {
        let plan = self.plan_func(name, cls, f, variadic);
        if self.emitted_names.contains(&plan.mangled) {
            return plan.mangled;
        }
        self.emitted_names.push(plan.mangled.clone());
        let retf = self.is_float(plan.ret);
        let mut fw = FnWalk {
            cur: String::new(),
            vcount: 1000,
            scopes: vec![HashMap::new()],
            loops: Vec::new(),
            ret: plan.ret,
            ret_alloca: String::new(),
            ret_flag: String::new(),
            bb: 0,
            term: false,
            end_label: "^end".to_string(),
        };
        let rf = fw.v();
        fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", rf));
        fw.ret_flag = rf;
        if !self.is_unit(plan.ret) {
            let ra = fw.v();
            let rty = if retf { "memref<1xf64>" } else { "memref<1xi64>" };
            fw.ret_alloca = ra.clone();
            fw.op(&format!("    {} = memref.alloca() : {}", ra, rty));
        }
        // function params: %pN fed through allocas; variadic packs later
        let mut argtxts: Vec<String> = Vec::new();
        for (i, (pn, pt, fl)) in plan.params.iter().enumerate() {
            let src = format!("%p{}", i);
            argtxts.push(if *fl { "f64".to_string() } else { "i64".to_string() });
            let mty = if *fl { "memref<1xf64>" } else { "memref<1xi64>" };
            let a = fw.v();
            let zi = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", zi));
            fw.op(&format!("    {} = memref.alloca() : {}", a, mty));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : {}",
                src, a, zi, mty
            ));
            fw.scopes.last_mut().unwrap().insert(pn.clone(), (a, *pt));
        }
        // emit body statements into the entry block
        self.walk_body(&mut fw, &f.body);
        fw.jump(&"^end");
        let entry_text = fw.cur.clone();
        fw.cur = String::new();
        fw.term = false;
        fw.label(&"^end");
        let mut retval = String::new();
        if !self.is_unit(plan.ret) {
            let zi = fw.v();
            let v = fw.v();
            let rty = if retf { "memref<1xf64>" } else { "memref<1xi64>" };
            fw.op(&format!("    {} = arith.constant 0 : index", zi));
            fw.op(&format!("    {} = memref.load {}[{}] : {}", v, fw.ret_alloca, zi, rty));
            retval = format!(" {}", v);
        }
        if self.is_unit(plan.ret) {
            fw.op("    return");
        } else {
            let rt = if retf { "f64" } else { "i64" };
            fw.op(&format!("    return {} : {}", retval.trim_start(), rt));
        }
        let ret_text = fw.cur.clone();
        let sigtxt: Vec<String> = argtxts
            .iter()
            .enumerate()
            .map(|(i, t)| format!("%p{}: {}", i, t))
            .collect();
        let sigtxt = sigtxt.join(", ");
        if entry {
            self.out.push_str(&format!(
                "  func.func @sloth_main({}) -> {} attributes {{llvm.emit_c_interface}} {{\n",
                sigtxt,
                mlir_ret_ty(self, plan.ret),
            ));
        } else {
            self.out.push_str(&format!(
                "  func.func @{}({}) -> {} {{\n",
                plan.mangled, sigtxt, mlir_ret_ty(self, plan.ret)
            ));
        }
        self.out.push_str(&entry_text);
        self.out.push_str(&ret_text);
        self.out.push_str("  }\n");
        plan.mangled
    }
}


fn mlir_ret_ty(me: &ModEmitter, t: TyId) -> String {
    match me.r.get(t) {
        Ty::Unit => "()".to_string(),
        Ty::F64 => "f64".to_string(),
        _ => "i64".to_string(),
    }
}

// ---------------- statements ----------------

impl ModEmitter {
    fn walk_body(&mut self, fw: &mut FnWalk, s: &Stmt) {
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

    fn walk_stmt(&mut self, fw: &mut FnWalk, s: &Stmt) {
        match &s.node {
            StmtNode::Expr(e) => {
                let _ = self.emit_expr(fw, e);
            }
            StmtNode::Let { mutable: _, name, ty, init } => {
                let (v, t) = self.emit_expr(fw, init);
                let fl = self.is_float(t);
                // declared type (if given) must match; MVP: trust inferred
                let _ = ty;
                fw.declare(name, t, fl);
                fw.assign(name, &v, fl);
            }
            StmtNode::Assign { target, value } => {
                let (v, _vt) = self.emit_expr(fw, value);
                // object-field target: [name, field] where head is a local receiver
                if target.len() >= 2 {
                    if let (Some(PathSeg::Name(h)), Some(PathSeg::Name(f))) = (target.first(), target.last()) {
                        if let Some((at, rty)) = fw.lookup(&h.clone()) {
                            if let Ty::Named(c, _) = self.r.get(rty) {
                                // receiver word: alloca stores the object pointer word
                                let z = fw.v();
                                let recv = fw.v();
                                let mty = if self.is_float(rty) { "memref<1xf64>" } else { "memref<1xi64>" };
                                fw.op(&format!("    {} = arith.constant 0 : index", z));
                                fw.op(&format!("    {} = memref.load {}[{}] : {}", recv, at, z, mty));
                                let idx = self.field_index(&c, &f.clone());
                                let zi = fw.v();
                                fw.op(&format!("    {} = arith.constant {} : i64", zi, idx));
                                fw.op(&format!(
                                    "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                                    recv, zi, v
                                ));
                                return;
                            }
                        }
                    }
                }
                match target.last() {
                    Some(PathSeg::Name(n)) => {
                        let fl = match fw.lookup(n) {
                            Some((_, t)) => self.is_float(t),
                            None => false,
                        };
                        fw.assign(n, &v, fl);
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
    fn emit_ret_flag_store(&mut self, fw: &mut FnWalk) {
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
                fw.op(&format!(
                    "    {} = arith.constant 0.0 : f64",
                    z
                ));
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

    fn jump_to_ret(&mut self, fw: &mut FnWalk) {
        let l = fw.end_label.clone();
        fw.jump(&l);
    }
}

// ---------------- control-flow statements ----------------

impl ModEmitter {
    fn walk_return_value(&mut self, fw: &mut FnWalk, e: &Expr) {
        let (v, t) = self.emit_expr(fw, e);
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
    fn walk_if(&mut self, fw: &mut FnWalk, cond: &Expr, then_: &Stmt, else_: Option<&Stmt>, pos: &Pos) {
        if !fw.noterm() {
            self.err(pos, "unreachable code".to_string());
            return;
        }
        let (c, _ct) = self.emit_expr(fw, cond);
        let thlab = fw.newlabel("t");
        let ellab = fw.newlabel("e");
        let endlab = fw.newlabel("fi");
        fw.cjump(&c, &thlab, &ellab);
        fw.label(&thlab);
        self.walk_body(fw, then_);
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
    fn walk_while(&mut self, fw: &mut FnWalk, cond: &Expr, body: &Stmt, pos: &Pos) {
        if !fw.noterm() {
            self.err(pos, "unreachable code".to_string());
            return;
        }
        let head = fw.newlabel("wh");
        let doo = fw.newlabel("do");
        let done = fw.newlabel("wd");
        fw.jump(&head);
        fw.label(&head);
        let (c, _ct) = self.emit_expr(fw, cond);
        fw.cjump(&c, &doo, &done);
        fw.label(&doo);
        fw.loops.push((done.clone(), head.clone()));
        self.walk_body(fw, body);
        fw.loops.pop();
        fw.jump(&head);
        fw.label(&done);
    }

    fn walk_for(&mut self, fw: &mut FnWalk, var: &str, iter: &Expr, body: &Stmt, pos: &Pos) {
        if !fw.noterm() {
            self.err(pos, "unreachable code".to_string());
            return;
        }
        match &iter.node {
            ExprNode::Range { low, high, inclusive } => {
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
                fw.op(&format!("    memref.store {}, {}[{}] : memref<1xi64>", lo, islot, z));
                let head = fw.newlabel("fr");
                let doo = fw.newlabel("fb");
                let done = fw.newlabel("fd");
                fw.jump(&head);
                fw.label(&head);
                let iv = fw.v();
                fw.op(&format!("    {} = memref.load {}[{}] : memref<1xi64>", iv, islot, z));
                let c = fw.v();
                fw.op(&format!("    {} = arith.cmpi slt, {}, {} : i64", c, iv, hi));
                let c1 = fw.v();
                fw.op(&format!("    {} = arith.extsi {} : i1 to i64", c1, c));
                fw.cjump(&c1, &doo, &done);
                fw.label(&doo);
                fw.loops.push((done.clone(), head.clone()));
                // bind loop var
                let vs = fw.v();
                fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
                fw.op(&format!("    memref.store {}, {}[{}] : memref<1xi64>", iv, vs, z));
                fw.scopes.last_mut().unwrap().insert(var.to_string(), (vs, self.r.mk(Ty::I64)));
                self.walk_body(fw, body);
                fw.loops.pop();
                // idx += 1
                let one2 = fw.v();
                fw.op(&format!("    {} = arith.constant 1 : i64", one2));
                let nx = fw.v();
                fw.op(&format!("    {} = arith.addi {}, {} : i64", nx, iv, one2));
                fw.op(&format!("    memref.store {}, {}[{}] : memref<1xi64>", nx, islot, z));
                fw.jump(&head);
                fw.label(&done);
                fw.pop_scope();
            }
            _ => {
                // array iteration: desugar via len/push runtime below (MVP: error)
                self.err(pos, "iterate over arrays not yet supported".to_string());
            }
        }
    }

    fn walk_break(&mut self, fw: &mut FnWalk, pos: &Pos) {
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

    fn walk_continue(&mut self, fw: &mut FnWalk, pos: &Pos) {
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

// ---------------- expressions ----------------

fn mlir_word_ty(t: TyId, r: &Reg) -> String {
    match r.get(t) {
        Ty::F64 => "f64".to_string(),
        _ => "i64".to_string(),
    }
}

impl ModEmitter {
    /// field offsets in words: idx counted from slot 2 (slot 0: cls info, 1: unused?id)
    fn field_index(&self, clsname: &str, field: &str) -> usize {
        let depths: Vec<&str> = Vec::new();
        let _ = depths;
        let mut cur = Some(clsname.to_string());
        let mut out: usize = 0;
        // superclass fields first
        while let Some(c) = cur {
            if let Some(ci) = self.classes.get(&c) {
                for (n, _t, _m) in ci.fields.iter() {
                    if n == field {
                        return out;
                    }
                    out += 1;
                }
                cur = ci.superclass.clone();
                continue;
            }
            break;
        }
        out
    }
}

impl ModEmitter {
    fn intern_str(&mut self, s: &str) -> String {
        for (i, x) in self.strpool.iter().enumerate() {
            if x == s {
                return format!("@sl_str{}", i);
            }
        }
        self.strpool.push(s.to_string());
        format!("@sl_str{}", self.strpool.len() - 1)
    }
}

impl ModEmitter {
    pub fn emit_expr(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Int(v) => {
                let r = fw.v();
                fw.op(&format!("    {} = arith.constant {} : i64", r, v));
                let t = self.r.mk(Ty::I64);
                (r, t)
            }
            ExprNode::Float(v) => {
                let r = fw.v();
                let s = format!("{}", v);
                let t = self.r.mk(Ty::F64);
                fw.op(&format!("    {} = arith.constant {} : f64", r, s));
                (r, t)
            }
            ExprNode::Bool(v) => {
                let r = fw.v();
                let b = if *v { 1 } else { 0 };
                fw.op(&format!("    {} = arith.constant {} : i64", r, b));
                let t = self.r.mk(Ty::Bool);
                (r, t)
            }
            ExprNode::Nil => {
                let r = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", r));
                (r, self.r.mk(Ty::Unit))
            }
            _ => self.emit_expr_rest(fw, e),
        }
    }
}

impl ModEmitter {
    fn emit_expr_rest(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Str(ss) => {
                // packed 8-byte words across the runtime; buffer token chains
                let plain: Vec<u8> = ss.plain().unwrap_or("").bytes().collect();
                let len = plain.len();
                let mut pads = plain.clone();
                while pads.len() % 8 != 0 {
                    pads.push(0);
                }
                let mut curw = String::new();
                {
                    let c0 = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", c0));
                    curw = c0;
                    for (i, ch) in pads.chunks(8).enumerate() {
                        let mut w: u64 = 0;
                        for (k, b) in ch.iter().enumerate() {
                            w |= (*b as u64) << (8 * k);
                        }
                        let c1 = fw.v();
                        fw.op(&format!("    {} = arith.constant {} : i64", c1, w as i64));
                        let n = std::cmp::min(8usize, len - i * 8);
                        let c2 = fw.v();
                        fw.op(&format!("    {} = arith.constant {} : i64", c2, n));
                        let r = fw.v();
                        fw.op(&format!(
                            "    {} = call @sloth_str_push({}, {}, {}) : (i64, i64, i64) -> i64",
                            r, curw, c1, c2
                        ));
                        curw = r;
                    }
                }
                let fin = fw.v();
                fw.op(&format!(
                    "    {} = call @sloth_str_finish({}) : (i64) -> i64",
                    fin, curw
                ));
                let t = self.r.mk(Ty::Str);
                (fin, t)
            }
            ExprNode::Ident(name) => {
                if name == "true" || name == "false" {
                    let b = name == "true";
                    let t = self.r.mk(Ty::Bool);
                    let r = fw.v();
                    let bv = if b { 1 } else { 0 };
                    fw.op(&format!("    {} = arith.constant {} : i64", r, bv));
                    return (r, t);
                }
                if let Some((a, t)) = fw.lookup(name) {
                    let z = fw.v();
                    let v = fw.v();
                    let fl = self.is_float(t);
                    let mty = if fl { "memref<1xf64>" } else { "memref<1xi64>" };
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    fw.op(&format!("    {} = memref.load {}[{}] : {}", v, a, z, mty));
                    (v, t)
                } else if let Some((gsym, t, _)) = self.globals.get(name).cloned() {
                    let (v, _vt) = self.emit_global_read(fw, &gsym, t);
                    (v, t)
                } else {
                    self.err(&e.pos, format!("unknown identifier `{}`", name));
                    (String::new(), self.r.mk(Ty::Unit))
                }
            }
            _ => self.emit_expr_arith_codes(fw, e),
        }
    }
}

impl ModEmitter {
    fn emit_expr_arith_codes(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Arith { op, lhs, rhs } => {
                let (a, at) = self.emit_expr(fw, lhs);
                let (b, bt) = self.emit_expr(fw, rhs);
                if *op == ArithOp::Add && self.is_str(at) && self.is_str(bt) {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_str_concat({}, {}) : (i64, i64) -> i64",
                        r, a, b
                    ));
                    return (r, self.r.mk(Ty::Str));
                }
                let fl = self.is_float(at) || self.is_float(bt);
                if fl {
                    let r = fw.v();
                    let ao = match op {
                        ArithOp::Add => "arith.addf",
                        ArithOp::Sub => "arith.subf",
                        ArithOp::Mul => "arith.mulf",
                        ArithOp::Div => "arith.divf",
                        ArithOp::Mod => "arith.remf",
                    };
                    fw.op(&format!("    {} = {} {}, {} : f64", r, ao, a, b));
                    return (r, self.r.mk(Ty::F64));
                }
                let r = fw.v();
                let ao = match op {
                    ArithOp::Add => "arith.addi",
                    ArithOp::Sub => "arith.subi",
                    ArithOp::Mul => "arith.muli",
                    ArithOp::Div => "arith.divsi",
                    ArithOp::Mod => "arith.remsi",
                };
                fw.op(&format!("    {} = {} {}, {} : i64", r, ao, a, b));
                (r, self.r.mk(Ty::I64))
            }
            _ => self.emit_expr_rest2(fw, e),
        }
    }
}


impl ModEmitter {
    fn emit_expr_rest2(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Bin { op, lhs, rhs } => {
                let (a, at) = self.emit_expr(fw, lhs);
                let (b, bt) = self.emit_expr(fw, rhs);
                return self.emit_binop(fw, op, a, b, at, bt, &e.pos);
            }
            ExprNode::Un { op, expr } => {
                let (v, t) = self.emit_expr(fw, expr);
                let fl = self.is_float(t);
                let r = fw.v();
                match op {
                    UnOp::Neg => {
                        let z = if fl {
                            fw.v()
                        } else {
                            let z2 = fw.v();
                            let _ = z2;
                            String::new()
                        };
                        let _ = &r;
                        if fl {
                            fw.op(&format!(
                                "    {} = arith.constant 0.0 : f64",
                                r
                            ));
                            let nz = fw.v();
                            fw.op(&format!("    {} = arith.subf {}, {} : f64", nz, r, v));
                            return (nz, t);
                        }
                        let zi = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", zi));
                        let nr = fw.v();
                        fw.op(&format!("    {} = arith.subi {}, {} : i64", nr, zi, v));
                        (nr, t)
                    }
                    UnOp::Not => {
                        let one = fw.v();
                        fw.op(&format!("    {} = arith.constant 1 : i64", one));
                        let z2 = fw.v();
                        fw.op(&format!("    {} = arith.subi {}, {} : i64", z2, one, v));
                        (z2, t)
                    }
                }
            }
            _ => self.emit_expr_leaf_codes(fw, e),
        }
    }
}

impl ModEmitter {
    fn emit_binop(
        &mut self,
        fw: &mut FnWalk,
        op: &BinOp,
        a: String,
        b: String,
        at: TyId,
        bt: TyId,
        pos: &Pos,
    ) -> (String, TyId) {
        let fl = self.is_float(at) || self.is_float(bt);
        let cmp_ty_id = self.r.mk(Ty::Bool);
        if fl {
            let pred = match op {
                BinOp::EqEq => "oeq",
                BinOp::NotEq => "une",
                BinOp::Lt => "olt",
                BinOp::Le => "ole",
                BinOp::Gt => "ogt",
                BinOp::Ge => "oge",
                BinOp::And | BinOp::Or => {
                    self.err(pos, "logical on float".to_string());
                    return (String::new(), cmp_ty_id);
                }
            };
            let r = fw.v();
            let mk = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", mk));
            fw.op(&format!("    {} = arith.cmpf {}, {}, {} : f64", r, pred, a, b));
            let z = fw.v();
            fw.op(&format!("    {} = arith.extsi {} : i1 to i64", z, r));
            (z, cmp_ty_id)
        } else {
            let z = fw.v();
            let mk = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", mk));
            let pr = match op {
                BinOp::EqEq => "eq",
                BinOp::NotEq => "ne",
                BinOp::Lt => "slt",
                BinOp::Le => "sle",
                BinOp::Gt => "sgt",
                BinOp::Ge => "sge",
                BinOp::And => {
                    let r2 = fw.v();
                    fw.op(&format!("    {} = arith.andi {}, {} : i64", r2, a, b));
                    return (r2, cmp_ty_id);
                }
                BinOp::Or => {
                    let r2 = fw.v();
                    fw.op(&format!("    {} = arith.ori {}, {} : i64", r2, a, b));
                    return (r2, cmp_ty_id);
                }
            };
            fw.op(&format!("    {} = arith.cmpi {}, {}, {} : i64", z, pr, a, b));
            let z2 = fw.v();
            fw.op(&format!("    {} = arith.extsi {} : i1 to i64", z2, z));
            (z2, cmp_ty_id)
        }
    }
}


impl ModEmitter {
    fn emit_expr_leaf_codes(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Call { callee, args } => {
                return self.emit_call(fw, callee, args, &e.pos);
            }
            ExprNode::Field { obj, name } => {
                // qualified foreign-global read: lib.g / lib.Cls.f handled in arith path only for globals
                if let ExprNode::Ident(m) = &obj.node {
                    let key = format!("{}.{}", m, name);
                    if let Some((g, gt)) = self.fglobals.get(&key).cloned() {
                        return self.emit_global_read(fw, &g, gt);
                    }
                }
                let (recv, rt) = self.emit_expr(fw, obj);
                if let Ty::Named(c, _) = self.r.get(rt) {
                    let idx = self.field_index(c, name);
                    let zi = fw.v();
                    fw.op(&format!("    {} = arith.constant {} : i64", zi, idx));
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
                        r, recv, zi
                    ));
                    return (r, self.r.mk(Ty::I64));
                }
                self.err(&e.pos, format!("field `{}` on unknown type", name));
                (String::new(), self.r.mk(Ty::Unit))
            }
            ExprNode::Range { low, high, inclusive } => {
                // MVP range lit as two-word repr: [lo, hi(+1)] held as i64 lo packed
                let (lo, _lt) = self.emit_expr(fw, low);
                let (hi, _ht) = self.emit_expr(fw, high);
                let one = fw.v();
                fw.op(&format!("    {} = arith.constant 1 : i64", one));
                let hi2 = fw.v();
                fw.op(&format!("    {} = arith.addi {}, {} : i64", hi2, hi, one));
                let _ = inclusive;
                // pack lo in high word positions: MVP sloth.range helper
                let r = fw.v();
                fw.op(&format!(
                    "    {} = call @sloth_range_pack({}, {}) : (i64, i64) -> i64",
                    r, lo, hi2
                ));
                (r, self.r.mk(Ty::Range))
            }
            ExprNode::This => match fw.scopes.first().and_then(|sc| sc.get("this")).map(|s| (
                s.0.clone(), s.1
            )) {
                Some((a, t)) => {
                    let z = fw.v();
                    let v = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    fw.op(&format!("    {} = memref.load {}[{}] : memref<1xi64>", v, a, z));
                    (v, t)
                }
                None => {
                    self.err(&e.pos, "this outside method".to_string());
                    (String::new(), self.r.mk(Ty::Unit))
                }
            },
            _ => {
                self.err(&e.pos, format!("expression not yet supported"));
                (String::new(), self.r.mk(Ty::Unit))
            }
        }
    }
}

impl ModEmitter {
    /// method or function call. obj.method() => callee Field{obj,name}: direct dispatch
    fn emit_call(
        &mut self,
        fw: &mut FnWalk,
        callee: &Expr,
        args: &Vec<Expr>,
        pos: &Pos,
    ) -> (String, TyId) {
        // resolve name
        let mut name = match &callee.node {
            ExprNode::Ident(n) => n.clone(),
            ExprNode::Field { obj: _, name } => name.clone(),
            _ => {
                self.err(pos, "only named calls supported".to_string());
                return (String::new(), self.r.mk(Ty::Unit));
            }
        };
        let mut argv: Vec<(String, TyId)> = Vec::new();
        let mut sigargs: Vec<String> = Vec::new();
        // method call: receiver becomes first argument
        let mut recv: Option<(String, TyId)> = None;
        if let ExprNode::Field { obj, name: mname2 } = &callee.node {
            // qualified cross-module call: lib.fn(...) or alias.fn(...)
            if let ExprNode::Ident(m) = &obj.node {
                let key = format!("{}.{}", m, mname2);
                if let Some(fs) = self.cross_funcs.get(&key).cloned() {
                    let mut cargv: Vec<(String, TyId)> = Vec::new();
                    let mut csig: Vec<String> = Vec::new();
                    for a in args {
                        let (v, t) = self.emit_expr(fw, a);
                        cargv.push((v, t));
                        csig.push(mlir_word_ty(t, &self.r));
                    }
                    let r = fw.v();
                    let vals: Vec<String> = cargv.iter().map(|x| x.0.clone()).collect();
                    let rt = mlir_ret_ty(self, fs.1);
                    fw.op(&format!(
                        "    {} = call @{}({}) : ({}) -> {}",
                        r, fs.0, vals.join(", "), csig.join(", "), rt
                    ));
                    return (r, fs.1);
                }
                if let Some((g, gt)) = self.fglobals.get(&key).cloned() {
                    return self.emit_global_read(fw, &g, gt);
                }
            }
            let (rv, rt) = self.emit_expr(fw, obj);
            recv = Some((rv.clone(), rt));
            argv.insert(0, (rv, rt));
            sigargs.insert(0, "i64".to_string());
            name = match &callee.node {
                ExprNode::Field { name: fname, .. } => fname.clone(),
                _ => name.clone(),
            };
        }
        for a in args {
            let (v, t) = self.emit_expr(fw, a);
            argv.push((v.clone(), t));
            sigargs.push(mlir_word_ty(t, &self.r));
        }
        // constructor: bare class name call
        if let ExprNode::Ident(ctor) = &callee.node {
            if self.classes.contains_key(&name) && name == *ctor {
                return self.emit_new_obj(fw, &name, &argv, &sigargs, pos);
            }
        }
        // method call inside classes
        if let Some((rv, rt)) = recv.clone() {
            if let Ty::Named(cls, _) = self.r.get(rt) {
                let clsname = cls.to_string();
                let fd = self
                    .classes
                    .get(&clsname)
                    .and_then(|ci| ci.methods.iter().find(|(n, _)| *n == name).map(|(_, f)| f.clone()));
                if let Some(m) = fd {
                    return self.emit_method_call(fw, &clsname, &name, &m, &argv, &sigargs, pos);
                }
                let _ = rv;
            }
        }
        // direct function call
        if let Some(fd) = self.funcs.get(&name).cloned() {
            let plan = self.plan_func(&name, None, &fd, None);
            let r = fw.v();
            let sym = plan.mangled.clone();
            let vals: Vec<String> = argv.iter().map(|x| x.0.clone()).collect();
            let tys = sigargs.join(", ");
            let rt = mlir_ret_ty(self, plan.ret);
            fw.op(&format!(
                "    {} = call @{}({}) : ({}) -> {}",
                r, sym, vals.join(", "), tys, rt
            ));
            return (r, plan.ret);
        }
        // foreign-module function: symbol was pre-mangled at import time
        if let Some(fs) = self.cross_funcs.get(&name).cloned() {
            let r = fw.v();
            let vals: Vec<String> = argv.iter().map(|x| x.0.clone()).collect();
            let rt = mlir_ret_ty(self, fs.1);
            fw.op(&format!(
                "    {} = call @{}({}) : ({}) -> {}",
                r, fs.0, vals.join(", "), sigargs.join(", "), rt
            ));
            return (r, fs.1);
        }
        // builtins
        let r = fw.v();
        match name.as_str() {
            "print" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let mty = mlir_word_ty(t, &self.r);
                let sym = match self.r.get(t) {
                    Ty::Str => "sloth_rt_print_str",
                    Ty::F64 => "sloth_rt_print_f64",
                    Ty::Bool => "sloth_rt_print_bool",
                    _ => "sloth_rt_print_i64",
                };
                fw.op(&format!(
                    "    {} = call @{}({}) : ({}) -> i64",
                    r, sym, v, mty
                ));
                (r, self.r.mk(Ty::Unit))
            }
            "len" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let ts = self.r.get(t).clone();
                let sym = match &ts {
                    Ty::Str => "sloth_str_len",
                    Ty::Array(_) => "sloth_array_len",
                    _ => "sloth_str_len",
                };
                fw.op(&format!(
                    "    {} = call @{}({}) : (i64) -> i64",
                    r, sym, v
                ));
                (r, self.r.mk(Ty::I64))
            }
            _ => {
                self.err(pos, format!("call to unknown `{}`", name));
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                (z, self.r.mk(Ty::Unit))
            }
        }
    }
}

// ---------------- module driver ----------------

/// external runtime symbols used by generated code
pub fn rt_decls() -> String {
    let mut s = String::new();
    s.push_str("  func.func private @sloth_rt_print_i64(i64) -> i64\n");
    s.push_str("  func.func private @sloth_rt_print_f64(f64) -> i64\n");
    s.push_str("  func.func private @sloth_rt_print_bool(i64) -> i64\n");
    s.push_str("  func.func private @sloth_rt_print_str(i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_intern(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_range_pack(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_push(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_finish(i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_len(i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_concat(i64, i64) -> i64\n");
    s
}


/// number of actual code bytes to reserve for a string global (with NUL)
fn str_slot_len(s: &str) -> usize {
    s.as_bytes().len() + 1
}

fn emit_str_globals(me: &ModEmitter) -> String {
    let mut out = String::new();
    for (i, s) in me.strpool.iter().enumerate() {
        let mut bytes: Vec<u8> = s.bytes().collect();
        bytes.push(0);
        let mut arr = String::new();
        for (k, b) in bytes.iter().enumerate() {
            if k > 0 {
                arr.push_str(", ");
            }
            arr.push_str(&format!("{}", b));
        }
        let n = bytes.len();
        out.push_str(&format!(
            "  llvm.mlir.global private constant @sl_str{} = dense<[{}]> : !llvm.array<i8 x {}>\n",
            i, arr, n
        ));
    }
    out
}

impl ModEmitter {
    pub fn emit_module(&mut self, prog: &Program) -> Vec<Diag> {
        self.collect(prog);
        // 1) top-level funcs
        for d in &prog.decls {
            if let DeclNode::Func(f) = &d.node {
                let entry = d.name == "main";
                self.emit_func(&d.name, None, f, None, entry);
            }
        }
        // 1b) classes: emit methods + ctor
        for d in &prog.decls {
            if let DeclNode::Class(c) = &d.node {
                for m in &c.methods {
                    self.emit_func(&m.name, Some(&d.name), &m.fd, None, false);
                }
            }
        }
        // 2) script statements run in entry if no main() was declared
        let has_main = prog.decls.iter().any(|d| d.name == "main" && matches!(d.node, DeclNode::Func(_)));
        if !has_main && !prog.stmts.is_empty() {
            let mut fw = FnWalk {
                cur: String::new(),
                vcount: 1000,
                scopes: vec![HashMap::new()],
                loops: Vec::new(),
                ret: self.r.mk(Ty::Unit),
                ret_alloca: String::new(),
                ret_flag: String::new(),
                bb: 0,
                term: false,
                end_label: "^smt".to_string(),
            };
            let rf = fw.v();
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", rf));
            fw.ret_flag = rf;
            fw.push_scope();
            // run imported modules' variable initializers first
            for m in &self.init_mods {
                fw.op(&format!("    call @sloth_{}__ginit() : () -> ()", m));
            }
            // top-level var/let decls become prelude statements
            for d in &prog.decls {
                if let DeclNode::Var { ty, init } = &d.node {
                    let st = Stmt {
                        pos: d.pos.clone(),
                        node: StmtNode::Let {
                            mutable: d.kind == DeclKind::Var,
                            name: d.name.clone(),
                            ty: ty.clone(),
                            init: init.clone(),
                        },
                    };
                    self.walk_stmt(&mut fw, &st);
                }
            }
            for s in &prog.stmts {
                self.walk_stmt(&mut fw, s);
            }
            fw.pop_scope();
            self.finish_entry(&mut fw);
        }
        let diag = self.diags.clone();
        diag
    }

    /// wrap up script entry function text into self.out
    fn finish_entry(&mut self, fw: &mut FnWalk) {
        let endlab = fw.newlabel("smt");
        fw.jump(&endlab);
        let body_text = fw.cur.clone();
        fw.cur = String::new();
        fw.term = false;
        fw.label(&endlab);
        fw.op("    return");
        let rt_text = fw.cur.clone();
        self.out.push_str(&format!(
            "  func.func @sloth_main() -> () attributes {{llvm.emit_c_interface}} {{\n{}{}  }}\n",
            body_text, rt_text
        ));
    }

    /// reserve a mangled global symbol and register its decl line (deduped)
    fn declare_global(&mut self, modname: &str, name: &str, t: TyId) -> String {
        let sym = format!("sloth_{}_g_{}", modname, name);
        if self.declared_syms.insert(sym.clone()) {
            let mty = memref_cell_ty(self, t);
            let init = if mty == "memref<1xf64>" { "dense<0.0>" } else { "dense<0>" };
            self.global_decls
                .push(format!("  memref.global @{} : {} = {} {{mutable}}\n", sym, mty, init));
        }
        sym
    }

    /// emit get_global + load for a global cell reference
    fn emit_global_read(&mut self, fw: &mut FnWalk, sym: &str, t: TyId) -> (String, TyId) {
        let mty = memref_cell_ty(self, t);
        let g = fw.v();
        fw.op(&format!("    {} = memref.get_global @{} : {}", g, sym, mty));
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        let v = fw.v();
        fw.op(&format!("    {} = memref.load {}[{}] : {}", v, g, z, mty));
        (v, t)
    }

    /// full MLIR text of the module
    pub fn take_ir(me: &mut ModEmitter) -> String {
        let mut m = format!("module @{} {{\n", me.name);
        m.push_str(&emit_str_globals(me));
        m.push_str(&rt_decls());
        m.push_str(&obj_rt_decls());
        for gd in &me.global_decls {
            m.push_str(gd);
        }
        m.push_str("\n");
        m.push_str(&me.out);
        // now the out is func bodies only; globals were prepended
        // (we already integrated globals above; emit closing brace)
        m.push_str("}\n");
        m
    }
}

fn memref_cell_ty(me: &ModEmitter, t: TyId) -> &'static str {
    if me.is_float(t) { "memref<1xf64>" } else { "memref<1xi64>" }
}

fn fresh_walk(me: &mut ModEmitter) -> FnWalk {
    FnWalk {
        cur: String::new(),
        vcount: 1000,
        scopes: vec![HashMap::new()],
        loops: Vec::new(),
        ret: me.r.mk(Ty::Unit),
        ret_alloca: String::new(),
        ret_flag: String::new(),
        bb: 0,
        term: false,
        end_label: "^ginit".to_string(),
    }
}

// final IR normalization: rewrite zero consts used as memref indices to `index`
pub fn normalize_indices(src: &str) -> String {
    // process per-function so that per-fn scoped SSA names don't collide
    let mut out = String::new();
    let mut chunk: Vec<String> = Vec::new();
    for line in src.lines() {
        let starts_fn = line.trim_start().starts_with("func.func");
        if starts_fn && !chunk.is_empty() {
            out.push_str(&normalize_chunk(&chunk));
            chunk = Vec::new();
        }
        chunk.push(line.to_string());
    }
    if !chunk.is_empty() {
        out.push_str(&normalize_chunk(&chunk));
    }
    out
}

fn normalize_chunk(lines: &[String]) -> String {
    use std::collections::HashSet;
    let mut idx_tokens: HashSet<String> = HashSet::new();
    for line in lines {
        let t = line.trim();
        if !(t.contains('[') && t.contains(']')) {
            continue;
        }
        let open = t.find('[').unwrap();
        let close = t.find(']').unwrap();
        let inner = &t[open + 1..close];
        for tok in inner.split(',') {
            let tok = tok.trim();
            if tok.starts_with('%') {
                idx_tokens.insert(tok.to_string());
            }
        }
    }
    let mut out = String::new();
    for line in lines {
        let t = line.trim();
        if let Some(i) = t.find(" = arith.constant ") {
            let tok = t[..i].trim().to_string();
            if idx_tokens.contains(&tok) {
                let after = &t[i + " = arith.constant ".len()..];
                if after.trim_end() == "0 : i64" {
                    out.push_str(&line.replace("0 : i64", "0 : index"));
                    out.push('\n');
                    continue;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

// ---------------- classes (MVP: flat classes this-module) ----------------

pub fn obj_rt_decls() -> String {
    let mut s = String::new();
    s.push_str("  func.func private @sloth_obj_new(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_field(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_field_f64(i64, i64) -> f64\n");
    s.push_str("  func.func private @sloth_obj_set_field(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_set_field_f64(i64, i64, f64) -> i64\n");
    s.push_str("  func.func private @sloth_cls_info(i64, i64) -> i64\n");
    s
}

fn words_for_cls(me: &ModEmitter, clsname: &str) -> usize {
    me.classes
        .get(clsname)
        .map(|ci| ci.fields.len())
        .unwrap_or(0)
    + 2
}

impl ModEmitter {
    /// class ctor: named `sloth_main_<Cls>_cls`
    fn class_ctor_name(&self, cls: &str) -> String {
        format!("{}_{}", self.name, cls)
    }
}


impl ModEmitter {
    /// allocate object with GC; fields set after ctor body
    fn emit_new_obj(
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
        let fdinit = self
            .classes
            .get(clsname)
            .and_then(|ci| ci.methods.iter().find(|(n, _)| n == "__init__").map(|(_, f)| f.clone()));
        if let Some(fd) = fdinit {
            let plan = self.plan_mangled("__init__", Some(clsname), &fd, None);
            let vals = [r2.clone()].iter().cloned()
                .chain(argv.iter().map(|x| x.0.clone()))
                .collect::<Vec<_>>().join(", ");
            let tys = plan.params.iter()
                .map(|(_n, t, fl)| if self.is_float(*t) { "f64".to_string() } else { "i64".to_string() })
                .collect::<Vec<_>>().join(", ");
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
    fn emit_method_call(
        &mut self,
        fw: &mut FnWalk,
        cls: &str,
        mname: &str,
        m: &FuncDef,
        argv: &Vec<(String, TyId)>,
        sigargs: &Vec<String>,
        pos: &Pos,
    ) -> (String, TyId) {
        // ctor: emit the class ctor wrapper (allocates then runs __init__)
        if mname == "__init__" {
            return self.emit_new_obj(fw, cls, argv, sigargs, pos);
        }
        // direct method dispatch: obj.method(args) => sloth_main_Cls__method(this, args...)
        let plan = self.plan_mangled(mname, Some(cls), m, None);
        let r = fw.v();
        let vals: Vec<String> = argv.iter().map(|x| x.0.clone()).collect();
        let tys = sigargs.join(", ");
        let rt = mlir_ret_ty(self, plan.ret);
        fw.op(&format!(
            "    {} = call @{}({}) : ({}) -> {}",
            r, plan.mangled, vals.join(", "), tys, rt
        ));
        (r, plan.ret)
    }
}

/// parse + lower a sloth2 source to normalized MLIR text (no execution)
pub fn compile_to_ir(src: &str, mod_name: &str) -> Result<String, String> {
    let prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    let mut me = ModEmitter::new(mod_name);
    me.emit_module(&prog);
    if !me.diags.is_empty() {
        return Err(format!("codegen diags: {:?}", me.diags));
    }
    let ir0 = ModEmitter::take_ir(&mut me);
    Ok(normalize_indices(&ir0))
}

/// multi-module driver: resolves `import "x.sl"` recursively (dedupe by
/// canonical path), emits each imported module's surface first, then runs the
/// root module's entry statements under @main
pub fn compile_multimod(root_src: &str, base_dir: &std::path::Path) -> Result<String, String> {
    let (root, mods) = resolve_program(root_src, base_dir, &mut Vec::new())?;
    let mut me = ModEmitter::new("main");
    for (mname, prog, alias) in &mods {
        me.register_import(mname, alias.as_deref(), prog);
    }
    // hmm: root module runs under @sloth_main through emit_module
    me.emit_module(&root);
    let ir0 = ModEmitter::take_ir(&mut me);
    Ok(normalize_indices(&ir0))
}

fn resolve_program(
    src: &str,
    dir: &std::path::Path,
    seen: &mut Vec<std::path::PathBuf>,
) -> Result<(Program, Vec<(String, Program, Option<String>)>), String> {
    let prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    let mut mods: Vec<(String, Program, Option<String>)> = Vec::new();
    _ = seen;
    for imp in &prog.imports {
        let pb = dir.join(&imp.path);
        let pb2 = match std::fs::canonicalize(&pb) {
            Ok(p) => p,
            Err(_) => pb.clone(),
        };
        if seen.iter().any(|x| x == &pb2) {
            continue;
        }
        seen.push(pb2.clone());
        let src2 = std::fs::read_to_string(&pb2).map_err(|e| format!("read {:?}: {}", pb2, e))?;
        let dir2 = pb2.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| std::path::PathBuf::from("."));
        let (_p2, mut m2) = resolve_program(&src2, &dir2, seen)?;
        let stem = pb2
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "mod".to_string());
        mods.push((stem, prog2_of(&_p2), imp.alias.clone()));
        mods.append(&mut m2);
    }
    Ok((prog, mods))
}

fn prog2_of(prog: &Program) -> Program {
    Program {
        imports: Vec::new(),
        decls: prog.decls.clone(),
        stmts: Vec::new(),
    }
}
