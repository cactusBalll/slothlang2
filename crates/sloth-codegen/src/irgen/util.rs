//! Small shared helpers: mangling, word types, super-init scanners, and the
//! word encode/decode emit points (de-tag word plane: reference = raw
//! payload pointer, int = native i64, float = f64 bits, bool = 0/1, nil = 0;
//! the codec is identity/bitcast so callers are unchanged).

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

/// base of compile-time-assigned ids for monomorphic non-class reference types
/// (classes/dyn use the small `ObjInfo.cls_id` space; the two never collide)
pub(crate) const TYPEID_BASE: i64 = 1 << 40;

// ---------------- word encode/decode emit points ----------------

/// int literal -> word form (de-tag: identity, full 64-bit)
pub(crate) fn enc_i_lit(v: i64) -> i64 {
    v
}

/// f64 literal -> word form (de-tag: raw IEEE bits)
pub(crate) fn enc_f_lit(v: f64) -> i64 {
    v.to_bits() as i64
}

/// int scalar -> word (de-tag: identity)
pub(crate) fn emit_enc_int(_fw: &mut FnWalk, v: &str) -> String {
    v.to_string()
}

/// word -> raw int scalar (de-tag: identity)
pub(crate) fn emit_dec_int(_fw: &mut FnWalk, w: &str) -> String {
    w.to_string()
}

/// f64 scalar -> word: bitcast to the raw bit pattern
pub(crate) fn emit_enc_f(fw: &mut FnWalk, v: &str) -> String {
    let b = fw.v();
    fw.op(&format!("    {} = llvm.bitcast {} : f64 to i64", b, v));
    b
}

/// word -> f64 scalar: bitcast back
pub(crate) fn emit_dec_f(fw: &mut FnWalk, w: &str) -> String {
    let r = fw.v();
    fw.op(&format!("    {} = llvm.bitcast {} : i64 to f64", r, w));
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

/// i1 -> bool word (0 / 1)
pub(crate) fn ext_bool(fw: &mut FnWalk, c: &str) -> String {
    let z = fw.v();
    // zero-extend: an i1 `true` sign-extends to -1, which would corrupt the
    // bool word (we need exactly 1)
    fw.op(&format!("    {} = arith.extui {} : i1 to i64", z, c));
    z
}

/// invert an encoded bool word (0/1) when `neg` is set
pub(crate) fn neg_bool_word(fw: &mut FnWalk, b: &str, neg: bool) -> String {
    if !neg {
        return b.to_string();
    }
    let one = fw.v();
    fw.op(&format!(
        "    {} = arith.constant {} : i64",
        one,
        enc_i_lit(1)
    ));
    let o = fw.v();
    fw.op(&format!("    {} = arith.xori {}, {} : i64", o, b, one));
    o
}

/// convert an int word into an f64 word (decode, promote, re-encode)
pub(crate) fn iw_to_f64_word(fw: &mut FnWalk, w: &str) -> String {
    let d = emit_dec_int(fw, w);
    let f = fw.v();
    fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", f, d));
    emit_enc_f(fw, &f)
}

/// convert an unsigned int word into an f64 word (unsigned promotion: needed
/// for `uint` values whose bit pattern is negative when read signed)
pub(crate) fn iw_to_f64_word_u(fw: &mut FnWalk, w: &str) -> String {
    let d = emit_dec_int(fw, w);
    let f = fw.v();
    fw.op(&format!("    {} = arith.uitofp {} : i64 to f64", f, d));
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
        StmtNode::AssignOp { value, .. } => expr_has_super_init(value),
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

/// rebuild an lvalue expression from an assignment path. Used by the
/// compound-assign desugaring (`t op= v` ⇒ `t = t op v`) to re-read the
/// target's current value through the ordinary expression route.
pub(crate) fn path_to_expr(target: &[PathSeg], pos: &Pos) -> Expr {
    let mut acc: Option<Expr> = None;
    for seg in target {
        acc = Some(match seg {
            PathSeg::Name(n) => match &acc {
                None => Expr {
                    id: 0,
                    pos: pos.clone(),
                    node: match n.as_str() {
                        "this" => ExprNode::This,
                        "super" => ExprNode::Super,
                        _ => ExprNode::Ident(n.clone()),
                    },
                },
                Some(base) => Expr {
                    id: 0,
                    pos: pos.clone(),
                    node: ExprNode::Field {
                        obj: Box::new(base.clone()),
                        name: n.clone(),
                    },
                },
            },
            PathSeg::Index(ix) => Expr {
                id: 0,
                pos: pos.clone(),
                node: ExprNode::Index {
                    obj: Box::new(acc.take().unwrap_or(Expr {
                        id: 0,
                        pos: pos.clone(),
                        node: ExprNode::Nil,
                    })),
                    idx: Box::new(ix.clone()),
                },
            },
        });
    }
    acc.unwrap_or(Expr {
        id: 0,
        pos: pos.clone(),
        node: ExprNode::Nil,
    })
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
