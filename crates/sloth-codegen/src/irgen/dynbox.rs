//! Auto-boxing builtin value types (`int`/`float`/`bool`) into `dyn Trait`.
//!
//! Value types have no object header, so a value coerced to a `dyn Trait`
//! surface is wrapped in a synthetic object: reserved per-kind `ObjInfo`,
//! the builtin's vtable, and the value word in field 0. The predefined traits
//! (Display/Equatable/Hashable/Comparable) are served by thin `llvm.func`
//! bridges forwarding to the `@sloth_dyn_*` runtime helpers; an empty trait
//! needs no slots at all.

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

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// `Display`-family member?
fn is_str_method(m: &str) -> bool {
    matches!(
        m,
        "to_str" | "toString" | "__str__" | "display" | "str" | "to_string"
    )
}

/// `Hashable`-family member?
fn is_hash_method(m: &str) -> bool {
    matches!(m, "__hash__" | "hash" | "hashKey" | "hash_key")
}

/// comparison-family op code: 0 eq, 1 ne, 2 lt, 3 le, 4 gt, 5 ge
fn cmp_op(m: &str) -> Option<i64> {
    Some(match m {
        "__eq__" | "eq" | "equals" | "equal" => 0,
        "__ne__" | "ne" => 1,
        "__lt__" | "lt" => 2,
        "__le__" | "le" => 3,
        "__gt__" | "gt" => 4,
        "__ge__" | "ge" => 5,
        _ => return None,
    })
}

impl ModEmitter {
    /// word-plane kind of a builtin value type: 0 int, 1 float, 2 bool
    pub(crate) fn value_kind(&self, t: TyId) -> Option<i64> {
        match self.r.get(t) {
            Ty::I64 => Some(0),
            Ty::F64 => Some(1),
            Ty::Bool => Some(2),
            _ => None,
        }
    }

    /// runtime class id reserved for a builtin kind (mirrors rt/builtins.rs)
    pub(crate) fn value_cls_id(kind: i64) -> i64 {
        match kind {
            1 => -2,
            2 => -3,
            _ => -1,
        }
    }

    /// does a builtin value type satisfy trait `tname`? The predefined traits
    /// are auto-implemented; a trait with no methods is vacuously satisfied.
    pub(crate) fn value_impls_trait(&self, tname: &str) -> bool {
        Self::is_predef_trait(tname)
            || self
                .traits
                .get(tname)
                .map(|ms| ms.is_empty())
                .unwrap_or(false)
    }

