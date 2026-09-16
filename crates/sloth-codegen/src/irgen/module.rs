//! Module-level emission: driver, globals, imports, runtime decls.

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
    /// adopt the surface of a foreign module (emits its funcs/classes under that
    /// module's name and registers them for cross-module calls)
    pub fn register_import(&mut self, mname: &str, alias: Option<&str>, prog: &Program) {
        self.cur_mod = mname.to_string();
        let _hide_mark = ();

        if let Some(a) = alias {
            self.mod_alias.insert(a.to_string(), mname.to_string());
        }
        let qname = alias.unwrap_or(mname).to_string();
        // foreign global cells (declared lazily; inits run in modinit):
        // register BEFORE func emission so foreign bodies can read their own
        // module's globals via fglobals
        for d in &prog.decls {
            if let DeclNode::Var { ty, .. } = &d.node {
                let t = match ty {
                    Some(t) => self.ty_of(t),
                    None => self.r.mk(Ty::Unit),
                };
                let sym = self.declare_global(mname, &d.name, t);
                self.fglobals
                    .insert(format!("{}.{}", qname, d.name), (sym.clone(), t));
                self.fglobals
                    .insert(format!("{}.{}", mname, d.name), (sym.clone(), t));
                if !visible_of(d) {
                    self.hidden.insert(format!("{}.{}", qname, d.name));
                }
            }
        }
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
                        self.cross_funcs
                            .insert(d.name.clone(), (mangled.clone(), plan.ret));
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
                _ => {}
            }
        }
        // pass 2b: effective trait surfaces (incl. inherited) -> vtable slots
        for d in &prog.decls {
            if let DeclNode::Class(_c) = &d.node {
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
}

pub fn rt_decls() -> String {
    let mut s = String::new();
    // rc core (ARC migration patch B): counting primitives
    s.push_str("  func.func private @sloth_rc_retain(i64) -> i64\n");
    s.push_str("  func.func private @sloth_rc_release(i64) -> i64\n");
    s.push_str("  func.func private @sloth_rc_live() -> i64\n");
    s.push_str("  func.func private @sloth_rc_drops() -> i64\n");
    s.push_str("  func.func private @sloth_weak_new(i64) -> i64\n");
    s.push_str("  func.func private @sloth_weak_upgrade(i64) -> i64\n");
    s.push_str("  func.func private @sloth_weak_release(i64) -> i64\n");
    s.push_str("  func.func private @sloth_rt_print_i64(i64) -> i64\n");
    s.push_str("  func.func private @sloth_rt_print_f64(f64) -> i64\n");
    s.push_str("  func.func private @sloth_rt_print_bool(i64) -> i64\n");
    s.push_str("  func.func private @sloth_rt_print_str(i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_intern(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_range_pack(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_push(i64, i64, i64) -> i64\n");
    s.push_str(
        "  func.func private @sloth_arr_push(i64, i64) -> i64
  func.func private @sloth_arr_push_f64(i64, f64) -> i64
  func.func private @sloth_arr_pop(i64) -> i64
  func.func private @sloth_arr_pop_f64(i64) -> f64
  func.func private @sloth_str_finish(i64) -> i64
  func.func private @sloth_str_pushp(i64, i64) -> i64
  func.func private @sloth_str_push_i(i64, i64) -> i64
  func.func private @sloth_str_push_f(i64, f64) -> i64
  func.func private @sloth_str_push_b(i64, i64) -> i64\n",
    );
    s.push_str("  func.func private @sloth_str_len(i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_char(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_concat(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_str_eq(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_new(i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_len(i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_get(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_get_f64(i64, i64) -> f64\n");
    s.push_str("  func.func private @sloth_arr_set(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_set_f64(i64, i64, f64) -> i64\n");
    s.push_str(
        "  func.func private @sloth_map_new(i64) -> i64
  func.func private @sloth_map_len(i64) -> i64
  func.func private @sloth_map_get(i64, i64) -> i64
  func.func private @sloth_map_get_f64(i64, i64) -> f64
  func.func private @sloth_map_set(i64, i64, i64) -> i64
  func.func private @sloth_map_set_f64(i64, i64, f64) -> i64
  func.func private @sloth_map_get_h(i64, i64, i64) -> i64
  func.func private @sloth_map_get_h_f64(i64, i64, i64) -> f64
  func.func private @sloth_map_set_h(i64, i64, i64, i64) -> i64
  func.func private @sloth_map_set_h_f64(i64, i64, i64, f64) -> i64
  func.func private @sloth_map_str_get(i64, i64) -> i64
  func.func private @sloth_map_str_get_f64(i64, i64) -> f64
  func.func private @sloth_map_str_set(i64, i64, i64) -> i64
  func.func private @sloth_map_str_set_f64(i64, i64, f64) -> i64
  func.func private @sloth_map_keys(i64) -> i64
  func.func private @sloth_map_values(i64) -> i64\n",
    );
    s
}

pub(crate) fn emit_str_globals(me: &ModEmitter) -> String {
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
                 class Entry<T, E> {\n\
                 var key: T;\n\
                 var val: E;\n\
                 }\n\
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
        let stmts2 = prog.stmts.clone();
        let imps2 = prog.imports.clone();
        let prog = &mut Program {
            decls: decls2,
            stmts: stmts2,
            imports: imps2,
        };
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
        let has_main = prog
            .decls
            .iter()
            .any(|d| d.name == "main" && matches!(d.node, DeclNode::Func(_)));
        if !has_main {
            let mut fw = FnWalk {
                cur: String::new(),
                vcount: 1000,
                scopes: vec![HashMap::new()],
                imms: vec![HashMap::new()],
                scope_decls: vec![HashMap::new()],
                dangling: Vec::new(),
                loops: Vec::new(),
                loopvars: Vec::new(),
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
}

impl ModEmitter {
    /// wrap up script entry function text into self.out
    pub(crate) fn finish_entry(&mut self, fw: &mut FnWalk) {
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
}

impl ModEmitter {
    /// reserve a mangled global symbol and register its decl line (deduped)
    pub(crate) fn declare_global(&mut self, modname: &str, name: &str, t: TyId) -> String {
        let sym = format!("sloth_{}_g_{}", modname, name);
        if self.declared_syms.insert(sym.clone()) {
            let mty = memref_cell_ty(self, t);
            let init = if mty == "memref<1xf64>" {
                "dense<0.0>"
            } else {
                "dense<0>"
            };
            self.global_decls.push(format!(
                "  memref.global @{} : {} = {} {{mutable}}\n",
                sym, mty, init
            ));
        }
        sym
    }
}

impl ModEmitter {
    /// emit get_global + load for a global cell reference
    pub(crate) fn emit_global_read(
        &mut self,
        fw: &mut FnWalk,
        sym: &str,
        t: TyId,
    ) -> (String, TyId) {
        let mty = memref_cell_ty(self, t);
        let g = fw.v();
        fw.op(&format!("    {} = memref.get_global @{} : {}", g, sym, mty));
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        let v = fw.v();
        fw.op(&format!("    {} = memref.load {}[{}] : {}", v, g, z, mty));
        (v, t)
    }
}

impl ModEmitter {
    /// full MLIR text of the module
    pub fn take_ir(me: &mut ModEmitter) -> String {
        if std::env::var("SLOTH_STATS").as_deref() == Ok("1") {
            eprintln!(
                "sloth-stats: module={} direct-method-calls={} dyn-calls={} generic-instances={} extern-decls={} per-cls-vtables={}",
                me.name, me.stat_dcalls, me.stat_dyncalls, me.stat_ginsts, me.stat_extdecls, me.stat_vtbuilds
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

pub fn obj_rt_decls() -> String {
    let mut s = String::new();
    s.push_str("  func.func private @sloth_obj_new(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_field(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_field_f64(i64, i64) -> f64\n");
    s.push_str("  func.func private @sloth_obj_set_field(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_set_field_f64(i64, i64, f64) -> i64\n");
    s.push_str("  func.func private @sloth_cls_info(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_cls_refmask(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_arr_new_k(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_cls_id(i64) -> i64\n");
    s.push_str("  func.func private @sloth_vt_new(i64) -> i64\n");
    s.push_str("  func.func private @sloth_vt_set(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_vt_get(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_set_vtable(i64, i64) -> i64\n");
    s.push_str("  func.func private @sloth_obj_vtable(i64) -> i64\n");
    s.push_str("  func.func private @sloth_panic_noimpl(i64) -> i64\n");
    s
}

impl ModEmitter {
    /// access to a non-pub symbol from another module: diagnostic
    pub(crate) fn guard_hidden(&mut self, qualifier: &str, name: &str, pos: &Pos) {
        let key = format!("{}.{}", qualifier, name);
        if self.hidden.contains(&key) {
            self.err(
                pos,
                format!("`{}` is private to its module (not `pub`)", key),
            );
        }
    }
}
