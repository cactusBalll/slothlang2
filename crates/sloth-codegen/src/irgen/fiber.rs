//! Coroutine extension CE-P1: `fiber.*` compiler builtins.
//!
//! `fiber.create/resume/yield/transfer/error/check/resumable/cancel` are
//! recognized at the call site (like `tensor.*`) and lowered straight to the
//! `sloth_fiber_*` runtime entry points. Payload words cross the boundary
//! untouched; the runtime handles the retain/release balance, so the emitter
//! only has to type the result and mark owned reference returns.

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
    /// payload type `Y` of a `Fiber<Y>` surface
    pub(crate) fn fiber_inner(&self, t: TyId) -> Option<TyId> {
        match self.r.get(t) {
            Ty::Fiber(e) => Some(*e),
            _ => None,
        }
    }

    /// payload type of an entry closure `(Y) -> unit`
    fn fiber_entry_payload(&self, t: TyId) -> Option<TyId> {
        match self.r.get(t) {
            Ty::Fn(f) => f.params.first().copied(),
            _ => None,
        }
    }

    /// value payloads ride boxed optionals so a yielded word `0` never reads
    /// as completion nil
    fn fiber_boxed(&self, y: TyId) -> bool {
        matches!(self.r.get(y), Ty::I64 | Ty::Int(_) | Ty::F64 | Ty::Bool)
    }

    fn fiber_bail(&mut self, fw: &mut FnWalk, pos: &Pos, msg: String) -> (String, TyId) {
        self.err(pos, msg);
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }

    pub(crate) fn emit_fiber_create(
        &mut self,
        fw: &mut FnWalk,
        farg: &Expr,
        iarg: &Expr,
        sarg: Option<&Expr>,
        pos: &Pos,
    ) -> (String, TyId) {
        let (fv, ft) = self.emit_expr(fw, farg);
        let (iv, _it) = self.emit_expr(fw, iarg);
        let y = match self.fiber_entry_payload(ft) {
            Some(y) => y,
            None => {
                return self.fiber_bail(
                    fw,
                    pos,
                    "fiber.create requires an entry function `(Y) -> unit`".into(),
                )
            }
        };
        let sym = if sarg.is_some() {
            "sloth_fiber_create_with"
        } else {
            "sloth_fiber_create"
        };
        let eref = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            eref,
            if self.is_ref(y) { 1 } else { 0 }
        ));
        let r = fw.v();
        match sarg {
            Some(s) => {
                let (sv, _st) = self.emit_expr(fw, s);
                fw.op(&format!(
                    "    {} = func.call @{}({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
                    r, sym, fv, iv, sv, eref
                ));
            }
            None => {
                fw.op(&format!(
                    "    {} = func.call @{}({}, {}, {}) : (i64, i64, i64) -> i64",
                    r, sym, fv, iv, eref
                ));
            }
        }
        let rt = self.r.mk(Ty::Fiber(y));
        self.dangling_producer(fw, &r, rt);
        (r, rt)
    }

    fn emit_fiber_switch(
        &mut self,
        fw: &mut FnWalk,
        sym: &str,
        farg: &Expr,
        varg: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (fv, ft) = self.emit_expr(fw, farg);
        let y = match self.fiber_inner(ft) {
            Some(y) => y,
            None => {
                return self.fiber_bail(fw, pos, format!("`{}` expects a `Fiber<Y>` receiver", sym))
            }
        };
        let (vv, vt) = self.emit_expr(fw, varg);
        if !self.surface_compat(self.r.get(vt), self.r.get(y)) {
            return self.fiber_bail(
                fw,
                pos,
                format!(
                    "fiber payload mismatch: expected {}, got {}",
                    self.surface_name(self.r.get(y)),
                    self.surface_name(self.r.get(vt))
                ),
            );
        }
        let boxf = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            boxf,
            if self.fiber_boxed(y) { 1 } else { 0 }
        ));
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_fiber_{}({}, {}, {}) : (i64, i64, i64) -> i64",
            r, sym, fv, vv, boxf
        ));
        let rt = self.r.mk(Ty::Opt(y));
        if self.is_ref(rt) {
            fw.rc_mark_xfer(&r);
        }
        (r, rt)
    }

    pub(crate) fn emit_fiber_resume(
        &mut self,
        fw: &mut FnWalk,
        farg: &Expr,
        varg: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        self.emit_fiber_switch(fw, "resume", farg, varg, pos)
    }

    pub(crate) fn emit_fiber_transfer(
        &mut self,
        fw: &mut FnWalk,
        farg: &Expr,
        varg: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        self.emit_fiber_switch(fw, "transfer", farg, varg, pos)
    }

    pub(crate) fn emit_fiber_yield(
        &mut self,
        fw: &mut FnWalk,
        varg: &Expr,
        _pos: &Pos,
    ) -> (String, TyId) {
        let (vv, vt) = self.emit_expr(fw, varg);
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_fiber_yield({}) : (i64) -> i64",
            r, vv
        ));
        // cooperative cancellation: when the fiber was cancelled while
        // suspended, unwind this frame (settle its owned locals/temps) before
        // longjmping to the entry landing pad
        let c = fw.v();
        fw.op(&format!(
            "    {} = func.call @sloth_fiber_cancelled() : () -> i64",
            c
        ));
        let lbl_abort = fw.newlabel("fx");
        let lbl_cont = fw.newlabel("fx");
        let saved_dangling = std::mem::take(&mut fw.dangling);
        let saved_xfer = std::mem::take(&mut fw.xfer);
        fw.cjump(&c, &lbl_abort, &lbl_cont);
        fw.label(&lbl_abort);
        fw.dangling = saved_dangling.clone();
        fw.xfer = saved_xfer.clone();
        fw.rc_flush();
        fw.rc_release_scope_slots();
        // the resume payload returned by this yield is abandoned on cancel
        fw.op(&format!("    sloth.rc_release {} : i64", r));
        fw.op("    func.call @sloth_fiber_cancel_abort() : () -> ()");
        fw.jump(&lbl_cont);
        fw.label(&lbl_cont);
        fw.dangling = saved_dangling;
        fw.xfer = saved_xfer;
        if self.is_ref(vt) {
            fw.rc_mark_xfer(&r);
        }
        (r, vt)
    }

    pub(crate) fn emit_fiber_error(&mut self, fw: &mut FnWalk, marg: &Expr) -> (String, TyId) {
        let (mv, _mt) = self.emit_expr(fw, marg);
        // transfer an owning +1 on the message to the runtime (it prints then
        // releases), so the skipped caller frame cannot leak the value
        let owned = if fw.rc_consume(&mv) {
            mv.clone()
        } else {
            self.emit_retain(fw, &mv)
        };
        // `fiber.error` never returns: settle this frame's owned temps/locals
        // now (the normal scope teardown is unreachable)
        fw.rc_flush();
        fw.rc_release_scope_slots();
        fw.op(&format!(
            "    func.call @sloth_fiber_error({}) : (i64) -> i64",
            owned
        ));
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }

    fn emit_fiber_pred(&mut self, fw: &mut FnWalk, farg: &Expr, sym: &str) -> (String, TyId) {
        let (fv, _ft) = self.emit_expr(fw, farg);
        let r = fw.v();
        fw.op(&format!("    {} = func.call @{}({}) : (i64) -> i64", r, sym, fv));
        (r, self.r.mk(Ty::Bool))
    }

    pub(crate) fn emit_fiber_check(&mut self, fw: &mut FnWalk, farg: &Expr) -> (String, TyId) {
        self.emit_fiber_pred(fw, farg, "sloth_fiber_check")
    }

    pub(crate) fn emit_fiber_resumable(&mut self, fw: &mut FnWalk, farg: &Expr) -> (String, TyId) {
        self.emit_fiber_pred(fw, farg, "sloth_fiber_resumable")
    }

    pub(crate) fn emit_fiber_cancel(&mut self, fw: &mut FnWalk, farg: &Expr) -> (String, TyId) {
        let (fv, _ft) = self.emit_expr(fw, farg);
        fw.op(&format!(
            "    func.call @sloth_fiber_cancel({}) : (i64) -> i64",
            fv
        ));
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }
}
