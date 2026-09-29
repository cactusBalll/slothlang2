//! Expression lowering: arith/leaf codes, calls, generics.

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

/// upper bound on the number of live monomorphic instances; a generic whose
/// instantiation never reaches a fixed point (e.g. `f<T>` calling `f<Box<T>>`)
/// is rejected rather than looping forever.
const MONO_INST_CAP: usize = 10_000;

impl ModEmitter {
    /// stmt text of the callee Identifier for diagnostics
    pub(crate) fn init_str2(callee: &Expr) -> String {
        match &callee.node {
            ExprNode::Ident(id) => id.clone(),
            _ => String::new(),
        }
    }

    /// write the (possibly relocated) push handle back into the receiver's
    /// storage: local variable slot or enclosing object field; other receiver
    /// faces (rvalue chains, nested container elements) are compile errors
    pub(crate) fn emit_arr_push_writeback(
        &mut self,
        fw: &mut FnWalk,
        callee: &Expr,
        handle: &str,
        pos: &Pos,
    ) {
        let (recv, _fname) = match &callee.node {
            ExprNode::Field { obj, name } => (obj, name.clone()),
            _ => return,
        };
        match &recv.node {
            // `a.push(v)` — local variable slot
            ExprNode::Ident(n) => match fw.lookup(n) {
                Some((slot, ty)) => match self.r.get(ty) {
                    Ty::Array(_) => {
                        // rc patch B: slot ownership MOVES untouched — the
                        // relocate (realloc) already transferred the count
                        // entry (rt transfer), so the raw store must not
                        // release the old word (same address = live handle)
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : index", z));
                        fw.op(&format!(
                            "    memref.store {}, {}[{}] : memref<1xi64>",
                            handle, slot, z
                        ));
                    }
                    _ => self.err(pos, format!("push receiver `{}` is not an array", n)),
                },
                None => self.err(pos, format!("push receiver unknown `{}`", n)),
            },
            // `this.data.push(v)` / `o.inner.push(v)` — set field on the
            // container object that owns the array field
            ExprNode::Field { obj, name } => {
                let (rv, rt) = self.emit_expr(fw, obj);
                match self.r.get(rt) {
                    Ty::Named(c, _) => {
                        let idx = self.field_index(c, name) as i64;
                        // stable-handle push: the returned word equals the
                        // field's own handle (raw store, no release)
                        let zi = fw.v();
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            zi,
                            enc_i_lit(idx)
                        ));
                        fw.op(&format!(
                            "    func.call @__sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
                            rv, zi, handle
                        ));
                    }
                    _ => self.err(
                        pos,
                        "push receiver must be a local or object field".to_string(),
                    ),
                }
            }
            // rvalue chains (nested container elements `g[i].push(v)`,
            // returned arrays, ...) need no writeback: array handles are
            // stable, so the element/alias already holds the live handle
            _ => {
                let _ = pos;
            }
        }
    }
}

impl ModEmitter {
    /// Pass-boundary wrapper for expression lowering. Pass 1 (`check_mode`)
    /// records each node's inferred type into the NodeId side table; Pass 2
    /// consumes it as the authoritative expression type. The table is complete
    /// over the whole program (verified: no misses across the spec/seed suites),
    /// so Pass 2 never contributes a new expression type — it only lowers.
    pub(crate) fn emit_expr(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        let (w, t) = self.emit_expr_inner(fw, e);
        if e.id != 0 {
            let key = (self.cur_frame.clone(), e.id);
            if self.check_mode {
                self.type_table.insert(key, t);
            } else if let Some(&tt) = self.type_table.get(&key) {
                return (w, tt);
            } else {
                // Pass 1 records every expression node; a miss means the two
                // passes walked different code. Keep the locally computed type
                // so release builds still emit, but surface the bug in debug.
                debug_assert!(
                    false,
                    "type side table miss: frame={} node={}",
                    self.cur_frame, e.id
                );
            }
        }
        (w, t)
    }

    fn emit_expr_inner(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Int(v) => {
                let r = fw.v();
                fw.op(&format!(
                    "    {} = arith.constant {} : i64",
                    r,
                    enc_i_lit(*v)
                ));
                let t = self.r.mk(Ty::I64);
                (r, t)
            }
            ExprNode::UInt(v) => {
                // unsigned literal: the word is the two's-complement bit pattern
                let r = fw.v();
                fw.op(&format!("    {} = arith.constant {} : i64", r, *v as i64));
                let t = self.r.mk(Ty::Int(sloth_frontend::ty::IntKind::U64));
                (r, t)
            }
            ExprNode::Float(v) => {
                // tag migration: the word plane carries the encoded f64 bits
                let r = fw.v();
                fw.op(&format!(
                    "    {} = arith.constant {} : i64",
                    r,
                    enc_f_lit(*v)
                ));
                (r, self.r.mk(Ty::F64))
            }
            ExprNode::Bool(v) => {
                let r = fw.v();
                let b = enc_i_lit(if *v { 1 } else { 0 });
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
    pub(crate) fn emit_expr_rest(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Str(ss) => {
                if !ss.is_plain() {
                    // interpolated string: chain per-part push onto a builder
                    let c0 = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", c0));
                    let mut curw = c0;
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
                                    fw.op(&format!(
                                        "    {} = arith.constant {} : i64",
                                        c1, w as i64
                                    ));
                                    let tail = std::cmp::min(8usize, blen - ci_ * 8);
                                    let c2 = fw.v();
                                    fw.op(&format!(
                                        "    {} = arith.constant {} : i64",
                                        c2,
                                        enc_i_lit(tail as i64)
                                    ));
                                    let r = fw.v();
                                    fw.op(&format!(
                                        "    {} = func.call @__sloth_str_push({}, {}, {}) : (i64, i64, i64) -> i64",
                                        r, curw, c1, c2
                                    ));
                                    curw = r;
                                }
                            }
                            StrPart::ExprAst(e) => {
                                // render through the runtime-typed writer so
                                // containers/objects interpolate uniformly
                                let (v, t) = self.emit_expr(fw, e);
                                let (av, _at) = self.coerce_into_any(fw, &v, t);
                                let sv = fw.v();
                                fw.op(&format!(
                                    "    {} = func.call @__sloth_rt_write({}) : (i64) -> i64",
                                    sv, av
                                ));
                                // fresh owned str from the writer: track so the
                                // temp is released at statement close
                                let stra = self.r.mk(Ty::Str);
                                self.dangling_producer(fw, &sv, stra);
                                let r = fw.v();
                                fw.op(&format!(
                                    "    {} = func.call @__sloth_str_pushp({}, {}) : (i64, i64) -> i64",
                                    r, curw, sv
                                ));
                                curw = r;
                            }
                            _ => {}
                        }
                    }
                    let fin = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_str_finish({}) : (i64) -> i64",
                        fin, curw
                    ));
                    let t = self.r.mk(Ty::Str);
                    // rc patch B: fresh `str` producer (owned +1)
                    self.dangling_producer(fw, &fin, t);
                    return (fin, t);
                }
                // packed 8-byte words across the runtime; buffer token chains
                let plain: Vec<u8> = ss.plain().unwrap_or("").bytes().collect();
                let len = plain.len();
                let mut pads = plain.clone();
                while pads.len() % 8 != 0 {
                    pads.push(0);
                }
                let c0 = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", c0));
                let mut curw = c0;
                {
                    for (i, ch) in pads.chunks(8).enumerate() {
                        let mut w: u64 = 0;
                        for (k, b) in ch.iter().enumerate() {
                            w |= (*b as u64) << (8 * k);
                        }
                        let c1 = fw.v();
                        fw.op(&format!("    {} = arith.constant {} : i64", c1, w as i64));
                        let n = std::cmp::min(8usize, len - i * 8);
                        let c2 = fw.v();
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            c2,
                            enc_i_lit(n as i64)
                        ));
                        let r = fw.v();
                        fw.op(&format!(
                            "    {} = func.call @__sloth_str_push({}, {}, {}) : (i64, i64, i64) -> i64",
                            r, curw, c1, c2
                        ));
                        curw = r;
                    }
                }
                let fin = fw.v();
                fw.op(&format!(
                    "    {} = func.call @__sloth_str_finish({}) : (i64) -> i64",
                    fin, curw
                ));
                let t = self.r.mk(Ty::Str);
                // rc patch B: fresh `str` producer (owned +1)
                self.dangling_producer(fw, &fin, t);
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
                    let bv = enc_i_lit(if b { 1 } else { 0 });
                    fw.op(&format!("    {} = arith.constant {} : i64", r, bv));
                    return (r, t);
                }
                if let Some((a, t)) = fw.lookup(name) {
                    let z = fw.v();
                    let v = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", z));
                    fw.op(&format!(
                        "    {} = memref.load {}[{}] : memref<1xi64>",
                        v, a, z
                    ));
                    (v, t)
                } else {
                    // own-module global while emitting a foreign body wins over
                    // a same-named local global
                    let cur_mod = self.cur_mod.clone();
                    let fg = self.fglobals.get(&format!("{}.{}", cur_mod, name)).cloned();
                    let lg = self.globals.get(name).cloned();
                    let picked = if cur_mod != self.name {
                        fg.or(lg)
                    } else {
                        lg.or(fg)
                    };
                    match picked {
                        Some((gsym, t, _)) => {
                            let (v, _vt) = self.emit_global_read(fw, &gsym, t);
                            (v, t)
                        }
                        None => {
                            // named function used as a first-class value
                            if let Some(out) = self.emit_fn_named_value(fw, name, &e.pos) {
                                out
                            } else {
                                self.err(&e.pos, format!("unknown identifier `{}`", name));
                                (String::new(), self.r.mk(Ty::Unit))
                            }
                        }
                    }
                }
            }
            _ => self.emit_expr_arith_codes(fw, e),
        }
    }
}

