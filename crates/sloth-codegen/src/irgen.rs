//! End-to-end codegen: untyped AST -> textual MLIR (func/arith/cf/memref).
//! One pass does name resolution + type inference + emission. Diagnostics are
//! collected rather than aborting. Lowering uses plain `cf` basic blocks built
//! with SSA env captured at emission time (locals live in memref allocas).

use crate::sys;
use sloth_frontend::ast::*;
use sloth_frontend::lexer::{Pos, StrPart};
use sloth_frontend::ty::{Diag, FnTy, LamMeta, Reg, Ty, TyId};
use std::collections::{HashMap, HashSet};

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
    /// raw class defs (for field-initializer emission at ctor time)
    pub class_defs: HashMap<String, (String, ClassDef)>,
    /// non-pub symbols exported by imported modules; access = diagnostic
    pub hidden: HashSet<String>,
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
    /// foreign classes imported (ids 100+; offset starts here)
    pub foreign_cls: std::collections::HashSet<String>,
    /// module that owns a class (own modules use `name`; foreign classes owned mod)
    pub cls_mod: HashMap<String, String>,
    /// lambda function counter (unique symbols per lambda site)
    pub lamcount: usize,
    /// generic-class instances registered during typing: (inst name, T-frame)
    pub pending_insts: Vec<(String, HashMap<String, TyId>)>,
    /// next fresh native class id for synthesized instances
    pub native_cls_id: i64,
    /// Result<T,E> instances (builtins `ok(v)`/`err(e)` target them)
    pub result_insts: std::collections::HashSet<String>,
    /// `extern type` declared opaque surfaces (no ctor/fields/methods)
    pub extern_types: std::collections::HashSet<String>,
    /// generic base T-frame per registered instance (plan-time T resolution)
    pub class_frames: HashMap<String, HashMap<String, TyId>>,
    /// devirt/inline observation counters (patch #18c; SLOTH_STATS=1 prints)
    pub stat_dcalls: usize,
    pub stat_dyncalls: usize,
    pub stat_ginsts: usize,
    pub stat_extdecls: usize,
    /// registration order of classes (deterministic dyn-dispatch chain)
    pub class_order: Vec<String>,
    /// vtable slot assignment: (trait, method) -> (index, ret-float, ret-unit)
    pub vt_slots: HashMap<(String, String), usize>,
    /// methods emitted inside llvm.func (vtable-addressable)
    pub llvm_method: std::collections::HashSet<(String, String)>,
    /// object bodies carry a fixed vtable capacity (total slots)
    pub vt_cap: usize,
    /// active type-param substitution for the generic instance being emitted
    /// (stacked for nesting; ty positions resolve T against the top frame)
    tp_subst: Vec<HashMap<String, TyId>>,
    /// mangled instance name forced for the next plan_func/emit_func
    tp_mangled: Vec<String>,
    /// generic instance cache: base mangled -> concrete word spelling key
    insts: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub fields: Vec<(String, TyId, bool)>, // (name, ty, mutable)
    pub methods: Vec<(String, FuncDef)>,
    pub superclass: Option<String>,
    pub impls: Vec<String>,
}

