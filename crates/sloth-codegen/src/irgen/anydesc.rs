//! Compiler-emitted `any` type descriptors + the module's descriptor-init
//! function. Each descriptor is a `memref<11xi64>` global whose fields mirror
//! `crates/sloth-rt/src/any.rs`; `__anyinit` fills them at module init so the
//! runtime can walk child pointers (avoids pointer-in-global-initializer).

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use sloth_frontend::ast::*;
#[allow(unused_imports)]
use sloth_frontend::lexer::Pos;
#[allow(unused_imports)]
use sloth_frontend::ty::{Ty, TyId};

// kind codes (mirror sloth-rt/src/any.rs)
pub(crate) const AK_UNIT: i64 = 0;
pub(crate) const AK_BOOL: i64 = 1;
pub(crate) const AK_I64: i64 = 2;
pub(crate) const AK_F64: i64 = 3;
pub(crate) const AK_STR: i64 = 4;
pub(crate) const AK_RANGE: i64 = 5;
pub(crate) const AK_ARRAY: i64 = 6;
pub(crate) const AK_MAP: i64 = 7;
pub(crate) const AK_OPT: i64 = 8;
pub(crate) const AK_TENSOR: i64 = 9;
pub(crate) const AK_FN: i64 = 10;
pub(crate) const AK_NAMED: i64 = 11;
pub(crate) const AK_DYN: i64 = 12;
pub(crate) const AK_WEAK: i64 = 13;
pub(crate) const AK_FIBER: i64 = 14;
pub(crate) const AK_JOINHANDLE: i64 = 15;
pub(crate) const AK_CHANNEL: i64 = 16;
pub(crate) const AK_MUTEX: i64 = 17;
pub(crate) const AK_ATOMICINT: i64 = 18;
pub(crate) const AK_ANY: i64 = 19;
pub(crate) const AK_U64: i64 = 20;
pub(crate) const AK_I8: i64 = 21;
pub(crate) const AK_U8: i64 = 22;
pub(crate) const AK_I16: i64 = 23;
pub(crate) const AK_U16: i64 = 24;
pub(crate) const AK_I32: i64 = 25;
pub(crate) const AK_U32: i64 = 26;

pub(crate) const FLAG_REF: i64 = 1;
pub(crate) const FLAG_NILCAP: i64 = 2;

pub(crate) const DESC_WORDS: usize = 11;

/// `any` descriptor kind for a fixed-width integer surface
pub(crate) fn ak_for_int(k: sloth_frontend::ty::IntKind) -> i64 {
    use sloth_frontend::ty::IntKind::*;
    match k {
        U64 => AK_U64,
        I8 => AK_I8,
        U8 => AK_U8,
        I16 => AK_I16,
        U16 => AK_U16,
        I32 => AK_I32,
        U32 => AK_U32,
    }
}

/// `any` descriptor kind for a fixed-width integer source spelling
pub(crate) fn ak_for_int_name(n: &str) -> Option<i64> {
    sloth_frontend::ty::IntKind::from_name(n).map(ak_for_int)
}

impl ModEmitter {
    fn any_kind_of(&self, t: TyId) -> i64 {
        match self.r.get(t) {
            Ty::Unit => AK_UNIT,
            Ty::Bool => AK_BOOL,
            Ty::I64 => AK_I64,
            Ty::Int(k) => ak_for_int(*k),
            Ty::F64 => AK_F64,
            Ty::Str => AK_STR,
            Ty::Range => AK_RANGE,
            Ty::Array(_) => AK_ARRAY,
            Ty::Map(_, _) => AK_MAP,
            Ty::Opt(_) => AK_OPT,
            Ty::Tensor(_, _) => AK_TENSOR,
            Ty::Fn(_) => AK_FN,
            Ty::Named(_, _) => AK_NAMED,
            Ty::Dyn(_) => AK_DYN,
            Ty::Weak(_) => AK_WEAK,
            Ty::Fiber(_) => AK_FIBER,
            Ty::JoinHandle(_) => AK_JOINHANDLE,
            Ty::Channel(_) => AK_CHANNEL,
            Ty::Mutex => AK_MUTEX,
            Ty::AtomicInt => AK_ATOMICINT,
            Ty::Any => AK_ANY,
            Ty::Tp(_) => AK_ANY,
        }
    }