impl ModEmitter {
    pub(crate) fn emit_expr_arith_codes(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Is { negated, lhs, rhs } => {
                let (lv, lt) = self.emit_expr(fw, lhs);
                // nil test
                if let ExprNode::Nil = &rhs.node {
                    // only nil-capable surfaces may be nil-tested: `T?`, `dyn`,
                    // `Weak<T>`, the `nil`/unknown surface, or a generic
                    // placeholder. A plain value/ref (int/str/class/...) can
                    // never be nil, so `x is nil` there is a type error.
                    if !self.nil_capable(lt) {
                        let tn = self.surface_name(&self.r.get(lt).clone());
                        self.err(
                            &e.pos,
                            format!(
                                "`is nil` on non-optional type `{}` (only `T?`, `dyn` or `Weak<T>` can be nil)",
                                tn
                            ),
                        );
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", z));
                        return (z, self.r.mk(Ty::Bool));
                    }
                    let zc = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", zc));
                    let c = fw.v();
                    fw.op(&format!("    {} = arith.cmpi eq, {}, {} : i64", c, lv, zc));
                    let c1 = ext_bool(fw, &c);
                    let r = if *negated {
                        let one = fw.v();
                        let o = fw.v();
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            one,
                            enc_i_lit(1)
                        ));
                        fw.op(&format!("    {} = arith.xori {}, {} : i64", o, c1, one));
                        o
                    } else {
                        c1
                    };
                    return (r, self.r.mk(Ty::Bool));
                }
                // `any` runtime type tests against primitive kinds and classes
                if matches!(self.r.get(lt), Ty::Any) {
                    if let ExprNode::Ident(cn) = &rhs.node {
                        let akind = match cn.as_str() {
                            "int" | "i64" | "int64" => Some(super::anydesc::AK_I64),
                            "float" | "f64" => Some(super::anydesc::AK_F64),
                            "bool" => Some(super::anydesc::AK_BOOL),
                            "str" => Some(super::anydesc::AK_STR),
                            "range" => Some(super::anydesc::AK_RANGE),
                            other => super::anydesc::ak_for_int_name(other),
                        };
                        if let Some(k) = akind {
                            let kv = fw.v();
                            fw.op(&format!("    {} = arith.constant {} : i64", kv, k));
                            let ak = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_any_kind({}) : (i64) -> i64",
                                ak, lv
                            ));
                            let eq = fw.v();
                            fw.op(&format!("    {} = arith.cmpi eq, {}, {} : i64", eq, ak, kv));
                            let b = ext_bool(fw, &eq);
                            return (neg_bool_word(fw, &b, *negated), self.r.mk(Ty::Bool));
                        }
                        if self.class_ids.get(cn).is_some() {
                            let mut idsv: Vec<i64> = Vec::new();
                            if let Some(id) = self.class_ids.get(cn) {
                                idsv.push(*id);
                            }
                            for candv in self.class_order.clone() {
                                let cand = candv.clone();
                                let mut cur = Some(cand.clone());
                                while let Some(pn) = cur {
                                    cur =
                                        self.classes.get(&pn).and_then(|ci| ci.superclass.clone());
                                    if cur.as_deref() == Some(cn.as_str()) {
                                        if let Some(id) = self.class_ids.get(&cand) {
                                            idsv.push(*id);
                                        }
                                        break;
                                    }
                                }
                            }
                            let clsid = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_any_cls_id({}) : (i64) -> i64",
                                clsid, lv
                            ));
                            let mut acc: Option<String> = None;
                            for id in &idsv {
                                let ci = fw.v();
                                fw.op(&format!(
                                    "    {} = arith.constant {} : i64",
                                    ci,
                                    enc_i_lit(*id)
                                ));
                                let eq = fw.v();
                                fw.op(&format!(
                                    "    {} = arith.cmpi eq, {}, {} : i64",
                                    eq, clsid, ci
                                ));
                                let eq1 = ext_bool(fw, &eq);
                                acc = match acc {
                                    None => Some(eq1),
                                    Some(a) => {
                                        let o = fw.v();
                                        fw.op(&format!(
                                            "    {} = arith.ori {}, {} : i64",
                                            o, a, eq1
                                        ));
                                        Some(o)
                                    }
                                };
                            }
                            let base = match acc {
                                Some(x) => x,
                                None => {
                                    let z = fw.v();
                                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                                    z
                                }
                            };
                            return (neg_bool_word(fw, &base, *negated), self.r.mk(Ty::Bool));
                        }
                        self.err(
                            &e.pos,
                            format!("`is` type `{}` not a known class or builtin", cn),
                        );
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                }
                // class membership test via static ancestor chain of cls ids
                if let ExprNode::Ident(cn) = &rhs.node {
                    // patch #32: builtin type surfaces (`is str` / `is int` /
                    // ...) — static word-class compare, const true/false
                    if self.class_ids.get(cn).is_none() && Self::is_builtin_type_name(cn) {
                        // unwrap option layers for the word surface
                        let mut wt = self.r.get(lt).clone();
                        while let Ty::Opt(inner) = wt {
                            wt = self.r.get(inner).clone();
                        }
                        // `dyn T` value: the concrete builtin kind is only
                        // known at runtime (reserved class id on the box)
                        if matches!(&wt, Ty::Dyn(_)) {
                            let cid = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_obj_cls_id({}) : (i64) -> i64",
                                cid, lv
                            ));
                            let cls = match cn.as_str() {
                                "int" | "i64" | "int64" => Some(Self::value_cls_id(0)),
                                "float" | "f64" => Some(Self::value_cls_id(1)),
                                "bool" => Some(Self::value_cls_id(2)),
                                other => sloth_frontend::ty::IntKind::from_name(other).map(|k| {
                                    let t = self.r.mk(Ty::Int(k));
                                    let vk = self.value_kind(t).unwrap_or(0);
                                    Self::value_cls_id(vk)
                                }),
                            };
                            let base = match cls {
                                Some(c) => {
                                    let cv = fw.v();
                                    fw.op(&format!(
                                        "    {} = arith.constant {} : i64",
                                        cv,
                                        enc_i_lit(c)
                                    ));
                                    let c2 = fw.v();
                                    fw.op(&format!(
                                        "    {} = arith.cmpi eq, {}, {} : i64",
                                        c2, cid, cv
                                    ));
                                    ext_bool(fw, &c2)
                                }
                                None => {
                                    let z = fw.v();
                                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                                    z
                                }
                            };
                            let r = if *negated {
                                let one = fw.v();
                                let o = fw.v();
                                fw.op(&format!(
                                    "    {} = arith.constant {} : i64",
                                    one,
                                    enc_i_lit(1)
                                ));
                                fw.op(&format!("    {} = arith.xori {}, {} : i64", o, base, one));
                                o
                            } else {
                                base
                            };
                            return (r, self.r.mk(Ty::Bool));
                        }
                        let matches = match (&wt, cn.as_str()) {
                            (Ty::I64, "int") | (Ty::I64, "i64") | (Ty::I64, "int64") => true,
                            (Ty::F64, "float") | (Ty::F64, "f64") => true,
                            (Ty::Str, "str") => true,
                            (Ty::Bool, "bool") => true,
                            (Ty::Range, "range") => true,
                            (Ty::Int(k), n) => {
                                sloth_frontend::ty::IntKind::from_name(n) == Some(*k)
                            }
                            _ => false,
                        };
                        // patch 42: an optional word is a runtime box-or-nil —
                        // `x is int` resolves against liveness, not statically.
                        // Reference optionals (`str?`/`C?`) hold the bare
                        // handle (0 = nil), so they take the same liveness path.
                        if matches!(self.r.get(lt).clone(), Ty::Opt(_)) {
                            let zc = fw.v();
                            fw.op(&format!("    {} = arith.constant 0 : i64", zc));
                            let live = fw.v();
                            fw.op(&format!(
                                "    {} = arith.cmpi ne, {}, {} : i64",
                                live, lv, zc
                            ));
                            let live1 = ext_bool(fw, &live);
                            let r = if matches {
                                if *negated {
                                    // live inverted: enc(1) - live == invert
                                    let one = fw.v();
                                    let o = fw.v();
                                    fw.op(&format!(
                                        "    {} = arith.constant {} : i64",
                                        one,
                                        enc_i_lit(1)
                                    ));
                                    fw.op(&format!(
                                        "    {} = arith.subi {}, {} : i64",
                                        o, one, live1
                                    ));
                                    o
                                } else {
                                    live1
                                }
                            } else {
                                // a non-nil box still fails the family test…
                                // matching family by word view; only does nil lose
                                let z2 = fw.v();
                                fw.op(&format!(
                                    "    {} = arith.constant {} : i64",
                                    z2,
                                    if *negated { 1 } else { 0 }
                                ));
                                z2
                            };
                            return (r, self.r.mk(Ty::Bool));
                        }
                        let hit = if matches { !*negated } else { *negated };
                        let c = fw.v();
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            c,
                            enc_i_lit(if hit { 1i64 } else { 0i64 })
                        ));
                        return (c, self.r.mk(Ty::Bool));
                    }
                    if self.class_ids.get(cn).is_none() {
                        self.err(&e.pos, format!("`is` type `{}` not a known class", cn));
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    if self.is_float(lt) {
                        self.err(&e.pos, "`is` on float is unsupported".to_string());
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    // comparability (patch #25): both sides must share a class
                    // chain relation (C is D / D is C)
                    if let Ty::Named(lc, _) = self.r.get(lt).clone() {
                        if &lc != cn
                            && !self.class_chain_has(&lc, cn)
                            && !self.class_chain_has(cn, &lc)
                        {
                            self.err(
                                &e.pos,
                                format!(
                                    "types `{}` and `{}` have no class relation for `is`",
                                    lc, cn
                                ),
                            );
                            return (String::new(), self.r.mk(Ty::Unit));
                        }
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
                            cur = self.classes.get(&pn).and_then(|ci| ci.superclass.clone());
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
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            ci,
                            enc_i_lit(*id)
                        ));
                        let clsid = fw.v();
                        fw.op(&format!(
                            "    {} = func.call @__sloth_obj_cls_id({}) : (i64) -> i64",
                            clsid, lv
                        ));
                        let eq = fw.v();
                        fw.op(&format!(
                            "    {} = arith.cmpi eq, {}, {} : i64",
                            eq, clsid, ci
                        ));
                        let eq1 = ext_bool(fw, &eq);
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
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            one,
                            enc_i_lit(1)
                        ));
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
                let (rv, rt) = self.emit_expr(fw, rhs);
                // patch 42: boxed value optionals unwrap-or-0 on the taken
                // branch; bare float-Elvis stays rejected
                let (u, ut) = self.unwrap_opt_word(fw, &lv, lt);
                // reference optional (`C?`): the handle word already is the
                // payload, so `?:` yields the inner type (value optionals were
                // already unboxed by unwrap_opt_word)
                let ut = match self.r.get(ut).clone() {
                    Ty::Opt(e) => e,
                    _ => ut,
                };
                // design §2.1: `?:` joins two operands of the same type — not
                // just equal float-ness. A reference optional vs a scalar
                // (`C? ?: 5`) otherwise fell through and the select word was
                // mistyped as the class handle, so `print` released a scalar
                // (rc underflow; bug core/bug2). A `nil` operand is exempt
                // (keeps the optional surface).
                let uts = self.r.get(ut).clone();
                let rts = self.r.get(rt).clone();
                if !matches!(rts, Ty::Unit)
                    && !matches!(uts, Ty::Unit)
                    && !self.surface_compat(&uts, &rts)
                {
                    let an = self.surface_name(&uts);
                    let bn = self.surface_name(&rts);
                    self.err_diff(&e.pos, "elvis operand", &an, &bn);
                }
                // encoded words select word-wise; nil/0.0 both ride word 0
                let zc = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", zc));
                let c = fw.v();
                fw.op(&format!("    {} = arith.cmpi ne, {}, {} : i64", c, lv, zc));
                let r = fw.v();
                fw.op(&format!(
                    "    {} = arith.select {}, {}, {} : i64",
                    r, c, u, rv
                ));
                (r, ut)
            }
            ExprNode::Arith { op, lhs, rhs } => {
                let lit_l = matches!(lhs.node, ExprNode::Int(_) | ExprNode::UInt(_));
                let lit_r = matches!(rhs.node, ExprNode::Int(_) | ExprNode::UInt(_));
                let (a, at) = self.emit_expr(fw, lhs);
                let (a, at) = self.unwrap_opt_word(fw, &a, at);
                let (b, bt) = self.emit_expr(fw, rhs);
                let (b, bt) = self.unwrap_opt_word(fw, &b, bt);
                // int-only bitwise/shift operators (design §3.3): no float
                // route and no operator overload — both operands must be
                // `int` words. Valid cases fall through to the int route.
                if matches!(
                    op,
                    ArithOp::BitAnd
                        | ArithOp::BitOr
                        | ArithOp::BitXor
                        | ArithOp::Shl
                        | ArithOp::Shr
                ) && !(self.is_int_like(at) && self.is_int_like(bt))
                {
                    self.err(
                        &e.pos,
                        "bitwise operators require integer operands".to_string(),
                    );
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    return (z, self.r.mk(Ty::I64));
                }
                // operator overload: class receiver dispatches __add__ etc;
                // carry the rhs word as payload (a + b ≡ a.__op__(b))
                if let Ty::Named(cls, _) = self.r.get(at).clone() {
                    let oname = match op {
                        ArithOp::Add => "__add__",
                        ArithOp::Sub => "__sub__",
                        ArithOp::Mul => "__mul__",
                        ArithOp::Div => "__div__",
                        ArithOp::Mod => "__mod__",
                        ArithOp::BitAnd => "__and__",
                        ArithOp::BitOr => "__or__",
                        ArithOp::BitXor => "__xor__",
                        ArithOp::Shl => "__shl__",
                        ArithOp::Shr => "__shr__",
                    };
                    if let Some((defcls, fd)) = self.find_method(&cls, oname) {
                        let oargv = vec![(a.clone(), at), (b.clone(), bt)];
                        let osig = vec![mlir_word_ty(at, &self.r), mlir_word_ty(bt, &self.r)];
                        return self.emit_method_call(
                            fw, &defcls, oname, &fd, false, &oargv, &osig, &e.pos,
                        );
                    }
                    self.err(
                        &e.pos,
                        format!(
                            "operator `{:?}` on class `{}` requires a `{}` overload",
                            op, cls, oname
                        ),
                    );
                    return (String::new(), self.r.mk(Ty::Unit));
                }
                if *op == ArithOp::Add && self.is_str(at) && self.is_str(bt) {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_str_concat({}, {}) : (i64, i64) -> i64",
                        r, a, b
                    ));
                    let st = self.r.mk(Ty::Str);
                    // rc patch B: fresh `str` producer (owned +1)
                    self.dangling_producer(fw, &r, st);
                    return (r, st);
                }
                // design §2.1: no implicit numeric conversion — arithmetic
                // operands must share their type. A mixed int/float pair is a
                // compile error rather than a silent promotion.
                if self.is_float(at) != self.is_float(bt) {
                    let an = sloth_frontend::ty::ty_name(self.r.get(at));
                    let bn = sloth_frontend::ty::ty_name(self.r.get(bt));
                    self.err_diff(&e.pos, "arithmetic operand", &an, &bn);
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    return (z, self.r.mk(Ty::I64));
                }
                let fl = self.is_float(at) && self.is_float(bt);
                // design §7.1 / book ch07: `+ - * / %` are numeric operators;
                // the only non-numeric builtin operand is `str + str`
                // (handled above). Everything else in the int route must be
                // integer-like, otherwise pointer-word arithmetic leaks
                // silently (Array/Map/str/bool/range/closures).
                // `unit` is the "unknown surface" word (an unannotated global
                // before its initializer types it) — keep those lenient.
                let unknown_l = matches!(self.r.get(at), Ty::Unit);
                let unknown_r = matches!(self.r.get(bt), Ty::Unit);
                if !fl
                    && !(self.is_int_like(at) && self.is_int_like(bt))
                    && !unknown_l
                    && !unknown_r
                {
                    let an = sloth_frontend::ty::ty_name(self.r.get(at));
                    let bn = sloth_frontend::ty::ty_name(self.r.get(bt));
                    self.err_diff(&e.pos, "arithmetic operand", &an, &bn);
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    return (z, self.r.mk(Ty::I64));
                }
                if fl {
                    // tagged words -> f64 scalars for the typed op, then back
                    let a = emit_dec_f(fw, &a);
                    let b = emit_dec_f(fw, &b);
                    let rf = fw.v();
                    let ao = match op {
                        ArithOp::Add => "arith.addf",
                        ArithOp::Sub => "arith.subf",
                        ArithOp::Mul => "arith.mulf",
                        ArithOp::Div => "arith.divf",
                        ArithOp::Mod => "arith.remf",
                        // unreachable: bitwise operands are rejected above
                        ArithOp::BitAnd
                        | ArithOp::BitOr
                        | ArithOp::BitXor
                        | ArithOp::Shl
                        | ArithOp::Shr => "arith.addf",
                    };
                    fw.op(&format!("    {} = {} {}, {} : f64", rf, ao, a, b));
                    let r = emit_enc_f(fw, &rf);
                    return (r, self.r.mk(Ty::F64));
                }
                // int route: unify the operand integer surfaces (an `int`
                // literal adopts the other side's fixed width), truncate both
                // operands to that width, then compute signed/unsigned per the
                // shared surface and truncate the result back.
                let rty = if self.is_int_like(at) && self.is_int_like(bt) {
                    match self.unify_int(at, lit_l, bt, lit_r) {
                        Some(t) => t,
                        None => {
                            let an = sloth_frontend::ty::ty_name(self.r.get(at));
                            let bn = sloth_frontend::ty::ty_name(self.r.get(bt));
                            self.err_diff(&e.pos, "arithmetic operand", &an, &bn);
                            let z = fw.v();
                            fw.op(&format!("    {} = arith.constant 0 : i64", z));
                            return (z, self.r.mk(Ty::I64));
                        }
                    }
                } else {
                    self.r.mk(Ty::I64)
                };
                let a = self.coerce_int_word(fw, &a, rty);
                let b = self.coerce_int_word(fw, &b, rty);
                let uns = self.is_unsigned_int(rty);
                let ad = emit_dec_int(fw, &a);
                let bd = emit_dec_int(fw, &b);
                let ao = match op {
                    ArithOp::Add => "arith.addi",
                    ArithOp::Sub => "arith.subi",
                    ArithOp::Mul => "arith.muli",
                    ArithOp::Div => {
                        if uns {
                            "arith.divui"
                        } else {
                            "arith.divsi"
                        }
                    }
                    ArithOp::Mod => {
                        if uns {
                            "arith.remui"
                        } else {
                            "arith.remsi"
                        }
                    }
                    ArithOp::BitAnd => "arith.andi",
                    ArithOp::BitOr => "arith.ori",
                    ArithOp::BitXor => "arith.xori",
                    ArithOp::Shl => "arith.shli",
                    ArithOp::Shr => {
                        if uns {
                            "arith.shrui"
                        } else {
                            "arith.shrsi"
                        }
                    }
                };
                if matches!(op, ArithOp::Div | ArithOp::Mod) {
                    // design §5.5: integer divide/modulo by zero is a panic.
                    // Split the CFG around the guard; the result travels in a
                    // slot so both sides dominate the merge.
                    let resslot = fw.v();
                    fw.op(&format!(
                        "    {} = memref.alloca() : memref<1xi64>",
                        resslot
                    ));
                    let zc = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", zc));
                    let isz = fw.v();
                    fw.op(&format!(
                        "    {} = arith.cmpi eq, {}, {} : i64",
                        isz, bd, zc
                    ));
                    let isze = fw.v();
                    fw.op(&format!("    {} = arith.extui {} : i1 to i64", isze, isz));
                    let lbl_panic = fw.newlabel("dz");
                    let lbl_ok = fw.newlabel("dz");
                    let lbl_end = fw.newlabel("dz");
                    let saved_dangling = std::mem::take(&mut fw.dangling);
                    let saved_xfer = std::mem::take(&mut fw.xfer);
                    fw.cjump(&isze, &lbl_panic, &lbl_ok);
                    fw.label(&lbl_panic);
                    fw.op("    func.call @__sloth_panic_divzero() : () -> i64");
                    let pz = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", pz));
                    fw.op(&format!(
                        "    memref.store {}, {}[{}] : memref<1xi64>",
                        zc, resslot, pz
                    ));
                    fw.jump(&lbl_end);
                    fw.label(&lbl_ok);
                    let rr = fw.v();
                    fw.op(&format!("    {} = {} {}, {} : i64", rr, ao, ad, bd));
                    let oz = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", oz));
                    fw.op(&format!(
                        "    memref.store {}, {}[{}] : memref<1xi64>",
                        rr, resslot, oz
                    ));
                    fw.jump(&lbl_end);
                    fw.label(&lbl_end);
                    fw.dangling = saved_dangling;
                    fw.xfer = saved_xfer;
                    let lz = fw.v();
                    let rw = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", lz));
                    fw.op(&format!(
                        "    {} = memref.load {}[{}] : memref<1xi64>",
                        rw, resslot, lz
                    ));
                    let r = emit_enc_int(fw, &rw);
                    let r = self.coerce_int_word(fw, &r, rty);
                    return (r, rty);
                }
                let rr = fw.v();
                fw.op(&format!("    {} = {} {}, {} : i64", rr, ao, ad, bd));
                let r = emit_enc_int(fw, &rr);
                let r = self.coerce_int_word(fw, &r, rty);
                (r, rty)
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
    pub(crate) fn emit_expr_rest2(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Bin { op, lhs, rhs } => {
                let (a, at) = self.emit_expr(fw, lhs);
                let (a, at) = self.unwrap_opt_word(fw, &a, at);
                // logical operators short-circuit: `a and b` only evaluates b
                // when a is true, `a or b` only when a is false (guard idiom
                // `i < len and a[i] == x` must not evaluate a[i] out of range)
                if matches!(op, BinOp::And | BinOp::Or) {
                    return self.emit_logical(fw, op, a, at, rhs, &e.pos);
                }
                let (b, bt) = self.emit_expr(fw, rhs);
                let (b, bt) = self.unwrap_opt_word(fw, &b, bt);
                let lit_l = matches!(lhs.node, ExprNode::Int(_) | ExprNode::UInt(_));
                let lit_r = matches!(rhs.node, ExprNode::Int(_) | ExprNode::UInt(_));
                return self.emit_binop(fw, op, a, b, at, bt, lit_l, lit_r, &e.pos);
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
                            // design §20: missing magic overload is a compile
                            // error, not a raw-word negation of the object ptr
                            self.err(
                                &e.pos,
                                format!(
                                    "operator `-` on class `{}` requires a `__neg__` overload",
                                    cls
                                ),
                            );
                            return (String::new(), self.r.mk(Ty::Unit));
                        }
                        if !fl && !self.is_int_like(t) {
                            // str/Array/Map/range/bool have no unary minus
                            let tn = sloth_frontend::ty::ty_name(self.r.get(t));
                            self.err(
                                &e.pos,
                                format!("unary `-` requires a numeric operand, got `{}`", tn),
                            );
                            let z = fw.v();
                            fw.op(&format!("    {} = arith.constant 0 : i64", z));
                            return (z, self.r.mk(Ty::I64));
                        }
                        let _ = &r;
                        if fl {
                            // IEEE negation preserves the sign of -0.0; `0.0 - x`
                            // would collapse -0.0 to +0.0 (bug H1)
                            let vd = emit_dec_f(fw, &v);
                            let nz = fw.v();
                            fw.op(&format!("    {} = arith.negf {} : f64", nz, vd));
                            let r2 = emit_enc_f(fw, &nz);
                            return (r2, t);
                        }
                        let vd = emit_dec_int(fw, &v);
                        let zi = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", zi));
                        let nr = fw.v();
                        fw.op(&format!("    {} = arith.subi {}, {} : i64", nr, zi, vd));
                        let r = emit_enc_int(fw, &nr);
                        let r = self.coerce_int_word(fw, &r, t);
                        (r, t)
                    }
                    UnOp::Not => {
                        if !matches!(self.r.get(t), Ty::Bool) {
                            let tn = sloth_frontend::ty::ty_name(self.r.get(t));
                            self.err(
                                &e.pos,
                                format!("`not` requires a `bool` operand, got `{}`", tn),
                            );
                            let z = fw.v();
                            fw.op(&format!("    {} = arith.constant 0 : i64", z));
                            return (z, self.r.mk(Ty::Bool));
                        }
                        // compute `1 - dec(w)` in raw ints, then encode once:
                        // (enc(1) - dec(w)) would double-encode the result
                        let one = fw.v();
                        fw.op(&format!("    {} = arith.constant 1 : i64", one));
                        let vd = emit_dec_int(fw, &v);
                        let z2 = fw.v();
                        fw.op(&format!("    {} = arith.subi {}, {} : i64", z2, one, vd));
                        (emit_enc_int(fw, &z2), t)
                    }
                    UnOp::BitNot => {
                        // int-only: `~x` = decode, xor with -1, encode
                        if !self.is_int_like(t) {
                            self.err(&e.pos, "`~` requires an integer operand".to_string());
                            let z = fw.v();
                            fw.op(&format!("    {} = arith.constant 0 : i64", z));
                            return (z, self.r.mk(Ty::I64));
                        }
                        let vd = emit_dec_int(fw, &v);
                        let m1 = fw.v();
                        fw.op(&format!("    {} = arith.constant -1 : i64", m1));
                        let r2 = fw.v();
                        fw.op(&format!("    {} = arith.xori {}, {} : i64", r2, vd, m1));
                        let r = emit_enc_int(fw, &r2);
                        let r = self.coerce_int_word(fw, &r, t);
                        (r, t)
                    }
                }
            }
            _ => self.emit_expr_leaf_codes(fw, e),
        }
    }
}