/// for-in index loop element kinds
#[derive(Clone, Copy, PartialEq)]
enum IdxKind {
    /// array word backed by sloth_arr_len + sloth_arr_get(_f64)
    Arr,
    /// interned string: sloth_str_len + sloth_str_char returns Str words
    StrChar,
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
            class_defs: HashMap::new(),
            hidden: HashSet::new(),
            fglobals: HashMap::new(),
            global_decls: Vec::new(),
            declared_syms: std::collections::HashSet::new(),
            init_mods: Vec::new(),
            mod_alias: HashMap::new(),
            foreign_cls: std::collections::HashSet::new(),
            cls_mod: HashMap::new(),
            lamcount: 0,
            stat_dcalls: 0,
            stat_dyncalls: 0,
            stat_ginsts: 0,
            stat_extdecls: 0,
            pending_insts: Vec::new(),
            native_cls_id: 0,
            result_insts: std::collections::HashSet::new(),
            extern_types: std::collections::HashSet::new(),
            class_frames: HashMap::new(),
            class_order: Vec::new(),
            vt_slots: HashMap::new(),
            llvm_method: std::collections::HashSet::new(),
            vt_cap: 0,
            tp_subst: Vec::new(),
            tp_mangled: Vec::new(),
            insts: std::collections::HashMap::new(),
        }
    }

    /// adopt the surface of a foreign module (emits its funcs/classes under that
    /// module's name and registers them for cross-module calls)
    pub fn register_import(&mut self, mname: &str, alias: Option<&str>, prog: &Program) {
        self.cur_mod = mname.to_string();
        let _hide_mark = ();

        if let Some(a) = alias {
            self.mod_alias.insert(a.to_string(), mname.to_string());
        }
        let qname = alias.unwrap_or(mname).to_string();
        // foreign traits first (impl checks resolve against them)
        for d in &prog.decls {
            if let DeclNode::Trait(t) = &d.node {
                self.traits.insert(d.name.clone(), t.methods.clone());
            }
        }
        // pass 2a: register classes + symbols (before emission)
        for d in &prog.decls {
            match &d.node {
                DeclNode::Func(f) => {
                    let mangled = mangle(mname, None, &d.name);
                    let plan = self.plan_func(&d.name, None, f, None);
                    if d.visible {
                        self.cross_funcs.insert(d.name.clone(), (mangled.clone(), plan.ret));
                    } else {
                        self.hidden.insert(d.name.clone());
                    }
                    self.cross_funcs
                        .insert(format!("{}.{}", qname, d.name), (mangled, plan.ret));
                    if !d.visible {
                        self.hidden.insert(format!("{}.{}", qname, d.name));
                    }
                }
                DeclNode::Class(c) => {
                    let cid: i64 = 100 + self.foreign_cls.len() as i64;
                    self.foreign_cls.insert(d.name.clone());
                    if !d.visible {
                        self.hidden.insert(format!("{}.{}", qname, d.name));
                    }
                    self.cls_mod.insert(d.name.clone(), mname.to_string());
                    self.class_ids.insert(d.name.clone(), cid);
                    if !self.class_order.contains(&d.name) {
                        self.class_order.push(d.name.clone());
                    }
                    // register info for ctor + method dispatch
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
                    self.class_defs.insert(
                        d.name.clone(),
                        (self.cur_mod.clone(), (**c).clone()),
                    );
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
                _ => {}
            }
        }
        // pass 2b: effective trait surfaces (incl. inherited) -> vtable slots
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
        // pass 2c: emit method bodies (llvm.func decision now settled)
        for d in &prog.decls {
            match &d.node {
                DeclNode::Func(f) => {
                    self.emit_func(&d.name, None, f, None, false);
                }
                DeclNode::Class(_) => {
                    let meths = match self.classes.get(&d.name) {
                        Some(ci) => ci.methods.clone(),
                        None => Vec::new(),
                    };
                    for m in meths {
                        self.emit_func(&m.0, Some(&d.name), &m.1, None, false);
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
                if !visible_of(d) {
                    self.hidden.insert(format!("{}.{}", qname, d.name));
                }
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
        self.finalize_vt();
    }

    /// snapshot-capturing lambda: value = frame obj; symbol = clo function
    fn emit_lambda(&mut self, fw: &mut FnWalk, l: &Lambda, pos: &Pos) -> (String, TyId) {
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
        // snapshot each captured variable into the frame words
        for (j, cn) in caps.iter().enumerate() {
            let (cv, _ct) = match fw.lookup(cn) {
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
            fw.op(&format!(
                "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                frame, zi, cv
            ));
        }
        // construct the closured function body
        let mut params: Vec<Param> = caps
            .iter()
            .map(|c| Param { name: c.clone(), ty: None })
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
    /// per-scope bindovable map: value true = immutable (let)
    imms: Vec<HashMap<String, bool>>,
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
    /// class this function/method body belongs to (for this/super resolution)
    cur_cls: Option<String>,
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
        self.imms.push(HashMap::new());
    }
    fn pop_scope(&mut self) {
        self.scopes.pop();
        self.imms.pop();
    }
    fn declare(&mut self, name: &str, t: TyId, fl: bool, mutable: bool) -> String {
        let a = self.declare_raw(name, t, fl);
        self.imms
            .last_mut()
            .unwrap()
            .insert(name.to_string(), !mutable);
        a
    }
    fn declare_raw(&mut self, name: &str, t: TyId, fl: bool) -> String {
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
                self.r.mk(Ty::Fn(FnTy { params: ps, ret: nr, lam: None }))
            }
            SimpleType::Dyn(t) => self.r.mk(Ty::Dyn(t.clone())),
            SimpleType::Named(n, args) => {
                let a: Vec<TyId> = args.iter().map(|t| self.ty_of(t)).collect();
                self.ty_named(n, a)
            }
            SimpleType::Ident(n) => {
                // generic type-param reference in a generic instance being emitted
                if let Some(frame) = self.tp_subst.last() {
                    if let Some(t) = frame.get(n) {
                        return *t;
                    }
                }
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
            _ => {
                // trait name as a type position: an interface reference
                if a.is_empty() && self.traits.contains_key(n) {
                    return self.r.mk(Ty::Dyn(n.to_string()));
                }
                // generic class instance: C<A1,A2> -> monomorphic C_<A>_...
                if !a.is_empty() {
                    if let Some((_, cdef)) = self.class_defs.get(n).cloned() {
                        if !cdef.type_params.is_empty() {
                            return self.declare_class_inst(n, &a);
                        }
                    }
                }
                self.r.mk(Ty::Named(n.to_string(), a))
            }
        }
    }


    /// register (or fetch) the monomorphic instance of generic class `n`
    /// with text args `a`; fields are typed under the substitution frame
    fn declare_class_inst(&mut self, n: &str, a: &[TyId]) -> TyId {
        let inst = format!("{}{}", n, mangle_t(a, &self.r));
        if self.class_ids.contains_key(&inst) {
            return self.r.mk(Ty::Named(inst.clone(), a.to_vec()));
        }
        let (defmod, cdef) = match self.class_defs.get(n).cloned() {
            Some(x) => x,
            None => return self.r.mk(Ty::Named(n.to_string(), a.to_vec())),
        };
        let mut frame: HashMap<String, TyId> = HashMap::new();
        for (tp, ty) in cdef.type_params.iter().zip(a.iter()) {
            frame.insert(tp.name.clone(), *ty);
        }
        let fields: Vec<(String, TyId, bool)> = {
            self.tp_subst.push(frame.clone());
            let f = cdef
                .fields
                .iter()
                .map(|fd| (fd.name.clone(), self.ty_of(&fd.ty), fd.mutable))
                .collect();
            self.tp_subst.pop();
            f
        };
        if !self.class_order.contains(&inst) {
            self.class_order.push(inst.clone());
        }
        self.native_cls_id += 1;
        let mut nid = self.native_cls_id;
        while self.class_ids.values().any(|&v| v == nid) {
            nid += 1;
        }
        self.class_ids.insert(inst.clone(), nid);
        self.cls_mod.insert(inst.clone(), defmod.clone());
        self.class_defs
            .insert(inst.clone(), (defmod.clone(), cdef.clone()));
        let meth: Vec<(String, FuncDef)> = cdef
            .methods
            .iter()
            .map(|m| (m.name.clone(), m.fd.clone()))
            .collect();
        self.classes.insert(
            inst.clone(),
            ClassInfo {
                name: inst.clone(),
                fields,
                methods: meth,
                superclass: cdef.superclass.clone(),
                impls: cdef.impls.clone(),
            },
        );
        if n == "Result" {
            self.result_insts.insert(inst.clone());
        }
        self.class_frames.insert(inst.clone(), frame.clone());
        self.pending_insts.push((inst.clone(), frame));
        self.r.mk(Ty::Named(inst.clone(), a.to_vec()))
    }

    /// Result ctor synth for `let r: Result<T,E> = ok(v) / err(e)`
    fn emit_result_ctor(
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
            self.err(pos, "type mismatch: Result slot is a word but a float value was passed".to_string());
        }
        // object + default zero fields
        let (obj, _ot) = self.emit_new_obj(fw, inst, &Vec::new(), &Vec::new(), pos);
        // ok flag & payload / err pair
        let zi = fw.v();
        let zc = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zi));


        let okv = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", okv, if is_ok { 1 } else { 0 }));
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

    /// stmt text of the callee Identifier for diagnostics
    fn init_str2(callee: &Expr) -> String {
        match &callee.node {
            ExprNode::Ident(id) => id.clone(),
            _ => String::new(),
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

    /// structural surface compatibility (patch #22): equal-by-interning,
    /// nil (word 0) into anything, Opt target lenient (word view), dyn
    /// target accepts concrete class instances, element-wise arrays/maps.
    fn surface_compat(&self, a: &Ty, b: &Ty) -> bool {
        if a == b {
            return true;
        }
        match (a, b) {
            (_, Ty::Unit) => true,
            (Ty::Opt(..), _) => true,
            (Ty::Dyn(_), Ty::Named(..)) => true,
            (Ty::Array(x), Ty::Array(y)) => {
                self.surface_compat(self.r.get(*x), self.r.get(*y))
            }
            (Ty::Map(k1, v1), Ty::Map(k2, v2)) => {
                self.surface_compat(self.r.get(*k1), self.r.get(*k2))
                    && self.surface_compat(self.r.get(*v1), self.r.get(*v2))
            }
            (Ty::Named(p, _), Ty::Named(c, _)) => {
                // same mangled instance name covers Result_int_str([args] vs []),
                // else subclass-targets-superclass chain
                if p == c {
                    true
                } else {
                    self.class_chain_has(c, p)
                }
            }
            _ => false,
        }
    }

    /// does the superclass chain of `cls` include `base`? (is-a ranking)
    fn class_chain_has(&self, cls: &str, target: &str) -> bool {
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            if c == target {
                return true;
            }
            match self.classes.get(&c) {
                Some(ci) => cur = ci.superclass.clone(),
                None => return false,
            }
        }
        false
    }

    /// diagnostic-friendly surface name: composites spill their element kinds
    fn surface_name(&self, t: &Ty) -> String {
        match t {
            Ty::Array(e) => {
                format!("Array<{}>", sloth_frontend::ty::ty_name(self.r.get(*e)))
            }
            Ty::Map(k, v) => {
                format!(
                    "Map<{}, {}>",
                    sloth_frontend::ty::ty_name(self.r.get(*k)),
                    sloth_frontend::ty::ty_name(self.r.get(*v))
                )
            }
            other => sloth_frontend::ty::ty_name(other),
        }
    }

    /// plain-name assignment checked against the declared/inferred surface
    /// type recorded at declare time (patch #22): float target promotes int
    /// words; float value into non-float target diagnosed; structurally
    /// different i64-word surfaces (int/str/bool/class/array/map) diagnosed;
    /// nil (word 0) accepted into any non-float target.
    fn check_named_assign(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        dt: TyId,
        v: &str,
        vty: TyId,
        pos: &Pos,
    ) {
        if self.is_float(dt) {
            if self.is_float(vty) {
                fw.assign(name, v, true);
            } else {
                let cv = fw.v();
                fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                fw.assign(name, &cv, true);
            }
            return;
        }
        if self.is_float(vty) {
            let dtn = sloth_frontend::ty::ty_name(self.r.get(dt));
            self.err(
                pos,
                format!("type mismatch: cannot assign `float` to `{}` (`{}`)", dtn, name),
            );
            // keep IR parseable: store with the value's own float spelling
            fw.assign(name, v, true);
            return;
        }
        let dts = self.r.get(dt).clone();
        let vts = self.r.get(vty).clone();
        if self.surface_compat(&dts, &vts) {
            fw.assign(name, v, false);
        } else {
            let dtn = self.surface_name(&dts);
            let vtn = self.surface_name(&vts);
            self.err(
                pos,
                format!(
                    "type mismatch: cannot assign `{}` to `{}` variable `{}`",
                    vtn, dtn, name
                ),
            );
            fw.assign(name, v, false);
        }
    }

    /// does a class chain (cls + superclasses) implement trait `tr`?
    fn impl_chain_has(&self, cls: &str, tr: &str) -> bool {
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            match self.classes.get(&c) {
                Some(ci) => {
                    if ci.impls.iter().any(|x| x == tr) {
                        return true;
                    }
                    cur = ci.superclass.clone();
                }
                None => return false,
            }
        }
        false
    }

    /// predefined trait surface: builtin kinds satisfy these without declares
    fn is_predef_trait(bound: &str) -> bool {
        matches!(
            bound,
            "Hashable" | "Equatable" | "Comparable" | "Display"
        )
    }

    /// does a type satisfy the trait bound? (builtin kinds cover predefined
    /// traits; user classes need the impl chain; Opt looks through)
    fn satisfies_bound(&self, t: TyId, bound: &str) -> bool {
        match self.r.get(t).clone() {
            Ty::I64 | Ty::F64 | Ty::Str | Ty::Bool | Ty::Range | Ty::Array(_) | Ty::Map(_, _) => {
                Self::is_predef_trait(bound)
            }
            Ty::Named(cls, _) => self.impl_chain_has(&cls, bound),
            Ty::Opt(e) => self.satisfies_bound(e, bound),
            _ => false,
        }
    }

    /// typed-bound lint in builtin positions with a clearer message context
    fn satisfies_bound_check(&mut self, pos: &Pos, t: &TyId, bound: &str, ctx: &str) {
        let ok = self.satisfies_bound(*t, bound);
        if !ok {
            self.err(
                pos,
                format!(
                    "`{}` requires trait bound `{}` (`{}` does not satisfy it)",
                    ctx,
                    bound,
                    sloth_frontend::ty::ty_name(self.r.get(*t)),
                ),
            );
        }
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
                    self.globals.insert(d.name.clone(), (sym, t, d.kind == DeclKind::Var));
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
                    self.class_defs.insert(
                        d.name.clone(),
                        (self.cur_mod.clone(), (**c).clone()),
                    );
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
    }

    /// check a class's declared traits: known + every method satisfied
    /// by the class chain with the same arity (this param excluded).
    /// Also assigns global vtable slots, marks slot-resolved methods
    /// for llvm.func emission, and checks the call ABI (word kinds).
    fn check_impls(&mut self, cls: &str, impls: &[String], pos: &Pos) {
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
                                format!("trait `{}` method `{}` arity: want {}, `{}`.{} has {}",
                                    tr, m.name, want, defcls, m.name, got),
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
                                self.err(pos, format!(
                                    "trait `{}` method `{}` param {}: ABI word mismatch",
                                    tr, m.name, i + 1));
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
                            self.err(pos, format!(
                                "trait `{}` method `{}` return: ABI word mismatch",
                                tr, m.name));
                        }
                        self.llvm_method
                            .insert((defcls.clone(), m.name.clone()));
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
                                    self.err(pos, format!("trait `{}` impl on missing class `{}`", tr, cls));
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
                            format!("trait `{}` method `{}` not implemented by `{}`", tr, m.name, cls),
                        );
                    }
                }
            }
        }
    }

    /// (trait, method) -> slot; allocates on first sight
    fn vt_slot(&mut self, tr: &str, m: &str) -> usize {
        let key = (tr.to_string(), m.to_string());
        if let Some(&s) = self.vt_slots.get(&key) {
            return s;
        }
        let s = self.vt_slots.len();
        self.vt_slots.insert(key, s);
        s
    }

    /// float word? for a syntactic param/ret type (None / unit -> i64)
    fn sig_word_float(&self, t: Option<Type>) -> bool {
        match t {
            Some(ty) => !matches!(ty, Type::Unit) && matches!(ty, Type::Simple(SimpleType::Float)),
            None => false,
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
        let mangled = if let Some(m) = self.tp_mangled.last() {
            m.clone()
        } else {
            mangle(&self.cur_mod.clone(), cls, name)
        };
        FuncPlan { mangled, params, ret }
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
        // extern func: body-less declaration kept under its raw C-ABI name
        if f.is_extern {
            self.stat_extdecls += 1;
            self.emitted_names.push(name.to_string());
            self.out.push_str(&format!(
                "  func.func private @{}({}) -> {}\n",
                name,
                plan.params
                    .iter()
                    .map(|p| if self.is_float(p.1) { "f64" } else { "i64" })
                    .collect::<Vec<&str>>()
                    .join(", "),
                mlir_ret_ty(self, plan.ret),
            ));
            return plan.mangled;
        }
        // methods addressable by vtable slots are emitted as llvm.func
        let is_ll = cls
            .map(|c| self.llvm_method.contains(&(c.to_string(), name.to_string())))
            .unwrap_or(false);
        self.emitted_names.push(plan.mangled.clone());
        let retf = self.is_float(plan.ret);
        let mut fw = FnWalk {
            cur: String::new(),
            vcount: 1000,
            scopes: vec![HashMap::new()],
            imms: vec![HashMap::new()],
            loops: Vec::new(),
            ret: plan.ret,
            ret_alloca: String::new(),
            ret_flag: String::new(),
            bb: 0,
            term: false,
            end_label: "^end".to_string(),
            cur_cls: None,
        };
        fw.cur_cls = cls.map(|c| c.to_string());
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
        if is_ll {
            if self.is_unit(plan.ret) {
                fw.op("    llvm.return");
            } else {
                let rt = if retf { "f64" } else { "i64" };
                fw.op(&format!("    llvm.return {} : {}", retval.trim_start(), rt));
            }
        } else if self.is_unit(plan.ret) {
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
        } else if is_ll {
            self.out.push_str(&format!(
                "  llvm.func @{}({}) -> {} {{\n",
                plan.mangled, sigtxt, mlir_ret_ty(self, plan.ret)
            ));
        } else {
            self.out.push_str(&format!(
                "  func.func @{}({}) -> {} {{\n",
                plan.mangled, sigtxt, mlir_ret_ty(self, plan.ret)
            ));
        }
        // bare `call` is func-dialect sugar valid only in func.func regions;
        // inside llvm.func bodies it must be spelled func.call
        let entry_text = if is_ll { rename_plain_calls(&entry_text) } else { entry_text };
        let ret_text = if is_ll { rename_plain_calls(&ret_text) } else { ret_text };
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
            StmtNode::Let { mutable, name, ty, init } => {
                // typed(s) ok()/err() ctor fast-path: Result init (annotation
                // provides the T/E binding; absent annotation diagnosed)
                let pre: Option<(String, TyId)> = match &init.node {
                    ExprNode::Call { callee, args }
                        if args.len() == 1
                            && matches!(
                                callee.node,
                                ExprNode::Ident(ref id) if id == "ok" || id == "err",
                            ) =>
                    {
                        let inst = ty.as_ref().and_then(|te| {
                            let dt = self.ty_of(te);
                            match self.r.get(dt).clone() {
                                Ty::Named(nm, _) if self.result_insts.contains(&nm) => Some(nm),
                                _ => None,
                            }
                        });
                        match inst {
                            Some(inst) => {
                                let is_ok = matches!(
                                    &callee.node,
                                    ExprNode::Ident(ref id) if id == "ok"
                                );
                                Some(self.emit_result_ctor(fw, &inst, &args[0], is_ok, &init.pos))
                            }
                            None => {
                                self.err(
                                    &init.pos,
                                    format!(
                                        "ctor `{}` requires a declared Result target (let/var with Result<_, _>)",
                                        if Self::init_str2(callee) == "err" { "err" } else { "ok" }
                                    ),
                                );
                                None
                            }
                        }
                    }
                    _ => None,
                };
                let (v, t) = match &pre {
                    Some((w, tt)) => (w.clone(), *tt),
                    None => self.emit_expr(fw, init),
                };
                // declared `dyn T` / trait positions coerce the binding's type
                let t = match ty {
                    Some(te) => {
                        let dt = self.ty_of(te);
                        match self.r.get(dt) {
                            Ty::Dyn(_) => dt,
                            // declared Array<dyn T> coerces a list literal binding
                            Ty::Array(el) => match self.r.get(*el) {
                                Ty::Dyn(_) => match self.r.get(t) {
                                    Ty::Array(_) => dt,
                                    _ => t,
                                },
                                _ => t,
                            },
                            Ty::Named(n, _) if self.traits.contains_key(n.as_str()) => {
                                // only coerce actual object values (class instances)
                                match self.r.get(t) {
                                    Ty::Named(_, _) => self.r.mk(Ty::Dyn(n.clone())),
                                    _ => t,
                                }
                            }
                            _ => t,
                        }
                    }
                    None => t,
                };
                let _ = ty;
                // declared scalar-kind conflict check (MVP: word-family match)
                let (v, t) = match ty {
                    Some(te) => {
                        let dt = self.ty_of(te);
                        let df = self.is_float(dt);
                        let vf = self.is_float(t);
                        if df && !vf {
                            // promote the int word to f64 spelling
                            let cv = fw.v();
                            fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                            (cv, dt)
                        } else if !df && vf {
                            self.err(&s.pos, "type mismatch: initializer is float but declared type is not".to_string());
                            (v, t)
                        } else {
                            // declared word-surface check (patch #22):
                            // cross-kind i64-word declarations (int/str/bool/
                            // class/array/map) conflict structurally
                            let dts = self.r.get(dt).clone();
                            let vts = self.r.get(t).clone();
                            if !self.surface_compat(&dts, &vts) {
                                let dtn = self.surface_name(&dts);
                                let vtn = self.surface_name(&vts);
                                self.err(
                                    &s.pos,
                                    format!(
                                        "type mismatch: initializer is `{}` but declared type is `{}`",
                                        vtn, dtn
                                    ),
                                );
                            }
                            (v, t)
                        }
                    }
                    None => (v, t),
                };
                let fl = self.is_float(t);
                fw.declare(name, t, fl, *mutable);
                fw.assign(name, &v, fl);
            }
            StmtNode::Assign { target, value } => {
                let (mut v, vty) = self.emit_expr(fw, value);
                // super.x = v: store into an inherited field slot of this
                if let (Some(PathSeg::Name(h)), Some(PathSeg::Name(f))) = (target.first(), target.last()) {
                    if *h == "super" && target.len() == 2 {
                        match fw.cur_cls.clone() {
                            Some(cur) => {
                                let idx = self.field_index(&cur, f);
                                let (rv, _t) = self.emit_expr(fw, &Expr { pos: s.pos.clone(), node: ExprNode::This });
                                let zi = fw.v();
                                fw.op(&format!("    {} = arith.constant {} : i64", zi, idx));
                                fw.op(&format!(
                                    "    call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                                    rv, zi, v
                                ));
                                return;
                            }
                            None => self.err(&s.pos, "super.x assignment outside method".to_string()),
                        }
                    }
                }
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
                                // f64 field route: int words promote; float->i64 rejects
                                let fty = self
                                    .classes
                                    .get(c)
                                    .and_then(|ci| ci.fields.iter().find(|fd| fd.0 == *f))
                                    .map(|fd| fd.1)
                                    .unwrap_or_else(|| self.r.mk(Ty::I64));
                                let mut vc = v.clone();
                                if self.is_float(fty) && !self.is_float(vty) {
                                    let cv = fw.v();
                                    fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                                    vc = cv;
                                } else if !self.is_float(fty) && self.is_float(vty) {
                                    self.err(&s.pos, "type mismatch: cannot assign float to non-float field".to_string());
                                }
                                self.op_set_field(fw, &recv, &zi, &vc, fty, s.pos.clone());
                                return;
                            }
                        }
                    }
                }
                match target.last() {
                    Some(PathSeg::Name(n)) => {
                        if fw.imm_of(n) {
                            self.err(&s.pos, format!("cannot assign to immutable `{}` (declared with `let`)", n));
                        }
                        match fw.lookup(n) {
                            Some((_, dt)) => {
                                self.check_named_assign(fw, n, dt, &v, vty, &s.pos);
                            }
                            None => {
                                fw.assign(n, &v, false);
                            }
                        }
                    }
                    Some(PathSeg::Index(ix)) => {
                        // a[i] = v / m[k] = v (single-index MVP)
                        if target.len() != 2 {
                            self.err(&s.pos, "nested index assignment unsupported".to_string());
                            return;
                        }
                        if let Some(PathSeg::Name(h)) = target.first() {
                            match fw.lookup(h) {
                                Some((aa, at)) => {
                                    let ats = self.r.get(at).clone();
                                    let z = fw.v();
                                    fw.op(&format!("    {} = arith.constant 0 : index", z));
                                    let av = fw.v();
                                    fw.op(&format!(
                                        "    {} = memref.load {}[{}] : memref<1xi64>",
                                        av, aa, z
                                    ));
                                    let (iv, _it) = self.emit_expr(fw, ix);
                                    match ats {
                                        Ty::Array(el) => {
                                            if self.is_float(el) {
                                                if !self.is_float(vty) {
                                                    let cv = fw.v();
                                                    fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                                                    v = cv;
                                                }
                                                fw.op(&format!(
                                                    "    call @sloth_arr_set_f64({}, {}, {}) : (i64, i64, f64) -> i64",
                                                    av, iv, v
                                                ));
                                            } else {
                                                fw.op(&format!(
                                                    "    call @sloth_arr_set({}, {}, {}) : (i64, i64, i64) -> i64",
                                                    av, iv, v
                                                ));
                                            }
                                        }
                                        Ty::Map(k, v2) => {
                                            let kkind = matches!(self.r.get(k), Ty::Str);
                                            let vf = self.is_float(v2);
                                            let sym = match (kkind, vf) {
                                                (true, true) => "sloth_map_str_set_f64",
                                                (true, false) => "sloth_map_str_set",
                                                (false, true) => "sloth_map_set_f64",
                                                (false, false) => "sloth_map_set",
                                            };
                                            let vsig = if vf { "f64" } else { "i64" };
                                            if vf && !self.is_float(vty) {
                                                let cv = fw.v();
                                                fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                                                v = cv;
                                            }
                                            fw.op(&format!(
                                                "    call @{}({}, {}, {}) : (i64, i64, {}) -> i64",
                                                sym, av, iv, v, vsig
                                            ));
                                        }
                                        Ty::Named(cn, _) => {
                                            // Indexable overload: a[i] = v ≡ a.__assign__(i, v)
                                            match self.find_method(cn.as_str(), "__assign__") {
                                                Some((defcls, fd)) => {
                                                    let mut vc = v.clone();
                                                    let retf = fd.params.len() >= 3 && {
                                                        let pt = fd.params[2].ty.clone();
                                                        pt.as_ref()
                                                            .map(|t| {
                                                                let dt = self.ty_of(t);
                                                                self.is_float(dt)
                                                            })
                                                            .unwrap_or(false)
                                                    };
                                                    if retf && !self.is_float(vty) {
                                                        let cv = fw.v();
                                                        fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                                                        vc = cv;
                                                    }
                                                    let oargv = vec![
                                                        (av.clone(), at),
                                                        (iv.clone(), self.r.mk(Ty::I64)),
                                                        (vc, vty),
                                                    ];
                                                    let osig = vec![
                                                        mlir_word_ty(at, &self.r),
                                                        "i64".to_string(),
                                                        mlir_word_ty(vty, &self.r),
                                                    ];
                                                    self.emit_method_call(
                                                        fw, &defcls, "__assign__", &fd, false,
                                                        &oargv, &osig, &s.pos,
                                                    );
                                                }
                                                None => {
                                                    self.err(
                                                        &s.pos,
                                                        format!("class `{}` requires an `__assign__` overload for index assignment", cn),
                                                    );
                                                }
                                            }
                                        }
                                        _ => {
                                            self.err(&s.pos, "index assignment on non-array".to_string());
                                            return;
                                        }
                                    }
                                }
                                None => {
                                    self.err(&s.pos, format!("unknown array `{}`", h));
                                }
                            }
                        }
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
        // typed ok()/err() ctor fast-path in return position (patch #23):
        // the T/E binding comes from the function's declared Result<_ of,_>
        // return annotation
        let mut pre: Option<(String, TyId)> = None;
        if let ExprNode::Call { callee, args } = &e.node {
            if args.len() == 1
                && matches!(
                    callee.node,
                    ExprNode::Ident(ref id) if id == "ok" || id == "err",
                )
            {
                let inst = match self.r.get(fw.ret).clone() {
                    Ty::Named(nm, _) if self.result_insts.contains(&nm) => Some(nm),
                    _ => None,
                };
                match inst {
                    Some(inst) => {
                        let is_ok = matches!(
                            &callee.node,
                            ExprNode::Ident(ref id) if id == "ok"
                        );
                        pre = Some(self.emit_result_ctor(fw, &inst, &args[0], is_ok, &e.pos));
                    }
                    None => {
                        self.err(
                            &e.pos,
                            format!(
                                "ctor `{}` requires a declared Result target (Result<_, _> return annotation)",
                                if Self::init_str2(callee) == "err" { "err" } else { "ok" }
                            ),
                        );
                    }
                }
            }
        }
        let (v, t) = match pre {
            Some((w, tt)) => (w, tt),
            None => self.emit_expr(fw, e),
        };
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
    /// flow-typing MVP: `x is C` / `x is not nil` narrows x inside then-branch
    /// by shadowing its slot binding with the narrowed type
    fn narrow_pattern(&mut self, fw: &mut FnWalk, cond: &Expr) -> Option<(String, TyId)> {
        match &cond.node {
            ExprNode::Is { negated: false, lhs, rhs } => {
                let x = match &lhs.node {
                    ExprNode::Ident(n) => n.clone(),
                    _ => return None,
                };
                match &rhs.node {
                    ExprNode::Ident(cn) if self.class_ids.contains_key(cn.as_str()) => {
                        let nty = self.r.mk(Ty::Named(cn.to_string(), Vec::new()));
                        Some((x, nty))
                    }
                    _ => None,
                }
            }
            ExprNode::Is { negated: true, lhs, rhs } if matches!(&lhs.node, ExprNode::Ident(_)) => {
                if !matches!(&rhs.node, ExprNode::Nil) {
                    return None;
                }
                let x = match &lhs.node {
                    ExprNode::Ident(n) => n.clone(),
                    _ => return None,
                };
                let narrowed = match fw.lookup(&x).map(|(_a, t)| self.r.get(t).clone()) {
                    Some(Ty::Opt(e)) => e,
                    _ => return None,
                };
                Some((x, narrowed))
            }
            _ => None,
        }
    }

    fn walk_if(&mut self, fw: &mut FnWalk, cond: &Expr, then_: &Stmt, else_: Option<&Stmt>, pos: &Pos) {
        if !fw.noterm() {
            self.err(pos, "unreachable code".to_string());
            return;
        }
        let (c, _ct) = self.emit_expr(fw, cond);
        let narrow = if self.diags.is_empty() {
            self.narrow_pattern(fw, cond)
        } else {
            None
        };
        let thlab = fw.newlabel("t");
        let ellab = fw.newlabel("e");
        let endlab = fw.newlabel("fi");
        fw.cjump(&c, &thlab, &ellab);
        fw.label(&thlab);
        match narrow {
            Some((x, nty)) => {
                let slot = fw.lookup(&x).map(|(a, _)| a);
                if let Some(a) = slot {
                    fw.push_scope();
                    fw.scopes.last_mut().unwrap().insert(x.clone(), (a, nty));
                    self.walk_body(fw, then_);
                    fw.pop_scope();
                } else {
                    self.walk_body(fw, then_);
                }
            }
            None => self.walk_body(fw, then_),
        }
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
                // array/map iteration: for x in arr|map { ... } with a slotted counter
                let (mav, at) = self.emit_expr(fw, iter);
                let ats = self.r.get(at).clone();
                match &ats {
                    Ty::Array(e) => {
                        self.emit_index_loop(fw, var, body, mav, *e, IdxKind::Arr, pos);
                        return;
                    }
                    // map iteration MVP: for-in yields the key set (Array<K> route)
                    Ty::Map(k, _v) => {
                        let ks = fw.v();
                        fw.op(&format!(
                            "    {} = call @sloth_map_keys({}) : (i64) -> i64",
                            ks, mav
                        ));
                        self.emit_index_loop(fw, var, body, ks, *k, IdxKind::Arr, pos);
                        return;
                    }
                    // str iteration: per-char 1-byte strings
                    Ty::Str => {
                        { let et = self.r.mk(Ty::Str); self.emit_index_loop(fw, var, body, mav, et, IdxKind::StrChar, pos); }
                        return;
                    }
                    // iterator protocol: iterator object (or iter())/next() -> Opt<el>
                    Ty::Named(c, _) => {
                        self.emit_proto_loop(fw, var, body, mav.clone(), at.clone(), pos);
                        return;
                    }
                    _ => {
                        self.err(pos, "for-iteration requires a range, array, map, string, array-backed or iterator value".to_string());
                        return;
                    }
                };
            }
        }
    }

    fn emit_index_loop(
            &mut self,
            fw: &mut FnWalk,
            var: &str,
            body: &Stmt,
            arr: String,
            el: TyId,
            kind: IdxKind,
            pos: &Pos,
        ) {
            let (countfn, getfn, getty) = match kind {
                IdxKind::Arr => {
                    if self.is_float(el) {
                        ("sloth_arr_len", "sloth_arr_get_f64", "(i64, i64) -> f64")
                    } else {
                        ("sloth_arr_len", "sloth_arr_get", "(i64, i64) -> i64")
                    }
                }
                IdxKind::StrChar => ("sloth_str_len", "sloth_str_char", "(i64, i64) -> i64"),
            };
            let lenv = fw.v();
            fw.op(&format!("    {} = call @{}({}) : (i64) -> i64", lenv, countfn, arr));
            fw.push_scope();
            let islot = fw.v();
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", islot));
            let zi2 = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", zi2));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                zi2, islot, z
            ));
            let head = fw.newlabel("af");
            let doo = fw.newlabel("ab");
            let done = fw.newlabel("ae");
            fw.jump(&head);
            fw.label(&head);
            let iv = fw.v();
            fw.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                iv, islot, z
            ));
            let c = fw.v();
            fw.op(&format!("    {} = arith.cmpi slt, {}, {} : i64", c, iv, lenv));
            let c1 = fw.v();
            fw.op(&format!("    {} = arith.extsi {} : i1 to i64", c1, c));
            fw.cjump(&c1, &doo, &done);
            fw.label(&doo);
            fw.loops.push((done.clone(), head.clone()));
            // loop var = seq[i]
            let gtv = fw.v();
            fw.op(&format!(
                "    {} = call @{}({}, {}) : {}",
                gtv, getfn, arr, iv, getty
            ));
            let gety = if kind == IdxKind::StrChar { self.r.mk(Ty::Str) } else { el };
            let vs = fw.v();
            if self.is_float(gety) {
                fw.op(&format!("    {} = memref.alloca() : memref<1xf64>", vs));
                fw.op(&format!("    memref.store {}, {}[{}] : memref<1xf64>", gtv, vs, z));
            } else {
                fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
                fw.op(&format!("    memref.store {}, {}[{}] : memref<1xi64>", gtv, vs, z));
            }
            fw.scopes.last_mut().unwrap().insert(var.to_string(), (vs, gety));
            self.walk_body(fw, body);
            fw.loops.pop();
            // idx += 1
            let one2 = fw.v();
            fw.op(&format!("    {} = arith.constant 1 : i64", one2));
            let nx = fw.v();
            fw.op(&format!("    {} = arith.addi {}, {} : i64", nx, iv, one2));
            fw.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                nx, islot, z
            ));
            fw.jump(&head);
            fw.label(&done);
            fw.pop_scope();
        }

    /// iterator protocol: iterator object (or it.iter())/next() -> Opt<el>
        /// (worded MVP: Opt as i64 word, 0 = nil; float elements rejected)
        fn emit_proto_loop(
            &mut self,
            fw: &mut FnWalk,
            var: &str,
            body: &Stmt,
            recv: String,
            recvty: TyId,
            pos: &Pos,
        ) {
            // normalize: value itself an iterator, or expose iter() first
            let cls = match self.r.get(recvty) {
                Ty::Named(c, _) => c.clone(),
                _ => {
                    self.err(&pos, "for-iteration requires a range, array, map or iterator value".to_string());
                    return;
                }
            };
            let (itv, itty);
            if let Some((defcls, fd)) = self.find_method(&cls, "iter") {
                let (v, t) = self.emit_method_call(
                    fw, &defcls, "iter", &fd, false,
                    &vec![(recv.clone(), recvty.clone())],
                    &vec!["i64".to_string()], &pos,
                );
                if let Ty::Named(c, _) = self.r.get(t) {
                    itv = v;
                    itty = t;
                    _ = c;
                } else {
                    self.err(&pos, "iter() must return an iterator object".to_string());
                    return;
                }
            } else {
                itv = recv;
                itty = recvty;
            }
            let icls = match self.r.get(itty) {
                Ty::Named(c, _) => c.clone(),
                _ => unreachable!(),
            };
            let nfd = self.find_method(&icls, "next");
            if nfd.is_none() {
                self.err(&pos, format!("class `{}` has no `next()` (iterator protocol)", icls));
                return;
            }
            let (ndefcls, nfn) = nfd.unwrap();
            let nplan = self.plan_for_class("next", &ndefcls, &nfn);
            let el = match self.r.get(nplan.ret) {
                Ty::Opt(e) => *e,
                _ => {
                    self.err(&pos, format!("iterator `next()` must return Opt (class `{}`)", icls));
                    return;
                }
            };
            if self.is_float(el) {
                self.err(&pos, "iterator element type float unsupported (MVP)".to_string());
                return;
            }
            // iterator slot storage
            let islot = fw.v();
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", islot));
            fw.op(&format!("    memref.store {}, {}[{}] : memref<1xi64>", itv, islot, z));
            fw.push_scope();
            let head = fw.newlabel("if");
            let doo = fw.newlabel("ib");
            let done = fw.newlabel("ie");
            fw.jump(&head);
            fw.label(&head);
            let itw = fw.v();
            fw.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                itw, islot, z
            ));
            let (ov, _ot) = self.emit_method_call(
                fw, &ndefcls, "next", &nfn, false,
                &vec![(itw, itty.clone())],
                &vec!["i64".to_string()], &pos,
            );
            let nc = fw.v();
            let znil = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", znil));
            fw.op(&format!(
                "    {} = arith.cmpi eq, {}, {} : i64",
                nc, ov, znil
            ));
            let nc1 = fw.v();
            fw.op(&format!("    {} = arith.extsi {} : i1 to i64", nc1, nc));
            fw.cjump(&nc1, &done, &doo);
            fw.label(&doo);
            fw.loops.push((done.clone(), head.clone()));
            // loop var = unwrap(next())
            let vs = fw.v();
            fw.op(&format!("    {} = memref.alloca() : memref<1xi64>", vs));
            fw.op(&format!("    memref.store {}, {}[{}] : memref<1xi64>", ov, vs, z));
            fw.scopes.last_mut().unwrap().insert(var.to_string(), (vs, el));
            self.walk_body(fw, body);
            fw.loops.pop();
            fw.jump(&head);
            fw.label(&done);
            fw.pop_scope();
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
    fn find_method(&self, cls: &str, name: &str) -> Option<(String, FuncDef)> {
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
                let mut s = format!("{}", v);
                if !s.contains('.') && !s.contains('e') && !s.contains("inf") && !s.contains("nan") {
                    s.push_str(".0");
                }
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
                if !ss.is_plain() {
                    // interpolated string: chain per-part push onto a builder
                    let mut curw = String::new();
                    let c0 = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", c0));
                    curw = c0;
                    for part in &ss.parts {
                        match part {
                            StrPart::Lit(l) => {
                                let bytes: Vec<u8> = l.bytes().collect();
                                let mut pads = bytes.clone();
                                while !pads.is_empty() && pads.len() % 8 != 0 {
                                    pads.push(0);
                                }
                                if pads.is_empty() {
                                    continue;
                                }
                                let blen = bytes.len();
                                for (ci_, ch) in pads.chunks(8).enumerate() {
                                    let mut w: u64 = 0;
                                    for (k, b) in ch.iter().enumerate() {
                                        w |= (*b as u64) << (8 * k);
                                    }
                                    let c1 = fw.v();
                                    fw.op(&format!("    {} = arith.constant {} : i64", c1, w as i64));
                                    let tail = std::cmp::min(8usize, blen - ci_ * 8);
                                    let c2 = fw.v();
                                    fw.op(&format!("    {} = arith.constant {} : i64", c2, tail as i64));
                                    let r = fw.v();
                                    fw.op(&format!(
                                        "    {} = call @sloth_str_push({}, {}, {}) : (i64, i64, i64) -> i64",
                                        r, curw, c1, c2
                                    ));
                                    curw = r;
                                }
                            }
                            StrPart::ExprAst(e) => {
                                let (v, t) = self.emit_expr(fw, e);
                                let r = fw.v();
                                if self.is_str(t) {
                                    fw.op(&format!(
                                        "    {} = call @sloth_str_pushp({}, {}) : (i64, i64) -> i64",
                                        r, curw, v
                                    ));
                                } else if self.is_float(t) {
                                    fw.op(&format!(
                                        "    {} = call @sloth_str_push_f({}, {}) : (i64, f64) -> i64",
                                        r, curw, v
                                    ));
                                } else if self.r.get(t) == &Ty::Bool {
                                    fw.op(&format!(
                                        "    {} = call @sloth_str_push_b({}, {}) : (i64, i64) -> i64",
                                        r, curw, v
                                    ));
                                } else if let Ty::Named(ref cls, _) = self.r.get(t).clone() {
                                    // Display-plumbed interpolation, symmetric with
                                    // print: user class needs impl Display + to_str()
                                    self.satisfies_bound_check(&e.pos, &t, "Display", "interpolation");
                                    match self.find_method(cls, "to_str") {
                                        Some((defcls, fd)) => {
                                            let (sv, _st) = self.emit_method_call(
                                                fw, &defcls, "to_str", &fd, false,
                                                &vec![(v.clone(), t)],
                                                &vec!["i64".to_string()], &e.pos,
                                            );
                                            fw.op(&format!(
                                                "    {} = call @sloth_str_pushp({}, {}) : (i64, i64) -> i64",
                                                r, curw, sv
                                            ));
                                        }
                                        None => {
                                            self.err(
                                                &e.pos,
                                                "`${}` on class requires impl Display with `to_str`".to_string(),
                                            );
                                            fw.op(&format!(
                                                "    {} = call @sloth_str_push_i({}, {}) : (i64, i64) -> i64",
                                                r, curw, v
                                            ));
                                        }
                                    }
                                } else {
                                    fw.op(&format!(
                                        "    {} = call @sloth_str_push_i({}, {}) : (i64, i64) -> i64",
                                        r, curw, v
                                    ));
                                }
                                curw = r;
                            }
                            _ => {}
                        }
                    }
                    let fin = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_str_finish({}) : (i64) -> i64",
                        fin, curw
                    ));
                    let t = self.r.mk(Ty::Str);
                    return (fin, t);
                }
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
            ExprNode::Lambda(l) => {
                return self.emit_lambda(fw, l, &e.pos);
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
            ExprNode::Is { negated, lhs, rhs } => {
                let (lv, lt) = self.emit_expr(fw, lhs);
                // nil test
                if let ExprNode::Nil = &rhs.node {
                    let zc = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", zc));
                    let c = fw.v();
                    fw.op(&format!("    {} = arith.cmpi eq, {}, {} : i64", c, lv, zc));
                    let c1 = fw.v();
                    fw.op(&format!("    {} = arith.extsi {} : i1 to i64", c1, c));
                    let r = if *negated {
                        let one = fw.v();
                        let o = fw.v();
                        fw.op(&format!("    {} = arith.constant 1 : i64", one));
                        fw.op(&format!("    {} = arith.xori {}, {} : i64", o, c1, one));
                        o
                    } else {
                        c1
                    };
                    return (r, self.r.mk(Ty::I64));
                }
                // class membership test via static ancestor chain of cls ids
                if let ExprNode::Ident(cn) = &rhs.node {
                    if self.class_ids.get(cn).is_none() {
                        self.err(&e.pos, format!("`is` type `{}` not a known class", cn));
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    if self.is_float(lt) {
                        self.err(&e.pos, "`is` on float is unsupported".to_string());
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    // membership set: cn and every class whose ancestor chain reaches cn
                    let mut idsv: Vec<i64> = Vec::new();
                    if let Some(id) = self.class_ids.get(cn) {
                        idsv.push(*id);
                    }
                    for candv in self.class_order.clone() {
                        let cand = candv.clone();
                        let mut cur = Some(cand.clone());
                        while let Some(pn) = cur {
                            cur = self
                                .classes
                                .get(&pn)
                                .and_then(|ci| ci.superclass.clone());
                            if cur.as_deref() == Some(cn.as_str()) {
                                if let Some(id) = self.class_ids.get(&cand.clone()) {
                                    idsv.push(*id);
                                }
                                break;
                            }
                        }
                    }
                    if idsv.is_empty() {
                        self.err(&e.pos, "`is` chain unavailable".to_string());
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    // or-chain of cmpi eq over cls ids
                    let mut acc: Option<String> = None;
                    for id in &idsv {
                        let ci = fw.v();
                        fw.op(&format!("    {} = arith.constant {} : i64", ci, id));
                        let clsid = fw.v();
                        fw.op(&format!(
                            "    {} = call @sloth_obj_cls_id({}) : (i64) -> i64",
                            clsid, lv
                        ));
                        let eq = fw.v();
                        fw.op(&format!("    {} = arith.cmpi eq, {}, {} : i64", eq, clsid, ci));
                        let eq1 = fw.v();
                        fw.op(&format!("    {} = arith.extsi {} : i1 to i64", eq1, eq));
                        acc = match acc {
                            None => Some(eq1),
                            Some(a) => {
                                let o = fw.v();
                                fw.op(&format!("    {} = arith.ori {}, {} : i64", o, a, eq1));
                                Some(o)
                            }
                        };
                    }
                    let base = acc.unwrap();
                    let r = if *negated {
                        let one = fw.v();
                        let o = fw.v();
                        fw.op(&format!("    {} = arith.constant 1 : i64", one));
                        fw.op(&format!("    {} = arith.xori {}, {} : i64", o, base, one));
                        o
                    } else {
                        base
                    };
                    return (r, self.r.mk(Ty::Bool));
                }
                self.err(&e.pos, "unsupported `is` right side".to_string());
                return (String::new(), self.r.mk(Ty::Unit));
            }
            ExprNode::Elvis { lhs, rhs } => {
                let (lv, lt) = self.emit_expr(fw, lhs);
                let (rv, _rt) = self.emit_expr(fw, rhs);
                if self.is_float(lt) {
                    self.err(&e.pos, "`?:` on float is unsupported".to_string());
                    return (String::new(), self.r.mk(Ty::Unit));
                }
                let zc = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", zc));
                let c = fw.v();
                fw.op(&format!("    {} = arith.cmpi ne, {}, {} : i64", c, lv, zc));
                let r = fw.v();
                fw.op(&format!("    {} = arith.select {}, {}, {} : i64", r, c, lv, rv));
                (r, lt)
            }
            ExprNode::Arith { op, lhs, rhs } => {
                let (a, at) = self.emit_expr(fw, lhs);
                let (b, bt) = self.emit_expr(fw, rhs);
                // operator overload: class receiver dispatches __add__ etc;
                // carry the rhs word as payload (a + b ≡ a.__op__(b))
                if let Ty::Named(cls, _) = self.r.get(at).clone() {
                    let oname = match op {
                        ArithOp::Add => "__add__",
                        ArithOp::Sub => "__sub__",
                        ArithOp::Mul => "__mul__",
                        ArithOp::Div => "__div__",
                        ArithOp::Mod => "__mod__",
                    };
                    if let Some((defcls, fd)) = self.find_method(&cls, oname) {
                        let oargv = vec![(a.clone(), at), (b.clone(), bt)];
                        let osig = vec![
                            mlir_word_ty(at, &self.r),
                            mlir_word_ty(bt, &self.r),
                        ];
                        return self.emit_method_call(fw, &defcls, oname, &fd, false, &oargv, &osig, &e.pos);
                    }
                    self.err(
                        &e.pos,
                        format!("operator `{:?}` on class `{}` requires a `{}` overload", op, cls, oname),
                    );
                    return (String::new(), self.r.mk(Ty::Unit));
                }
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
                    // promote int operands to f64 for float ops
                    let a = if self.is_float(at) { a } else {
                        let cv = fw.v();
                        fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, a));
                        cv
                    };
                    let b = if self.is_float(bt) { b } else {
                        let cv = fw.v();
                        fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, b));
                        cv
                    };
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
            ExprNode::Pipe { lhs, rhs } => {
                // x |> f  ≡ f(x); x |> f(a, b) ≡ f(a, b, x) — x goes last
                match &rhs.node {
                    ExprNode::Ident(_) | ExprNode::Call { .. } => {
                        let mut args2: Vec<Expr> = Vec::new();
                        let callee: Box<Expr> = match &rhs.node {
                            ExprNode::Call { callee, args } => {
                                args2.extend(args.iter().cloned());
                                callee.as_ref().clone().into()
                            }
                            _ => (**rhs).clone().into(),
                        };
                        args2.push((**lhs).clone());
                        self.emit_call(fw, &callee, &args2, &e.pos, None)
                    }
                    _ => {
                        self.err(&e.pos, "pipe rhs must be a function or call".to_string());
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", z));
                        (z, self.r.mk(Ty::Unit))
                    }
                }
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
                        // operator overload: -x on a class receiver dispatches __neg__
                        if let Ty::Named(cls, _) = self.r.get(t).clone() {
                            if let Some((defcls, fd)) = self.find_method(&cls, "__neg__") {
                                let oargv = vec![(v.clone(), t)];
                                let osig = vec![mlir_word_ty(t, &self.r)];
                                return self.emit_method_call(
                                    fw, &defcls, "__neg__", &fd, false, &oargv, &osig, &e.pos,
                                );
                            }
                        }
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
        // operator overload: comparison family dispatches __gt__/__eq__ etc
        // on a class receiver (a < b ≡ a.__lt__(b)); result is the method's
        // own type (conventionally bool). No overload keeps the numeric path.
        if matches!(
            op,
            BinOp::EqEq | BinOp::NotEq | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge
        ) {
            if let Ty::Named(cls, _) = self.r.get(at).clone() {
                let oname = match op {
                    BinOp::EqEq => "__eq__",
                    BinOp::NotEq => "__ne__",
                    BinOp::Lt => "__lt__",
                    BinOp::Le => "__le__",
                    BinOp::Gt => "__gt__",
                    BinOp::Ge => "__ge__",
                    _ => unreachable!(),
                };
                if let Some((defcls, fd)) = self.find_method(&cls, oname) {
                    let oargv = vec![(a.clone(), at), (b.clone(), bt)];
                    let osig = vec![
                        mlir_word_ty(at, &self.r),
                        mlir_word_ty(bt, &self.r),
                    ];
                    return self.emit_method_call(fw, &defcls, oname, &fd, false, &oargv, &osig, pos);
                }
            }
        }
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
                return self.emit_call(fw, callee, args, &e.pos, None);
            }
            ExprNode::GenCall { callee, targs, args } => {
                return self.emit_call(fw, callee, args, &e.pos, Some(targs));
            }
            ExprNode::Field { obj, name } => {
                // qualified foreign-global read: lib.g / lib.Cls.f handled in arith path only for globals
                if let ExprNode::Ident(m) = &obj.node {
                    self.guard_hidden(m, name, &e.pos);
                    let key = format!("{}.{}", m, name);
                    if let Some((g, gt)) = self.fglobals.get(&key).cloned() {
                        return self.emit_global_read(fw, &g, gt);
                    }
                }
                let (recv, rt) = self.emit_expr(fw, obj);
                if let Ty::Named(c, _) = self.r.get(rt) {
                    if self.extern_types.contains(c.as_str()) {
                        self.err(
                            &e.pos,
                            format!("extern type `{}` is opaque (cannot access fields)", c),
                        );
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    let idx = self.field_index(c, name);
                    let zi = fw.v();
                    fw.op(&format!("    {} = arith.constant {} : i64", zi, idx));
                    let fty = self
                        .classes
                        .get(c)
                        .and_then(|ci| ci.fields.iter().find(|f| f.0 == *name))
                        .map(|f| f.1);
                    let fty2 = match fty { Some(x) => x, None => self.r.mk(Ty::I64) };
                    match self.r.get(fty2) {
                        Ty::Str => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, self.r.mk(Ty::Str));
                        }
                        Ty::F64 => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = call @sloth_obj_field_f64({}, {}) : (i64, i64) -> f64",
                                r, recv, zi
                            ));
                            return (r, fty2);
                        }
                        Ty::Named(_, _) | Ty::Dyn(_) | Ty::Array(_) | Ty::Map(..)
                        | Ty::Bool => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, fty2);
                        }
                        _ => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, self.r.mk(Ty::I64));
                        }
                    }
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
            ExprNode::List(xs) => {
                // array literal: fixed-length gc allocation of i64/f64 words
                let mut evs: Vec<String> = Vec::new();
                let mut ets: Vec<TyId> = Vec::new();
                for x in xs {
                    let (v, t) = self.emit_expr(fw, x);
                    evs.push(v);
                    ets.push(t);
                }
                let anyf = ets.iter().any(|t| self.is_float(*t));
                if anyf {
                    for (v, t) in evs.iter_mut().zip(ets.iter_mut()) {
                        if !self.is_float(*t) {
                            let cv = fw.v();
                            fw.op(&format!(
                                "    {} = arith.sitofp {} : i64 to f64",
                                cv, v.clone()
                            ));
                            *v = cv;
                            *t = self.r.mk(Ty::F64);
                        }
                    }
                }
                let n = fw.v();
                fw.op(&format!("    {} = arith.constant {} : i64", n, evs.len()));
                let arr = fw.v();
                fw.op(&format!(
                    "    {} = call @sloth_arr_new({}) : (i64) -> i64",
                    arr, n
                ));
                for (i, v) in evs.iter().enumerate() {
                    let zi = fw.v();
                    fw.op(&format!("    {} = arith.constant {} : i64", zi, i));
                    if anyf {
                        fw.op(&format!(
                            "    call @sloth_arr_set_f64({}, {}, {}) : (i64, i64, f64) -> i64",
                            arr, zi, v
                        ));
                    } else {
                        fw.op(&format!(
                            "    call @sloth_arr_set({}, {}, {}) : (i64, i64, i64) -> i64",
                            arr, zi, v
                        ));
                    }
                }
                let ty = if !ets.is_empty() && {
                    let first = *ets.first().unwrap();
                    ets.iter().all(|t| self.r.get(*t) == self.r.get(first))
                } {
                    self.r.mk(Ty::Array(ets[0]))
                } else if anyf {
                    let ef = self.r.mk(Ty::F64);
                    self.r.mk(Ty::Array(ef))
                } else {
                    let ei = self.r.mk(Ty::I64);
                    self.r.mk(Ty::Array(ei))
                };
                (arr, ty)
            }
            ExprNode::Map(pairs) => {
                // map literal: reproducible open-addressing rt table
                let mut kevs: Vec<(String, TyId)> = Vec::new();
                let mut vevs: Vec<(String, TyId)> = Vec::new();
                for (k, v) in pairs {
                    let (kv, kt) = self.emit_expr(fw, k);
                    let (vv, vt) = self.emit_expr(fw, v);
                    kevs.push((kv, kt));
                    vevs.push((vv, vt));
                }
                // key kind: str handles vs i64 words (uniform family check)
                let kvm: Vec<TyId> = kevs.iter().map(|x| x.1).collect();
                let anyk_str = kvm.iter().any(|t| self.is_str(*t));
                let anyk_obj = if anyk_str {
                    false
                } else {
                    kvm.iter()
                        .any(|t| matches!(self.r.get(*t).clone(), Ty::Named(_, _)))
                };
                if kvm.iter().any(|t| self.is_float(*t)) {
                    self.err(&e.pos, "map keys must be int, str or Hashable".to_string());
                }
                if anyk_str && kvm.iter().any(|t| !self.is_str(*t)) {
                    self.err(&e.pos, "mixed map key types".to_string());
                }
                if anyk_obj && kvm.iter().any(|t| !matches!(self.r.get(*t).clone(), Ty::Named(_, _))) {
                    self.err(&e.pos, "mixed map key types".to_string());
                }
                if anyk_obj {
                    for t in kvm.iter() {
                        match self.r.get(*t).clone() {
                            Ty::Named(cls, _) => {
                                if !self.impl_chain_has(&cls, "Hashable") {
                                    self.err(&e.pos, format!("map key `{}` does not implement Hashable", cls));
                                }
                            }
                            _ => {}
                        }
                    }
                }
                let kty = if anyk_str {
                    self.r.mk(Ty::Str)
                } else if anyk_obj {
                    kvm[0]
                } else {
                    self.r.mk(Ty::I64)
                };
                // value kind: unify int/float like list literals (float wins)
                let anyf = vevs.iter().any(|x| self.is_float(x.1));
                if anyf {
                    for x in vevs.iter_mut() {
                        if !self.is_float(x.1) {
                            let cv = fw.v();
                            fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, x.0));
                            x.0 = cv;
                            x.1 = self.r.mk(Ty::F64);
                        }
                    }
                }
                let vty = match vevs.first() {
                    Some(x) if vevs.iter().all(|y| self.r.get(y.1) == self.r.get(x.1)) => x.1,
                    _ => {
                        if anyf {
                            self.r.mk(Ty::F64)
                        } else {
                            self.r.mk(Ty::I64)
                        }
                    }
                };
                let kk = if anyk_str { 1i64 } else if anyk_obj { 2i64 } else { 0i64 };
                let kv0 = fw.v();
                fw.op(&format!("    {} = arith.constant {} : i64", kv0, kk));
                let m = fw.v();
                fw.op(&format!(
                    "    {} = call @sloth_map_new({}) : (i64) -> i64",
                    m, kv0
                ));
                for (kev, vev) in kevs.iter().zip(vevs.iter()) {
                    let (setsym, vsig) = match (anyk_str, anyf) {
                        (true, true) => ("sloth_map_str_set_f64", "f64"),
                        (true, false) => ("sloth_map_str_set", "i64"),
                        (false, true) => ("sloth_map_set_f64", "f64"),
                        (false, false) => ("sloth_map_set", "i64"),
                    };
                    fw.op(&format!(
                        "    call @{}({}, {}, {}) : (i64, i64, {}) -> i64",
                        setsym, m, kev.0, vev.0, vsig
                    ));
                }
                (m, self.r.mk(Ty::Map(kty, vty)))
            }
            ExprNode::Index { obj, idx } => {
                let (av, at) = self.emit_expr(fw, obj);
                let (iv, it) = self.emit_expr(fw, idx);
                let _ = it;
                let ats = self.r.get(at).clone();
                let (el, getsym, retty) = match &ats {
                    Ty::Array(e) => {
                        if self.is_float(*e) {
                            (*e, "sloth_arr_get_f64", "f64")
                        } else {
                            (*e, "sloth_arr_get", "i64")
                        }
                    }
                    Ty::Map(k, v) => {
                        let kkind = matches!(self.r.get(*k), Ty::Str);
                        let _ = it;
                        let (sym, retty) = match (kkind, self.is_float(*v)) {
                            (true, true) => ("sloth_map_str_get_f64", "f64"),
                            (true, false) => ("sloth_map_str_get", "i64"),
                            (false, true) => ("sloth_map_get_f64", "f64"),
                            (false, false) => ("sloth_map_get", "i64"),
                        };
                        (*v, sym, retty)
                    }
                    Ty::Named(cn, _) => {
                        // Indexable overload: a[i] ≡ a.__index__(i); element
                        // type is the method's own return type
                        match self.find_method(cn, "__index__") {
                            Some((defcls, fd)) => {
                                let oargv = vec![(av.clone(), at), (iv.clone(), it)];
                                let osig = vec![
                                    mlir_word_ty(at, &self.r),
                                    mlir_word_ty(it, &self.r),
                                ];
                                return self.emit_method_call(
                                    fw, &defcls, "__index__", &fd, false, &oargv, &osig, &e.pos,
                                );
                            }
                            None => {
                                self.err(
                                    &e.pos,
                                    format!("class `{}` requires an `__index__` overload for indexing", cn),
                                );
                                (self.r.mk(Ty::Unit), "sloth_arr_get", "i64")
                            }
                        }
                    }
                    _ => {
                        self.err(&e.pos, format!("indexing non-array"));
                        (self.r.mk(Ty::Unit), "sloth_arr_get", "i64")
                    }
                };
                let r = fw.v();
                fw.op(&format!(
                    "    {} = call @{}({}, {}) : (i64, i64) -> {}",
                    r, getsym, av, iv, retty
                ));
                (r, el)
            }
            ExprNode::This | ExprNode::Super => match fw.scopes.first().and_then(|sc| sc.get("this")).map(|s| (
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
        targs_in: Option<&Vec<sloth_frontend::ast::Type>>,
    ) -> (String, TyId) {
        // explicit type args on a bare-name callee: generic class Ctor or
        // fully general monomorphized function call (patch #19)
        if let Some(ta) = targs_in {
            if let ExprNode::Ident(base) = &callee.node {
                let cdef = self.class_defs.get(base).cloned();
                let fd = self.funcs.get(base).cloned();
                let ty_len = cdef.as_ref().map(|(_, c)| c.type_params.len());
                let fd_len = fd.as_ref().map(|f| f.type_params.len());
                if ty_len == Some(ta.len()) {
                    // generic class ctor: register/fetch instance, then default ctor
                    let tys: Vec<TyId> = ta.iter().map(|t| self.ty_of(t)).collect();
                    let it = self.declare_class_inst(base, &tys);
                    let iname = match self.r.get(it) { Ty::Named(n, _) => n.clone(), _ => String::new() };
                    if !iname.is_empty() {
                        let cargs: Vec<(String, TyId)> = args
                            .iter()
                            .map(|a| self.emit_expr(fw, a))
                            .collect();
                        return self.emit_new_obj(fw, &iname, &cargs, &Vec::new(), pos);
                    }
                }
                // generic function call with explicit type args
                if let Some(fd) = &fd {
                    if fd_len == Some(ta.len()) && fd.type_params.len() == ta.len() {
                        let tnames: Vec<String> =
                            fd.type_params.iter().map(|p| p.name.clone()).collect();
                        let mut map: std::collections::HashMap<String, TyId> =
                            std::collections::HashMap::new();
                        for (tp, tt) in fd.type_params.iter().zip(ta.iter()) {
                            map.insert(tp.name.clone(), self.ty_of(tt));
                        }
                        let _ = tnames;
                        let argv: Vec<(String, TyId)> = args
                            .iter()
                            .map(|a| self.emit_expr(fw, a))
                            .collect();
                        return self.emit_ginst_call(fw, base, fd, &argv, pos, map);
                    }
                }
            }
        }
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
                self.guard_hidden(m, mname2, pos);
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
                    if self.is_unit(fs.1) {
                        fw.op(&format!(
                            "    call @{}({}) : ({}) -> ()",
                            fs.0, vals.join(", "), csig.join(", ")
                        ));
                        return (String::new(), fs.1);
                    }
                    fw.op(&format!(
                        "    {} = call @{}({}) : ({}) -> {}",
                        r, fs.0, vals.join(", "), csig.join(", "), rt
                    ));
                    return (r, fs.1);
                }
                if let Some((g, gt)) = self.fglobals.get(&key).cloned() {
                    return self.emit_global_read(fw, &g, gt);
                }
                // qualified constructor: lib.Cls(args)
                if self.classes.contains_key(mname2.as_str())
                    && self.class_ids.contains_key(mname2.as_str())
                {
                    self.guard_hidden(m, mname2, pos);
                    let mut cargv: Vec<(String, TyId)> = Vec::new();
                    for a in args {
                        let (v, t) = self.emit_expr(fw, a);
                        cargv.push((v, t));
                    }
                    return self.emit_new_obj(fw, mname2, &cargv, &Vec::new(), pos);
                }
            }
            let is_super = matches!(&obj.node, ExprNode::Super);
            if is_super && fw.cur_cls.is_none() {
                self.err(pos, "super outside method".to_string());
            }
            let (rv, rt) = self.emit_expr(fw, obj);
            argv.insert(0, (rv.clone(), rt));
            sigargs.insert(0, "i64".to_string());
            recv = Some((rv, rt));
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
        // builtin container methods: a.push(v) / a.pop() / a.len()
        if let Some((recvv, rt)) = recv.clone() {
            if let Ty::Map(_k, _v) = self.r.get(rt) {
                if name == "len" {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_map_len({}) : (i64) -> i64",
                        r, recvv
                    ));
                    return (r, self.r.mk(Ty::I64));
                }
            }
            if let Ty::Array(el) = self.r.get(rt) {
                let fel = self.is_float(*el);
                match name.as_str() {
                    "push" => {
                        let (mut v, at) = match argv.get(1).cloned() {
                            Some(x) => (x.0, x.1),
                            None => {
                                self.err(pos, "push requires one argument".to_string());
                                return (String::new(), self.r.mk(Ty::Unit));
                            }
                        };
                        if fel && !self.is_float(at) {
                            let cv = fw.v();
                            fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                            v = cv;
                        }
                        let callv = fw.v();
                        if fel {
                            fw.op(&format!(
                                "    {} = call @sloth_arr_push_f64({}, {}) : (i64, f64) -> i64",
                                callv, recvv, v
                            ));
                        } else {
                            fw.op(&format!(
                                "    {} = call @sloth_arr_push({}, {}) : (i64, i64) -> i64",
                                callv, recvv, v
                            ));
                        }
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", z));
                        return (z, self.r.mk(Ty::Unit));
                    }
                    "pop" => {
                        let r = fw.v();
                        if fel {
                            fw.op(&format!(
                                "    {} = call @sloth_arr_pop_f64({}) : (i64) -> f64",
                                r, recvv
                            ));
                        } else {
                            fw.op(&format!(
                                "    {} = call @sloth_arr_pop({}) : (i64) -> i64",
                                r, recvv
                            ));
                        }
                        return (r, *el);
                    }
                    "len" => {
                        let r = fw.v();
                        fw.op(&format!(
                            "    {} = call @sloth_arr_len({}) : (i64) -> i64",
                            r, recvv
                        ));
                        return (r, self.r.mk(Ty::I64));
                    }
                    _ => {}
                }
            }
        }
        // method call inside classes (chain-walks for inherited methods)
        if let Some((recvv, rt)) = recv.clone() {
            if let Ty::Dyn(tname) = self.r.get(rt) {
                let tname = tname.clone();
                // dynamic dispatch on the trait surface
                return self.emit_dyn_call(fw, &tname, &name, &recvv, &argv, &sigargs, pos);
            }
            if let Ty::Named(cls, _) = self.r.get(rt) {
                // super.m(...) dispatches at the superclass, skipping own overrides
                let is_super = matches!(&callee.node, ExprNode::Field { obj, .. } if matches!(&obj.node, ExprNode::Super));
                let start = if is_super {
                    match self.classes.get(cls).and_then(|ci| ci.superclass.clone()) {
                        Some(s) => Some(s),
                        None => {
                            self.err(pos, format!("class `{}` has no superclass", cls));
                            None
                        }
                    }
                } else {
                    Some(cls.to_string())
                };
                let m = start.and_then(|sc: String| self.find_method(&sc, &name));
                if let Some((defcls, fd)) = m {
                    return self.emit_method_call(fw, &defcls, &name, &fd, is_super, &argv, &sigargs, pos);
                }
            }
        }        // direct function call
        if let Some(fd) = self.funcs.get(&name).cloned() {
            if !fd.type_params.is_empty() {
                return self.emit_gfunc_call(fw, &name, &fd, &argv, pos);
            }
            let variadic = fd.variadic.clone();
            let plan = self.plan_func(&name, None, &fd, variadic.as_ref());
            let r = fw.v();
            // extern funcs resolve under their raw C-ABI symbol
            let sym = if fd.is_extern { name.clone() } else { plan.mangled.clone() };
            let (mut vals, tys) = match &variadic {
                Some(vd) => {
                    // extra args pack into one Array<T> word; the declared elem
                    // kind decides the slot route (int extras sitofp to f64)
                    let elty = self.ty_of(&vd.elem);
                    let fels = self.is_float(elty);
                    let fixed = plan.params.len() - 1;
                    let mut vals: Vec<String> = argv[..fixed].iter().map(|x| x.0.clone()).collect();
                    let mut sigs: Vec<String> =
                        argv[..fixed].iter().map(|x| mlir_word_ty(x.1, &self.r)).collect();
                    let mut pv: Vec<String> = Vec::new();
                    for (v, t) in &argv[fixed.min(argv.len())..] {
                        if fels && !self.is_float(*t) {
                            let cv = fw.v();
                            fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                            pv.push(cv);
                        } else if !fels && self.is_float(*t) {
                            self.err(pos, "type mismatch: variadic argument is float but elem type is not".to_string());
                            pv.push(v.clone());
                        } else {
                            pv.push(v.clone());
                        }
                    }
                    let packed = self.pack_variadic(fw, fels, &pv);
                    vals.push(packed);
                    sigs.push("i64".to_string());
                    (vals, sigs.join(", "))
                }
                None => (
                    argv.iter().map(|x| x.0.clone()).collect(),
                    sigargs.join(", "),
                ),
            };
            let rt = mlir_ret_ty(self, plan.ret);
            if self.is_unit(plan.ret) {
                fw.op(&format!(
                    "    call @{}({}) : ({}) -> ()",
                    sym, vals.join(", "), tys
                ));
                return (String::new(), plan.ret);
            }
            fw.op(&format!(
                "    {} = call @{}({}) : ({}) -> {}",
                r, sym, vals.join(", "), tys, rt
            ));
            return (r, plan.ret);
        }
        // lambda value call: local symbol carrying a lambda frame dispatches via its sym
        if let Some((_lv, lt)) = fw.lookup(&name) {
            let larr = match self.r.get(lt) {
                Ty::Fn(ft) => ft.lam.clone().map(|lam| (ft.ret, lam)),
                _ => None,
            };
            if let Some((lret, lam)) = larr {
                // lambda invoked via its frame value: capt words loaded back in order
                let framev = match fw.lookup(&name) {
                    Some((fv, _)) => {
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", z));
                        let vv = fw.v();
                        fw.op(&format!(
                            "    {} = memref.load {}[{}] : memref<1xi64>",
                            vv, fv, z
                        ));
                        Some((fv, vv))
                    }
                    None => None,
                };
                let mut vals: Vec<String> = Vec::new();
                let mut tys: Vec<String> = Vec::new();
                if let Some((_fv, frame)) = framev {
                    for j in 0..lam.caps.len() {
                        let zi = fw.v();
                        fw.op(&format!("    {} = arith.constant {} : i64", zi, j));
                        let cv = fw.v();
                        fw.op(&format!(
                            "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
                            cv, frame, zi
                        ));
                        vals.push(cv);
                        tys.push("i64".to_string());
                    }
                }
                for a in args {
                    let (v, t) = self.emit_expr(fw, a);
                    vals.push(v);
                    tys.push(mlir_word_ty(t, &self.r));
                }
                let sig = tys.join(", ");
                let r = fw.v();
                let rt = mlir_ret_ty(self, lret);
                fw.op(&format!(
                    "    {} = call @{}({}) : ({}) -> {}",
                    r, lam.sym, vals.join(", "), sig, rt
                ));
                return (r, lret);
            }
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
                match self.r.get(t).clone() {
                    // Display-plumbed print: user class needs impl Display + to_str()
                    Ty::Named(ref cls, _) => {
                        self.satisfies_bound_check(pos, &t, "Display", "print");
                        let sfd = self.find_method(cls, "to_str").ok_or_else(|| ()).ok();
                        if let Some((defcls, fd)) = sfd {
                            let (sv, _st) = self.emit_method_call(
                                fw, &defcls, "to_str", &fd, false,
                                &vec![(v.clone(), t.clone())],
                                &vec!["i64".to_string()], pos,
                            );
                            let r2 = fw.v();
                            fw.op(&format!(
                                "    {} = call @sloth_rt_print_str({}) : (i64) -> i64",
                                r2, sv
                            ));
                            (r2, self.r.mk(Ty::Unit))
                        } else {
                            self.err(pos, "`print` on class requires impl Display with `to_str`".to_string());
                            (r, self.r.mk(Ty::Unit))
                        }
                    }
                    _ => {
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
                }
            }
            "int" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let ts = self.r.get(t).clone();
                match ts {
                    Ty::F64 => {
                        fw.op(&format!(
                            "    {} = arith.fptosi {} : f64 to i64",
                            r, v
                        ));
                        (r, self.r.mk(Ty::I64))
                    }
                    Ty::Str => {
                        self.err(pos, "int() of str unsupported (MVP)".to_string());
                        (v, self.r.mk(Ty::I64))
                    }
                    _ => (v, self.r.mk(Ty::I64)),
                }
            }
            "float" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let ts = self.r.get(t).clone();
                match ts {
                    Ty::F64 => (v, self.r.mk(Ty::F64)),
                    Ty::Str => {
                        self.err(pos, "float() of str unsupported (MVP)".to_string());
                        (v, self.r.mk(Ty::F64))
                    }
                    _ => {
                        fw.op(&format!(
                            "    {} = arith.sitofp {} : i64 to f64",
                            r, v
                        ));
                        (r, self.r.mk(Ty::F64))
                    }
                }
            }
            "len" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let ts = self.r.get(t).clone();
                let sym = match &ts {
                    Ty::Str => "sloth_str_len",
                    Ty::Array(_) => "sloth_arr_len",
                    Ty::Map(..) => "sloth_map_len",
                    _ => "sloth_str_len",
                };
                fw.op(&format!(
                    "    {} = call @{}({}) : (i64) -> i64",
                    r, sym, v
                ));
                (r, self.r.mk(Ty::I64))
            }
            "keys" if !argv.is_empty() => {
                let (v, _t) = argv[0].clone();
                fw.op(&format!(
                    "    {} = call @sloth_map_keys({}) : (i64) -> i64",
                    r, v
                ));
                let ei = self.r.mk(Ty::I64);
                (r, self.r.mk(Ty::Array(ei)))
            }
            "values" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let vt = match self.r.get(t).clone() {
                    Ty::Map(_k, v3) => v3,
                    _ => self.r.mk(Ty::I64),
                };
                fw.op(&format!(
                    "    {} = call @sloth_map_values({}) : (i64) -> i64",
                    r, v
                ));
                (r, self.r.mk(Ty::Array(vt)))
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