    /// descriptor spec for an object/dyn surface: Display's `to_str` through
    /// the receiver's vtable (the slot is global, populated when the class
    /// implements Display).
    fn any_disp_spec(&mut self, t: TyId) -> Option<DispSpec> {
        match self.r.get(t).clone() {
            Ty::Named(cls, _) => {
                if !self.satisfies_bound(t, "Display") {
                    return None;
                }
                if self.find_method(&cls, "to_str").is_none() {
                    return None;
                }
                let slot = self.vt_slot("Display", "to_str");
                Some(DispSpec::Dyn { slot })
            }
            Ty::Dyn(tn) => {
                let has = self
                    .traits
                    .get(&tn)
                    .map(|ms| ms.iter().any(|m| m.name == "to_str"))
                    .unwrap_or(false);
                if !has {
                    return None;
                }
                let slot = self.vt_slot(&tn, "to_str");
                Some(DispSpec::Dyn { slot })
            }
            _ => None,
        }
    }

    /// register (or fetch) the structural descriptor for `t`; returns the
    /// descriptor global symbol (no leading `@`).
    pub(crate) fn declare_any_desc(&mut self, t: TyId) -> String {
        let key = sloth_frontend::ty::ty_key(self.r.get(t));
        if let Some(s) = self.anydesc_syms.get(&key) {
            return s.clone();
        }
        let idx = self.anydescs.len();
        let sym = format!("sloth_anyd_{}", idx);
        self.anydesc_syms.insert(key.clone(), sym.clone());
        // placeholder to break any accidental re-entry
        self.anydescs.push(AnyDesc {
            sym: sym.clone(),
            kind: 0,
            flags: 0,
            type_id: 0,
            name: None,
            elem: None,
            key: None,
            val: None,
            disp: None,
            cls_id: 0,
            rank: 0,
        });
        let kind = self.any_kind_of(t);
        let flags = (if self.is_ref(t) { FLAG_REF } else { 0 })
            | (if self.nil_capable(t) { FLAG_NILCAP } else { 0 });
        let type_id = self.typeid_const(&key);
        let name = {
            let n = self.pretty_ty(t);
            let len = n.len();
            Some((self.declare_tyname_global(&n), len))
        };
        let (elem, keyd, vald, rank) = match self.r.get(t).clone() {
            Ty::Array(e) => (Some(self.declare_any_desc(e)), None, None, 0),
            Ty::Opt(e) => (Some(self.declare_any_desc(e)), None, None, 0),
            Ty::Tensor(e, r) => (Some(self.declare_any_desc(e)), None, None, r as i64),
            Ty::Map(k, v) => {
                let ks = self.declare_any_desc(k);
                let vs = self.declare_any_desc(v);
                (None, Some(ks), Some(vs), 0)
            }
            _ => (None, None, None, 0),
        };
        let disp = self.any_disp_spec(t);
        self.anydescs[idx] = AnyDesc {
            sym: sym.clone(),
            kind,
            flags,
            type_id,
            name,
            elem,
            key: keyd,
            val: vald,
            disp,
            cls_id: 0,
            rank,
        };
        sym
    }

    /// emit the i64 address of a type's descriptor (materialising it on demand)
    pub(crate) fn emit_any_desc_ptr(&mut self, fw: &mut FnWalk, t: TyId) -> String {
        let sym = self.declare_any_desc(t);
        let g = fw.v();
        fw.op(&format!(
            "    {} = memref.get_global @{} : memref<{}xi64>",
            g, sym, DESC_WORDS
        ));
        let i = fw.v();
        fw.op(&format!(
            "    {} = memref.extract_aligned_pointer_as_index {} : memref<{}xi64> -> index",
            i, g, DESC_WORDS
        ));
        let v = fw.v();
        fw.op(&format!(
            "    {} = arith.index_cast {} : index to i64",
            v, i
        ));
        v
    }
}

/// the descriptor global declarations (memref cells, zero initialised)
pub(crate) fn emit_any_desc_globals(me: &ModEmitter) -> String {
    let mut out = String::new();
    for d in &me.anydescs {
        out.push_str(&format!(
            "  memref.global @{} : memref<{}xi64> = dense<0> {{mutable}}\n",
            d.sym, DESC_WORDS
        ));
    }
    out
}

/// display wrappers: `llvm.func (i64) -> i64` reading the receiver's vtable
/// `to_str` slot and returning the fresh str handle.
pub(crate) fn emit_any_disp_wrappers(me: &ModEmitter) -> String {
    let mut out = String::new();
    for (i, d) in me.anydescs.iter().enumerate() {
        if let Some(DispSpec::Dyn { slot }) = d.disp {
            out.push_str(&format!(
                "  llvm.func @sloth_anydisp_{}(%p0: i64) -> i64 {{\n\
                 \x20   %vt = func.call @sloth_obj_vtable(%p0) : (i64) -> i64\n\
                 \x20   %sl = arith.constant {} : i64\n\
                 \x20   %fp = func.call @sloth_vt_get(%vt, %sl) : (i64, i64) -> i64\n\
                 \x20   %fn = llvm.inttoptr %fp : i64 to !llvm.ptr\n\
                 \x20   %r = llvm.call %fn(%p0) : !llvm.ptr, (i64) -> i64\n\
                 \x20   llvm.return %r : i64\n\
                 \x20 }}\n",
                i, slot
            ));
        }
    }
    out
}

