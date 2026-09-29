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
        // imported opaque extern types: register so `is_ref` never treats their
        // raw C-pointer handles as rc-managed (de-tag: retain/release on a raw
        // foreign pointer would corrupt memory)
        for d in &prog.decls {
            if let DeclNode::ExternType = &d.node {
                self.extern_types.insert(d.name.clone());
            }
        }
        // foreign global cells (declared lazily; inits run in modinit):
        // register BEFORE func emission so foreign bodies can read their own
        // module's globals via fglobals
        for d in &prog.decls {
            if let DeclNode::Var { ty, .. } = &d.node {
                let t = match ty {
                    Some(t) => self.ty_of(t),
                    None => self.r.mk(Ty::Unit),
                };
                let mutable = d.kind == DeclKind::Var;
                let sym = self.declare_global(mname, &d.name, t);
                self.fglobals
                    .insert(format!("{}.{}", qname, d.name), (sym.clone(), t, mutable));
                self.fglobals
                    .insert(format!("{}.{}", mname, d.name), (sym.clone(), t, mutable));
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
                    // extern funcs keep their raw C-ABI symbol (their body-less
                    // declaration is emitted under it); sloth funcs are mangled
                    let mangled = if f.is_extern {
                        d.name.clone()
                    } else {
                        mangle(mname, None, &d.name)
                    };
                    // an extern declaration has no body and a globally unique
                    // C symbol, so expose it on the ordinary `funcs` table too:
                    // unqualified calls inside this module (and the root) then
                    // take the extern ABI path (arg decode / return re-encode)
                    // instead of the mangled foreign-call path
                    if f.is_extern {
                        self.funcs.insert(d.name.clone(), (**f).clone());
                    } else if d.name == "print" {
                        // the injected `print` prelude resolves as a local
                        // function inside every module so its `any` param is
                        // boxed at the call site
                        self.funcs
                            .entry("print".to_string())
                            .or_insert((**f).clone());
                    } else if self.fixed_syms.contains(&d.name) {
                        // injected reserved/fixed symbols (container/core
                        // prelude) must resolve by bare name inside this module
                        // as well, so intra-prelude calls keep the raw symbol
                        self.funcs.entry(d.name.clone()).or_insert((**f).clone());
                    }
                    let plan = self.plan_func(&d.name, None, f, None);
                    if d.visible {
                        self.cross_funcs
                            .insert(d.name.clone(), (mangled.clone(), plan.ret));
                    } else {
                        self.hidden.insert(d.name.clone());
                    }
                    self.cross_funcs
                        .insert(format!("{}.{}", qname, d.name), (mangled.clone(), plan.ret));
                    // also expose the module's own name as a qualifier: nested
                    // emission (and private-helper resolution) runs with
                    // `cur_mod = mname` even when the import site used an alias
                    if qname != mname {
                        self.cross_funcs
                            .insert(format!("{}.{}", mname, d.name), (mangled, plan.ret));
                    }
                    // keep the def so qualified calls to an imported generic
                    // function can be monomorphized in the caller
                    if !f.is_extern && !f.type_params.is_empty() {
                        self.foreign_func_defs
                            .insert(d.name.clone(), (mname.to_string(), (**f).clone()));
                        self.foreign_func_defs.insert(
                            format!("{}.{}", qname, d.name),
                            (mname.to_string(), (**f).clone()),
                        );
                        if qname != mname {
                            self.foreign_func_defs.insert(
                                format!("{}.{}", mname, d.name),
                                (mname.to_string(), (**f).clone()),
                            );
                        }
                    }
                    if !d.visible {
                        self.hidden.insert(format!("{}.{}", qname, d.name));
                        if qname != mname {
                            self.hidden.insert(format!("{}.{}", mname, d.name));
                        }
                    }
                }
                DeclNode::Class(c) => {
                    // preserve class-level visibility (bug M4). Class members
                    // default to public in the implementation (the stdlib relies
                    // on cross-module member access); see appendix A.
                    if !d.visible {
                        self.hidden_classes.insert(d.name.clone());
                    }
                    let cid: i64 = 100 + self.foreign_cls.len() as i64;
                    self.foreign_cls.insert(d.name.clone());
                    if !d.visible {
                        self.hidden.insert(format!("{}.{}", qname, d.name));
                    }
                    self.cls_mod.insert(d.name.clone(), mname.to_string());
                    self.class_ids.insert(d.name.clone(), cid);
                    self.cls_display.insert(d.name.clone(), d.name.clone());
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
                    self.register_class_vt_surface(&d.name);
                }
                _ => {}
            }
        }
        // pass 2a': override signature discipline (mirrors root `collect`)
        self.check_override_sigs(prog);
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
        // vt_cap must be settled before any method body is emitted: bodies can
        // construct instances, whose vtable builders bake in the capacity
        self.finalize_vt();
        // pass 2c: emit method bodies (llvm.func decision now settled).
        // Imported generic functions emit no template body — their monomorphic
        // instances are emitted on demand at the (root/nested) call sites.
        for d in &prog.decls {
            match &d.node {
                DeclNode::Func(f) => {
                    if !f.type_params.is_empty() {
                        continue;
                    }
                    self.emit_func(&d.name, None, f, None, false);
                }
                DeclNode::Class(c) => {
                    // generic-class templates emit no body here; their
                    // monomorphized instances are emitted once by the root pass
                    // under the defining module's name (bug M1)
                    if !c.type_params.is_empty() {
                        continue;
                    }
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
            if let DeclNode::Var { ty, init } = &d.node {
                let key = format!("{}.{}", qname, d.name);
                let (sym, t, _mut) = match self.fglobals.get(&key).cloned() {
                    Some(x) => x,
                    None => continue,
                };
                let (v, vt) = self.emit_expr(&mut fw, &init);
                // unannotated foreign global: infer the surface from its init
                if ty.is_none() {
                    for k in [
                        format!("{}.{}", qname, d.name),
                        format!("{}.{}", mname, d.name),
                    ] {
                        if let Some(g) = self.fglobals.get_mut(&k) {
                            g.1 = vt;
                        }
                    }
                }
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

    /// register an additional import alias for a module that was already
    /// registered under `mname` (bug M7): the module body is emitted once, but
    /// qualified lookups under the new alias must resolve. Every table is keyed
    /// `"<qualifier>.<name>"`, and `register_import` always registers the
    /// module's own name as a qualifier, so copy the `mname.` prefix.
    pub(crate) fn register_module_alias(&mut self, mname: &str, alias: &str) {
        self.mod_alias.insert(alias.to_string(), mname.to_string());
        let pre = format!("{}.", mname);
        let ap = format!("{}.", alias);
        let cf: Vec<(String, (String, TyId))> = self
            .cross_funcs
            .iter()
            .filter(|(k, _)| k.starts_with(&pre))
            .map(|(k, v)| (format!("{}{}", ap, &k[pre.len()..]), v.clone()))
            .collect();
        for (k, v) in cf {
            self.cross_funcs.insert(k, v);
        }
        let ff: Vec<(String, (String, FuncDef))> = self
            .foreign_func_defs
            .iter()
            .filter(|(k, _)| k.starts_with(&pre))
            .map(|(k, v)| (format!("{}{}", ap, &k[pre.len()..]), v.clone()))
            .collect();
        for (k, v) in ff {
            self.foreign_func_defs.insert(k, v);
        }
        let fg: Vec<(String, (String, TyId, bool))> = self
            .fglobals
            .iter()
            .filter(|(k, _)| k.starts_with(&pre))
            .map(|(k, v)| (format!("{}{}", ap, &k[pre.len()..]), v.clone()))
            .collect();
        for (k, v) in fg {
            self.fglobals.insert(k, v);
        }
        let hs: Vec<String> = self
            .hidden
            .iter()
            .filter(|k| k.starts_with(&pre))
            .map(|k| format!("{}{}", ap, &k[pre.len()..]))
            .collect();
        for k in hs {
            self.hidden.insert(k);
        }
    }
}

/// stdlib `print` prelude, embedded from `lib/prelude/print.slt`. `print` is
/// implemented in Sloth on top of the runtime writer (`__sloth_rt_write` /
/// `__sloth_rt_puts`), not a compiler builtin. Injected into the root and every
/// imported module so bare `print` resolves locally.
pub(crate) const PRINT_PRELUDE: &str = include_str!("../../../../lib/prelude/print.slt");

pub(crate) fn inject_print_prelude(decls: &mut Vec<Decl>) {
    if decls
        .iter()
        .any(|d| d.name == "print" && matches!(d.node, DeclNode::Func(_)))
    {
        return;
    }
    if let Ok(stdp) = sloth_frontend::parser::parse(PRINT_PRELUDE) {
        for d in stdp.decls.into_iter().rev() {
            decls.insert(0, d);
        }
    }
}

/// Reserved runtime ABI surface (`__sloth_*`), embedded from
/// `lib/prelude/abi.slt`. This is the single declaration site for the runtime
/// externs the standard library calls; injected into every module (root and
/// imports) so stdlib bodies resolve `__sloth_*` without declaring it itself.
pub(crate) const ABI_PRELUDE: &str = include_str!("../../../../lib/prelude/abi.slt");

pub(crate) fn inject_abi_prelude(decls: &mut Vec<Decl>) {
    // already present? (root injects once; imports inject once each)
    if decls.iter().any(|d| {
        matches!(d.node, DeclNode::Func(ref f) if f.is_extern) && d.name == "__sloth_bytes_new"
    }) {
        return;
    }
    if let Ok(stdp) = sloth_frontend::parser::parse(ABI_PRELUDE) {
        for d in stdp.decls.into_iter().rev() {
            decls.insert(0, d);
        }
    }
}

/// Self-hosted Array/Map runtime, embedded from `lib/prelude/containers.slt`.
/// Injected once into the root module; the container ABI symbols below are
/// emitted under their raw names (no mangling) so the hardcoded
/// `@__sloth_arr_*`/`@__sloth_map_*` call sites in the emitter resolve here.
pub(crate) const CONTAINER_PRELUDE: &str = include_str!("../../../../lib/prelude/containers.slt");

/// container ABI symbols defined by the prelude (fixed, unmangled)
pub(crate) const CONTAINER_SYMS: &[&str] = &[
    "__sloth_arr_new",
    "__sloth_arr_new_k",
    "__sloth_arr_len",
    "__sloth_arr_get",
    "__sloth_arr_set",
    "__sloth_arr_push",
    "__sloth_arr_pop",
    "__sloth_arr_slice",
    "__sloth_arr_slice_set",
    "__sloth_map_new",
    "__sloth_map_len",
    "__sloth_map_get",
    "__sloth_map_set",
    "__sloth_map_get_h",
    "__sloth_map_set_h",
    "__sloth_map_str_get",
    "__sloth_map_str_set",
    "__sloth_map_keys",
    "__sloth_map_values",
];

/// Self-hosted range/value-box runtime, embedded from `lib/prelude/core.slt`.
/// Same mechanism as the container prelude: the hardcoded
/// `@__sloth_range_*`/`@__sloth_box_*` call sites resolve here.
pub(crate) const CORE_PRELUDE: &str = include_str!("../../../../lib/prelude/core.slt");

/// core prelude ABI symbols defined by the prelude (fixed, unmangled)
pub(crate) const CORE_SYMS: &[&str] = &[
    "__sloth_range_pack",
    "__sloth_range_lo",
    "__sloth_range_hi",
    "__sloth_box_new",
    "__sloth_box_get",
];

/// stdlib `Entry<K,V>` / `Result<T,E>` generic types, embedded from
/// `lib/prelude/result.slt`. Injected once into the root module; the emitter
/// recognizes `ok`/`err` calls against a declared `Result<T,E>` target.
pub(crate) const RESULT_PRELUDE: &str = include_str!("../../../../lib/prelude/result.slt");

/// stdlib `StrChars` lazy char iterator backing `s.chars()`, embedded from
/// `lib/prelude/strchars.slt`. Each `next()` yields the Unicode scalar value of
/// one character as an `int` (fixed 4-byte code point). Injected once per
/// emitter — into the root module, or into the first imported module when
/// `chars()` appears there before the root is collected (imported bodies emit
/// first).
pub(crate) const STRCHARS_PRELUDE: &str = include_str!("../../../../lib/prelude/strchars.slt");

/// prelude `__dispose__` routines whose address is taken (`fn_addr`): emitted
/// as `llvm.func` so `llvm.mlir.addressof` is legal
pub(crate) const CONTAINER_DISPOSERS: &[&str] = &["__sloth_arr_dispose", "__sloth_map_dispose"];

impl ModEmitter {
    /// inject the container + core preludes into the root module (once) and
    /// register their fixed symbols / addressable disposers
    pub(crate) fn inject_container_prelude(&mut self, decls: &mut Vec<Decl>) {
        let has_container = decls.iter().any(|d| {
            matches!(d.node, DeclNode::Func(_)) && CONTAINER_SYMS.contains(&d.name.as_str())
        });
        if !has_container {
            if let Ok(stdp) = sloth_frontend::parser::parse(CONTAINER_PRELUDE) {
                for s in CONTAINER_SYMS {
                    self.fixed_syms.insert((*s).to_string());
                }
                for s in CONTAINER_DISPOSERS {
                    // death-hook routines are taken by address (`fn_addr`), so
                    // they must keep their raw symbol too
                    self.fixed_syms.insert((*s).to_string());
                    self.addressable.insert((*s).to_string());
                }
                for d in stdp.decls.into_iter().rev() {
                    decls.insert(0, d);
                }
            }
        }
        let has_core = decls
            .iter()
            .any(|d| matches!(d.node, DeclNode::Func(_)) && CORE_SYMS.contains(&d.name.as_str()));
        if !has_core {
            if let Ok(stdp) = sloth_frontend::parser::parse(CORE_PRELUDE) {
                for s in CORE_SYMS {
                    self.fixed_syms.insert((*s).to_string());
                }
                for d in stdp.decls.into_iter().rev() {
                    decls.insert(0, d);
                }
            }
        }
    }

    /// inject the `StrChars` class (backing `s.chars()`) once per emitter.
    /// `emit_module` covers the single-module case; `compile_multimod` calls
    /// this for each imported module so bodies that use `chars()` emit before
    /// the root is collected. Guarded on `self.classes` so the class is
    /// registered exactly once in the shared emitter.
    pub(crate) fn inject_strchars_prelude(&mut self, decls: &mut Vec<Decl>) {
        if self.classes.contains_key("StrChars")
            || decls
                .iter()
                .any(|d| d.name == "StrChars" && matches!(d.node, DeclNode::Class(_)))
        {
            return;
        }
        if let Ok(stdp) = sloth_frontend::parser::parse(STRCHARS_PRELUDE) {
            for d in stdp.decls.into_iter().rev() {
                decls.insert(0, d);
            }
        }
    }
}

pub fn rt_decls() -> String {
    let mut s = String::new();
    // rc core (ARC migration patch B): counting primitives
    s.push_str("  func.func private @__sloth_rc_retain(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_rc_release(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_rc_live() -> i64\n");
    s.push_str("  func.func private @__sloth_rc_drops() -> i64\n");
    s.push_str("  func.func private @__sloth_weak_new(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_weak_upgrade(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_weak_release(i64) -> i64\n");
    // patch 42: value-optional payload boxes + nil-aware print/interp faces.
    // The box core (`__sloth_box_new`/`__sloth_box_get`) is self-hosted in
    // `lib/prelude/core.slt` — no private declarations here.
    s.push_str("  func.func private @__sloth_rt_print_opt(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_push_opt(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_rt_print_i64(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_rt_print_f64(f64) -> i64\n");
    s.push_str("  func.func private @__sloth_rt_print_bool(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_rt_print_str(i64) -> i64\n");
    // `any` top type + runtime renderer
    s.push_str("  func.func private @__sloth_any_from(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_desc(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_word(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_kind(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_cls_id(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_ref(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_retain(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_is(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_type_id(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_any_type_name(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_rt_write(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_rt_puts(i64) -> ()\n");
    s.push_str("  func.func private @__sloth_str_intern(i64, i64) -> i64\n");
    // range ABI (`__sloth_range_pack/lo/hi`) is self-hosted in
    // `lib/prelude/core.slt` — no private declarations here.
    s.push_str("  func.func private @__sloth_str_push(i64, i64, i64) -> i64\n");
    s.push_str(
        "  func.func private @__sloth_str_finish(i64) -> i64
  func.func private @__sloth_str_pushp(i64, i64) -> i64
  func.func private @__sloth_str_push_i(i64, i64) -> i64
  func.func private @__sloth_str_push_f(i64, f64) -> i64
  func.func private @__sloth_str_push_b(i64, i64) -> i64\n",
    );
    s.push_str("  func.func private @__sloth_str_len(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_clen(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_char(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_codepoint(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_byte(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_slice(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_concat(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_str_eq(i64, i64) -> i64\n");
    // Array/Map ABI is defined by the self-hosted container prelude
    // (`lib/prelude/containers.slt`, injected in emit_module): no runtime
    // declarations here, so the prelude definitions are the single source.
    // scalar math faces (design D6): libm wrappers for sloth source calls
    s.push_str(
        "  func.func private @__sloth_rt_sqrt(f64) -> f64
  func.func private @__sloth_rt_exp(f64) -> f64
  func.func private @__sloth_rt_sin(f64) -> f64
  func.func private @__sloth_rt_cos(f64) -> f64
  func.func private @__sloth_rt_tan(f64) -> f64
  func.func private @__sloth_rt_pow(f64, f64) -> f64
  func.func private @__sloth_rt_floor(f64) -> f64\n",
    );
    // tensor extension TE-P1: descriptor + views + element access
    s.push_str(
        "  func.func private @__sloth_tensor_new_1(i64, i64) -> i64
  func.func private @__sloth_tensor_new_2(i64, i64, i64) -> i64
  func.func private @__sloth_tensor_new_3(i64, i64, i64, i64) -> i64
  func.func private @__sloth_tensor_view(i64, i64, i64, i64) -> i64
  func.func private @__sloth_tensor_get1(i64, i64) -> i64
  func.func private @__sloth_tensor_set1(i64, i64, i64) -> i64
  func.func private @__sloth_tensor_copy_into(i64, i64) -> i64
  func.func private @__sloth_tensor_copy_from_array(i64, i64) -> i64
  func.func private @__sloth_tensor_rank(i64) -> i64
  func.func private @__sloth_tensor_dim(i64, i64) -> i64
  func.func private @__sloth_tensor_stride(i64, i64) -> i64
  func.func private @__sloth_tensor_fill_zero(i64) -> i64
  func.func private @__sloth_tensor_basis_f64(i64) -> memref<?xf64, strided<[?], offset: ?>>
  func.func private @__sloth_tensor_basis_i64(i64) -> memref<?xi64, strided<[?], offset: ?>>
  func.func private @__sloth_tensor_shape_eq(i64, i64) -> i64
  func.func private @__sloth_tensor_dim_eq(i64, i64, i64, i64) -> i64\n",
    );
    // coroutine extension CE-P1: stackful fiber entry points
    s.push_str(
        "  func.func private @__sloth_fiber_create(i64, i64, i64) -> i64
  func.func private @__sloth_fiber_create_with(i64, i64, i64, i64) -> i64
  func.func private @__sloth_fiber_resume(i64, i64, i64) -> i64
  func.func private @__sloth_fiber_transfer(i64, i64, i64) -> i64
  func.func private @__sloth_fiber_yield(i64) -> i64
  func.func private @__sloth_fiber_error(i64) -> i64
  func.func private @__sloth_fiber_check(i64) -> i64
  func.func private @__sloth_fiber_resumable(i64) -> i64
  func.func private @__sloth_fiber_cancel(i64) -> i64
  func.func private @__sloth_fiber_cancelled() -> i64
  func.func private @__sloth_fiber_cancel_abort() -> ()
  func.func private @__sloth_fiber_track(i64) -> i64
  func.func private @__sloth_fiber_untrack(i64) -> i64\n",
    );
    // multithreading extension TH-P1/P2: threads, channels, mutexes, atomics
    s.push_str(
        "  func.func private @__sloth_thread_spawn(i64, i64, i64, i64) -> i64
  func.func private @__sloth_thread_join(i64) -> i64
  func.func private @__sloth_thread_detach(i64) -> i64
  func.func private @__sloth_thread_current_id() -> i64
  func.func private @__sloth_thread_yield_now() -> i64
  func.func private @__sloth_chan_new(i64, i64) -> i64
  func.func private @__sloth_chan_send(i64, i64) -> i64
  func.func private @__sloth_chan_recv(i64, i64) -> i64
  func.func private @__sloth_chan_close(i64) -> i64
  func.func private @__sloth_mutex_new() -> i64
  func.func private @__sloth_mutex_lock(i64) -> i64
  func.func private @__sloth_mutex_unlock(i64) -> i64
  func.func private @__sloth_mutex_try_lock(i64) -> i64
  func.func private @__sloth_mutex_with(i64, i64) -> i64
  func.func private @__sloth_atomic_new(i64) -> i64
  func.func private @__sloth_atomic_load(i64) -> i64
  func.func private @__sloth_atomic_store(i64, i64) -> i64
  func.func private @__sloth_atomic_add(i64, i64) -> i64
  func.func private @__sloth_atomic_sub(i64, i64) -> i64
  func.func private @__sloth_atomic_cas(i64, i64, i64) -> i64\n",
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

/// byte globals backing `type_name` results (NUL-terminated; the runtime is
/// handed the explicit length so the terminator is only a safety cap)
pub(crate) fn emit_tyname_globals(me: &ModEmitter) -> String {
    let mut out = String::new();
    for (i, s) in me.tyname_pool.iter().enumerate() {
        let mut bytes: Vec<u8> = s.bytes().collect();
        bytes.push(0);
        let n = bytes.len();
        let mut lit = String::new();
        for b in &bytes {
            match b {
                b'"' => lit.push_str("\\\""),
                b'\\' => lit.push_str("\\\\"),
                0x20..=0x7e => lit.push(*b as char),
                _ => lit.push_str(&format!("\\{:02X}", b)),
            }
        }
        out.push_str(&format!(
            "  llvm.mlir.global private constant @sloth_tynm_{}(\"{}\") : !llvm.array<{} x i8>\n",
            i, lit, n
        ));
    }
    out
}

impl ModEmitter {
    /// reserve (or fetch) a pooled byte global for a type-name string
    pub(crate) fn declare_tyname_global(&mut self, s: &str) -> String {
        if let Some(sym) = self.tyname_syms.get(s) {
            return sym.clone();
        }
        let sym = format!("sloth_tynm_{}", self.tyname_pool.len());
        self.tyname_pool.push(s.to_string());
        self.tyname_syms.insert(s.to_string(), sym.clone());
        sym
    }

    /// emit the raw address word (i64) of a pooled type-name byte global
    pub(crate) fn emit_tyname_ptr(&mut self, fw: &mut FnWalk, s: &str) -> String {
        let sym = self.declare_tyname_global(s);
        let a = fw.v();
        fw.op(&format!(
            "    {} = llvm.mlir.addressof @{} : !llvm.ptr",
            a, sym
        ));
        let p = fw.v();
        fw.op(&format!(
            "    {} = llvm.ptrtoint {} : !llvm.ptr to i64",
            p, a
        ));
        p
    }

    /// materialize a fresh owned `str` naming a statically-known reference
    /// type; `v` guards nil (returns "nil"). Marks the result a producer.
    pub(crate) fn emit_tyname_lit(&mut self, fw: &mut FnWalk, v: &str, s: &str) -> String {
        let p = self.emit_tyname_ptr(fw, s);
        let lc = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            lc,
            enc_i_lit(s.len() as i64)
        ));
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_type_name_or({}, {}, {}) : (i64, i64, i64) -> i64",
            r, v, p, lc
        ));
        let t = self.r.mk(Ty::Str);
        self.dangling_producer(fw, &r, t);
        r
    }
}

impl ModEmitter {
    pub fn emit_module(&mut self, prog: &Program) -> Vec<Diag> {
        // reserved `__sloth_*` gate: check the raw module before any prelude
        // (which legitimately declares/uses reserved symbols) is injected
        let allowed = has_sloth_import(&prog.imports);
        self.check_reserved_module(prog, allowed);
        // stdlib Entry<K,V> / Result<T,E> prelude (only injected once)
        let mut decls2: Vec<Decl> = prog.decls.clone();
        if !decls2
            .iter()
            .any(|d| d.name == "Result" && matches!(d.node, DeclNode::Class(_)))
        {
            match sloth_frontend::parser::parse(RESULT_PRELUDE) {
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
        // stdlib StrChars lazy char iterator for `s.chars()` (injected once)
        self.inject_strchars_prelude(&mut decls2);
        let stmts2 = prog.stmts.clone();
        let imps2 = prog.imports.clone();
        inject_print_prelude(&mut decls2);
        inject_abi_prelude(&mut decls2);
        self.inject_container_prelude(&mut decls2);
        let prog = &mut Program {
            decls: decls2,
            stmts: stmts2,
            imports: imps2,
        };
        self.register_override_index(&[&*prog]);
        self.collect(prog);
        self.finalize_vt();
        // 0) local module init: store top-level var initializers into the
        //    module's global cells. Emitted for every module so @sloth_main
        //    can invoke it unconditionally before running the user body.
        {
            let gframe = format!("{}__ginit", self.name);
            let saved_frame = std::mem::replace(&mut self.cur_frame, gframe);
            let mut fw = fresh_walk(self);
            for d in &prog.decls {
                if let DeclNode::Var { ty, init } = &d.node {
                    let (sym, t) = match self.globals.get(&d.name).cloned() {
                        Some((s, t, _)) => (s, t),
                        None => continue,
                    };
                    let (v, vt) = self.emit_expr(&mut fw, init);
                    // unannotated global: infer its surface from the initializer
                    // so later reads/assignments carry the real word kind
                    if ty.is_none() {
                        if let Some(g) = self.globals.get_mut(&d.name) {
                            g.1 = vt;
                        }
                    }
                    let mty = memref_cell_ty(self, t);
                    let g = fw.v();
                    fw.op(&format!("    {} = memref.get_global @{} : {}", g, sym, mty));
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", z));
                    fw.op(&format!("    memref.store {}, {}[{}] : {}", v, g, z, mty));
                }
            }
            let body = fw.cur.clone();
            self.out.push_str(&format!(
                "  func.func @sloth_{}__ginit() -> () {{\n    func.call @sloth_{}__anyinit() : () -> ()\n{}    return\n  }}\n",
                self.name, self.name, body
            ));
            self.cur_frame = saved_frame;
        }
        // 1) top-level funcs. Generic functions emit no template body — only
        // their monomorphic instances are real; emitting a template would type
        // its body against unresolved type params (bogus nested-bound failures
        // and wrong ref ARC). Every call site monomorphizes instead.
        for d in &prog.decls {
            if let DeclNode::Func(f) = &d.node {
                if !f.type_params.is_empty() {
                    continue;
                }
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
        // 1c) monomorphic instances: drain the generic fn/class worklists to a
        // fixed point (eager monomorphization, A1)
        self.emit_pending_instances();
        // 2) script statements run in entry if no main() was declared
        let has_main = prog
            .decls
            .iter()
            .any(|d| d.name == "main" && matches!(d.node, DeclNode::Func(_)));
        if !has_main {
            let sframe = format!("{}__script", self.name);
            let saved_frame = std::mem::replace(&mut self.cur_frame, sframe);
            let mut fw = FnWalk {
                cur: String::new(),
                vcount: 1000,
                scopes: vec![HashMap::new()],
                imms: vec![HashMap::new()],
                scope_decls: vec![HashMap::new()],
                dangling: Vec::new(),
                loops: Vec::new(),
                loop_bases: Vec::new(),
                loopvars: Vec::new(),
                loop_owned_elems: Vec::new(),
                xfer: Vec::new(),
                lambda_env: None,
                fiber_payload: None,
                params: std::collections::HashSet::new(),
                param_owned: std::collections::HashSet::new(),
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
            let init_mods = self.init_mods.clone();
            for m in &init_mods {
                fw.op(&format!("    func.call @sloth_{}__ginit() : () -> ()", m));
            }
            // script mode skips the local ginit; initialise local descriptors
            let mname = self.name.clone();
            fw.op(&format!(
                "    func.call @sloth_{}__anyinit() : () -> ()",
                mname
            ));
            // top-level var/let decls become prelude statements (script mode
            // keeps the historical local-slot route so container/lambda
            // writeback and declaration checking work unchanged)
            for d in &prog.decls {
                if let DeclNode::Var { ty, init } = &d.node {
                    let st = Stmt {
                        id: 0,
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
            self.cur_frame = saved_frame;
        }
        // script/top-level statements can themselves trigger instances; drain
        // the worklists once more so nothing discovered late is left unemitted
        self.emit_pending_instances();
        let diag = self.diags.clone();
        diag
    }

    /// Drain the generic-function and generic-class worklists to a fixed point:
    /// emitting one instance body can discover further instances, which are
    /// appended to the worklists and picked up by the outer loop. This is the
    /// eager-monomorphization core (A1): Pass 1 reaches the complete instance
    /// set here, and Pass 2 replays it from the frozen plan.
    ///
    /// Class instances are re-scanned each round: a queued instance whose class
    /// surface is not registered yet (a Pass-2 plan entry discovered only
    /// transitively) is retried after the enclosing body registers it, instead
    /// of being dropped.
    pub(crate) fn emit_pending_instances(&mut self) {
        let mut fn_idx = 0usize;
        let mut cls_done: HashSet<String> = HashSet::new();
        loop {
            let mut progress = false;
            // generic-function instances (append-only; the index is stable)
            while fn_idx < self.pending_fn_insts.len() {
                let inst = self.pending_fn_insts[fn_idx].clone();
                fn_idx += 1;
                progress = true;
                self.tp_subst.push(inst.frame.clone());
                self.tp_mangled.push(inst.mangled.clone());
                let saved_mod = self.cur_mod.clone();
                self.cur_mod = inst.module.clone();
                self.emit_func(&inst.name, None, &inst.fd, None, false);
                self.cur_mod = saved_mod;
                self.tp_subst.pop();
                self.tp_mangled.pop();
            }
            // generic-class instances: methods emitted under the T-frame
            let mut i = 0usize;
            while i < self.pending_insts.len() {
                let (inst, frame) = self.pending_insts[i].clone();
                i += 1;
                if cls_done.contains(&inst) {
                    continue;
                }
                let meths = match self.classes.get(&inst) {
                    // not registered yet: retry next round
                    None => continue,
                    Some(ci) => ci.methods.clone(),
                };
                cls_done.insert(inst.clone());
                progress = true;
                self.tp_subst.push(frame);
                // emit the instance's methods under its *defining* module: the
                // ctor/method call sites mangle with `cls_mod[inst]`, so the
                // definition must match (bug M1)
                let saved_mod = self.cur_mod.clone();
                self.cur_mod = self
                    .cls_mod
                    .get(&inst)
                    .cloned()
                    .unwrap_or_else(|| self.name.clone());
                for (mname, fd) in meths {
                    self.emit_func(&mname, Some(&inst), &fd, None, false);
                }
                self.cur_mod = saved_mod;
                self.tp_subst.pop();
            }
            if !progress {
                break;
            }
        }
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
            let init = "dense<0>";
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
                "sloth-stats: module={} direct-method-calls={} dyn-calls={} class-vt-calls={} generic-instances={} extern-decls={} per-cls-vtables={}",
                me.name, me.stat_dcalls, me.stat_dyncalls, me.stat_cvcalls, me.stat_ginsts, me.stat_extdecls, me.stat_vtbuilds
            );
        }
        let mut m = format!("module @{} {{\n", me.name);
        m.push_str(&emit_str_globals(me));
        m.push_str(&emit_tyname_globals(me));
        m.push_str(&super::anydesc::emit_any_desc_globals(me));
        m.push_str(&rt_decls());
        m.push_str(&obj_rt_decls());
        for gd in &me.global_decls {
            m.push_str(gd);
        }
        m.push_str("\n");
        m.push_str(&me.out);
        m.push_str(&super::anydesc::emit_any_disp_wrappers(me));
        m.push_str(&format!(
            "  func.func @sloth_{}__anyinit() -> () {{\n{}    return\n  }}\n",
            me.name,
            super::anydesc::emit_anyinit(me)
        ));
        // now the out is func bodies only; globals were prepended
        // (we already integrated globals above; emit closing brace)
        m.push_str("}\n");
        m
    }
}

pub fn obj_rt_decls() -> String {
    let mut s = String::new();
    s.push_str("  func.func private @__sloth_obj_new(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_closure_new(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_obj_field(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_obj_set_field(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_cls_info(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_cls_name(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_obj_type_name(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_type_name_or(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_obj_cls_id(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_vt_new(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_vt_set(i64, i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_vt_get(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_obj_set_vtable(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_obj_vtable(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_panic_noimpl(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_panic_divzero() -> i64\n");
    s.push_str("  func.func private @__sloth_builtin_info(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_dyn_unbox(i64) -> i64\n");
    s.push_str("  func.func private @__sloth_dyn_to_str(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_dyn_hash(i64, i64) -> i64\n");
    s.push_str("  func.func private @__sloth_dyn_binop(i64, i64, i64, i64) -> i64\n");
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

    /// is `cls` defined in another module than the one currently emitted?
    pub(crate) fn foreign_cls(&self, cls: &str) -> bool {
        self.cls_mod
            .get(cls)
            .map(|m| m != &self.cur_mod)
            .unwrap_or(false)
    }

    /// a class registered as non-`pub` by an import (bug M4); instance names
    /// (`Box_int`) match their generic base (`Box`) by the mangle separator
    pub(crate) fn class_is_hidden(&self, cls: &str) -> bool {
        self.hidden_classes.contains(cls)
            || self
                .hidden_classes
                .iter()
                .any(|h| cls.starts_with(h.as_str()) && cls[h.len()..].starts_with('_'))
    }

    pub(crate) fn guard_class(&mut self, cls: &str, pos: &Pos) {
        if self.foreign_cls(cls) && self.class_is_hidden(cls) {
            self.err(
                pos,
                format!("class `{}` is private to its module (not `pub`)", cls),
            );
        }
    }
}

/// Reserved namespace prefix for the runtime/prelude ABI surface. Only the
/// compiler-injected prelude may declare these symbols; only modules carrying
/// the `import "__sloth";` pseudo-import may call them.
pub(crate) const RESERVED_PREFIX: &str = "__sloth_";

/// Pseudo-import that grants a module the right to call `__sloth_*` symbols.
pub(crate) const SLOTH_IMPORT: &str = "__sloth";

pub(crate) fn has_sloth_import(imports: &[Import]) -> bool {
    imports.iter().any(|i| i.path == SLOTH_IMPORT)
}

impl ModEmitter {
    /// reserved-symbol gate: `__sloth_*` declarations are prelude-only and
    /// `__sloth_*` uses require the `import "__sloth";` token. Must run on the
    /// raw parsed module (before the compiler injects prelude decls that
    /// legitimately use reserved symbols).
    pub(crate) fn check_reserved_module(&mut self, prog: &Program, allowed: bool) {
        for d in &prog.decls {
            self.check_reserved_decl(d, allowed);
        }
        for s in &prog.stmts {
            self.check_reserved_stmt(s, allowed);
        }
    }

    fn check_reserved_decl(&mut self, d: &Decl, allowed: bool) {
        self.check_reserved_name(&d.name, &d.pos);
        match &d.node {
            DeclNode::Func(f) => self.check_reserved_func(f, &d.pos, allowed),
            DeclNode::Class(c) => {
                for fd in &c.fields {
                    self.check_reserved_name(&fd.name, &d.pos);
                    if let Some(init) = &fd.init {
                        self.check_reserved_expr(init, allowed);
                    }
                }
                for m in &c.methods {
                    self.check_reserved_name(&m.name, &d.pos);
                    self.check_reserved_func(&m.fd, &d.pos, allowed);
                }
            }
            DeclNode::Trait(t) => {
                for m in &t.methods {
                    self.check_reserved_name(&m.name, &d.pos);
                    for p in &m.params {
                        self.check_reserved_name(&p.name, &d.pos);
                    }
                    if let Some(body) = &m.body {
                        self.check_reserved_stmt(body, allowed);
                    }
                }
            }
            DeclNode::Var { init, .. } => self.check_reserved_expr(init, allowed),
            DeclNode::ExternType => {}
        }
    }

    fn check_reserved_func(&mut self, f: &FuncDef, pos: &Pos, allowed: bool) {
        for p in &f.params {
            self.check_reserved_name(&p.name, pos);
        }
        if let Some(v) = &f.variadic {
            self.check_reserved_name(&v.name, pos);
        }
        self.check_reserved_stmt(&f.body, allowed);
    }

    /// a declaration site (function/field/param/local/binding) of a reserved
    /// name is always rejected — the token only enables calls
    fn check_reserved_name(&mut self, name: &str, pos: &Pos) {
        if name.starts_with(RESERVED_PREFIX) {
            self.err(
                pos,
                format!(
                    "reserved symbol `{}` may only be declared in the prelude",
                    name
                ),
            );
        }
    }

    fn check_reserved_use(&mut self, name: &str, pos: &Pos, allowed: bool) {
        if !allowed && name.starts_with(RESERVED_PREFIX) {
            self.err(
                pos,
                format!(
                    "reserved symbol `{}` may only be called from a module that imports \"__sloth\"",
                    name
                ),
            );
        }
    }

    fn check_reserved_stmt(&mut self, s: &Stmt, allowed: bool) {
        match &s.node {
            StmtNode::Expr(e) => self.check_reserved_expr(e, allowed),
            StmtNode::Let { name, init, .. } => {
                self.check_reserved_name(name, &s.pos);
                self.check_reserved_expr(init, allowed);
            }
            StmtNode::Assign { target, value } | StmtNode::AssignOp { target, value, .. } => {
                self.check_reserved_expr(value, allowed);
                for seg in target {
                    match seg {
                        PathSeg::Name(n) => self.check_reserved_use(n, &s.pos, allowed),
                        PathSeg::Index(e) => self.check_reserved_expr(e, allowed),
                    }
                }
            }
            StmtNode::While { cond, body } => {
                self.check_reserved_expr(cond, allowed);
                self.check_reserved_stmt(body, allowed);
            }
            StmtNode::If { cond, then_, else_ } => {
                self.check_reserved_expr(cond, allowed);
                self.check_reserved_stmt(then_, allowed);
                if let Some(els) = else_ {
                    self.check_reserved_stmt(els, allowed);
                }
            }
            StmtNode::For { var, iter, body } => {
                self.check_reserved_name(var, &s.pos);
                self.check_reserved_expr(iter, allowed);
                self.check_reserved_stmt(body, allowed);
            }
            StmtNode::Return(Some(e)) => self.check_reserved_expr(e, allowed),
            StmtNode::Break | StmtNode::Continue | StmtNode::Return(None) => {}
            StmtNode::Block(ss) => {
                for st in ss {
                    self.check_reserved_stmt(st, allowed);
                }
            }
        }
    }

    fn check_reserved_expr(&mut self, e: &Expr, allowed: bool) {
        match &e.node {
            ExprNode::Ident(n) => self.check_reserved_use(n, &e.pos, allowed),
            ExprNode::Call { callee, args } | ExprNode::GenCall { callee, args, .. } => {
                self.check_reserved_expr(callee, allowed);
                for a in args {
                    self.check_reserved_expr(a, allowed);
                }
            }
            ExprNode::Map(pairs) => {
                for (k, v) in pairs {
                    self.check_reserved_expr(k, allowed);
                    self.check_reserved_expr(v, allowed);
                }
            }
            ExprNode::List(xs) => {
                for x in xs {
                    self.check_reserved_expr(x, allowed);
                }
            }
            ExprNode::Range { low, high, .. } => {
                self.check_reserved_expr(low, allowed);
                self.check_reserved_expr(high, allowed);
            }
            ExprNode::Pipe { lhs, rhs } | ExprNode::Elvis { lhs, rhs } => {
                self.check_reserved_expr(lhs, allowed);
                self.check_reserved_expr(rhs, allowed);
            }
            ExprNode::Is { lhs, rhs, .. } => {
                self.check_reserved_expr(lhs, allowed);
                self.check_reserved_expr(rhs, allowed);
            }
            ExprNode::Index { obj, idx } => {
                self.check_reserved_expr(obj, allowed);
                self.check_reserved_expr(idx, allowed);
            }
            ExprNode::Field { obj, name } => {
                self.check_reserved_expr(obj, allowed);
                self.check_reserved_use(name, &e.pos, allowed);
            }
            ExprNode::Arith { lhs, rhs, .. } | ExprNode::Bin { lhs, rhs, .. } => {
                self.check_reserved_expr(lhs, allowed);
                self.check_reserved_expr(rhs, allowed);
            }
            ExprNode::Un { expr, .. } => self.check_reserved_expr(expr, allowed),
            ExprNode::Str(sp) => {
                for p in &sp.parts {
                    if let StrPart::ExprAst(inner) = p {
                        self.check_reserved_expr(inner, allowed);
                    }
                }
            }
            ExprNode::Lambda(l) => {
                for p in &l.params {
                    self.check_reserved_name(&p.name, &e.pos);
                }
                self.check_reserved_stmt(&l.body, allowed);
            }
            ExprNode::Int(_)
            | ExprNode::UInt(_)
            | ExprNode::Float(_)
            | ExprNode::Bool(_)
            | ExprNode::Nil
            | ExprNode::This
            | ExprNode::Super => {}
        }
    }
}
