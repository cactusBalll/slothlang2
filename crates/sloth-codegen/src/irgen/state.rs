//! Emitter state: `ModEmitter` tables + class registry structs.

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
    pub stat_vtbuilds: usize,
    /// registration order of classes (deterministic dyn-dispatch chain)
    pub class_order: Vec<String>,
    /// vtable slot assignment: (trait, method) -> (index, ret-float, ret-unit)
    pub vt_slots: HashMap<(String, String), usize>,
    /// per-cls vtable builders already emitted (patch #28 global cache)
    pub vt_built: std::collections::HashSet<String>,
    /// methods emitted inside llvm.func (vtable-addressable)
    pub llvm_method: std::collections::HashSet<(String, String)>,
    /// object bodies carry a fixed vtable capacity (total slots)
    pub vt_cap: usize,
    /// active type-param substitution for the generic instance being emitted
    /// (stacked for nesting; ty positions resolve T against the top frame)
    pub(crate) tp_subst: Vec<HashMap<String, TyId>>,
    /// mangled instance name forced for the next plan_func/emit_func
    pub(crate) tp_mangled: Vec<String>,
    /// generic instance cache: base mangled -> concrete word spelling key
    pub(crate) insts: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub fields: Vec<(String, TyId, bool)>, // (name, ty, mutable)
    pub methods: Vec<(String, FuncDef)>,
    pub superclass: Option<String>,
    pub impls: Vec<String>,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum IdxKind {
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
            stat_vtbuilds: 0,
            pending_insts: Vec::new(),
            native_cls_id: 0,
            result_insts: std::collections::HashSet::new(),
            extern_types: std::collections::HashSet::new(),
            class_frames: HashMap::new(),
            class_order: Vec::new(),
            vt_slots: HashMap::new(),
            vt_built: std::collections::HashSet::new(),
            llvm_method: std::collections::HashSet::new(),
            vt_cap: 0,
            tp_subst: Vec::new(),
            tp_mangled: Vec::new(),
            insts: std::collections::HashMap::new(),
        }
    }
}

impl ModEmitter {
    pub(crate) fn err(&mut self, pos: &Pos, msg: String) {
        self.diags.push(Diag {
            line: pos.line,
            col: pos.col,
            msg,
        });
    }
}
