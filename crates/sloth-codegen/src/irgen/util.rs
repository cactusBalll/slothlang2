//! Small shared helpers: mangling, word types, super-init scanners, and
//! the tagged-word encode/decode emit points (word plane: LSB = tag bit;
//! ref = `ptr | 1`, int = `v << 1` 63-bit, f64 = the f64 bits with the
//! mantissa LSB cleared and shifted right once, nil = 0).

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

pub const WW: usize = 8;

// ---------------- tagged-word encode/decode emit points ----------------

/// int literal -> tagged word form (63-bit)
pub(crate) fn enc_i_lit(v: i64) -> i64 {
    v << 1
}

/// f64 literal -> tagged word form (mantissa LSB sacrificed)
pub(crate) fn enc_f_lit(v: f64) -> i64 {
    ((v.to_bits() & !1) as i64) >> 1
}

/// tag an int scalar into a word (encode)
pub(crate) fn emit_enc_int(fw: &mut FnWalk, v: &str) -> String {
    let one = fw.v();
    fw.op(&format!("    {} = arith.constant 1 : i64", one));
    let r = fw.v();
    fw.op(&format!("    {} = arith.shli {}, {} : i64", r, v, one));
    r
}

/// decode a word into a raw int scalar (63-bit arithmetic)
pub(crate) fn emit_dec_int(fw: &mut FnWalk, w: &str) -> String {
    let one = fw.v();
    fw.op(&format!("    {} = arith.constant 1 : i64", one));
    let r = fw.v();
    fw.op(&format!("    {} = arith.shrsi {}, {} : i64", r, w, one));
    r
}

/// tag an f64 scalar into a word: bitcast to the bit pattern, clear the
/// mantissa LSB, shift right
pub(crate) fn emit_enc_f(fw: &mut FnWalk, v: &str) -> String {
    let b = fw.v();
    fw.op(&format!("    {} = llvm.bitcast {} : f64 to i64", b, v));
    let mask = fw.v();
    fw.op(&format!("    {} = arith.constant -3 : i64", mask));
    let m = fw.v();
    fw.op(&format!("    {} = arith.andi {}, {} : i64", m, b, mask));
    let one = fw.v();
    fw.op(&format!("    {} = arith.constant 1 : i64", one));
    let r = fw.v();
    fw.op(&format!("    {} = arith.shrsi {}, {} : i64", r, m, one));
    r
}

/// decode a word into an f64 scalar: shift left once and bitcast back
pub(crate) fn emit_dec_f(fw: &mut FnWalk, w: &str) -> String {
    let one = fw.v();
    fw.op(&format!("    {} = arith.constant 1 : i64", one));
    let s = fw.v();
    fw.op(&format!("    {} = arith.shli {}, {} : i64", s, w, one));
    let r = fw.v();
    fw.op(&format!("    {} = llvm.bitcast {} : i64 to f64", r, s));
    r
}

/// decode a word into a scalar by target type (refs stay words: only the
/// num valuations below reach the scalar plane)
#[allow(dead_code)]
pub(crate) fn w_dec_word(me: &mut ModEmitter, fw: &mut FnWalk, w: &str, ty: TyId) -> String {
    if me.is_float(ty) {
        emit_dec_f(fw, w)
    } else {
        emit_dec_int(fw, w)
    }
}

/// i1 -> encoded bool word (0 / enc(1)); bool words ride the int codec
pub(crate) fn ext_bool(fw: &mut FnWalk, c: &str) -> String {
    let z = fw.v();
    // zero-extend: an i1 `true` sign-extends to -1, which would corrupt
    // the encoded bool word (we need exactly 1)
    fw.op(&format!("    {} = arith.extui {} : i1 to i64", z, c));
    emit_enc_int(fw, &z)
}

/// convert an int word into an f64 word (decode, promote, re-encode);
/// replaces the legacy bare `sitofp` int-word boundary
pub(crate) fn iw_to_f64_word(fw: &mut FnWalk, w: &str) -> String {
    let d = emit_dec_int(fw, w);
    let f = fw.v();
    fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", f, d));
    emit_enc_f(fw, &f)
}

