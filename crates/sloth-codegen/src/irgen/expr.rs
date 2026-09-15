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

impl ModEmitter {
    /// stmt text of the callee Identifier for diagnostics
    pub(crate) fn init_str2(callee: &Expr) -> String {
        match &callee.node {
            ExprNode::Ident(id) => id.clone(),
            _ => String::new(),
        }
    }
}

impl ModEmitter {
    pub(crate) fn intern_str(&mut self, s: &str) -> String {
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
                if !s.contains('.') && !s.contains('e') && !s.contains("inf") && !s.contains("nan")
                {
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
    pub(crate) fn emit_expr_rest(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
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
                                    fw.op(&format!(
                                        "    {} = arith.constant {} : i64",
                                        c1, w as i64
                                    ));
                                    let tail = std::cmp::min(8usize, blen - ci_ * 8);
                                    let c2 = fw.v();
                                    fw.op(&format!(
                                        "    {} = arith.constant {} : i64",
                                        c2, tail as i64
                                    ));
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
                                    self.satisfies_bound_check(
                                        &e.pos,
                                        &t,
                                        "Display",
                                        "interpolation",
                                    );
                                    match self.find_method(cls, "to_str") {
                                        Some((defcls, fd)) => {
                                            let (sv, _st) = self.emit_method_call(
                                                fw,
                                                &defcls,
                                                "to_str",
                                                &fd,
                                                false,
                                                &vec![(v.clone(), t)],
                                                &vec!["i64".to_string()],
                                                &e.pos,
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
    pub(crate) fn emit_expr_arith_codes(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
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
                    return (r, self.r.mk(Ty::Bool));
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
                        fw.op(&format!("    {} = arith.constant {} : i64", ci, id));
                        let clsid = fw.v();
                        fw.op(&format!(
                            "    {} = call @sloth_obj_cls_id({}) : (i64) -> i64",
                            clsid, lv
                        ));
                        let eq = fw.v();
                        fw.op(&format!(
                            "    {} = arith.cmpi eq, {}, {} : i64",
                            eq, clsid, ci
                        ));
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
                fw.op(&format!(
                    "    {} = arith.select {}, {}, {} : i64",
                    r, c, lv, rv
                ));
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
                        "    {} = call @sloth_str_concat({}, {}) : (i64, i64) -> i64",
                        r, a, b
                    ));
                    return (r, self.r.mk(Ty::Str));
                }
                let fl = self.is_float(at) || self.is_float(bt);
                if fl {
                    // promote int operands to f64 for float ops
                    let a = if self.is_float(at) {
                        a
                    } else {
                        let cv = fw.v();
                        fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, a));
                        cv
                    };
                    let b = if self.is_float(bt) {
                        b
                    } else {
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
    pub(crate) fn emit_expr_rest2(&mut self, fw: &mut FnWalk, e: &Expr) -> (String, TyId) {
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
                            fw.op(&format!("    {} = arith.constant 0.0 : f64", r));
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
    pub(crate) fn emit_binop(
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
                    let osig = vec![mlir_word_ty(at, &self.r), mlir_word_ty(bt, &self.r)];
                    return self
                        .emit_method_call(fw, &defcls, oname, &fd, false, &oargv, &osig, pos);
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
            fw.op(&format!(
                "    {} = arith.cmpf {}, {}, {} : f64",
                r, pred, a, b
            ));
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
            fw.op(&format!(
                "    {} = arith.cmpi {}, {}, {} : i64",
                z, pr, a, b
            ));
            let z2 = fw.v();
            fw.op(&format!("    {} = arith.extsi {} : i1 to i64", z2, z));
            (z2, cmp_ty_id)
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
                    let fty2 = match fty {
                        Some(x) => x,
                        None => self.r.mk(Ty::I64),
                    };
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
                        Ty::Named(_, _) | Ty::Dyn(_) | Ty::Array(_) | Ty::Map(..) | Ty::Bool => {
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
            ExprNode::Range {
                low,
                high,
                inclusive,
            } => {
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
                                cv,
                                v.clone()
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
                if anyk_obj
                    && kvm
                        .iter()
                        .any(|t| !matches!(self.r.get(*t).clone(), Ty::Named(_, _)))
                {
                    self.err(&e.pos, "mixed map key types".to_string());
                }
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
                let kk = if anyk_str {
                    1i64
                } else if anyk_obj {
                    2i64
                } else {
                    0i64
                };
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
            ExprNode::This | ExprNode::Super => match fw
                .scopes
                .first()
                .and_then(|sc| sc.get("this"))
                .map(|s| (s.0.clone(), s.1))
            {
                Some((a, t)) => {
                    let z = fw.v();
                    let v = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
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
                    let tys: Vec<TyId> = ta.iter().map(|t| self.ty_of(t)).collect();
                    let it = self.declare_class_inst(base, &tys);
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
                            fs.0,
                            vals.join(", "),
                            csig.join(", ")
                        ));
                        return (String::new(), fs.1);
                    }
                    fw.op(&format!(
                        "    {} = call @{}({}) : ({}) -> {}",
                        r,
                        fs.0,
                        vals.join(", "),
                        csig.join(", "),
                        rt
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
                    return self
                        .emit_method_call(fw, &defcls, &name, &fd, is_super, &argv, &sigargs, pos);
                }
            }
        } // direct function call
        if let Some(fd) = self.funcs.get(&name).cloned() {
            if !fd.type_params.is_empty() {
                return self.emit_gfunc_call(fw, &name, &fd, &argv, pos);
            }
            let variadic = fd.variadic.clone();
            let plan = self.plan_func(&name, None, &fd, variadic.as_ref());
            let r = fw.v();
            // extern funcs resolve under their raw C-ABI symbol
            let sym = if fd.is_extern {
                name.clone()
            } else {
                plan.mangled.clone()
            };
            let (mut vals, tys) = match &variadic {
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
                            let cv = fw.v();
                            fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", cv, v));
                            pv.push(cv);
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
                    sym,
                    vals.join(", "),
                    tys
                ));
                return (String::new(), plan.ret);
            }
            fw.op(&format!(
                "    {} = call @{}({}) : ({}) -> {}",
                r,
                sym,
                vals.join(", "),
                tys,
                rt
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
                    r,
                    lam.sym,
                    vals.join(", "),
                    sig,
                    rt
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
                r,
                fs.0,
                vals.join(", "),
                sigargs.join(", "),
                rt
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
                                fw,
                                &defcls,
                                "to_str",
                                &fd,
                                false,
                                &vec![(v.clone(), t.clone())],
                                &vec!["i64".to_string()],
                                pos,
                            );
                            let r2 = fw.v();
                            fw.op(&format!(
                                "    {} = call @sloth_rt_print_str({}) : (i64) -> i64",
                                r2, sv
                            ));
                            (r2, self.r.mk(Ty::Unit))
                        } else {
                            self.err(
                                pos,
                                "`print` on class requires impl Display with `to_str`".to_string(),
                            );
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
                        fw.op(&format!("    {} = arith.fptosi {} : f64 to i64", r, v));
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
                        fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", r, v));
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
                fw.op(&format!("    {} = call @{}({}) : (i64) -> i64", r, sym, v));
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
                mangled,
                vals.join(", "),
                sigs.join(", ")
            ));
            return (String::new(), plan.ret);
        }
        fw.op(&format!(
            "    {} = call @{}({}) : ({}) -> {}",
            r,
            mangled,
            vals.join(", "),
            sigs.join(", "),
            rt
        ));
        (r, plan.ret)
    }
}

impl ModEmitter {
    /// build one fresh Array<T> word from already-coerced words (variadic pack)
    pub(crate) fn pack_variadic(&mut self, fw: &mut FnWalk, fels: bool, vals: &[String]) -> String {
        let n = fw.v();
        fw.op(&format!("    {} = arith.constant {} : i64", n, vals.len()));
        let arr = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_arr_new({}) : (i64) -> i64",
            arr, n
        ));
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
