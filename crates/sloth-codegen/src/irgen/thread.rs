//! Multithreading extension (TH-P1/P2): `thread.*`, `channel.*`, `mutex.*`
//! and `atomic.*` compiler builtins.
//!
//! Recognized at the call site (like `fiber.*` / `tensor.*`) and lowered
//! straight to the `sloth_thread_*` / `sloth_chan_*` / `sloth_mutex_*` /
//! `sloth_atomic_*` runtime entry points. `JoinHandle<R>`/`Channel<T>` are
//! builtin reference types; `Mutex`/`AtomicInt` are builtin opaque handles.
//! The runtime owns the ARC retain/release balance at the spawn/join and
//! channel send/recv boundaries (same discipline as fiber payloads).

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
    pub(crate) fn th_bail(&mut self, fw: &mut FnWalk, pos: &Pos, msg: &str) -> (String, TyId) {
        self.err(pos, msg.to_string());
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }

    /// result type `R` of a `JoinHandle<R>` surface
    #[allow(dead_code)]
    pub(crate) fn joinhandle_inner(&self, t: TyId) -> Option<TyId> {
        match self.r.get(t) {
            Ty::JoinHandle(e) => Some(*e),
            _ => None,
        }
    }

    /// element type `T` of a `Channel<T>` surface
    #[allow(dead_code)]
    pub(crate) fn channel_inner(&self, t: TyId) -> Option<TyId> {
        match self.r.get(t) {
            Ty::Channel(e) => Some(*e),
            _ => None,
        }
    }

    /// value element types ride a boxed optional on `recv` (nil = closed)
    fn chan_boxed(&self, e: TyId) -> bool {
        matches!(self.r.get(e), Ty::I64 | Ty::F64 | Ty::Bool)
    }

    /// `thread.spawn(f: (T) -> R, arg: T): JoinHandle<R>`
    pub(crate) fn emit_thread_spawn(
        &mut self,
        fw: &mut FnWalk,
        farg: &Expr,
        iarg: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (fv, ft) = self.emit_expr(fw, farg);
        let (iv, it) = self.emit_expr(fw, iarg);
        let (t, r) = match self.r.get(ft).clone() {
            Ty::Fn(f) if f.params.len() == 1 => (f.params[0], f.ret),
            _ => {
                self.err(
                    pos,
                    "thread.spawn requires an entry function `(T) -> R`".into(),
                );
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                let u = self.r.mk(Ty::Unit);
                return (z, self.r.mk(Ty::JoinHandle(u)));
            }
        };
        if !self.surface_compat(self.r.get(it), self.r.get(t)) {
            self.err_diff(
                pos,
                "thread.spawn argument",
                &self.surface_name(self.r.get(t)).clone(),
                &self.surface_name(self.r.get(it)).clone(),
            );
        }
        if !self.send_ok(t, 0) || !self.send_ok(r, 0) {
            self.err(
                pos,
                "thread.spawn payload/result must satisfy `Send` (Fiber<Y> is thread-confined)"
                    .into(),
            );
        }
        let aref = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            aref,
            if self.is_ref(t) { 1 } else { 0 }
        ));
        let rref = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            rref,
            if self.is_ref(r) { 1 } else { 0 }
        ));
        let out = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_thread_spawn({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
            out, fv, iv, aref, rref
        ));
        let rt = self.r.mk(Ty::JoinHandle(r));
        self.dangling_producer(fw, &out, rt);
        (out, rt)
    }

    pub(crate) fn emit_thread_current_id(&mut self, fw: &mut FnWalk) -> (String, TyId) {
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_thread_current_id() : () -> i64",
            r
        ));
        (r, self.r.mk(Ty::I64))
    }

    pub(crate) fn emit_thread_yield(&mut self, fw: &mut FnWalk) -> (String, TyId) {
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_thread_yield_now() : () -> i64",
            r
        ));
        (r, self.r.mk(Ty::Unit))
    }

    /// bootstrap `channel.new<T>(capacity)`: returns a `Channel<T>` handle
    pub(crate) fn emit_channel_new(
        &mut self,
        fw: &mut FnWalk,
        targ: TyId,
        carg: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        if !self.send_ok(targ, 0) {
            self.err(pos, "channel element type must satisfy `Send`".into());
        }
        let (cv, ct) = self.emit_expr(fw, carg);
        if !matches!(self.r.get(ct), Ty::I64) {
            self.err(pos, "channel capacity must be an `int`".into());
        }
        let eref = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            eref,
            if self.is_ref(targ) { 1 } else { 0 }
        ));
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_chan_new({}, {}) : (i64, i64) -> i64",
            r, cv, eref
        ));
        let rt = self.r.mk(Ty::Channel(targ));
        self.dangling_producer(fw, &r, rt);
        (r, rt)
    }

    pub(crate) fn emit_mutex_new(&mut self, fw: &mut FnWalk) -> (String, TyId) {
        let r = fw.v();
        fw.op(&format!("    {} = call @sloth_mutex_new() : () -> i64", r));
        let rt = self.r.mk(Ty::Mutex);
        self.dangling_producer(fw, &r, rt);
        (r, rt)
    }

    pub(crate) fn emit_atomic_new(
        &mut self,
        fw: &mut FnWalk,
        iarg: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (iv, it) = self.emit_expr(fw, iarg);
        if !matches!(self.r.get(it), Ty::I64) {
            self.err(pos, "atomic.new requires an `int` initial value".into());
        }
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_atomic_new({}) : (i64) -> i64",
            r, iv
        ));
        let rt = self.r.mk(Ty::AtomicInt);
        self.dangling_producer(fw, &r, rt);
        (r, rt)
    }

    /// transfer the owned +1 of an already-emitted argument to a callee that
    /// takes ownership (channel send): consume a producer/xfer temp, else
    /// materialize a retain. Value element types are not rc-managed and pass
    /// raw (de-tag).
    fn transfer_arg(&mut self, fw: &mut FnWalk, vv: &str, et: TyId) -> String {
        if !self.is_ref(et) {
            return vv.to_string();
        }
        if fw.rc_consume(vv) || fw.rc_take_xfer(vv) {
            vv.to_string()
        } else {
            self.emit_retain(fw, vv)
        }
    }

    /// receiver method faces for the TH builtin types; `argv[0]` is the
    /// receiver (already emitted), later slots are the call arguments
    pub(crate) fn emit_thread_recv_method(
        &mut self,
        fw: &mut FnWalk,
        recvv: &str,
        rt: TyId,
        name: &str,
        argv: &[(String, TyId)],
        pos: &Pos,
    ) -> Option<(String, TyId)> {
        match self.r.get(rt).clone() {
            Ty::JoinHandle(r) => match name {
                "join" => {
                    let out = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_thread_join({}) : (i64) -> i64",
                        out, recvv
                    ));
                    if self.is_ref(r) {
                        fw.rc_mark_xfer(&out);
                    }
                    Some((out, r))
                }
                "detach" => {
                    fw.op(&format!(
                        "    call @sloth_thread_detach({}) : (i64) -> i64",
                        recvv
                    ));
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    Some((z, self.r.mk(Ty::Unit)))
                }
                _ => None,
            },
            Ty::Channel(t) => match name {
                "send" => {
                    if argv.len() != 2 {
                        self.err(pos, "channel.send requires one argument".into());
                        return Some((String::new(), self.r.mk(Ty::Unit)));
                    }
                    let owned = self.transfer_arg(fw, &argv[1].0, t);
                    fw.op(&format!(
                        "    call @sloth_chan_send({}, {}) : (i64, i64) -> i64",
                        recvv, owned
                    ));
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    Some((z, self.r.mk(Ty::Unit)))
                }
                "recv" => {
                    let bf = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        bf,
                        if self.chan_boxed(t) { 1 } else { 0 }
                    ));
                    let out = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_chan_recv({}, {}) : (i64, i64) -> i64",
                        out, recvv, bf
                    ));
                    let ot = self.r.mk(Ty::Opt(t));
                    if self.is_ref(ot) {
                        fw.rc_mark_xfer(&out);
                    }
                    Some((out, ot))
                }
                "close" => {
                    fw.op(&format!(
                        "    call @sloth_chan_close({}) : (i64) -> i64",
                        recvv
                    ));
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    Some((z, self.r.mk(Ty::Unit)))
                }
                _ => None,
            },
            Ty::Mutex => match name {
                "lock" | "unlock" => {
                    let sym = if name == "lock" {
                        "sloth_mutex_lock"
                    } else {
                        "sloth_mutex_unlock"
                    };
                    fw.op(&format!("    call @{}({}) : (i64) -> i64", sym, recvv));
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    Some((z, self.r.mk(Ty::Unit)))
                }
                "try_lock" => {
                    let out = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_mutex_try_lock({}) : (i64) -> i64",
                        out, recvv
                    ));
                    Some((out, self.r.mk(Ty::Bool)))
                }
                "with" => {
                    if argv.len() != 2 {
                        self.err(pos, "mutex.with requires a closure argument".into());
                        return Some((String::new(), self.r.mk(Ty::Unit)));
                    }
                    fw.op(&format!(
                        "    call @sloth_mutex_with({}, {}) : (i64, i64) -> i64",
                        recvv, argv[1].0
                    ));
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    Some((z, self.r.mk(Ty::Unit)))
                }
                _ => None,
            },
            Ty::AtomicInt => match name {
                "load" => {
                    let out = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_atomic_load({}) : (i64) -> i64",
                        out, recvv
                    ));
                    Some((out, self.r.mk(Ty::I64)))
                }
                "store" => {
                    if argv.len() != 2 {
                        self.err(pos, "atomic.store requires one argument".into());
                        return Some((String::new(), self.r.mk(Ty::Unit)));
                    }
                    fw.op(&format!(
                        "    call @sloth_atomic_store({}, {}) : (i64, i64) -> i64",
                        recvv, argv[1].0
                    ));
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    Some((z, self.r.mk(Ty::Unit)))
                }
                "add" | "sub" => {
                    if argv.len() != 2 {
                        self.err(pos, format!("atomic.{} requires one argument", name));
                        return Some((String::new(), self.r.mk(Ty::I64)));
                    }
                    let sym = if name == "add" {
                        "sloth_atomic_add"
                    } else {
                        "sloth_atomic_sub"
                    };
                    let out = fw.v();
                    fw.op(&format!(
                        "    {} = call @{}({}, {}) : (i64, i64) -> i64",
                        out, sym, recvv, argv[1].0
                    ));
                    Some((out, self.r.mk(Ty::I64)))
                }
                "cas" => {
                    if argv.len() != 3 {
                        self.err(
                            pos,
                            "atomic.cas requires an expected and a new value".into(),
                        );
                        return Some((String::new(), self.r.mk(Ty::Bool)));
                    }
                    let out = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_atomic_cas({}, {}, {}) : (i64, i64, i64) -> i64",
                        out, recvv, argv[1].0, argv[2].0
                    ));
                    Some((out, self.r.mk(Ty::Bool)))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