impl ModEmitter {
    /// short-circuit `and`/`or`. The lhs word is stored as the provisional
    /// result; the rhs block runs only when it can change the outcome.
    pub(crate) fn emit_logical(
        &mut self,
        fw: &mut FnWalk,
        op: &BinOp,
        a: String,
        at: TyId,
        rhs: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let bool_ty = self.r.mk(Ty::Bool);
        if self.r.get(at) != &Ty::Bool {
            let tn = sloth_frontend::ty::ty_name(self.r.get(at));
            self.err_diff(pos, "logical operand", "bool", &tn);
        }
        let resslot = fw.v();
        fw.op(&format!(
            "    {} = memref.alloca() : memref<1xi64>",
            resslot
        ));
        let rz = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", rz));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            a, resslot, rz
        ));
        // cond: dec(a) != 0
        let ad = emit_dec_int(fw, &a);
        let zc = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", zc));
        let nz = fw.v();
        fw.op(&format!("    {} = arith.cmpi ne, {}, {} : i64", nz, ad, zc));
        let nze = fw.v();
        fw.op(&format!("    {} = arith.extui {} : i1 to i64", nze, nz));
        let lbl_rhs = fw.newlabel("sc");
        let lbl_end = fw.newlabel("sc");
        let saved_dangling = std::mem::take(&mut fw.dangling);
        let saved_xfer = std::mem::take(&mut fw.xfer);
        match op {
            // and: a true -> evaluate rhs; a false -> keep a
            BinOp::And => fw.cjump(&nze, &lbl_rhs, &lbl_end),
            // or: a true -> keep a; a false -> evaluate rhs
            BinOp::Or => fw.cjump(&nze, &lbl_end, &lbl_rhs),
            _ => unreachable!(),
        }
        fw.label(&lbl_rhs);
        let (b, bt) = self.emit_expr(fw, rhs);
        let (b, bt) = self.unwrap_opt_word(fw, &b, bt);
        if self.r.get(bt) != &Ty::Bool {
            let tn = sloth_frontend::ty::ty_name(self.r.get(bt));
            self.err_diff(&rhs.pos, "logical operand", "bool", &tn);
        }
        let bz = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", bz));
        fw.op(&format!(
            "    memref.store {}, {}[{}] : memref<1xi64>",
            b, resslot, bz
        ));
        fw.rc_flush();
        fw.jump(&lbl_end);
        fw.label(&lbl_end);
        fw.dangling = saved_dangling;
        fw.xfer = saved_xfer;
        let lz = fw.v();
        let rv = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", lz));
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            rv, resslot, lz
        ));
        (rv, bool_ty)
    }

    pub(crate) fn emit_binop(
        &mut self,
        fw: &mut FnWalk,
        op: &BinOp,
        a: String,
        b: String,
        at: TyId,
        bt: TyId,
        lit_l: bool,
        lit_r: bool,
        pos: &Pos,
    ) -> (String, TyId) {
        let cmp_ty_id = self.r.mk(Ty::Bool);
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
                    let osig = vec![mlir_word_ty(at, &self.r), mlir_word_ty(bt, &self.r)];
                    return self
                        .emit_method_call(fw, &defcls, oname, &fd, false, &oargv, &osig, pos);
                }
                // patch #33: no overload on class surfaces -> never silently
                // degrade to word compare (cmpi/cmpf pointer identity)
                if let Ty::Named(bc, _) = self.r.get(bt).clone() {
                    if self.class_ids.get(&bc).is_some() || self.class_ids.get(&cls).is_some() {
                        self.err(
                            pos,
                            format!(
                            "comparison `{:?}` on classes `{}` and `{}` requires a `{}` overload",
                            op, cls, bc, oname
                        ),
                        );
                        return (String::new(), cmp_ty_id);
                    }
                }
            }
        }
        // str value equality (patch #36): ==/!= route to the content
        // comparison instead of handle identity
        if matches!(op, BinOp::EqEq | BinOp::NotEq) && self.is_str(at) && self.is_str(bt) {
            let r = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_str_eq({}, {}) : (i64, i64) -> i64",
                r, a, b
            ));
            let rv = if op == &BinOp::NotEq {
                let rd = emit_dec_int(fw, &r);
                let one = fw.v();
                fw.op(&format!("    {} = arith.constant 1 : i64", one));
                let o = fw.v();
                fw.op(&format!("    {} = arith.xori {}, {} : i64", o, rd, one));
                emit_enc_int(fw, &o)
            } else {
                r
            };
            return (rv, cmp_ty_id);
        }
        // range value equality (book ch19 §19.3 lists `range` as a builtin
        // Equatable): compare (lo, hi) content, not the box handle (bug C4/E2)
        if matches!(op, BinOp::EqEq | BinOp::NotEq)
            && matches!(self.r.get(at), Ty::Range)
            && matches!(self.r.get(bt), Ty::Range)
        {
            let la = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_range_lo({}) : (i64) -> i64",
                la, a
            ));
            let ha = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_range_hi({}) : (i64) -> i64",
                ha, a
            ));
            let lb = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_range_lo({}) : (i64) -> i64",
                lb, b
            ));
            let hb = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_range_hi({}) : (i64) -> i64",
                hb, b
            ));
            let e1 = fw.v();
            fw.op(&format!("    {} = arith.cmpi eq, {}, {} : i64", e1, la, lb));
            let e2 = fw.v();
            fw.op(&format!("    {} = arith.cmpi eq, {}, {} : i64", e2, ha, hb));
            let e3 = fw.v();
            fw.op(&format!("    {} = arith.andi {}, {} : i1", e3, e1, e2));
            let mut rv = ext_bool(fw, &e3);
            if op == &BinOp::NotEq {
                let one = fw.v();
                fw.op(&format!("    {} = arith.constant 1 : i64", one));
                let o = fw.v();
                fw.op(&format!("    {} = arith.xori {}, {} : i64", o, rv, one));
                rv = o;
            }
            return (rv, cmp_ty_id);
        }
        // design §2.1: no implicit numeric conversion — comparisons require
        // both operands to share their type; int vs float is a compile error
        if self.is_float(at) != self.is_float(bt) {
            let an = sloth_frontend::ty::ty_name(self.r.get(at));
            let bn = sloth_frontend::ty::ty_name(self.r.get(bt));
            self.err_diff(pos, "comparison operand", &an, &bn);
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, cmp_ty_id);
        }
        let fl = self.is_float(at) && self.is_float(bt);
        // relational `< <= > >=` are numeric (design §7.1); a non-numeric
        // surface would degrade to raw pointer-word compare (bug B1/C1).
        // `==`/`!=` stay word/content compare for bool/str/class handles.
        if matches!(op, BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge)
            && !fl
            && !(self.is_int_like(at) && self.is_int_like(bt))
            && !matches!(self.r.get(at), Ty::Unit)
            && !matches!(self.r.get(bt), Ty::Unit)
        {
            let an = sloth_frontend::ty::ty_name(self.r.get(at));
            let bn = sloth_frontend::ty::ty_name(self.r.get(bt));
            self.err_diff(pos, "comparison operand", &an, &bn);
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, cmp_ty_id);
        }
        if fl {
            // tagged f64 words -> raw scalars for cmpf
            let a = emit_dec_f(fw, &a);
            let b = emit_dec_f(fw, &b);
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
            fw.op(&format!(
                "    {} = arith.cmpf {}, {}, {} : f64",
                r, pred, a, b
            ));
            // bool words are encoded (0 / enc(1))
            (ext_bool(fw, &r), cmp_ty_id)
        } else {
            let z = fw.v();
            let mk = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", mk));
            // unify integer surfaces (an `int` literal adopts the other side's
            // fixed width) and compare at the shared width with signed vs
            // unsigned predicates
            let (a, b, uns) = if self.is_int_like(at) && self.is_int_like(bt) {
                match self.unify_int(at, lit_l, bt, lit_r) {
                    Some(t) => (
                        self.coerce_int_word(fw, &a, t),
                        self.coerce_int_word(fw, &b, t),
                        self.is_unsigned_int(t),
                    ),
                    None => {
                        let an = sloth_frontend::ty::ty_name(self.r.get(at));
                        let bn = sloth_frontend::ty::ty_name(self.r.get(bt));
                        self.err_diff(pos, "comparison operand", &an, &bn);
                        return (z, cmp_ty_id);
                    }
                }
            } else {
                (a, b, false)
            };
            let pr = match op {
                BinOp::EqEq => "eq",
                BinOp::NotEq => "ne",
                BinOp::Lt => {
                    if uns {
                        "ult"
                    } else {
                        "slt"
                    }
                }
                BinOp::Le => {
                    if uns {
                        "ule"
                    } else {
                        "sle"
                    }
                }
                BinOp::Gt => {
                    if uns {
                        "ugt"
                    } else {
                        "sgt"
                    }
                }
                BinOp::Ge => {
                    if uns {
                        "uge"
                    } else {
                        "sge"
                    }
                }
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
            fw.op(&format!(
                "    {} = arith.cmpi {}, {}, {} : i64",
                z, pr, a, b
            ));
            // bool words are encoded (0 / enc(1)); ordering cmps stay
            // monotonic under the x2 map so the word pair compares as-is
            (ext_bool(fw, &z), cmp_ty_id)
        }
    }
}