    /// box a value word into a synthetic `dyn tname` object (owned producer)
    pub(crate) fn emit_dyn_box(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
        tname: &str,
    ) -> String {
        let kind = self.value_kind(from).unwrap_or(0);
        let kw = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            kw,
            enc_i_lit(kind)
        ));
        let info = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_builtin_info({}) : (i64) -> i64",
            info, kw
        ));
        let one = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            one,
            enc_i_lit(1)
        ));
        let obj = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_obj_new({}, {}) : (i64, i64) -> i64",
            obj, info, one
        ));
        // field 0 = value word; the word is never a reference, so a raw store
        let z = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            z,
            enc_i_lit(0)
        ));
        fw.op(&format!(
            "    func.call @sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
            obj, z, v
        ));
        let builder = self.emit_builtin_vt_builder(kind, tname);
        let vt = fw.v();
        fw.op(&format!("    {} = func.call @{}() : () -> i64", vt, builder));
        fw.op(&format!(
            "    func.call @sloth_obj_set_vtable({}, {}) : (i64, i64) -> i64",
            obj, vt
        ));
        let dt = self.r.mk(Ty::Dyn(tname.to_string()));
        self.dangling_producer(fw, &obj, dt);
        obj
    }

    /// per-(kind, trait) vtable builder, lazily built and cached in a global
    /// cell (mirrors the per-class builder in `class.rs`).
    pub(crate) fn emit_builtin_vt_builder(&mut self, kind: i64, tname: &str) -> String {
        let fname = format!("sloth_vtb_b{}__{}", kind, sanitize(tname));
        if self.vt_built.contains(&fname) {
            return fname;
        }
        self.vt_built.insert(fname.clone());
        self.stat_vtbuilds += 1;
        let ity = self.r.mk(Ty::I64);
        let modname = self.name.clone();
        let gsym = self.declare_global(
            &modname,
            &format!("vtb_b{}__{}", kind, sanitize(tname)),
            ity,
        );
        let mut fw = fresh_walk(self);
        let g = fw.v();
        fw.op(&format!(
            "    {} = memref.get_global @{} : memref<1xi64>",
            g, gsym
        ));
        let zi = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", zi));
        let cached = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            cached, g, zi
        ));
        let zz = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zz));
        let cc = fw.v();
        fw.op(&format!(
            "    {} = arith.cmpi eq, {}, {} : i64",
            cc, cached, zz
        ));
        let cce = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", cce, cc));
        let lbl_build = fw.newlabel("bvtb");
        let lbl_done = fw.newlabel("bvtb");
        fw.cjump(&cce, &lbl_build, &lbl_done);
        fw.label(&lbl_build);
        let ncap = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            ncap,
            enc_i_lit(self.vt_cap as i64)
        ));
        let vt = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_vt_new({}) : (i64) -> i64",
            vt, ncap
        ));
        let methods = self.traits.get(tname).cloned().unwrap_or_default();
        for m in methods {
            let slot = self.vt_slot(tname, &m.name);
            let bridge = self.emit_builtin_bridge(kind, &m.name, m.params.len());
            let fpa = fw.v();
            fw.op(&format!(
                "    {} = llvm.mlir.addressof @{} : !llvm.ptr",
                fpa, bridge
            ));
            let fp = fw.v();
            fw.op(&format!(
                "    {} = llvm.ptrtoint {} : !llvm.ptr to i64",
                fp, fpa
            ));
            let slotc = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                slotc,
                enc_i_lit(slot as i64)
            ));
            fw.op(&format!(
                "    func.call @sloth_vt_set({}, {}, {}) : (i64, i64, i64) -> i64",
                vt, slotc, fp
            ));
        }
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            vt, g, zi
        ));
        fw.jump(&lbl_done);
        fw.label(&lbl_done);
        let res = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            res, g, zi
        ));
        fw.op(&format!("    return {} : i64", res));
        self.out.push_str(&format!(
            "  func.func private @{}() -> i64 {{\n{}  }}\n",
            fname, fw.cur
        ));
        fname
    }

    /// thin `(i64 self, i64 args...) -> i64` bridge for one predefined method
    fn emit_builtin_bridge(&mut self, kind: i64, method: &str, nparams: usize) -> String {
        let key = format!("bb:{}:{}:{}", kind, method, nparams);
        if let Some(b) = self.builtin_bridges.get(&key) {
            return b.clone();
        }
        let bname = format!(
            "sloth_{}__bb{}_{}",
            self.name,
            self.builtin_bridges.len(),
            sanitize(method)
        );
        self.builtin_bridges.insert(key, bname.clone());
        let kw = enc_i_lit(kind);
        let mut body = String::new();
        if nparams == 0 {
            if is_hash_method(method) {
                body.push_str(&format!("    %k = arith.constant {} : i64\n", kw));
                body.push_str("    %r = func.call @sloth_dyn_hash(%p0, %k) : (i64, i64) -> i64\n");
            } else if is_str_method(method) {
                body.push_str(&format!("    %k = arith.constant {} : i64\n", kw));
                body.push_str(
                    "    %r = func.call @sloth_dyn_to_str(%p0, %k) : (i64, i64) -> i64\n",
                );
            } else {
                body.push_str("    %r = func.call @sloth_dyn_unbox(%p0) : (i64) -> i64\n");
            }
        } else {
            let op = cmp_op(method).unwrap_or(0);
            body.push_str(&format!(
                "    %k = arith.constant {} : i64\n    %o = arith.constant {} : i64\n",
                kw,
                enc_i_lit(op)
            ));
            body.push_str(
                "    %r = func.call @sloth_dyn_binop(%p0, %p1, %k, %o) : (i64, i64, i64, i64) -> i64\n",
            );
        }
        body.push_str("    llvm.return %r : i64\n");
        let named: Vec<String> = (0..=nparams).map(|i| format!("%p{}: i64", i)).collect();
        self.out.push_str(&format!(
            "  llvm.func @{}({}) -> i64 {{\n{}}}\n",
            bname,
            named.join(", "),
            body
        ));
        bname
    }
}