/// the module's `@sloth_<mod>__anyinit()` function body (always emitted).
pub(crate) fn emit_anyinit(me: &ModEmitter) -> String {
    fn store(body: &mut String, g: &str, idx: isize, val: &str, c: &mut usize) {
        let ic = format!("%adi{}", *c);
        *c += 1;
        body.push_str(&format!("    {} = arith.constant {} : index\n", ic, idx));
        body.push_str(&format!(
            "    memref.store {}, {}[{}] : memref<{}xi64>\n",
            val, g, ic, DESC_WORDS
        ));
    }
    let mut body = String::new();
    let mut c = 0usize;
    macro_rules! nv {
        ($p:expr) => {{
            let s = format!("%{}{}", $p, c);
            c += 1;
            s
        }};
    }
    for d in &me.anydescs {
        let g = nv!("adg");
        body.push_str(&format!(
            "    {} = memref.get_global @{} : memref<{}xi64>\n",
            g, d.sym, DESC_WORDS
        ));
        // word 0/1/2: kind, flags, type_id
        let k = nv!("adk");
        body.push_str(&format!("    {} = arith.constant {} : i64\n", k, d.kind));
        store(&mut body, &g, 0, &k, &mut c);
        let fl = nv!("adf");
        body.push_str(&format!("    {} = arith.constant {} : i64\n", fl, d.flags));
        store(&mut body, &g, 1, &fl, &mut c);
        let ti = nv!("adt");
        body.push_str(&format!(
            "    {} = arith.constant {} : i64\n",
            ti, d.type_id
        ));
        store(&mut body, &g, 2, &ti, &mut c);
        // word 3/4: name pointer + length
        if let Some((nm, nlen)) = &d.name {
            let a = nv!("adna");
            body.push_str(&format!(
                "    {} = llvm.mlir.addressof @{} : !llvm.ptr\n",
                a, nm
            ));
            let v = nv!("adnp");
            body.push_str(&format!(
                "    {} = llvm.ptrtoint {} : !llvm.ptr to i64\n",
                v, a
            ));
            store(&mut body, &g, 3, &v, &mut c);
            let lv = nv!("adnl");
            body.push_str(&format!("    {} = arith.constant {} : i64\n", lv, nlen));
            store(&mut body, &g, 4, &lv, &mut c);
        }
        // word 5/6/7: elem/key/val child descriptor addresses
        for (off, ch) in [(5isize, &d.elem), (6, &d.key), (7, &d.val)] {
            if let Some(cs) = ch {
                let cg = nv!("adcg");
                body.push_str(&format!(
                    "    {} = memref.get_global @{} : memref<{}xi64>\n",
                    cg, cs, DESC_WORDS
                ));
                let ci = nv!("adci");
                body.push_str(&format!(
                    "    {} = memref.extract_aligned_pointer_as_index {} : memref<{}xi64> -> index\n",
                    ci, cg, DESC_WORDS
                ));
                let cv = nv!("adcv");
                body.push_str(&format!(
                    "    {} = arith.index_cast {} : index to i64\n",
                    cv, ci
                ));
                store(&mut body, &g, off, &cv, &mut c);
            }
        }
        // word 8: display wrapper address
        if d.disp.is_some() {
            let sym = format!("sloth_anydisp_{}", index_of(me, &d.sym));
            let a = nv!("adda");
            body.push_str(&format!(
                "    {} = llvm.mlir.addressof @{} : !llvm.ptr\n",
                a, sym
            ));
            let v = nv!("addv");
            body.push_str(&format!(
                "    {} = llvm.ptrtoint {} : !llvm.ptr to i64\n",
                v, a
            ));
            store(&mut body, &g, 8, &v, &mut c);
        }
        // word 9: cls_id
        let cid = nv!("adc");
        body.push_str(&format!(
            "    {} = arith.constant {} : i64\n",
            cid, d.cls_id
        ));
        store(&mut body, &g, 9, &cid, &mut c);
        // word 10: rank
        let rk = nv!("adr");
        body.push_str(&format!("    {} = arith.constant {} : i64\n", rk, d.rank));
        store(&mut body, &g, 10, &rk, &mut c);
    }
    body
}

fn index_of(me: &ModEmitter, sym: &str) -> usize {
    me.anydescs.iter().position(|d| d.sym == sym).unwrap_or(0)
}