impl ModEmitter {
    pub(crate) fn emit_expr_leaf_codes(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
        match &e.node {
            ExprNode::Call { callee, args } => {
                return self.emit_call(fw, callee, args, &e.pos, None);
            }
            ExprNode::GenCall {
                callee,
                targs,
                args,
            } => {
                return self.emit_call(fw, callee, args, &e.pos, Some(targs));
            }
            ExprNode::Field { obj, name } => {
                // qualified foreign-global read: lib.g / lib.Cls.f handled in arith path only for globals
                if let ExprNode::Ident(m) = &obj.node {
                    self.guard_hidden(m, name, &e.pos);
                    let key = format!("{}.{}", m, name);
                    if let Some((g, gt, _)) = self.fglobals.get(&key).cloned() {
                        return self.emit_global_read(fw, &g, gt);
                    }
                }
                let (recv, rt) = self.emit_expr(fw, obj);
                if let Ty::Named(c, _) = self.r.get(rt).clone() {
                    if self.extern_types.contains(c.as_str()) {
                        self.err(
                            &e.pos,
                            format!("extern type `{}` is opaque (cannot access fields)", c),
                        );
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    // a method reference `obj.method` (no field of that name)
                    // yields a first-class function value bound to `obj`
                    // (design §6 迁移表)
                    if !self.has_field(&c, name) {
                        if let Some((defcls, fd)) = self.find_method(&c, name) {
                            return self
                                .emit_method_ref_value(fw, &defcls, name, &fd, &recv, &e.pos);
                        }
                        // neither a field nor a method: a name miss is not part
                        // of the surface (book §5.4) — diagnose instead of
                        // reading one word past the object body
                        self.err(&e.pos, format!("unknown field `{}` on class `{}`", name, c));
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                    let idx = self.field_index(&c, name);
                    let zi = fw.v();
                    // rt decodes the field index word
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        zi,
                        enc_i_lit(idx as i64)
                    ));
                    let fty2 = self.field_type(&c, name);
                    match self.r.get(fty2) {
                        Ty::Str => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, self.r.mk(Ty::Str));
                        }
                        Ty::F64 => {
                            // tag migration: f64 words ride the word route
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, fty2);
                        }
                        Ty::Named(_, _)
                        | Ty::Dyn(_)
                        | Ty::Array(_)
                        | Ty::Map(..)
                        | Ty::Tensor(..)
                        | Ty::Bool
                        // builtin handle surfaces keep their type through a
                        // field read (fn / fiber / joinhandle / channel /
                        // mutex / atomic / any), or a later `.method()` call
                        // and indirect call can't dispatch on the receiver
                        | Ty::Fn(_)
                        | Ty::Fiber(_)
                        | Ty::JoinHandle(_)
                        | Ty::Channel(_)
                        | Ty::Mutex
                        | Ty::AtomicInt
                        | Ty::Any
                        | Ty::Range => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, fty2);
                        }
                        // patch 42: value-optional field words are box handles
                        Ty::Opt(_) => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, fty2);
                        }
                        // patch 43: weakbox handles ride their type surface
                        Ty::Weak(_) => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, fty2);
                        }
                        _ => {
                            let r = fw.v();
                            fw.op(&format!(
                                "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
                                r, recv, zi
                            ));
                            return (r, self.r.mk(Ty::I64));
                        }
                    }
                }
                if matches!(self.r.get(rt), Ty::Opt(_)) {
                    // design §3.6: `?.` is not in this version — an optional
                    // receiver must be narrowed with `is not nil` first
                    self.err(
                        &e.pos,
                        format!(
                            "field `{}` on an optional receiver; narrow it with `is not nil` first (design §3.6)",
                            name
                        ),
                    );
                    return (String::new(), self.r.mk(Ty::Unit));
                }
                self.err(&e.pos, format!("field `{}` on unknown type", name));
                (String::new(), self.r.mk(Ty::Unit))
            }
            ExprNode::Range {
                low,
                high,
                inclusive,
            } => {
                // range as a first-class value: rc box {lo, hi(exclusive)}.
                // Inclusive `a..=b` normalizes to hi=b+1 (encoded-word add).
                // Bound elements are `int` (book §5.1); anything else would be
                // reinterpreted as a raw word (float-range hang, bug B10).
                let (lo, lt) = self.emit_expr(fw, low);
                let (hi, ht) = self.emit_expr(fw, high);
                for (t, side) in [(lt, "start"), (ht, "end")] {
                    if !self.is_int_like(t) && !matches!(self.r.get(t), Ty::Unit) {
                        let got = sloth_frontend::ty::ty_name(self.r.get(t)).to_string();
                        self.err(
                            &e.pos,
                            format!("range {} must be `int`, got `{}`", side, got),
                        );
                    }
                }
                let hi2 = if *inclusive {
                    let one = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        one,
                        enc_i_lit(1)
                    ));
                    let h = fw.v();
                    fw.op(&format!("    {} = arith.addi {}, {} : i64", h, hi, one));
                    h
                } else {
                    hi
                };
                let r = fw.v();
                fw.op(&format!(
                    "    {} = func.call @__sloth_range_pack({}, {}) : (i64, i64) -> i64",
                    r, lo, hi2
                ));
                let rt = self.r.mk(Ty::Range);
                self.dangling_producer(fw, &r, rt);
                (r, rt)
            }
            ExprNode::List(xs) => {
                // array literal: fixed-length gc allocation of i64/f64 words
                let mut evs: Vec<String> = Vec::new();
                let mut ets: Vec<TyId> = Vec::new();
                // a declared element surface resolves `ok()/err()` ctor
                // elements inside the literal (bug OPT5/E4)
                let elem_expected =
                    match self.exp_ret.last().copied().map(|t| self.r.get(t).clone()) {
                        Some(Ty::Array(e)) => Some(e),
                        _ => None,
                    };
                for x in xs {
                    let exp = elem_expected.unwrap_or_else(|| self.r.mk(Ty::Unit));
                    if let Some(e) = elem_expected {
                        if let Some((v, t)) = self.try_literal_result_ctor(fw, x, e) {
                            evs.push(v);
                            ets.push(t);
                            continue;
                        }
                    }
                    // Scope the outer element surface to the element itself:
                    // without this a nested array literal `[[1]]` typed
                    // `Array<Array<int>?>` saw the *outer* `Array` annotation as
                    // its own element hint, mis-tagged its int element as
                    // `Array<int>?` and retained the raw int as a handle
                    // (rc subtract overflow; probe p_optmatrix).
                    self.exp_ret.push(exp);
                    let diag_before = self.diags.len();
                    let (v, t) = self.emit_expr(fw, x);
                    self.exp_ret.pop();
                    // a `unit` element has no word (probe containers/x7)
                    let report = self.diags.len() == diag_before;
                    let v = self.value_or_nil_word(fw, v, t, &x.pos, report);
                    evs.push(v);
                    ets.push(t);
                }
                // declared `Array<T?>`: the annotation's optional element
                // surface wins, including *reference* optionals (nil stays nil,
                // bare scalars box up) — otherwise `[C()]` collapses to
                // `Array<C>` and `is nil` becomes a false error (bug B9/OPT4)
                let hint_el0 = match self.exp_ret.last().copied().map(|t| self.r.get(t).clone()) {
                    Some(Ty::Array(e)) => Some(e),
                    _ => None,
                };
                if let Some(he) = hint_el0 {
                    if matches!(self.r.get(he), Ty::Opt(_)) {
                        for i in 0..evs.len() {
                            let et = self.r.get(ets[i]).clone();
                            if matches!(et, Ty::Unit) {
                                ets[i] = he;
                            } else if self.opt_inner(he).is_some() {
                                let (c2, t2) = self.coerce_into_opt(fw, &evs[i], ets[i], he);
                                evs[i] = c2;
                                ets[i] = t2;
                            } else {
                                ets[i] = he;
                            }
                        }
                    }
                }
                // design §2.1: no implicit numeric conversion — an array
                // literal mixing int and float elements has no common element
                // type and is a compile error
                let nfl = ets.iter().filter(|t| self.is_float(**t)).count();
                if nfl > 0 && nfl < ets.len() {
                    self.err_diff(&e.pos, "array literal element", "float", "int");
                }
                let anyf = nfl > 0;
                if anyf {
                    for (v, t) in evs.iter_mut().zip(ets.iter_mut()) {
                        if !self.is_float(*t) {
                            *v = self.int_to_f64_word(fw, v, *t);
                            *t = self.r.mk(Ty::F64);
                        }
                    }
                }
                // patch 42: a single Opt(值型) element family unifies the
                // whole literal under the Opt surface; bare scalars box up
                if !anyf {
                    let inner = ets.iter().find_map(|t| self.opt_inner(*t).map(|x| x.0));
                    if let Some(inner) = inner {
                        let ik = self.r.get(inner).clone();
                        let allok = ets.iter().all(|t| match self.opt_inner(*t) {
                            Some((in2, _)) => self.r.get(in2).clone() == ik,
                            None => self.r.get(*t).clone() == ik,
                        });
                        if allok {
                            let ot = self.r.mk(Ty::Opt(inner));
                            for i in 0..evs.len() {
                                let (c2, _t2) = self.coerce_into_opt(fw, &evs[i], ets[i], ot);
                                evs[i] = c2;
                                ets[i] = ot;
                            }
                        }
                    }
                }
                // declared store face (`var a: Array<Weak<T>> = [t]` or
                // `Array<int?> = [1, 2]`) coerces each element; without the
                // Weak wrap a strong handle would sit in a Weak slot and
                // upgrade() would misread it as a box
                let hint_el = match self.exp_ret.last().copied().map(|t| self.r.get(t).clone()) {
                    Some(Ty::Array(e)) => Some(e),
                    _ => None,
                };
                if !anyf {
                    if let Some(he) = hint_el {
                        if self.weak_inner(he).is_some()
                            || self.opt_inner(he).is_some()
                            || matches!(self.r.get(he), Ty::Dyn(_) | Ty::Any | Ty::Int(_))
                        {
                            for i in 0..evs.len() {
                                if self.r.get(ets[i]).clone() != self.r.get(he).clone() {
                                    let (c2, t2) = self.coerce_word_to(fw, &evs[i], ets[i], he);
                                    evs[i] = c2;
                                    ets[i] = t2;
                                }
                            }
                        }
                    }
                }
                // homogeneity (book ch12 §12.1): every element must share a
                // common element surface. A declared hint wins and is checked
                // element-wise; otherwise unrelated families (int/bool/str/
                // unrelated classes) are a compile error rather than a silent
                // `Array<int>` fallback that stores raw pointers (bug B5).
                if !ets.is_empty() && !anyf {
                    let first = ets[0];
                    let allsame = ets.iter().all(|t| self.r.get(*t) == self.r.get(first));
                    if !allsame {
                        if let Some(he) = hint_el {
                            for i in 0..ets.len() {
                                let ht = self.r.get(he).clone();
                                let et = self.r.get(ets[i]).clone();
                                if !matches!(et, Ty::Unit)
                                    && !matches!(ht, Ty::Unit)
                                    && !self.surface_compat(&ht, &et)
                                {
                                    let hn = self.surface_name(&ht);
                                    let en = self.surface_name(&et);
                                    self.err_diff(&xs[i].pos, "array literal element", &hn, &en);
                                }
                            }
                        } else if self.named_lub(&ets).is_none() {
                            self.err(
                                &e.pos,
                                "array literal elements have no common type".to_string(),
                            );
                        }
                    }
                }
                let n = fw.v();
                fw.op(&format!(
                    "    {} = arith.constant {} : i64",
                    n,
                    enc_i_lit(evs.len() as i64)
                ));
                let arr = fw.v();
                // element-refness flag drives the death cascade; an empty
                // literal takes it from the annotation hint
                let el_ref = if ets.is_empty() {
                    hint_el.map(|h| self.is_ref(h)).unwrap_or(false)
                } else {
                    ets.iter().any(|t| self.is_ref(*t))
                };
                if el_ref {
                    let k2 = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        k2,
                        enc_i_lit(1)
                    ));
                    fw.op(&format!(
                        "    {} = func.call @__sloth_arr_new_k({}, {}) : (i64, i64) -> i64",
                        arr, n, k2
                    ));
                } else {
                    fw.op(&format!(
                        "    {} = func.call @__sloth_arr_new({}) : (i64) -> i64",
                        arr, n
                    ));
                }
                let ael = self.r.mk(Ty::Unit);
                let at2 = self.r.mk(Ty::Array(ael));
                self.dangling_producer(fw, &arr, at2);
                for (i, v) in evs.iter().enumerate() {
                    let zi = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        zi,
                        enc_i_lit(i as i64)
                    ));
                    // rc patch B: array owns ref-typed elements
                    let mut vv = v.clone();
                    if !anyf && self.is_ref(ets[i]) {
                        vv = self.emit_retain(fw, v);
                    }
                    fw.op(&format!(
                        "    func.call @__sloth_arr_set({}, {}, {}) : (i64, i64, i64) -> i64",
                        arr, zi, vv
                    ));
                }
                let ty = if ets.is_empty() {
                    // empty literal: adopt the expected element type from the
                    // surrounding annotation (`var a: Array<float> = []`)
                    let ei = hint_el.unwrap_or_else(|| self.r.mk(Ty::I64));
                    self.r.mk(Ty::Array(ei))
                } else if {
                    let first = *ets.first().unwrap();
                    ets.iter().all(|t| self.r.get(*t) == self.r.get(first))
                } {
                    self.r.mk(Ty::Array(ets[0]))
                } else if anyf {
                    let ef = self.r.mk(Ty::F64);
                    self.r.mk(Ty::Array(ef))
                } else if let Some(n) = self.named_lub(&ets) {
                    // heterogeneous class instances upcast to their nearest
                    // common ancestor (e.g. [Animal, Dog, Puppy] -> Array<Animal>)
                    let e = self.r.mk(Ty::Named(n, Vec::new()));
                    self.r.mk(Ty::Array(e))
                } else if let Some(e) = hint_el {
                    self.r.mk(Ty::Array(e))
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
                // declared value surface resolves `ok()/err()` ctor values
                // inside the literal (bug OPT5/E4)
                let val_expected = match self.exp_ret.last().copied().map(|t| self.r.get(t).clone())
                {
                    Some(Ty::Map(_, v)) => Some(v),
                    _ => None,
                };
                for (k, v) in pairs {
                    let kd = self.diags.len();
                    let (kv, kt) = self.emit_expr(fw, k);
                    // a `unit` key/value has no word (probe containers/x7)
                    let kv = self.value_or_nil_word(fw, kv, kt, &k.pos, self.diags.len() == kd);
                    let vd = self.diags.len();
                    let (vv, vt) = match val_expected {
                        Some(e) => match self.try_literal_result_ctor(fw, v, e) {
                            Some((w, t)) => (w, t),
                            None => self.emit_expr(fw, v),
                        },
                        None => self.emit_expr(fw, v),
                    };
                    let vv = self.value_or_nil_word(fw, vv, vt, &v.pos, self.diags.len() == vd);
                    kevs.push((kv, kt));
                    vevs.push((vv, vt));
                }
                // key kind: str handles vs i64 words (uniform family check).
                // Families distinguish int / bool / float / str / range / class
                // so `@(1: …, true: …)` is rejected instead of collapsing
                // `true` onto the int word `1` (E3/C3).
                let kvm: Vec<TyId> = kevs.iter().map(|x| x.1).collect();
                let kfams: Vec<String> = kvm.iter().map(|t| self.map_key_family(*t)).collect();
                let known: Vec<&String> = kfams.iter().filter(|f| f.as_str() != "?").collect();
                if known.len() > 1 && known.iter().any(|f| *f != known[0]) {
                    self.err(&e.pos, "mixed map key types".to_string());
                }
                let anyk_str = kfams.iter().any(|f| f == "str");
                let anyk_float = kfams.iter().any(|f| f == "float");
                let anyk_obj = kfams.iter().any(|f| f == "class");
                if anyk_obj {
                    for t in kvm.iter() {
                        match self.r.get(*t).clone() {
                            Ty::Named(cls, _) => {
                                if !self.impl_chain_has(&cls, "Hashable") {
                                    self.err(
                                        &e.pos,
                                        format!("map key `{}` does not implement Hashable", cls),
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
                // empty map literal: adopt K/V from the surrounding annotation
                // (e.g. `var m: Map<str, int> = @()`)
                let hint_kv = if pairs.is_empty() {
                    match self.exp_ret.last().copied().map(|t| self.r.get(t).clone()) {
                        Some(Ty::Map(k, v)) => Some((k, v)),
                        _ => None,
                    }
                } else {
                    None
                };
                let kty = if !kvm.is_empty() {
                    // homogeneous surface: keep the exact key type so
                    // `Map<bool,_>` / `Map<int8,_>` / `Map<range,_>` survive
                    // (bug C3/E1); mixed widths collapse to i64
                    if kvm.iter().all(|t| *t == kvm[0]) {
                        kvm[0]
                    } else if anyk_str {
                        self.r.mk(Ty::Str)
                    } else if anyk_float {
                        self.r.mk(Ty::F64)
                    } else if anyk_obj {
                        kvm[0]
                    } else {
                        self.r.mk(Ty::I64)
                    }
                } else if let Some((k, _)) = hint_kv {
                    k
                } else {
                    self.r.mk(Ty::I64)
                };
                // declared `Map<_, T?>`: mirror the list-literal optional
                // element rule (reference optionals included) — bug B9/OPT4
                let hint_v0 = match self.exp_ret.last().copied().map(|t| self.r.get(t).clone()) {
                    Some(Ty::Map(_, v)) => Some(v),
                    _ => None,
                };
                if let Some(hv) = hint_v0 {
                    if matches!(self.r.get(hv), Ty::Opt(_)) {
                        for x in vevs.iter_mut() {
                            let et = self.r.get(x.1).clone();
                            if matches!(et, Ty::Unit) {
                                x.1 = hv;
                            } else if self.opt_inner(hv).is_some() {
                                let (c2, t2) = self.coerce_into_opt(fw, &x.0, x.1, hv);
                                x.0 = c2;
                                x.1 = t2;
                            } else {
                                x.1 = hv;
                            }
                        }
                    }
                }
                // value kind: no implicit numeric conversion — mixed int/float
                // values have no common type and are a compile error
                let nvf = vevs.iter().filter(|x| self.is_float(x.1)).count();
                if nvf > 0 && nvf < vevs.len() {
                    self.err_diff(&e.pos, "map value type", "float", "int");
                }
                let anyf = nvf > 0;
                if anyf {
                    for x in vevs.iter_mut() {
                        if !self.is_float(x.1) {
                            x.0 = self.int_to_f64_word(fw, &x.0, x.1);
                            x.1 = self.r.mk(Ty::F64);
                        }
                    }
                }
                // declared value store face (`Map<str, Weak<T>> = @(...)`)
                // coerces bare values the same way the list literal route does
                let hint_v = match self.exp_ret.last().copied().map(|t| self.r.get(t).clone()) {
                    Some(Ty::Map(_k, v)) => Some(v),
                    _ => None,
                };
                if !anyf {
                    if let Some(hv) = hint_v {
                        if self.weak_inner(hv).is_some()
                            || self.opt_inner(hv).is_some()
                            || matches!(self.r.get(hv), Ty::Dyn(_) | Ty::Any | Ty::Int(_))
                        {
                            for x in vevs.iter_mut() {
                                if self.r.get(x.1).clone() != self.r.get(hv).clone() {
                                    let (c2, t2) = self.coerce_word_to(fw, &x.0, x.1, hv);
                                    x.0 = c2;
                                    x.1 = t2;
                                }
                            }
                        }
                    }
                }
                let vts: Vec<TyId> = vevs.iter().map(|x| x.1).collect();
                // value homogeneity (book ch13 §13.1): mirrored from the list
                // literal — unrelated value families are a compile error, not a
                // silent `int` fallback that stores raw pointers (bug B5)
                if !vevs.is_empty() && !anyf {
                    let first = vevs[0].1;
                    let allsame = vevs.iter().all(|y| self.r.get(y.1) == self.r.get(first));
                    if !allsame {
                        if let Some((_, hv)) = hint_kv {
                            for x in vevs.iter() {
                                let ht = self.r.get(hv).clone();
                                let et = self.r.get(x.1).clone();
                                if !matches!(et, Ty::Unit)
                                    && !matches!(ht, Ty::Unit)
                                    && !self.surface_compat(&ht, &et)
                                {
                                    let hn = self.surface_name(&ht);
                                    let en = self.surface_name(&et);
                                    self.err_diff(&e.pos, "map value type", &hn, &en);
                                }
                            }
                        } else if self.named_lub(&vts).is_none() {
                            self.err(&e.pos, "map literal values have no common type".to_string());
                        }
                    }
                }
                let vty = match vevs.first() {
                    Some(x) if vevs.iter().all(|y| self.r.get(y.1) == self.r.get(x.1)) => x.1,
                    _ => {
                        if anyf {
                            self.r.mk(Ty::F64)
                        } else if let Some(n) = self.named_lub(&vts) {
                            self.r.mk(Ty::Named(n, Vec::new()))
                        } else if let Some((_, v)) = hint_kv {
                            v
                        } else {
                            self.r.mk(Ty::I64)
                        }
                    }
                };
                // patch 42: Opt(值型) value family unification (same rule as
                // list literals: bare scalars box under the Opt surface)
                if !anyf {
                    let inner = vevs.iter().find_map(|x| self.opt_inner(x.1).map(|z| z.0));
                    if let Some(inner) = inner {
                        let ik = self.r.get(inner).clone();
                        let allok = vevs.iter().all(|x| match self.opt_inner(x.1) {
                            Some((in2, _)) => self.r.get(in2).clone() == ik,
                            None => self.r.get(x.1).clone() == ik,
                        });
                        if allok {
                            let ot = self.r.mk(Ty::Opt(inner));
                            for x in vevs.iter_mut() {
                                let (c2, _t2) = self.coerce_into_opt(fw, &x.0, x.1, ot);
                                x.0 = c2;
                                x.1 = ot;
                            }
                        }
                    }
                }
                // key route follows the resolved key surface: str=1, hashable
                // class=2, int=0 (empty literals take it from the annotation);
                // bit 2 = value-ref flag driving the death cascade
                let kk = match self.r.get(kty).clone() {
                    Ty::Str => 1i64,
                    Ty::Named(_, _) => 2i64,
                    Ty::Range => 3i64,
                    _ => 0i64,
                };
                let vref = if !anyf && self.is_ref(vty) {
                    1i64
                } else {
                    0i64
                };
                let kv0 = fw.v();
                fw.op(&format!(
                    "    {} = arith.constant {} : i64",
                    kv0,
                    enc_i_lit(kk | (vref << 2))
                ));
                let m = fw.v();
                fw.op(&format!(
                    "    {} = func.call @__sloth_map_new({}) : (i64) -> i64",
                    m, kv0
                ));
                let mt2 = self.r.mk(Ty::Map(kty, vty));
                self.dangling_producer(fw, &m, mt2);
                for (kev, vev) in kevs.iter().zip(vevs.iter()) {
                    // object keys route the monomorphized hash() into the map
                    // (patch #35); pointer identity remains without one
                    let mut use_h: Option<String> = None;
                    if anyk_obj {
                        if let Ty::Named(kcls, _) = self.r.get(kev.1).clone() {
                            match self.find_map_key_hash(&kcls) {
                                Some((hmname, defcls, hfd)) => {
                                    let oargv = vec![(kev.0.clone(), kev.1)];
                                    let osig = vec![mlir_word_ty(kev.1, &self.r)];
                                    let (hv, ht) = self.emit_method_call(
                                        fw, &defcls, &hmname, &hfd, false, &oargv, &osig, &e.pos,
                                    );
                                    let _ = ht;
                                    use_h = Some(hv);
                                }
                                None => {
                                    self.err(
                                        &e.pos,
                                        format!(
                                            "map key `{}` implements no `hash()`-family method — keyed by pointer identity (Hashable surface needs `hash()`/`hashKey()`/`__hash__()`)",
                                            kcls
                                        ),
                                    );
                                }
                            }
                        }
                    }
                    let setsym = if anyk_str {
                        "__sloth_map_str_set"
                    } else {
                        "__sloth_map_set"
                    };
                    // rc patch B: map slots own ref-typed keys/values
                    let kref = matches!(self.r.get(kev.1), Ty::Str | Ty::Named(_, _) | Ty::Range);
                    if kref {
                        self.emit_retain(fw, &kev.0);
                    }
                    if !anyf && self.is_ref(vty) {
                        let rv2 = self.emit_retain(fw, &vev.0);
                        let _ = rv2;
                    }
                    match use_h {
                        Some(h) => {
                            fw.op(&format!(
                                "    func.call @{}({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
                                "__sloth_map_set_h", m, kev.0, h, vev.0
                            ));
                        }
                        None => {
                            fw.op(&format!(
                                "    func.call @{}({}, {}, {}) : (i64, i64, i64) -> i64",
                                setsym, m, kev.0, vev.0
                            ));
                        }
                    }
                }
                (m, self.r.mk(Ty::Map(kty, vty)))
            }
            ExprNode::Index { obj, idx } => {
                let (av, at) = self.emit_expr(fw, obj);
                // tensor extension TE-P1: `t[i]` (rank>1) is a shared-storage
                // view, rank-1 is the element; `t[a..b]` keeps the rank
                if let Ty::Tensor(elem, rank) = self.r.get(at).clone() {
                    return self.emit_tensor_index(fw, &av, elem, rank, idx, &e.pos);
                }
                // str subscript: `s[i]` = i-th raw byte (int), `s[a..b]` /
                // `s[a..=b]` = byte slice (fresh str). Byte-indexed by design;
                // the Unicode-scalar view is `s.chars()`.
                if matches!(self.r.get(at), Ty::Str) {
                    return self.emit_str_index(fw, &av, idx, &e.pos);
                }
                // Array range slicing: `a[lo..hi]` / `a[lo..=hi]` yields a fresh
                // `Array<T>` copy of the chosen span (elements retained).
                if let Ty::Array(el) = self.r.get(at).clone() {
                    if matches!(&idx.node, ExprNode::Range { .. }) {
                        return self.emit_arr_index(fw, &av, el, idx, &e.pos);
                    }
                }
                let (iv, it) = self.emit_expr(fw, idx);
                let _ = it;
                let ats = self.r.get(at).clone();
                let early: std::cell::RefCell<Option<(String, TyId)>> =
                    std::cell::RefCell::new(None);
                let (el, getsym, retty) = match &ats {
                    Ty::Array(_e) => {
                        // tag migration: f64 element words ride the word route
                        (*_e, "__sloth_arr_get", "i64")
                    }
                    Ty::Map(k, v) => {
                        let kkind = matches!(self.r.get(*k), Ty::Str);
                        let _ = it;
                        // object keys call the monomorphized hash() at the
                        // call site (patch #35) and route the _h ops
                        if let Ty::Named(kcls, _) = self.r.get(*k).clone() {
                            if self.class_ids.contains_key(&kcls) {
                                match self.find_map_key_hash(&kcls) {
                                    Some((hmname, defcls, hfd)) => {
                                        let oargv = vec![(iv.clone(), it)];
                                        let osig = vec![mlir_word_ty(it, &self.r)];
                                        let (hv, _ht) = self.emit_method_call(
                                            fw, &defcls, &hmname, &hfd, false, &oargv, &osig,
                                            &e.pos,
                                        );
                                        let (sym, retty) = ("__sloth_map_get_h", "i64");
                                        let r = fw.v();
                                        fw.op(&format!(
                                            "    {} = func.call @{}({}, {}, {}) : (i64, i64, i64) -> {}",
                                            r, sym, av, iv, hv, retty
                                        ));
                                        *early.borrow_mut() = Some((r, *v));
                                        (*v, "__sloth_map_get", "i64")
                                    }
                                    None => {
                                        self.err(
                                            &e.pos,
                                            format!(
                                                "map key `{}` implements no `hash()`-family method — keyed by pointer identity (Hashable surface needs `hash()`/`hashKey()`/`__hash__()`)",
                                                kcls
                                            ),
                                        );
                                        let (sym, retty) = if kkind {
                                            ("__sloth_map_str_get", "i64")
                                        } else {
                                            ("__sloth_map_get", "i64")
                                        };
                                        (*v, sym, retty)
                                    }
                                }
                            } else {
                                // generic type-param key surface: word route
                                let (sym, retty) = if kkind {
                                    ("__sloth_map_str_get", "i64")
                                } else {
                                    ("__sloth_map_get", "i64")
                                };
                                (*v, sym, retty)
                            }
                        } else {
                            let (sym, retty) = if kkind {
                                ("__sloth_map_str_get", "i64")
                            } else {
                                ("__sloth_map_get", "i64")
                            };
                            (*v, sym, retty)
                        }
                    }
                    Ty::Named(cn, _) => {
                        // Indexable overload: a[i] ≡ a.__index__(i); element
                        // type is the method's own return type
                        match self.find_method(cn, "__index__") {
                            Some((defcls, fd)) => {
                                let oargv = vec![(av.clone(), at), (iv.clone(), it)];
                                let osig =
                                    vec![mlir_word_ty(at, &self.r), mlir_word_ty(it, &self.r)];
                                return self.emit_method_call(
                                    fw,
                                    &defcls,
                                    "__index__",
                                    &fd,
                                    false,
                                    &oargv,
                                    &osig,
                                    &e.pos,
                                );
                            }
                            None => {
                                self.err(
                                    &e.pos,
                                    format!(
                                        "class `{}` requires an `__index__` overload for indexing",
                                        cn
                                    ),
                                );
                                (self.r.mk(Ty::Unit), "__sloth_arr_get", "i64")
                            }
                        }
                    }
                    _ => {
                        self.err(&e.pos, format!("indexing non-array"));
                        (self.r.mk(Ty::Unit), "__sloth_arr_get", "i64")
                    }
                };
                if let Some((r, rtv)) = early.borrow_mut().take() {
                    return (r, rtv);
                }
                let r = fw.v();
                fw.op(&format!(
                    "    {} = func.call @{}({}, {}) : (i64, i64) -> {}",
                    r, getsym, av, iv, retty
                ));
                (r, el)
            }
            ExprNode::This | ExprNode::Super => match fw
                .scopes
                .first()
                .and_then(|sc| sc.get("this"))
                .map(|s| (s.0.clone(), s.1))
            {
                Some((a, t)) => {
                    let z = fw.v();
                    let v = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : index", z));
                    fw.op(&format!(
                        "    {} = memref.load {}[{}] : memref<1xi64>",
                        v, a, z
                    ));
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
    /// emit `lo` and `hi`(exclusive) as `i64` words for a range subscript.
    /// `inclusive` normalizes `a..=b` to `hi = b + 1`. `what` names the
    /// container for the non-int diagnostic (e.g. "str range" / "array range").
    pub(crate) fn emit_range_bounds(
        &mut self,
        fw: &mut FnWalk,
        idx: &Expr,
        pos: &Pos,
        what: &str,
    ) -> (String, String) {
        let (low, high, inclusive) = match &idx.node {
            ExprNode::Range {
                low,
                high,
                inclusive,
            } => (low, high, *inclusive),
            _ => unreachable!("emit_range_bounds on a non-range subscript"),
        };
        let ity = self.r.mk(Ty::I64);
        let (lv, lt) = self.emit_expr(fw, low);
        let (hv, ht) = self.emit_expr(fw, high);
        for (t, side) in [(lt, "start"), (ht, "end")] {
            if !self.is_int_like(t) {
                let got = sloth_frontend::ty::ty_name(self.r.get(t)).to_string();
                self.err(
                    pos,
                    format!("{} {} must be `int`, got `{}`", what, side, got),
                );
            }
        }
        let lo = self.coerce_int_word(fw, &lv, ity);
        let mut hi = self.coerce_int_word(fw, &hv, ity);
        if inclusive {
            let one = fw.v();
            fw.op(&format!("    {} = arith.constant 1 : i64", one));
            let h = fw.v();
            fw.op(&format!("    {} = arith.addi {}, {} : i64", h, hi, one));
            hi = h;
        }
        (lo, hi)
    }

    /// array range slicing (`Index` on an `Array<T>`):
    ///   `a[lo..hi]`  -> fresh `Array<T>` copy of `[lo, hi)`
    ///   `a[lo..=hi]` -> fresh copy of `[lo, hi]` (inclusive)
    /// The copy owns retained reference elements; out-of-bounds / inverted
    /// ranges panic inside the runtime (`array slice ...` out of bounds).
    fn emit_arr_index(
        &mut self,
        fw: &mut FnWalk,
        a: &str,
        el: TyId,
        idx: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (lo, hi) = self.emit_range_bounds(fw, idx, pos, "array range");
        let len = fw.v();
        fw.op(&format!("    {} = arith.subi {}, {} : i64", len, hi, lo));
        // element-refness drives the fresh array's death cascade
        let elref = if self.is_ref(el) { 1 } else { 0 };
        let k2 = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            k2,
            enc_i_lit(elref)
        ));
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_arr_slice({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
            r, a, lo, len, k2
        ));
        let rt = self.r.mk(Ty::Array(el));
        // fresh allocation: an owned producer temp
        self.dangling_producer(fw, &r, rt);
        (r, rt)
    }

    /// string subscript (`Index` on a `str`):
    ///   `s[i]`     -> i-th raw byte as `int` (0..255)
    ///   `s[a..b]`  -> byte slice `[a, b)` as a fresh `str`
    ///   `s[a..=b]` -> byte slice `[a, b]` (inclusive)
    /// Byte-indexed by design; the Unicode-scalar view is `s.chars()`.
    /// Out-of-bounds / inverted ranges panic inside the runtime.
    fn emit_str_index(
        &mut self,
        fw: &mut FnWalk,
        s: &str,
        idx: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        if matches!(&idx.node, ExprNode::Range { .. }) {
            let (lo, hi) = self.emit_range_bounds(fw, idx, pos, "str range");
            let len = fw.v();
            fw.op(&format!("    {} = arith.subi {}, {} : i64", len, hi, lo));
            let r = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_str_slice({}, {}, {}) : (i64, i64, i64) -> i64",
                r, s, lo, len
            ));
            let rt = self.r.mk(Ty::Str);
            // fresh allocation: an owned producer temp
            self.dangling_producer(fw, &r, rt);
            return (r, rt);
        }
        let ity = self.r.mk(Ty::I64);
        let (iv, it) = self.emit_expr(fw, idx);
        if !self.is_int_like(it) {
            let got = sloth_frontend::ty::ty_name(self.r.get(it)).to_string();
            self.err(
                pos,
                format!("str index must be `int` (or use `s[a..b]`), got `{}`", got),
            );
        }
        let i = self.coerce_int_word(fw, &iv, ity);
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_str_byte({}, {}) : (i64, i64) -> i64",
            r, s, i
        ));
        (r, self.r.mk(Ty::I64))
    }
}

impl ModEmitter {
    /// method or function call. obj.method() => callee Field{obj,name}: direct dispatch
    pub(crate) fn emit_call(
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
                    for t in ta {
                        self.check_trait_type(t, pos);
                    }
                    let tys: Vec<TyId> = ta.iter().map(|t| self.ty_of(t)).collect();
                    let it = self.declare_class_inst(base, &tys, pos);
                    let iname = match self.r.get(it) {
                        Ty::Named(n, _) => n.clone(),
                        _ => String::new(),
                    };
                    if !iname.is_empty() {
                        let cargs: Vec<(String, TyId)> =
                            args.iter().map(|a| self.emit_expr(fw, a)).collect();
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
                        for tt in ta {
                            self.check_trait_type(tt, pos);
                        }
                        for (tp, tt) in fd.type_params.iter().zip(ta.iter()) {
                            map.insert(tp.name.clone(), self.ty_of(tt));
                        }
                        let _ = tnames;
                        let argv: Vec<(String, TyId)> =
                            args.iter().map(|a| self.emit_expr(fw, a)).collect();
                        return self.emit_ginst_call(fw, base, fd, &argv, pos, map);
                    }
                }
            }
        }
        // tensor extension TE-P1 stdlib face: `tensor.zeros` / `tensor.from_array`
        // (the `tensor` module's builtins are recognized here; the .slt wrapper
        // layer lands in TE-P3)
        if let ExprNode::Field { obj, name } = &callee.node {
            if matches!(&obj.node, ExprNode::Ident(m) if m == "tensor") {
                match name.as_str() {
                    "zeros" if args.len() == 1 => {
                        return self.emit_tensor_zeros(fw, &args[0], pos);
                    }
                    "from_array" if args.len() == 2 => {
                        return self.emit_tensor_from_array(fw, &args[0], &args[1], pos);
                    }
                    // TE-P2 channel-B linalg operators (expr.rs / tensor.rs)
                    "matvec" if args.len() == 2 => {
                        return self.emit_tensor_matvec(fw, &args[0], &args[1], pos);
                    }
                    "matmul" if args.len() == 2 => {
                        return self.emit_tensor_matmul(fw, &args[0], &args[1], pos);
                    }
                    "dot" if args.len() == 2 => {
                        return self.emit_tensor_dot(fw, &args[0], &args[1], pos);
                    }
                    "add" | "sub" | "mul" | "div" if args.len() == 2 => {
                        return self.emit_tensor_binop(fw, &args[0], &args[1], name.as_str(), pos);
                    }
                    "sum" if args.len() == 1 => {
                        return self.emit_tensor_sum(fw, &args[0], pos);
                    }
                    "add_into" if args.len() == 2 => {
                        return self.emit_tensor_add_into(fw, &args[0], &args[1], pos);
                    }
                    // TE-P3 fused kernels / math elementwise
                    "exp" | "sqrt" | "sin" | "cos" | "tan" if args.len() == 1 => {
                        return self.emit_tensor_unary(fw, &args[0], name.as_str(), pos);
                    }
                    "silu" if args.len() == 1 => {
                        return self.emit_tensor_silu(fw, &args[0], pos);
                    }
                    "silu_mul_into" if args.len() == 2 => {
                        return self.emit_tensor_silu_mul_into(fw, &args[0], &args[1], pos);
                    }
                    "rmsnorm" if args.len() == 2 => {
                        return self.emit_tensor_rmsnorm(fw, &args[0], &args[1], pos);
                    }
                    "softmax" if args.len() == 1 => {
                        return self.emit_tensor_softmax(fw, &args[0], false, pos);
                    }
                    "softmax_into" if args.len() == 1 => {
                        return self.emit_tensor_softmax(fw, &args[0], true, pos);
                    }
                    "add_scaled_into" if args.len() == 3 => {
                        return self
                            .emit_tensor_add_scaled_into(fw, &args[0], &args[1], &args[2], pos);
                    }
                    "div_scalar_into" if args.len() == 2 => {
                        return self.emit_tensor_div_scalar_into(fw, &args[0], &args[1], pos);
                    }
                    "fill_zero" if args.len() == 1 => {
                        let (v, _t) = self.emit_expr(fw, &args[0]);
                        fw.op(&format!(
                            "    func.call @__sloth_tensor_fill_zero({}) : (i64) -> i64",
                            v
                        ));
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", z));
                        return (z, self.r.mk(Ty::Unit));
                    }
                    _ => {}
                }
            }
        }
        // coroutine extension CE-P1 builtin module: `fiber.*` (recognized
        // before any receiver evaluation; payload ownership rides the runtime)
        if let ExprNode::Field { obj, name } = &callee.node {
            if matches!(&obj.node, ExprNode::Ident(m) if m == "fiber") {
                match name.as_str() {
                    "create" if args.len() == 2 => {
                        return self.emit_fiber_create(fw, &args[0], &args[1], None, pos);
                    }
                    "create_with" if args.len() == 3 => {
                        return self.emit_fiber_create(fw, &args[0], &args[1], Some(&args[2]), pos);
                    }
                    "resume" if args.len() == 2 => {
                        return self.emit_fiber_resume(fw, &args[0], &args[1], pos);
                    }
                    "transfer" if args.len() == 2 => {
                        return self.emit_fiber_transfer(fw, &args[0], &args[1], pos);
                    }
                    "yield" if args.len() == 1 => {
                        return self.emit_fiber_yield(fw, &args[0], pos);
                    }
                    "error" if args.len() == 1 => {
                        return self.emit_fiber_error(fw, &args[0], pos);
                    }
                    "check" if args.len() == 1 => {
                        return self.emit_fiber_check(fw, &args[0]);
                    }
                    "resumable" if args.len() == 1 => {
                        return self.emit_fiber_resumable(fw, &args[0]);
                    }
                    "cancel" if args.len() == 1 => {
                        return self.emit_fiber_cancel(fw, &args[0]);
                    }
                    _ => {}
                }
            }
        }
        // multithreading extension TH-P1/P2 builtin modules: `thread.*`,
        // `channel.*`, `mutex.*`, `atomic.*` (recognized before any receiver
        // evaluation; ownership is handled by the runtime entry points)
        if let ExprNode::Field { obj, name } = &callee.node {
            if matches!(&obj.node, ExprNode::Ident(m) if m == "thread") {
                match name.as_str() {
                    "spawn" if args.len() == 2 => {
                        return self.emit_thread_spawn(fw, &args[0], &args[1], pos);
                    }
                    "current_id" if args.is_empty() => {
                        return self.emit_thread_current_id(fw);
                    }
                    "yield_now" if args.is_empty() => {
                        return self.emit_thread_yield(fw);
                    }
                    _ => {}
                }
            }
            if matches!(&obj.node, ExprNode::Ident(m) if m == "channel") && name == "new" {
                let targ = match targs_in.and_then(|v| v.first()) {
                    Some(t) => self.ty_of(t),
                    None => return self.th_bail(
                        fw,
                        pos,
                        "channel.new requires an explicit element type: `channel.new<T>(capacity)`",
                    ),
                };
                if args.len() == 1 {
                    return self.emit_channel_new(fw, targ, &args[0], pos);
                }
                return self.th_bail(fw, pos, "channel.new requires a capacity argument");
            }
            if matches!(&obj.node, ExprNode::Ident(m) if m == "mutex")
                && name == "new"
                && args.is_empty()
            {
                return self.emit_mutex_new(fw);
            }
            if matches!(&obj.node, ExprNode::Ident(m) if m == "atomic")
                && name == "new"
                && args.len() == 1
            {
                return self.emit_atomic_new(fw, &args[0], pos);
            }
        }
        // TE-P3 scalar math faces (design D6): `float_sqrt/exp/sin/cos/tan/pow`
        if let ExprNode::Ident(fname) = &callee.node {
            let sym = match fname.as_str() {
                "float_sqrt" => Some("__sloth_rt_sqrt"),
                "float_exp" => Some("__sloth_rt_exp"),
                "float_sin" => Some("__sloth_rt_sin"),
                "float_cos" => Some("__sloth_rt_cos"),
                "float_tan" => Some("__sloth_rt_tan"),
                "float_floor" => Some("__sloth_rt_floor"),
                _ => None,
            };
            if let Some(sym) = sym {
                if args.len() == 1 {
                    let (av, at) = self.emit_expr(fw, &args[0]);
                    if !self.is_float(at) {
                        self.err(pos, format!("`{}` requires a `float` argument", fname));
                        return self.tensor_bail(fw);
                    }
                    let af = emit_dec_f(fw, &av);
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @{}({}) : (f64) -> f64",
                        r, sym, af
                    ));
                    return (emit_enc_f(fw, &r), self.r.mk(Ty::F64));
                }
            }
            if fname == "float_pow" && args.len() == 2 {
                let (av, at) = self.emit_expr(fw, &args[0]);
                let (bv, bt) = self.emit_expr(fw, &args[1]);
                if !self.is_float(at) || !self.is_float(bt) {
                    self.err(pos, "`float_pow` requires `float` arguments".into());
                    return self.tensor_bail(fw);
                }
                let af = emit_dec_f(fw, &av);
                let bf = emit_dec_f(fw, &bv);
                let r = fw.v();
                fw.op(&format!(
                    "    {} = func.call @__sloth_rt_pow({}, {}) : (f64, f64) -> f64",
                    r, af, bf
                ));
                return (emit_enc_f(fw, &r), self.r.mk(Ty::F64));
            }
        }
        let mut name = match &callee.node {
            ExprNode::Ident(n) => n.clone(),
            ExprNode::Field { obj: _, name } => name.clone(),
            _ => {
                // arbitrary callee expression yielding a first-class function
                // value, e.g. `make()(x)`, `arr[0](x)`, `(|x| {...})(1)`
                let (clo, ct) = self.emit_expr(fw, callee);
                if let Ty::Fn(ft) = self.r.get(ct).clone() {
                    return self.emit_fn_value_call(fw, &clo, &ft, args, pos);
                }
                self.err(pos, "only named calls supported".to_string());
                return (String::new(), self.r.mk(Ty::Unit));
            }
        };
        // `fn_addr(f)`: raw address of a compiled top-level function as an
        // `int` word. Intercepted before argument evaluation (the argument is
        // a function name, not a value). Used by the self-hosted container
        // prelude to install its `__dispose__` routine as an rc death hook.
        if let ExprNode::Ident(fname0) = &callee.node {
            if fname0 == "fn_addr" && args.len() == 1 {
                if let ExprNode::Ident(fname) = &args[0].node {
                    let sym = if let Some(fd) = self.funcs.get(fname) {
                        if fd.is_extern {
                            fname.clone()
                        } else if self.fixed_syms.contains(fname) {
                            fname.clone()
                        } else {
                            mangle(&self.cur_mod, None, fname)
                        }
                    } else if self.fixed_syms.contains(fname) {
                        // reserved/fixed symbol (e.g. the container prelude
                        // disposers) injected into this module
                        fname.clone()
                    } else if let Some((m, _)) = self.cross_funcs.get(fname) {
                        m.clone()
                    } else {
                        self.err(pos, format!("fn_addr: unknown function `{}`", fname));
                        fname.clone()
                    };
                    let a = fw.v();
                    fw.op(&format!(
                        "    {} = llvm.mlir.addressof @{} : !llvm.ptr",
                        a, sym
                    ));
                    let w = fw.v();
                    fw.op(&format!(
                        "    {} = llvm.ptrtoint {} : !llvm.ptr to i64",
                        w, a
                    ));
                    return (w, self.r.mk(Ty::I64));
                }
                self.err(pos, "fn_addr expects a function name".to_string());
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                return (z, self.r.mk(Ty::I64));
            }
        }
        // method call: receiver becomes first argument
        let mut argv: Vec<(String, TyId)> = Vec::new();
        let mut sigargs: Vec<String> = Vec::new();
        let mut recv: Option<(String, TyId)> = None;
        if let ExprNode::Field { obj, name: mname2 } = &callee.node {
            // qualified cross-module call: lib.fn(...) or alias.fn(...)
            if let ExprNode::Ident(m) = &obj.node {
                let key = format!("{}.{}", m, mname2);
                self.guard_hidden(m, mname2, pos);
                // qualified generic constructor: `lib.Box<int>(…)` must build
                // the monomorphic instance, not the unresolved template (M1)
                if let Some(ta) = targs_in {
                    if let Some((_dm, cdef)) = self.class_defs.get(mname2.as_str()).cloned() {
                        let ntp = cdef.type_params.len();
                        if ntp > 0 && ntp == ta.len() {
                            for t in ta {
                                self.check_trait_type(t, pos);
                            }
                            let tys: Vec<TyId> = ta.iter().map(|t| self.ty_of(t)).collect();
                            let it = self.declare_class_inst(mname2, &tys, pos);
                            let iname = match self.r.get(it) {
                                Ty::Named(n, _) => n.clone(),
                                _ => String::new(),
                            };
                            if !iname.is_empty() {
                                let cargv: Vec<(String, TyId)> =
                                    args.iter().map(|a| self.emit_expr(fw, a)).collect();
                                return self.emit_new_obj(fw, &iname, &cargv, &Vec::new(), pos);
                            }
                        }
                    }
                }
                if let Some((defmod, fd)) = self.foreign_func_defs.get(&key).cloned() {
                    // qualified call to an imported generic function: build the
                    // monomorphic instance in the defining module's namespace
                    let cargv: Vec<(String, TyId)> =
                        args.iter().map(|a| self.emit_expr(fw, a)).collect();
                    let saved = self.cur_mod.clone();
                    self.cur_mod = defmod;
                    let out = self.emit_gfunc_call(fw, mname2, &fd, &cargv, pos);
                    self.cur_mod = saved;
                    return out;
                }
                if let Some(fs) = self.cross_funcs.get(&key).cloned() {
                    // an imported `extern func` must marshal through the C ABI
                    // (f64/i64 spellings), not the word-call route (bug M5)
                    if let Some(fd) = self.funcs.get(mname2).cloned() {
                        if fd.is_extern {
                            let cargv: Vec<(String, TyId)> =
                                args.iter().map(|a| self.emit_expr(fw, a)).collect();
                            return self.emit_extern_call(fw, mname2, &fd, &cargv, pos);
                        }
                    }
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
                            "    func.call @{}({}) : ({}) -> ()",
                            fs.0,
                            vals.join(", "),
                            csig.join(", ")
                        ));
                        return (String::new(), fs.1);
                    }
                    fw.op(&format!(
                        "    {} = func.call @{}({}) : ({}) -> {}",
                        r,
                        fs.0,
                        vals.join(", "),
                        csig.join(", "),
                        rt
                    ));
                    // §5.1.1 rule 4: a sloth function's ref return carries an
                    // owned +1 across the module edge (the callee side
                    // transferred it); mark it so the receiver claims it
                    // instead of retaining a second time. Raw C extern symbols
                    // (`fs.0` unmangled) keep C ownership and are left alone.
                    if self.is_ref(fs.1) && fs.0.as_str() != mname2.as_str() {
                        fw.rc_mark_xfer(&r);
                    }
                    return (r, fs.1);
                }
                if let Some((g, gt, _)) = self.fglobals.get(&key).cloned() {
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
                        "    {} = func.call @__sloth_map_len({}) : (i64) -> i64",
                        r, recvv
                    ));
                    return (r, self.r.mk(Ty::I64));
                }
            }
            let el_tp = match self.r.get(rt) {
                Ty::Array(e) => Some(*e),
                _ => None,
            };
            if let Some(elid) = el_tp {
                let _ = el_tp;
                let el = elid;
                let fel = self.is_float(elid);
                let _ = el;
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
                            // design §2.1: no implicit int -> float on push
                            self.err_diff(pos, "push element", "float", "int");
                            v = self.int_to_f64_word(fw, &v, at);
                        }
                        // store-face coercion: an Opt/Weak element slot boxes a
                        // bare produced value (matches the `a[i] = v` route);
                        // without this a strong handle lands in a Weak slot and
                        // the container's death cascade misreads it as a box
                        if self.opt_inner(elid).is_some() || self.weak_inner(elid).is_some() {
                            let (vc, _tc) = self.coerce_word_to(fw, &v, at, elid);
                            v = vc;
                        }
                        if self.is_int_like(elid) && self.is_int_like(at) {
                            v = self.coerce_int_word(fw, &v, elid);
                        }
                        // general element-surface check (book ch12 §12.1:
                        // `Array<T>` is homogeneous; `push` is the same store
                        // face as `a[i] = v` and must diagnose too)
                        let handled = (fel && !self.is_float(at))
                            || self.opt_inner(elid).is_some()
                            || self.weak_inner(elid).is_some()
                            || (self.is_int_like(elid) && self.is_int_like(at))
                            || matches!(self.r.get(at), Ty::Unit)
                            || matches!(self.r.get(elid), Ty::Unit);
                        if !handled {
                            let els = self.r.get(elid).clone();
                            let ats = self.r.get(at).clone();
                            if !self.surface_compat(&els, &ats) {
                                let en = self.surface_name(&els);
                                let an = self.surface_name(&ats);
                                self.err_diff(pos, "push element", &en, &an);
                            }
                        }
                        let callv = fw.v();
                        // rc patch C: the array slot owns ref-typed
                        // elements (push of a scalar/nil word no-ops)
                        if self.is_ref(elid) {
                            let rv2 = self.emit_retain(fw, &v);
                            v = rv2;
                        }
                        fw.op(&format!(
                            "    {} = func.call @__sloth_arr_push({}, {}) : (i64, i64) -> i64",
                            callv, recvv, v
                        ));
                        // GC growth may relocate the array buffer: the runtime
                        // returns the new handle — write it back to the
                        // receiver's storage (local slot or object field)
                        self.emit_arr_push_writeback(fw, callee, &callv, pos);
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", z));
                        return (z, self.r.mk(Ty::Unit));
                    }
                    "pop" => {
                        let r = fw.v();
                        // rc patch C: pop of a ref element hands its
                        // count to the caller — the value is producer-
                        // owned (dies at statement close unless stored)
                        if self.is_ref(elid) {
                            self.dangling_producer(fw, &r, elid);
                        }
                        fw.op(&format!(
                            "    {} = func.call @__sloth_arr_pop({}) : (i64) -> i64",
                            r, recvv
                        ));
                        return (r, elid);
                    }
                    "len" => {
                        let r = fw.v();
                        fw.op(&format!(
                            "    {} = func.call @__sloth_arr_len({}) : (i64) -> i64",
                            r, recvv
                        ));
                        return (r, self.r.mk(Ty::I64));
                    }
                    _ => {}
                }
            }
        }
        // TH builtin receiver faces: JoinHandle/Channel/Mutex/AtomicInt
        if let Some((recvv, rt)) = recv.clone() {
            if let Some(out) = self.emit_thread_recv_method(fw, &recvv, rt, &name, &argv, pos) {
                return out;
            }
        }
        // Weak<T>.upgrade() receiver face (patch 43): returns the target
        // T? (ref targets keep the handle word; boxed value targets ride
        // their box), retained as a producer-owned strong borrow
        if let Some((recvv, rt)) = recv.clone() {
            if name == "upgrade" {
                if let Some(inner) = self.weak_inner(rt) {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_weak_upgrade({}) : (i64) -> i64",
                        r, recvv
                    ));
                    let ot = self.r.mk(Ty::Opt(inner));
                    if self.is_ref(ot) {
                        // TH: upgrade CAS-retains the target and returns an
                        // owned +1 (nil = dead); trailing as a producer temp
                        self.dangling_producer(fw, &r, ot);
                    }
                    return (r, ot);
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
                let cls = cls.clone();
                // super.m(...) dispatches at the superclass, skipping own overrides
                let is_super = matches!(&callee.node, ExprNode::Field { obj, .. } if matches!(&obj.node, ExprNode::Super));
                // virtual dispatch: a trait method invoked on a class value
                // (e.g. `this.area()` in a base method) goes through the
                // vtable so subclass overrides win (design §2.4). super.* and
                // non-trait methods keep the direct route.
                if !is_super {
                    if let Some((tr, _slot)) = self.trait_slot_for(&cls, &name) {
                        let sargs = sigargs.clone();
                        return self.emit_dyn_call(fw, &tr, &name, &recvv, &argv, &sargs, pos);
                    }
                    // design §2.4: plain class methods called through a
                    // base-class reference also dispatch on the runtime class.
                    // No descendant override -> devirtualize to a direct call.
                    if self.has_virtual_override(&cls, &name) {
                        if let Some(out) =
                            self.emit_class_vt_call(fw, &cls, &name, &recvv, &argv, pos)
                        {
                            return out;
                        }
                    }
                }
                let start = if is_super {
                    match self.classes.get(&cls).and_then(|ci| ci.superclass.clone()) {
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
                    return self
                        .emit_method_call(fw, &defcls, &name, &fd, is_super, &argv, &sigargs, pos);
                }
                // no method: a field holding a first-class function value
                let fty = self.field_type(&cls, &name);
                if let Ty::Fn(ft) = self.r.get(fty).clone() {
                    let fi = self.field_index(&cls, &name);
                    let zi = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        zi,
                        enc_i_lit(fi as i64)
                    ));
                    let fv = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
                        fv, recvv, zi
                    ));
                    return self.emit_fn_value_call(fw, &fv, &ft, args, pos);
                }
            }
        } // direct function call
        if let Some(fd) = self.funcs.get(&name).cloned() {
            if !fd.type_params.is_empty() {
                return self.emit_gfunc_call(fw, &name, &fd, &argv, pos);
            }
            let variadic = fd.variadic.clone();
            let plan = self.plan_func(&name, None, &fd, variadic.as_ref());
            // arity: a wrong argument count is a user error, not a malformed
            // `func.call` (variadic packs `>= fixed` args into one array)
            let arity_ok = match &variadic {
                None => self.check_call_arity(
                    pos,
                    &format!("call to `{}`", name),
                    plan.params.len(),
                    argv.len(),
                ),
                Some(_) => {
                    let fixed = plan.params.len().saturating_sub(1);
                    if argv.len() < fixed {
                        self.err(
                            pos,
                            format!(
                                "wrong number of arguments in call to `{}`: expected at least {}, got {}",
                                name, fixed, argv.len()
                            ),
                        );
                        false
                    } else {
                        true
                    }
                }
            };
            if !arity_ok {
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                return (z, self.r.mk(Ty::Unit));
            }
            // patch 42: caller-side Opt(值型)-param coercion (bare scalars
            // box up against the callee's declared parameter surfaces)
            let argv_c: Vec<(String, TyId)> = {
                // variadic: the trailing formal is the packed array, not a
                // positional parameter, so it must not be surface-checked
                // against the first extra argument
                let check_params: &[(String, TyId, bool)] = match &variadic {
                    Some(_) => &plan.params[..plan.params.len().saturating_sub(1)],
                    None => &plan.params,
                };
                let vals = self.coerce_args_to_params(fw, &argv, check_params, pos, !fd.is_extern);
                argv.iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if i < plan.params.len()
                            && (self.opt_inner(plan.params[i].1).is_some()
                                || matches!(self.r.get(plan.params[i].1), Ty::Any))
                        {
                            (vals[i].clone(), plan.params[i].1)
                        } else {
                            (vals[i].clone(), x.1)
                        }
                    })
                    .collect()
            };
            let argv = argv_c;
            let sigargs_c: Vec<String> = argv.iter().map(|x| mlir_word_ty(x.1, &self.r)).collect();
            let sigargs = sigargs_c;
            let r = fw.v();
            // §5.1.1 rule 5: ref-shaped sloth returns transfer their +1 across
            // the edge. Extern C functions follow C ownership: their word is
            // not rc-tracked, so the caller neither owns nor releases it.
            let xfer_expect = self.is_ref(plan.ret) && !fd.is_extern;
            // extern funcs resolve under their raw C-ABI symbol
            let sym = if fd.is_extern {
                name.clone()
            } else {
                plan.mangled.clone()
            };
            // raw C-ABI boundary (extern func): scalar word args decode to
            // their C spelling and scalar returns re-encode into words;
            // opaque extern-type/pointer words pass through untouched
            if fd.is_extern {
                let mut vals: Vec<String> = Vec::new();
                let mut tys: Vec<String> = Vec::new();
                for (i, (v, t)) in argv.iter().enumerate() {
                    let pt = plan.params.get(i).map(|p| p.1).unwrap_or(*t);
                    let pts = self.r.get(pt).clone();
                    if self.is_float(pt) {
                        vals.push(emit_dec_f(fw, v));
                        tys.push("f64".to_string());
                    } else if matches!(pts, Ty::I64 | Ty::Bool) {
                        vals.push(emit_dec_int(fw, v));
                        tys.push("i64".to_string());
                    } else {
                        vals.push(v.clone());
                        tys.push("i64".to_string());
                    }
                }
                let sig = tys.join(", ");
                if self.is_unit(plan.ret) {
                    fw.op(&format!(
                        "    func.call @{}({}) : ({}) -> ()",
                        sym,
                        vals.join(", "),
                        sig
                    ));
                    return (String::new(), plan.ret);
                }
                let retf = self.is_float(plan.ret);
                let ret_int = matches!(self.r.get(plan.ret).clone(), Ty::I64 | Ty::Bool);
                let rr = fw.v();
                fw.op(&format!(
                    "    {} = func.call @{}({}) : ({}) -> {}",
                    rr,
                    sym,
                    vals.join(", "),
                    sig,
                    if retf { "f64" } else { "i64" }
                ));
                if retf {
                    return (emit_enc_f(fw, &rr), plan.ret);
                }
                if ret_int {
                    return (emit_enc_int(fw, &rr), plan.ret);
                }
                // Reserved runtime externs that return a reference type are
                // fresh owned rc producers (`intern_bytes`/`rc_addr`, count 1):
                // `__sloth_str_slice`/`__sloth_str_of_byte`, `__sloth_bytes_*`,
                // `__sloth_mmap_str`, `__sloth_addr_ip`,
                // `__sloth_ev_backend_name`, `__sloth_tensor_*`,
                // `__sloth_rt_write`, … Track them as statement-dangling so an
                // unclaimed temp is released at statement end instead of
                // leaking. Ordinary user `extern func` C symbols keep C
                // ownership (their word may be a borrowed `char*`), and opaque
                // `extern type` handles are not `is_ref`, so both pass through.
                if sym.starts_with(RESERVED_PREFIX) && self.is_ref(plan.ret) {
                    self.dangling_producer(fw, &rr, plan.ret);
                }
                return (rr, plan.ret);
            }
            let (vals, tys) = match &variadic {
                Some(vd) => {
                    // extra args pack into one Array<T> word; the declared elem
                    // kind decides the slot route (int extras sitofp to f64)
                    let elty = self.ty_of(&vd.elem);
                    let fels = self.is_float(elty);
                    let fixed = plan.params.len() - 1;
                    let mut vals: Vec<String> = argv[..fixed].iter().map(|x| x.0.clone()).collect();
                    let mut sigs: Vec<String> = argv[..fixed]
                        .iter()
                        .map(|x| mlir_word_ty(x.1, &self.r))
                        .collect();
                    let mut pv: Vec<String> = Vec::new();
                    for (v, t) in &argv[fixed.min(argv.len())..] {
                        if fels && !self.is_float(*t) {
                            // design §2.1: no implicit int -> float in variadics
                            self.err_diff(pos, "variadic argument", "float", "int");
                            pv.push(self.int_to_f64_word(fw, v, *t));
                        } else if !fels && self.is_float(*t) {
                            self.err(
                                pos,
                                "type mismatch: variadic argument is float but elem type is not"
                                    .to_string(),
                            );
                            pv.push(v.clone());
                        } else {
                            pv.push(v.clone());
                        }
                    }
                    let packed = self.pack_variadic(fw, fels, elty, &pv);
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
                    "    func.call @{}({}) : ({}) -> ()",
                    sym,
                    vals.join(", "),
                    tys
                ));
                return (String::new(), plan.ret);
            }
            fw.op(&format!(
                "    {} = func.call @{}({}) : ({}) -> {}",
                r,
                sym,
                vals.join(", "),
                tys,
                rt
            ));
            if xfer_expect {
                fw.rc_mark_xfer(&r);
            }
            return (r, plan.ret);
        }
        // local/param carrying a first-class function value: indirect call
        // through the closure object (design §2.3/§2.6)
        if let Some((slot, lt)) = fw.lookup(&name) {
            if let Ty::Fn(ft) = self.r.get(lt).clone() {
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : index", z));
                let clo = fw.v();
                fw.op(&format!(
                    "    {} = memref.load {}[{}] : memref<1xi64>",
                    clo, slot, z
                ));
                return self.emit_fn_value_call(fw, &clo, &ft, args, pos);
            }
        }
        // global holding a first-class function value
        {
            let cur_mod = self.cur_mod.clone();
            let fg = self.fglobals.get(&format!("{}.{}", cur_mod, name)).cloned();
            let lg = self.globals.get(&name).cloned();
            let picked = if cur_mod != self.name {
                fg.or(lg)
            } else {
                lg.or(fg)
            };
            if let Some((gsym, gt, _)) = picked {
                if let Ty::Fn(ft) = self.r.get(gt).clone() {
                    let (clo, _) = self.emit_global_read(fw, &gsym, gt);
                    return self.emit_fn_value_call(fw, &clo, &ft, args, pos);
                }
            }
        }
        // private function of the module currently being emitted: its
        // unqualified name is not exported, but the module-qualified entry is
        // always present (other modules hit `hidden` via guard_hidden)
        let private_key = format!("{}.{}", self.cur_mod, name);
        let cross_lookup = if self.cross_funcs.contains_key(&name) {
            name.clone()
        } else {
            private_key
        };
        // imported *generic* function (public or private helper): monomorphize
        // in the defining module's namespace, exactly like the qualified
        // `lib.f(...)` path above. Imported generics have no template body (only
        // monomorphic instances are emitted), so the `cross_funcs` base symbol
        // must never be called directly — doing so would also mishandle
        // ref-element ARC for `Array<T>`/`T` returns.
        if let Some((defmod, fd)) = self.foreign_func_defs.get(&cross_lookup).cloned() {
            let saved = self.cur_mod.clone();
            self.cur_mod = defmod;
            let out = self.emit_gfunc_call(fw, &name, &fd, &argv, pos);
            self.cur_mod = saved;
            return out;
        }
        // foreign-module function: symbol was pre-mangled at import time
        if let Some(fs) = self.cross_funcs.get(&cross_lookup).cloned() {
            let vals: Vec<String> = argv.iter().map(|x| x.0.clone()).collect();
            if self.is_unit(fs.1) {
                fw.op(&format!(
                    "    func.call @{}({}) : ({}) -> ()",
                    fs.0,
                    vals.join(", "),
                    sigargs.join(", ")
                ));
                return (String::new(), fs.1);
            }
            let r = fw.v();
            let rt = mlir_ret_ty(self, fs.1);
            fw.op(&format!(
                "    {} = func.call @{}({}) : ({}) -> {}",
                r,
                fs.0,
                vals.join(", "),
                sigargs.join(", "),
                rt
            ));
            // §5.1.1 rule 4: cross-module sloth ref returns are owned (+1);
            // mark the transfer so a binding claims it (see the local-call
            // path's `xfer_expect`). Raw C extern symbols (unmangled `fs.0`,
            // == the call name) keep C ownership and are left alone.
            if self.is_ref(fs.1) && fs.0.as_str() != name.as_str() {
                fw.rc_mark_xfer(&r);
            }
            return (r, fs.1);
        }
        // builtins
        let r = fw.v();
        match name.as_str() {
            // rc diagnostics (SLOTH_STATS surface, predeclared in module.rs)
            "sloth_rc_live" if argv.is_empty() => {
                fw.op(&format!(
                    "    {} = func.call @__sloth_rc_live() : () -> i64",
                    r
                ));
                (r, self.r.mk(Ty::I64))
            }
            "sloth_rc_drops" if argv.is_empty() => {
                fw.op(&format!(
                    "    {} = func.call @__sloth_rc_drops() : () -> i64",
                    r
                ));
                (r, self.r.mk(Ty::I64))
            }
            // explicit fixed-width integer conversion: `int8(x)`, `uint32(x)`, …
            other
                if !argv.is_empty() && sloth_frontend::ty::IntKind::from_name(other).is_some() =>
            {
                let k = sloth_frontend::ty::IntKind::from_name(other).unwrap();
                let to = self.r.mk(Ty::Int(k));
                let (v, t) = argv[0].clone();
                let (v, t) = self.unwrap_opt_word(fw, &v, t);
                let ts = self.r.get(t).clone();
                let w = match ts {
                    Ty::F64 => f64w_to_iw(fw, &v),
                    _ => v,
                };
                (self.coerce_int_word(fw, &w, to), to)
            }
            "int" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                // patch 42: int(optional box) unwraps the payload (nil -> 0)
                let (v, t) = self.unwrap_opt_word(fw, &v, t);
                let ts = self.r.get(t).clone();
                match ts {
                    Ty::F64 => {
                        let rd = f64w_to_iw(fw, &v);
                        (rd, self.r.mk(Ty::I64))
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
                // patch 42: float(optional box) unwraps with promote
                let (v, t) = self.unwrap_opt_word(fw, &v, t);
                let ts = self.r.get(t).clone();
                match ts {
                    Ty::F64 => (v, self.r.mk(Ty::F64)),
                    Ty::Str => {
                        self.err(pos, "float() of str unsupported (MVP)".to_string());
                        (v, self.r.mk(Ty::F64))
                    }
                    _ => {
                        // int word -> f64 word (unsigned promotion for `uint`)
                        (self.int_to_f64_word(fw, &v, t), self.r.mk(Ty::F64))
                    }
                }
            }
            "len" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let ts = self.r.get(t).clone();
                let sym = match &ts {
                    Ty::Str => Some("__sloth_str_len"),
                    Ty::Array(_) => Some("__sloth_arr_len"),
                    Ty::Map(..) => Some("__sloth_map_len"),
                    // unknown (unannotated global) stays lenient
                    Ty::Unit => Some("__sloth_str_len"),
                    _ => None,
                };
                match sym {
                    Some(sym) => {
                        fw.op(&format!(
                            "    {} = func.call @{}({}) : (i64) -> i64",
                            r, sym, v
                        ));
                        (r, self.r.mk(Ty::I64))
                    }
                    None => {
                        let got = sloth_frontend::ty::ty_name(self.r.get(t)).to_string();
                        let hint = if matches!(ts, Ty::Opt(_)) {
                            "; narrow it with `is not nil` first"
                        } else {
                            ""
                        };
                        self.err(
                            pos,
                            format!("len() expects a `str`/`Array`/`Map`, got `{}`{}", got, hint),
                        );
                        (String::new(), self.r.mk(Ty::Unit))
                    }
                }
            }
            // `s.chars()` / `chars(s)`: lazy UTF-8 char iterator. Each `next()`
            // yields the Unicode scalar value of one character as an `int`
            // (fixed 4-byte code point); the injected `StrChars` class carries
            // the `iter()`/`next(): int?` protocol.
            "chars" if !argv.is_empty() => {
                let (_v, t) = argv[0].clone();
                if !matches!(self.r.get(t), Ty::Str) {
                    let got = sloth_frontend::ty::ty_name(self.r.get(t)).to_string();
                    self.err(pos, format!("chars() expects a `str`, got `{}`", got));
                    return (String::new(), self.r.mk(Ty::Unit));
                }
                return self.emit_new_obj(
                    fw,
                    "StrChars",
                    &vec![argv[0].clone()],
                    &vec!["i64".to_string()],
                    pos,
                );
            }
            "keys" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let kt = match self.r.get(t).clone() {
                    Ty::Map(k3, _v3) => k3,
                    // unknown (unannotated global) stays lenient
                    Ty::Unit => self.r.mk(Ty::I64),
                    _ => {
                        let got = sloth_frontend::ty::ty_name(self.r.get(t)).to_string();
                        self.err(pos, format!("keys() expects a `Map`, got `{}`", got));
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                };
                fw.op(&format!(
                    "    {} = func.call @__sloth_map_keys({}) : (i64) -> i64",
                    r, v
                ));
                let at = self.r.mk(Ty::Array(kt));
                // rc patch B: fresh keys array (producer)
                self.dangling_producer(fw, &r, at);
                (r, at)
            }
            "values" if !argv.is_empty() => {
                let (v, t) = argv[0].clone();
                let vt = match self.r.get(t).clone() {
                    Ty::Map(_k, v3) => v3,
                    // unknown (unannotated global) stays lenient
                    Ty::Unit => self.r.mk(Ty::I64),
                    _ => {
                        let got = sloth_frontend::ty::ty_name(self.r.get(t)).to_string();
                        self.err(pos, format!("values() expects a `Map`, got `{}`", got));
                        return (String::new(), self.r.mk(Ty::Unit));
                    }
                };
                fw.op(&format!(
                    "    {} = func.call @__sloth_map_values({}) : (i64) -> i64",
                    r, v
                ));
                let at = self.r.mk(Ty::Array(vt));
                // rc patch B: fresh values array (producer) — mirrors keys()
                self.dangling_producer(fw, &r, at);
                (r, at)
            }
            // runtime type identity over reference types: class/dyn resolve
            // most-derived through ObjInfo; monomorphic refs are constants
            "typeid" | "type_name" => self.emit_typeid_builtin(fw, &name, &argv, pos),
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
    /// generic function call: infer T bindings from the call-site arg types,
    /// emit/lookup the monomorphic instance, then dispatch the call word-wise
    pub(crate) fn emit_gfunc_call(
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
}

impl ModEmitter {
    /// monomorphize + emit an instance call for a fully-bound substitution
    pub(crate) fn emit_ginst_call(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        fd: &FuncDef,
        argv: &[(String, TyId)],
        pos: &Pos,
        mut map: std::collections::HashMap<String, TyId>,
    ) -> (String, TyId) {
        let tnames: Vec<String> = fd.type_params.iter().map(|p| p.name.clone()).collect();
        // unbound type params: infer from the expected return type hint (let
        // annotation / assignment target surface — patch #38)
        if tnames.iter().any(|n| !map.contains_key(n)) {
            if let Some(&hint) = self.exp_ret.last() {
                if !matches!(self.r.get(hint), Ty::Unit) {
                    let pat = self.shape_of_retched(fd.ret.clone(), &tnames);
                    self.unify_tp(&fd.type_params, pat, hint, &mut map);
                    // declared Result<T,E> surface: the registered instance's
                    // class frame provides the concrete T/E bindings
                    if let Ty::Named(hn, _) = self.r.get(hint).clone() {
                        if self.result_insts.contains(&hn) {
                            let frame = self.class_frames.get(&hn).cloned();
                            if let Some(frame) = frame {
                                if let Some(Type::Simple(SimpleType::Named(_, ra))) =
                                    fd.ret.as_ref()
                                {
                                    for ra_arg in ra.iter() {
                                        if let Type::Simple(SimpleType::Ident(rn)) = ra_arg {
                                            if tnames.contains(rn) {
                                                if let Some(&bt) = frame.get(rn) {
                                                    map.entry(rn.clone()).or_insert(bt);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
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
        // recursion guard: a self-referential generic that never reaches a
        // fixed point would grow the worklist without bound (the old guard keyed
        // off `insts`, which is never populated, so it never fired).
        if self.pending_fn_insts.len() + self.pending_insts.len() > MONO_INST_CAP {
            self.err(
                pos,
                "generic instantiation too deep (recursion?)".to_string(),
            );
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, self.r.mk(Ty::Unit));
        }
        let keys: Vec<TyId> = tnames.iter().map(|n| map[n]).collect();
        let base = mangle(&self.cur_mod.clone(), None, name);
        let mangled = format!("{}{}", base, mangle_t(&keys, &self.r));
        // eager monomorphization (A1): record the instance for the module-level
        // fixpoint instead of emitting its body inline here. The body is emitted
        // exactly once, after the walk, from the frozen set — so instances only
        // reachable *through* another instance are also covered.
        if self.pending_fn_seen.insert(mangled.clone()) {
            self.pending_fn_insts.push(crate::mono::FnInst {
                module: self.cur_mod.clone(),
                name: name.to_string(),
                fd: fd.clone(),
                frame: map.clone(),
                mangled: mangled.clone(),
            });
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
        // patch 42: caller-side Opt(值型)-param coercion against the
        // monomorphized plan surfaces (bare scalars box up)
        let argv_c: Vec<(String, TyId)> = {
            let vals9 = self.coerce_args_to_params(fw, argv, &plan.params, pos, true);
            argv.iter()
                .enumerate()
                .map(|(i, x)| (vals9[i].clone(), x.1))
                .collect()
        };
        let vals: Vec<String> = argv_c.iter().map(|x| x.0.clone()).collect();
        let sigs = argv
            .iter()
            .map(|x| mlir_word_ty(x.1, &self.r))
            .collect::<Vec<String>>();
        let rt = mlir_ret_ty(self, plan.ret);
        if self.is_unit(plan.ret) {
            fw.op(&format!(
                "    func.call @{}({}) : ({}) -> ()",
                mangled,
                vals.join(", "),
                sigs.join(", ")
            ));
            return (String::new(), plan.ret);
        }
        fw.op(&format!(
            "    {} = func.call @{}({}) : ({}) -> {}",
            r,
            mangled,
            vals.join(", "),
            sigs.join(", "),
            rt
        ));
        if self.is_ref(plan.ret) {
            fw.rc_mark_xfer(&r);
        }
        (r, plan.ret)
    }
}

impl ModEmitter {
    /// build one fresh Array<T> word from already-coerced words (variadic pack)
    pub(crate) fn pack_variadic(
        &mut self,
        fw: &mut FnWalk,
        _fels: bool,
        elem: TyId,
        vals: &[String],
    ) -> String {
        let n = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            n,
            enc_i_lit(vals.len() as i64)
        ));
        let arr = fw.v();
        let elref = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            elref,
            if self.is_ref(elem) { 1 } else { 0 }
        ));
        fw.op(&format!(
            "    {} = func.call @__sloth_arr_new_k({}, {}) : (i64, i64) -> i64",
            arr, n, elref
        ));
        for (i, v) in vals.iter().enumerate() {
            let zi = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                zi,
                enc_i_lit(i as i64)
            ));
            // rc patch B: the pack owns ref-typed elements
            let mut vv = v.clone();
            if self.is_ref(elem) {
                vv = self.emit_retain(fw, v);
            }
            fw.op(&format!(
                "    func.call @__sloth_arr_set({}, {}, {}) : (i64, i64, i64) -> i64",
                arr, zi, vv
            ));
        }
        arr
    }
}

/// emission plan for a `typeid`/`type_name` argument surface
pub(crate) enum TypeidPlan {
    /// class instance or `dyn` box: resolved at runtime through `ObjInfo`
    Dynamic,
    /// `any` box: resolved through the runtime type descriptor
    Any,
    /// monomorphic non-class reference type: compile-time id/name
    Const {
        key: String,
        name: String,
        nullable: bool,
    },
    /// value type / unresolved: compile diagnostic
    Value,
}

impl ModEmitter {
    /// assign (or fetch) the stable integer id of a canonical non-class
    /// reference type key (`< 2^40` class ids never collide with these)
    pub(crate) fn typeid_const(&mut self, key: &str) -> i64 {
        if let Some(id) = self.type_ids.get(key) {
            return *id;
        }
        let id = self.next_type_id;
        self.next_type_id += 1;
        self.type_ids.insert(key.to_string(), id);
        id
    }

    /// classify the argument surface of `typeid`/`type_name`
    pub(crate) fn typeid_plan(&self, t: TyId) -> TypeidPlan {
        let mut core = t;
        let mut nullable = false;
        loop {
            match self.r.get(core).clone() {
                Ty::Opt(e) => {
                    nullable = true;
                    if self.is_ref(e) {
                        core = e;
                        continue;
                    }
                    break;
                }
                _ => break,
            }
        }
        match self.r.get(core).clone() {
            // opaque C-ABI handles (`extern type`) carry no ObjInfo: reject
            Ty::Named(n, _) if self.extern_types.contains(&n) => TypeidPlan::Value,
            Ty::Named(..) | Ty::Dyn(_) => TypeidPlan::Dynamic,
            Ty::Any => TypeidPlan::Any,
            _ => {
                if self.is_ref(core) {
                    TypeidPlan::Const {
                        key: sloth_frontend::ty::ty_key(self.r.get(core)),
                        name: self.pretty_ty(core),
                        nullable,
                    }
                } else {
                    TypeidPlan::Value
                }
            }
        }
    }

    /// `typeid`/`type_name` builtin emission (single reference argument)
    pub(crate) fn emit_typeid_builtin(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        argv: &[(String, TyId)],
        pos: &Pos,
    ) -> (String, TyId) {
        let want_name = name == "type_name";
        let i64t = self.r.mk(Ty::I64);
        let strt = self.r.mk(Ty::Str);
        if argv.len() != 1 {
            self.err(pos, format!("`{}` takes exactly one argument", name));
            if want_name {
                return (self.empty_str_lit(fw), strt);
            }
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, i64t);
        }
        let (v, t) = argv[0].clone();
        match self.typeid_plan(t) {
            TypeidPlan::Value => {
                let tn = self.surface_name(&self.r.get(t).clone());
                self.err(
                    pos,
                    format!("`{}` requires a reference type, got `{}`", name, tn),
                );
                if want_name {
                    (self.empty_str_lit(fw), strt)
                } else {
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    (z, i64t)
                }
            }
            TypeidPlan::Dynamic => {
                if want_name {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_obj_type_name({}) : (i64) -> i64",
                        r, v
                    ));
                    self.dangling_producer(fw, &r, strt);
                    (r, strt)
                } else {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_obj_cls_id({}) : (i64) -> i64",
                        r, v
                    ));
                    (r, i64t)
                }
            }
            TypeidPlan::Any => {
                if want_name {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_any_type_name({}) : (i64) -> i64",
                        r, v
                    ));
                    self.dangling_producer(fw, &r, strt);
                    (r, strt)
                } else {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = func.call @__sloth_any_type_id({}) : (i64) -> i64",
                        r, v
                    ));
                    (r, i64t)
                }
            }
            TypeidPlan::Const {
                key,
                name: nm,
                nullable,
            } => {
                if want_name {
                    let r = self.emit_tyname_lit(fw, &v, &nm);
                    (r, strt)
                } else {
                    let id = self.typeid_const(&key);
                    if nullable {
                        let z = fw.v();
                        fw.op(&format!("    {} = arith.constant 0 : i64", z));
                        let c = fw.v();
                        fw.op(&format!("    {} = arith.cmpi eq, {}, {} : i64", c, v, z));
                        let ic = fw.v();
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            ic,
                            enc_i_lit(id)
                        ));
                        let r = fw.v();
                        fw.op(&format!(
                            "    {} = arith.select {}, {}, {} : i64",
                            r, c, z, ic
                        ));
                        (r, i64t)
                    } else {
                        let c = fw.v();
                        fw.op(&format!(
                            "    {} = arith.constant {} : i64",
                            c,
                            enc_i_lit(id)
                        ));
                        (c, i64t)
                    }
                }
            }
        }
    }

    /// fresh empty owned `str` (diagnostic fallback for `type_name`)
    pub(crate) fn empty_str_lit(&mut self, fw: &mut FnWalk) -> String {
        let c0 = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", c0));
        let fin = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_str_finish({}) : (i64) -> i64",
            fin, c0
        ));
        let t = self.r.mk(Ty::Str);
        self.dangling_producer(fw, &fin, t);
        fin
    }

    /// `ok(x)`/`err(x)` used as a container-literal element under a declared
    /// `Array<Result<_,_>>` / `Map<_, Result<_,_>>` surface (bug OPT5/E4).
    /// Returns the ctor value+type, or `None` for any other expression.
    pub(crate) fn try_literal_result_ctor(
        &mut self,
        fw: &mut FnWalk,
        x: &Expr,
        expected: TyId,
    ) -> Option<(String, TyId)> {
        let (callee, args) = match &x.node {
            ExprNode::Call { callee, args } if args.len() == 1 => (callee, args),
            _ => return None,
        };
        let is_ok = match &callee.node {
            ExprNode::Ident(id) if id == "ok" => true,
            ExprNode::Ident(id) if id == "err" => false,
            _ => return None,
        };
        let inst = match self.r.get(expected).clone() {
            Ty::Named(nm, _) if self.result_insts.contains(&nm) => nm,
            _ => return None,
        };
        Some(self.emit_result_ctor(fw, &inst, &args[0], is_ok, &x.pos))
    }
}