impl ModEmitter {
    /// build one fresh Array<T> word from already-coerced words (variadic pack)
    /// shape walker: declared param ty -> matchable Ty, holes as Named(T)
    fn shape_of(&mut self, pt: &Type, tnames: &[String]) -> TyId {
        match pt {
            Type::Simple(SimpleType::Ident(n)) if tnames.contains(n) => {
                self.r.mk(Ty::Named(n.to_string(), Vec::new()))
            }
            Type::Simple(SimpleType::Array(el))
                if matches!(el.as_ref(), Type::Simple(SimpleType::Ident(n)) if tnames.contains(n)) =>
            {
                match el.as_ref() {
                    Type::Simple(SimpleType::Ident(n)) => {
                        let tn = n.to_string();
                        let hole = self.r.mk(Ty::Named(tn, Vec::new()));
                        self.r.mk(Ty::Array(hole))
                    }
                    _ => unreachable!(),
                }
            }
            Type::Simple(SimpleType::Map(k, v)) => {
                let pk = self.shape_of(k, tnames);
                let pv = self.shape_of(v, tnames);
                self.r.mk(Ty::Map(pk, pv))
            }
            Type::Optional(inner) => self.shape_of(inner, tnames),
            other => self.ty_of(other),
        }
    }

    /// generic function call: infer T bindings from the call-site arg types,
    /// emit/lookup the monomorphic instance, then dispatch the call word-wise
    fn emit_gfunc_call(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        fd: &FuncDef,
        argv: &[(String, TyId)],
        pos: &Pos,
    ) -> (String, TyId) {
        if fd.variadic.is_some() {
            self.err(pos, "generic variadic unsupported (MVP)".to_string());
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, self.r.mk(Ty::Unit));
        }
        let tnames: Vec<String> = fd.type_params.iter().map(|p| p.name.clone()).collect();
        let mut map: std::collections::HashMap<String, TyId> = std::collections::HashMap::new();
        for (p, (_av, at)) in fd.params.iter().zip(argv.iter()) {
            if let Some(pt) = &p.ty {
                let pat = self.shape_of(pt, &tnames);
                self.unify_tp(&fd.type_params, pat, *at, &mut map);
            }
        }
        self.emit_ginst_call(fw, name, fd, argv, pos, map)
    }

    /// monomorphize + emit an instance call for a fully-bound substitution
    fn emit_ginst_call(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        fd: &FuncDef,
        argv: &[(String, TyId)],
        pos: &Pos,
        mut map: std::collections::HashMap<String, TyId>,
    ) -> (String, TyId) {
        let tnames: Vec<String> = fd.type_params.iter().map(|p| p.name.clone()).collect();
        for tn in &tnames {
            if !map.contains_key(tn) {
                self.err(pos, format!("cannot infer type parameter `{}`", tn));
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                return (z, self.r.mk(Ty::Unit));
            }
        }
        // trait constraint validation (§2.3): bound checks before instantiation
        for tp in &fd.type_params {
            if let Some(bound) = &tp.bound {
                let t = map[&tp.name];
                if !self.satisfies_bound(t, bound) {
                    self.err(
                        pos,
                        format!(
                            "type argument `{}` does not satisfy trait bound `{}`",
                            sloth_frontend::ty::ty_name(self.r.get(t)),
                            bound
                        ),
                    );
                }
            }
        }
        if self.insts.len() > 64 {
            self.err(pos, "generic instantiation too deep (recursion?)".to_string());
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, self.r.mk(Ty::Unit));
        }
        let keys: Vec<TyId> = tnames.iter().map(|n| map[n]).collect();
        let base = mangle(&self.cur_mod.clone(), None, name);
        let mangled = format!("{}{}", base, mangle_t(&keys, &self.r));
        if !self.emitted_names.contains(&mangled) {
            self.tp_subst.push(map.clone());
            self.tp_mangled.push(mangled.clone());
            self.emit_func(name, None, fd, None, false);
            self.tp_subst.pop();
            self.tp_mangled.pop();
        }
        self.stat_ginsts += 1;
        // instance plan: substitution frame active so T resolves to the bound type
        let plan = {
            self.tp_subst.push(map);
            self.tp_mangled.push(mangled.clone());
            let p = self.plan_func(name, None, fd, None);
            self.tp_subst.pop();
            self.tp_mangled.pop();
            p
        };
        let r = fw.v();
        let vals: Vec<String> = argv.iter().map(|x| x.0.clone()).collect();
        let sigs = argv
            .iter()
            .map(|x| mlir_word_ty(x.1, &self.r))
            .collect::<Vec<String>>();
        let rt = mlir_ret_ty(self, plan.ret);
        if self.is_unit(plan.ret) {
            fw.op(&format!(
                "    call @{}({}) : ({}) -> ()",
                mangled, vals.join(", "), sigs.join(", ")
            ));
            return (String::new(), plan.ret);
        }
        fw.op(&format!(
            "    {} = call @{}({}) : ({}) -> {}",
            r, mangled, vals.join(", "), sigs.join(", "), rt
        ));
        (r, plan.ret)
    }

    fn unify_tp(
        &mut self,
        tnames: &[TypeParam],
        pat: TyId,
        act: TyId,
        map: &mut std::collections::HashMap<String, TyId>,
    ) {
        let pt = self.r.get(pat).clone();
        let atc = self.r.get(act).clone();
        match (pt, atc) {
            (Ty::Named(n, a), _) if a.is_empty()
                && tnames.iter().any(|p| p.name == n) =>
            {
                map.entry(n).or_insert(act);
            }
            (Ty::Array(pe), Ty::Array(ae)) => self.unify_tp(tnames, pe, ae, map),
            (Ty::Opt(pe), Ty::Opt(ae)) => self.unify_tp(tnames, pe, ae, map),
            (Ty::Map(pk, pv), Ty::Map(ak, av)) => {
                self.unify_tp(tnames, pk, ak, map);
                self.unify_tp(tnames, pv, av, map);
            }
            _ => {}
        }
    }

    /// build one fresh Array<T> word from already-coerced words (variadic pack)
    fn pack_variadic(&mut self, fw: &mut FnWalk, fels: bool, vals: &[String]) -> String {
        let n = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", n, vals.len()));
        let arr = fw.v();
        fw.op(&format!("    {} = call @sloth_arr_new({}) : (i64) -> i64", arr, n));
        for (i, v) in vals.iter().enumerate() {
            let zi = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", zi, i));
            if fels {
                fw.op(&format!(
                    "    call @sloth_arr_set_f64({}, {}, {}) : (i64, i64, f64) -> i64",
                    arr, zi, v
                ));
            } else {
                fw.op(&format!(
                    "    call @sloth_arr_set({}, {}, {}) : (i64, i64, i64) -> i64",
                    arr, zi, v
                ));
            }
        }
        arr
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
    s.push_str("  func.func private @sloth_arr_push(i64, i64) -> i64
  func.func private @sloth_arr_push_f64(i64, f64) -> i64
  func.func private @sloth_arr_pop(i64) -> i64
  func.func private @sloth_arr_pop_f64(i64) -> f64
  func.func private @sloth_str_finish(i64) -> i64
  func.func private @sloth_str_pushp(i64, i64) -> i64
  func.func private @sloth_str_push_i(i64, i64) -> i64
  func.func private @sloth_str_push_f(i64, f64) -> i64
  func.func private @sloth_str_push_b(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_len(i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_char(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_concat(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_new(i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_len(i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_get(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_get_f64(i64, i64) -> f64\n");
    s.push_str("  func.func private @sloth_arr_set(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_set_f64(i64, i64, f64) -> i64\n");
    s.push_str("  func.func private @sloth_map_new(i64) -> i64
  func.func private @sloth_map_len(i64) -> i64
  func.func private @sloth_map_get(i64, i64) -> i64
  func.func private @sloth_map_get_f64(i64, i64) -> f64
  func.func private @sloth_map_set(i64, i64, i64) -> i64
  func.func private @sloth_map_set_f64(i64, i64, f64) -> i64
  func.func private @sloth_map_str_get(i64, i64) -> i64
  func.func private @sloth_map_str_get_f64(i64, i64) -> f64
  func.func private @sloth_map_str_set(i64, i64, i64) -> i64
  func.func private @sloth_map_str_set_f64(i64, i64, f64) -> i64
  func.func private @sloth_map_keys(i64) -> i64
  func.func private @sloth_map_values(i64) -> i64\n");
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
        // stdlib Result<T,E> prelude (only injected once)
        let mut decls2: Vec<Decl> = prog.decls.clone();
        if !decls2
            .iter()
            .any(|d| d.name == "Result" && matches!(d.node, DeclNode::Class(_)))
        {
            match sloth_frontend::parser::parse(
                "extern func sloth_panic_unwrap(): int;\n\
                 class Result<T, E> {\n\
                 var ok: bool = false;\n\
                 var v: T;\n\
                 var e: E;\n\
                 func is_ok(): bool {\n\
                 return this.ok;\n\
                 }\n\
                 func unwrap(): T {\n\
                 var flag: bool = this.ok;\n\
                 if flag {\n\
                 return this.v;\n\
                 }\n\
                 { sloth_panic_unwrap(); }\n\
                 return this.v;\n\
                 }\n\
                 func err(): E {\n\
                 return this.e;\n\
                 }\n\
                 }\n",
            ) {
                Ok(stdp) => {
                    for d in stdp.decls.into_iter().rev() {
                        decls2.insert(0, d);
                    }
                }
                Err(_) => {
                    // parser unreachable for a fixed literal; keep going
                }
            }
        }
        let mut stmts2 = prog.stmts.clone();
        let mut imps2 = prog.imports.clone();
        let prog = &mut Program { decls: decls2, stmts: stmts2, imports: imps2 };
        self.collect(prog);
        self.finalize_vt();
        // 1) top-level funcs
        for d in &prog.decls {
            if let DeclNode::Func(f) = &d.node {
                let entry = d.name == "main";
                self.emit_func(&d.name, None, f, f.variadic.as_ref(), entry);
            }
        }
        // 1b) classes: emit methods + ctor (incl. trait-synthesized defaults);
        // generic base defs are skipped (their instances emit below)
        for d in &prog.decls {
            if let DeclNode::Class(c) = &d.node {
                if !c.type_params.is_empty() {
                    continue;
                }
                let meths = match self.classes.get(&d.name) {
                    Some(ci) => ci.methods.clone(),
                    None => Vec::new(),
                };
                for (mname, fd) in meths {
                    self.emit_func(&mname, Some(&d.name), &fd, None, false);
                }
            }
        }
        // 1c) generic-class instances: methods emitted under the T-frame
        {
            let insts = self.pending_insts.clone();
            for (inst, frame) in insts {
                let meths = match self.classes.get(&inst) {
                    Some(ci) => ci.methods.clone(),
                    None => Vec::new(),
                };
                self.tp_subst.push(frame);
                for (mname, fd) in meths {
                    self.emit_func(&mname, Some(&inst), &fd, None, false);
                }
                self.tp_subst.pop();
            }
        }
        // 2) script statements run in entry if no main() was declared
        let has_main = prog.decls.iter().any(|d| d.name == "main" && matches!(d.node, DeclNode::Func(_)));
        if !has_main {
            let mut fw = FnWalk {
                cur: String::new(),
                vcount: 1000,
                scopes: vec![HashMap::new()],
            imms: vec![HashMap::new()],
                loops: Vec::new(),
                ret: self.r.mk(Ty::Unit),
                ret_alloca: String::new(),
                ret_flag: String::new(),
            bb: 0,
            term: false,
            end_label: "^smt".to_string(),
            cur_cls: None,
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
        if std::env::var("SLOTH_STATS").as_deref() == Ok("1") {
            eprintln!(
                "sloth-stats: module={} direct-method-calls={} dyn-calls={} generic-instances={} extern-decls={}",
                me.name, me.stat_dcalls, me.stat_dyncalls, me.stat_ginsts, me.stat_extdecls
            );
        }
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

/// free variables of a lambda body (used minus declared/params), in first-use order
fn lambda_caps(me: &ModEmitter, l: &Lambda) -> Vec<String> {
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
        if !decls.contains(&u) && !me.globals.contains_key(&u) && got.insert(u.clone()) {
            out.push(u);
        }
    }
    out
}

/// collect identifier uses inside statements, tracking declarations
fn walk_ids_stmt(
    s: &Stmt,
    push_use: &mut dyn FnMut(&String),
    decls: &mut std::collections::HashSet<String>,
) {
    match &s.node {
        StmtNode::Expr(e) => walk_ids_expr(e, push_use, decls),
        StmtNode::Let { name, init, .. } => {
            walk_ids_expr(init, push_use, decls);
            decls.insert(name.clone());
        }
        StmtNode::Assign { target, value } => {
            walk_ids_expr(value, push_use, decls);
            for seg in target {
                if let PathSeg::Name(n) = seg {
                    push_use(n);
                }
                if let PathSeg::Index(e) = seg {
                    walk_ids_expr(e, push_use, decls);
                }
            }
        }
        StmtNode::While { cond, body } => {
            walk_ids_expr(cond, push_use, decls);
            walk_ids_stmt(body, push_use, decls);
        }
        StmtNode::If { cond, then_, else_ } => {
            walk_ids_expr(cond, push_use, decls);
            walk_ids_stmt(then_, push_use, decls);
            if let Some(els) = else_ {
                walk_ids_stmt(els, push_use, decls);
            }
        }
        StmtNode::For { var, iter, body } => {
            walk_ids_expr(iter, push_use, decls);
            walk_ids_stmt(body, push_use, decls);
            decls.insert(var.clone());
        }
        StmtNode::Return(Some(e)) => {
            walk_ids_expr(e, push_use, decls);
        }
        StmtNode::Return(None) | StmtNode::Break | StmtNode::Continue => {}
        StmtNode::Block(ss) => {
            for st in ss {
                walk_ids_stmt(st, push_use, decls);
            }
        }
    }
}

/// collect identifier uses inside expressions (lambda-free path)
fn walk_ids_expr(
    e: &Expr,
    push_use: &mut dyn FnMut(&String),
    decls: &mut std::collections::HashSet<String>,
) {
    match &e.node {
        ExprNode::Ident(n) => push_use(n),
        ExprNode::Call { callee, args } => {
            walk_ids_expr(callee, push_use, decls);
            for a in args {
                walk_ids_expr(a, push_use, decls);
            }
        }
        ExprNode::Field { obj, .. } => walk_ids_expr(obj, push_use, decls),
        ExprNode::Index { obj, idx } => {
            walk_ids_expr(obj, push_use, decls);
            walk_ids_expr(idx, push_use, decls);
        }
        ExprNode::Arith { op: _, lhs, rhs }
        | ExprNode::Bin { op: _, lhs, rhs } => {
            walk_ids_expr(lhs, push_use, decls);
            walk_ids_expr(rhs, push_use, decls);
        }
        ExprNode::Un { expr, .. } => walk_ids_expr(expr, push_use, decls),
        ExprNode::List(xs) => {
            for x in xs {
                walk_ids_expr(x, push_use, decls);
            }
        }
        ExprNode::Int(_) | ExprNode::Float(_) | ExprNode::Bool(_)
        | ExprNode::Str(_) | ExprNode::Nil | ExprNode::This | ExprNode::Super => {}
        _ => {}
    }
}

fn fresh_walk(me: &mut ModEmitter) -> FnWalk {
    FnWalk {
        cur: String::new(),
        vcount: 1000,
        scopes: vec![HashMap::new()],
        imms: vec![HashMap::new()],
    loops: Vec::new(),
    ret: me.r.mk(Ty::Unit),
    ret_alloca: String::new(),
    ret_flag: String::new(),
    bb: 0,
    term: false,
    end_label: "^ginit".to_string(),
    cur_cls: None,
    }
}

// final IR normalization: rewrite zero consts used as memref indices to `index`
pub fn normalize_indices(src: &str) -> String {
    // process per-function so that per-fn scoped SSA names don't collide
    let mut out = String::new();
    let mut chunk: Vec<String> = Vec::new();
    for line in src.lines() {
        let t2 = line.trim_start();
        let starts_fn = t2.starts_with("func.func") || t2.starts_with("llvm.func");
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
    s.push_str("  func.func private @sloth_obj_cls_id(i64) -> i64\n");
    s.push_str("  func.func private @sloth_vt_new(i64) -> i64\n");
    s.push_str("  func.func private @sloth_vt_set(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_vt_get(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_set_vtable(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_vtable(i64) -> i64\n");
    s.push_str("  func.func private @sloth_panic_noimpl(i64) -> i64\n");
    s
}

fn words_for_cls(me: &ModEmitter, clsname: &str) -> usize {
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
        self.emit_vt_build(fw, clsname, &r2);
        // field initializers run before __init__
        {
            // field decls need the original program AST: find via decl map
            let defs: Vec<(String, Option<sloth_frontend::ast::Expr>, sloth_frontend::ast::Type)> = {
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
            let vals = [r2.clone()].iter().cloned()
                .chain(argvals.iter().cloned())
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
        let is_ll = self.llvm_method.contains(&(cls.to_string(), mname.to_string()));
        let vals: Vec<String> = argv.iter().map(|x| x.0.clone()).collect();
        let tys = sigargs.join(", ");
        let ret = mlir_ret_ty(self, plan.ret);
        let callkw = if is_ll { "llvm.call" } else { "call" };
        if self.is_unit(plan.ret) {
            fw.op(&format!(
                "    {} @{}({}) : ({}) -> ()",
                callkw, plan.mangled, vals.join(", "), tys
            ));
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, plan.ret);
        }
        let r = fw.v();
        fw.op(&format!(
            "    {} = {} @{}({}) : ({}) -> {}",
            r, callkw, plan.mangled, vals.join(", "), tys, ret
        ));
        (r, plan.ret)
    }
}

impl ModEmitter {
    /// resolve a method plan against a class, honoring its owning module
    fn plan_for_class(
        &mut self,
        mname: &str,
        cls: &str,
        m: &FuncDef,
    ) -> FuncPlan {
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

    /// stable slot capacity: cover every declared trait surface; called
    /// after registration/collect and before any emission
    fn finalize_vt(&mut self) {
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

    /// attach the class vtable to a fresh object (header word 1). Slot value =
    /// raw fn pointer (llvm.mlir.addressof + llvm.ptrtoint) of the resolved
    /// method emitting (llvm.func-marked).
    fn emit_vt_build(&mut self, fw: &mut FnWalk, clsname: &str, obj: &str) {
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
        fw.op(&format!("    {} = arith.constant {} : i64", ncap, self.vt_cap));
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

    /// `dyn T` receiver: vtable dispatch. Object word 1 holds the class
    /// vt pointer; slot = (trait, method) index; slot value = raw fn-pointer
    /// word of the llvm.func-emitted resolved method.
    fn emit_dyn_call(
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
                self.err(pos, format!("unknown trait `{}` for method `{}`", tname, mname));
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
            let mty = if ret_flt { "memref<1xf64>" } else { "memref<1xi64>" };
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
        fw.op(&format!("    {} = arith.cmpi ne, {}, {} : i64", cc, fp, zero));
        let ce = fw.v();
        fw.op(&format!("    {} = arith.extsi {} : i1 to i64", ce, cc));
        let lbl_call = fw.newlabel("dc");
        let lbl_panic = fw.newlabel("dp");
        let lbl_end = fw.newlabel("de");
        fw.cjump(&ce, &lbl_call, &lbl_panic);
        // resolved: call through the slot pointer
        fw.label(&lbl_call);
        let vp = fw.v();
        fw.op(&format!("    {} = llvm.inttoptr {} : i64 to !llvm.ptr", vp, fp));
        let mut vals: Vec<String> = Vec::new();
        vals.extend(argv.iter().map(|x| x.0.clone()));
        let sig = tys.join(", ");
        let ret_ty_txt = if ret_flt { "f64".to_string() } else { "i64".to_string() };
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
                let mty = if *fl { "memref<1xf64>" } else { "memref<1xi64>" };
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
            fw.op(&format!("    {} = memref.load {}[{}] : {}", v, slot2, zi, mty));
            let tret = if fl { self.r.mk(Ty::F64) } else { self.r.mk(Ty::I64) };
            return (v, tret);
        }
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
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
    let mut stack: Vec<std::path::PathBuf> = Vec::new();
    let mut done: HashSet<std::path::PathBuf> = HashSet::new();
    let (root, mods) = resolve_program(root_src, base_dir, &mut stack, &mut done)?;
    let mut me = ModEmitter::new("main");
    for (mname, prog, alias) in &mods {
        me.register_import(mname, alias.as_deref(), prog);
    }
    // hmm: root module runs under @sloth_main through emit_module
    me.emit_module(&root);
    if !me.diags.is_empty() {
        return Err(format!("codegen diags: {:?}", me.diags));
    }
    let ir0 = ModEmitter::take_ir(&mut me);
    Ok(normalize_indices(&ir0))
}

fn resolve_program(
    src: &str,
    dir: &std::path::Path,
    stack: &mut Vec<std::path::PathBuf>,
    done: &mut HashSet<std::path::PathBuf>,
) -> Result<(Program, Vec<(String, Program, Option<String>)>), String> {
    let prog = sloth_frontend::parser::parse(src).map_err(|e| format!("{:?}", e))?;
    let mut mods: Vec<(String, Program, Option<String>)> = Vec::new();
    for imp in &prog.imports {
        let pb = dir.join(&imp.path);
        let pb2 = match std::fs::canonicalize(&pb) {
            Ok(p) => p,
            Err(_) => pb.clone(),
        };
        if stack.iter().any(|x| x == &pb2) {
            let join = stack
                .iter()
                .filter_map(|x| x.file_name().map(|f| f.to_string_lossy().to_string()))
                .collect::<Vec<String>>()
                .join(" -> ");
            return Err(format!(
                "circular import: {} -> {}",
                join,
                pb2.file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| "self".to_string())
            ));
        }
        if done.contains(&pb2) {
            continue;
        }
        let src2 = std::fs::read_to_string(&pb2).map_err(|e| format!("read {:?}: {}", pb2, e))?;
        let dir2 = pb2.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| std::path::PathBuf::from("."));
        stack.push(pb2.clone());
        let (_p2, mut m2) = resolve_program(&src2, &dir2, stack, done)?;
        stack.pop();
        done.insert(pb2.clone());
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

/// visibility flag of a toplevel declaration
fn visible_of(d: &Decl) -> bool {
    d.visible
}

impl ModEmitter {
    /// access to a non-pub symbol from another module: diagnostic
    fn guard_hidden(&mut self, qualifier: &str, name: &str, pos: &Pos) {
        let key = format!("{}.{}", qualifier, name);
        if self.hidden.contains(&key) {
            self.err(pos, format!("`{}` is private to its module (not `pub`)", key));
        }
    }
}

/// inside llvm.func bodies bare `call @` must be spelled `func.call @`,
/// but `llvm.call @` (direct llvm-func calls) must not be touched
fn rename_plain_calls(t: &str) -> String {
    let ch: Vec<char> = t.chars().collect();
    let mut out = String::new();
    let mut i = 0usize;
    while i < ch.len() {
        if i + 6 <= ch.len() && ch[i] == 'c' && i >= 5 {
            let prev: String = ch[i - 5..i].iter().collect();
            if prev == "llvm." && ch[i..i + 6].iter().collect::<String>() == "call @" {
                out.push_str("call @");
                i += 6;
                continue;
            }
        }
        if i + 6 <= ch.len() && ch[i..i + 6].iter().collect::<String>() == "call @" {
            out.push_str("func.call @");
            i += 6;
            continue;
        }
        out.push(ch[i]);
        i += 1;
    }
    out
}

impl ModEmitter {
    /// typed field store (float fields use the f64 rt routine)
    fn op_set_field(
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

impl FnWalk {
    /// true when the outermost binding of `name` is immutable (let)
    fn imm_of(&self, name: &str) -> bool {
        for m in self.imms.iter().rev() {
            if let Some(v) = m.get(name) {
                return *v;
            }
        }
        false
    }
}
