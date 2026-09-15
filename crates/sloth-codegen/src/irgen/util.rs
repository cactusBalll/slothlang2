//! Small shared helpers: mangling, word types, super-init scanners.

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

pub(crate) fn words_scalar(t: &Ty) -> usize {
    match t {
        Ty::Unit => 0,
        _ => 1,
    }
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
        Ty::F64 => "f64".to_string(),
        _ => "i64".to_string(),
    }
}

pub(crate) fn mlir_word_ty(t: TyId, r: &Reg) -> String {
    match r.get(t) {
        Ty::F64 => "f64".to_string(),
        _ => "i64".to_string(),
    }
}

pub(crate) fn str_slot_len(s: &str) -> usize {
    s.as_bytes().len() + 1
}

pub(crate) fn memref_cell_ty(me: &ModEmitter, t: TyId) -> &'static str {
    if me.is_float(t) {
        "memref<1xf64>"
    } else {
        "memref<1xi64>"
    }
}