impl ModEmitter {
    /// raw C-ABI call to an `extern func`: scalar words decode to their C
    /// spelling and scalar returns re-encode into words; opaque extern-type
    /// handles pass through. Used by the qualified cross-module path so
    /// `mod.extern_fn(…)` marshals like the unqualified call (bug M5).
    pub(crate) fn emit_extern_call(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        fd: &FuncDef,
        argv: &[(String, TyId)],
        pos: &Pos,
    ) -> (String, TyId) {
        let plan = self.plan_func(name, None, fd, fd.variadic.as_ref());
        self.coerce_args_to_params(fw, argv, &plan.params, pos, false);
        let sym = name.to_string();
        let mut vals: Vec<String> = Vec::new();
        let mut tys: Vec<String> = Vec::new();
        for (i, (v, t)) in argv.iter().enumerate() {
            let pt = plan.params.get(i).map(|p| p.1).unwrap_or(*t);
            let pts = self.r.get(pt).clone();
            if self.is_float(pt) {
                vals.push(emit_dec_f(fw, v));
                tys.push("f64".to_string());
            } else if matches!(pts, Ty::I64 | Ty::Bool) {
                vals.push(emit_dec_int(fw, v));
                tys.push("i64".to_string());
            } else {
                vals.push(v.clone());
                tys.push("i64".to_string());
            }
        }
        let sig = tys.join(", ");
        if self.is_unit(plan.ret) {
            fw.op(&format!(
                "    func.call @{}({}) : ({}) -> ()",
                sym,
                vals.join(", "),
                sig
            ));
            return (String::new(), plan.ret);
        }
        let retf = self.is_float(plan.ret);
        let ret_int = matches!(self.r.get(plan.ret).clone(), Ty::I64 | Ty::Bool);
        let rr = fw.v();
        fw.op(&format!(
            "    {} = func.call @{}({}) : ({}) -> {}",
            rr,
            sym,
            vals.join(", "),
            sig,
            if retf { "f64" } else { "i64" }
        ));
        if retf {
            return (emit_enc_f(fw, &rr), plan.ret);
        }
        if ret_int {
            return (emit_enc_int(fw, &rr), plan.ret);
        }
        if sym.starts_with(RESERVED_PREFIX) && self.is_ref(plan.ret) {
            self.dangling_producer(fw, &rr, plan.ret);
        }
        (rr, plan.ret)
    }
}