/// convert an f64 word into an int word (decode, truncate, encode)
pub(crate) fn f64w_to_iw(fw: &mut FnWalk, w: &str) -> String {
    let f = emit_dec_f(fw, w);
    let d = fw.v();
    fw.op(&format!("    {} = arith.fptosi {} : f64 to i64", d, f));
    emit_enc_int(fw, &d)
}

pub(crate) fn expr_is_super_init(e: &Expr) -> bool {
    match &e.node {
        ExprNode::Field { obj, name } => name == "__init__" && matches!(obj.node, ExprNode::Super),
        _ => false,
    }
}

pub(crate) fn expr_has_super_init(e: &Expr) -> bool {
    if expr_is_super_init(e) {
        return true;
    }
    match &e.node {
        ExprNode::Call { callee, args } | ExprNode::GenCall { callee, args, .. } => {
            expr_has_super_init(callee) || args.iter().any(expr_has_super_init)
        }
        ExprNode::Field { obj, .. } => expr_has_super_init(obj),
        ExprNode::Index { obj, idx } => expr_has_super_init(obj) || expr_has_super_init(idx),
        ExprNode::Un { expr, .. } => expr_has_super_init(expr),
        ExprNode::Arith { lhs, rhs, .. } | ExprNode::Bin { lhs, rhs, .. } => {
            expr_has_super_init(lhs) || expr_has_super_init(rhs)
        }
        ExprNode::Elvis { lhs, rhs } => expr_has_super_init(lhs) || expr_has_super_init(rhs),
        ExprNode::Pipe { lhs, rhs } => expr_has_super_init(lhs) || expr_has_super_init(rhs),
        ExprNode::Is { lhs, rhs, .. } => expr_has_super_init(lhs) || expr_has_super_init(rhs),
        ExprNode::List(xs) => xs.iter().any(expr_has_super_init),
        ExprNode::Map(pairs) => pairs
            .iter()
            .any(|(k, v)| expr_has_super_init(k) || expr_has_super_init(v)),
        _ => false,
    }
}

pub(crate) fn stmt_has_super_init(s: &Stmt) -> bool {
    match &s.node {
        StmtNode::Expr(e) => expr_has_super_init(e),
        StmtNode::Assign { value, .. } => expr_has_super_init(value),
        StmtNode::Let { init, .. } => expr_has_super_init(init),
        StmtNode::If { cond, then_, else_ } => {
            expr_has_super_init(cond)
                || stmt_has_super_init(then_)
                || else_.as_deref().map(stmt_has_super_init).unwrap_or(false)
        }
        StmtNode::While { cond, body } => expr_has_super_init(cond) || stmt_has_super_init(body),
        StmtNode::For { iter, body, .. } => expr_has_super_init(iter) || stmt_has_super_init(body),
        StmtNode::Return(Some(e)) => expr_has_super_init(e),
        StmtNode::Block(ss) => ss.iter().any(stmt_has_super_init),
        _ => false,
    }
}

pub(crate) fn mangle(mod_name: &str, cls: Option<&str>, name: &str) -> String {
    match cls {
        Some(c) => format!("sloth_{}_{}__{}", mod_name, c, name),
        None => format!("sloth_{}__{}", mod_name, name),
    }
}

pub(crate) fn mangle_t(args: &[TyId], r: &Reg) -> String {
    let mut s = String::new();
    for a in args {
        s.push('_');
        s.push_str(&sloth_frontend::ty::ty_name(r.get(*a)).replace(':', "_"));
    }
    s
}

pub(crate) fn mlir_ret_ty(me: &ModEmitter, t: TyId) -> String {
    match me.r.get(t) {
        Ty::Unit => "()".to_string(),
        // tag migration: every word-plane return carries the tagged word
        _ => "i64".to_string(),
    }
}

pub(crate) fn mlir_word_ty(_t: TyId, _r: &Reg) -> String {
    // word plane: i64 tagged words for values and refs alike
    "i64".to_string()
}

pub(crate) fn memref_cell_ty(_me: &ModEmitter, _t: TyId) -> &'static str {
    // tag migration: every slot holds one tagged i64 word
    "memref<1xi64>"
}
