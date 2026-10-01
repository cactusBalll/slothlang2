//! Emission-side coercions and ARC helpers (Pass 2). Type predicates live in
//! `sem/types.rs`; these methods materialise the coercions they decide on.

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
    // ---------------- rc machinery (ARC migration, patch B) ----------------

    /// emit `sloth.rc_release(h)` (nil and untracked words are rt no-ops)
    pub(crate) fn emit_release(&mut self, fw: &mut FnWalk, h: &str) {
        fw.op(&format!("    sloth.rc_release {} : i64", h));
    }

    /// emit `sloth.rc_retain(h)` (value-preserving)
    pub(crate) fn emit_retain(&mut self, fw: &mut FnWalk, h: &str) -> String {
        let r = fw.v();
        fw.op(&format!("    {} = sloth.rc_retain {} : i64", r, h));
        r
    }

    /// load the current word stored in a slot alloca (i64 route; ref words
    /// never live in float slots)
    pub(crate) fn load_slot(&mut self, fw: &mut FnWalk, a: &str) -> String {
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        let w = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            w, a, z
        ));
        w
    }

    /// assignment to a declared name: release the overload word first, then
    /// store (unconditional — rt no-ops for non-ref/nil words)
    #[allow(dead_code)]
    pub(crate) fn rc_assign_slot(&mut self, fw: &mut FnWalk, a: &str) {
        let old = self.load_slot(fw, a);
        self.emit_release(fw, &old);
    }

    /// declare bookkeeping (call at fw.declare sites): a ref-typed local's
    /// alloca joins this scope's release set
    #[allow(dead_code)]
    pub(crate) fn declare_rc(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        t: TyId,
        fl: bool,
        mutable: bool,
    ) -> String {
        let a = fw.declare(name, t, fl, mutable);
        if self.is_ref(t) {
            fw.track_slot(&a);
            fw.scope_decls
                .last_mut()
                .unwrap()
                .insert(name.to_string(), a.clone());
        }
        a
    }

    /// count a freshly created handle as a statement-dangling temp: the
    /// producer owns it; released once after the enclosing statement ends
    pub(crate) fn dangling_producer(&mut self, fw: &mut FnWalk, h: &str, t: TyId) {
        if self.is_ref(t) {
            fw.dangling.push(h.to_string());
        }
    }

    // -------- value-optional box coercions (patch 42) --------

    /// wrap a produced word into its value-optional surface (`int?` etc):
    /// a nil/Unit word passes through as nil (0); a bare scalar is boxed
    /// (`__sloth_box_new[_f64]`), an already-opt word passes through (idempotent)
    pub(crate) fn coerce_into_opt(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
        to: TyId,
    ) -> (String, TyId) {
        let (inner, fli) = match self.opt_inner(to) {
            Some(x) => x,
            None => return (v.to_string(), from),
        };
        let froms = self.r.get(from).clone();
        if matches!(froms, Ty::Unit) || from == to {
            return (v.to_string(), to);
        }
        if matches!(froms, Ty::I64 | Ty::Bool | Ty::Int(_)) {
            // int/bool word boxes as-is; into a float? surface promote first
            let payload = if fli {
                self.int_to_f64_word(fw, v, from)
            } else if matches!(self.r.get(inner), Ty::Int(_)) {
                self.coerce_int_word(fw, v, inner)
            } else {
                v.to_string()
            };
            let r = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_box_new({}) : (i64) -> i64",
                r, payload
            ));
            self.dangling_producer(fw, &r, to);
            return (r, to);
        }
        if froms == Ty::F64 {
            if fli {
                // f64 word boxes as-is (the box holds the encoded word)
                let r = fw.v();
                fw.op(&format!(
                    "    {} = func.call @__sloth_box_new({}) : (i64) -> i64",
                    r, v
                ));
                self.dangling_producer(fw, &r, to);
                return (r, to);
            }
            // float word into int?/bool?: word-view fallback, no box
            return (v.to_string(), to);
        }
        if matches!(froms, Ty::Opt(_)) {
            // already-boxed word of another inner family: unwrap, promote,
            // rebox into the target family
            let (p, pt) = self.unwrap_opt_word(fw, v, from);
            let (r, _t2) = self.coerce_into_opt(fw, &p, pt, to);
            return (r, to);
        }
        // word-view fallback (cross optional families / incompatible words)
        (v.to_string(), to)
    }

    /// read an optional word as its inner payload (nil reads as 0/0.0 —
    /// unwrap-or-0 semantics keeps the historical word view behavior)
    pub(crate) fn unwrap_opt_word(&mut self, fw: &mut FnWalk, v: &str, t: TyId) -> (String, TyId) {
        let (inner, fli) = match self.opt_inner(t) {
            Some(x) => x,
            None => return (v.to_string(), t),
        };
        let _ = fli;
        // tag migration: the box holds one tagged payload word
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_box_get({}) : (i64) -> i64",
            r, v
        ));
        (r, inner)
    }

    /// caller-side coercion of args to Opt(值型) params (patch 42): bare
    /// scalars box, opt/nil words pass through; other pairs unchanged;
    /// Weak(值型)-typed params (patch 43) wrap their targets too.
    ///
    /// A3 (batch 7): the *decision* — which argument binds to which declared
    /// surface — belongs to `sem`. Pass 1 computes it with
    /// [`ModEmitter::arg_coercion_target`] and freezes it per call site;
    /// Pass 2 replays the plan and only materialises the coercions (the
    /// diagnostics below are Pass-1-only, as everywhere else).
    pub(crate) fn coerce_args_to_params(
        &mut self,
        fw: &mut FnWalk,
        argv: &[(String, TyId)],
        params: &[(String, TyId, bool)],
        pos: &Pos,
        // `false` for raw C-ABI extern targets, whose word-plane signatures
        // (e.g. str handles declared `int`) are intentionally unchecked
        strict: bool,
    ) -> Vec<String> {
        let n = argv.len().min(params.len());
        let plan = self.planned_arg_coercions(pos);
        let mut decided: Vec<Option<TyId>> = Vec::with_capacity(argv.len());
        let mut out: Vec<String> = Vec::with_capacity(argv.len());
        for i in 0..argv.len() {
            if i < n {
                let pt = params[i].1;
                let at = argv[i].1;
                let av = argv[i].0.clone();
                // a value-less (unit) actual cannot bind to any parameter word:
                // `print(print("hi"))` must diagnose, not emit a 0-operand call
                if av.is_empty() {
                    self.err(
                        pos,
                        format!(
                            "argument {} is a `unit` expression; expected a value",
                            i + 1
                        ),
                    );
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    decided.push(None);
                    out.push(z);
                    continue;
                }
                // a value actual into a `dyn T` formal only boxes when the
                // builtin satisfies T (predefined / method-free)
                if self.check_mode {
                    if let Ty::Dyn(tn) = self.r.get(pt).clone() {
                        if self.value_kind(at).is_some() && !self.value_impls_trait(&tn) {
                            let got = self.surface_name(self.r.get(at));
                            self.err_diff(pos, "function argument", &format!("dyn {}", tn), &got);
                        }
                    }
                }
                // coercion target: recorded by `sem` in Pass 1, replayed by
                // Pass 2. Both passes must agree — a divergence would make
                // the caller's signature disagree with the callee's.
                let target: Option<TyId> = if self.check_mode {
                    self.arg_coercion_target(pt)
                } else {
                    let replay = plan.as_ref().and_then(|p| p.get(i)).copied();
                    debug_assert!(
                        replay.is_some(),
                        "arg coercion plan miss: frame={} pos={}:{} arg={}",
                        self.cur_frame,
                        pos.line,
                        pos.col,
                        i
                    );
                    if let Some(t) = replay {
                        // the plan stores either `None` (the word binds
                        // as-is) or exactly the parameter surface
                        debug_assert!(
                            t.is_none() || t == Some(pt),
                            "arg coercion plan diverged: frame={} pos={}:{} arg={}",
                            self.cur_frame,
                            pos.line,
                            pos.col,
                            i
                        );
                        t
                    } else {
                        self.arg_coercion_target(pt)
                    }
                };
                decided.push(target);
                if let Some(t) = target {
                    out.push(self.coerce_word_to(fw, &av, at, t).0);
                    continue;
                }
                // general declared-surface check (design §2.2): a known actual
                // surface must be compatible with the declared parameter
                // surface. `unit` (nil literal / unknown) stays lenient.
                // Pass 1 owns it; Pass 2 never re-derives the surfaces.
                if self.check_mode && strict {
                    let pty = self.r.get(pt).clone();
                    let aty = self.r.get(at).clone();
                    // the receiver (`this`) is a raw word at the call ABI and
                    // is never surface-checked
                    if params[i].0 != "this"
                        && !matches!(aty, Ty::Unit)
                        && !matches!(pty, Ty::Unit)
                        && !self.surface_compat(&pty, &aty)
                    {
                        let pn = self.surface_name(&pty);
                        let an = self.surface_name(&aty);
                        self.err_diff(pos, "function argument", &pn, &an);
                    }
                }
                out.push(av);
            } else {
                decided.push(None);
                out.push(argv[i].0.clone());
            }
        }
        self.record_arg_coercions(pos, decided);
        out
    }

    /// A `unit` expression has no word: storing it would emit `memref.store , …`
    /// or a call with a missing operand — MLIR "expected SSA operand" (probe
    /// `containers/x7`). Return a placeholder nil word so the store stays
    /// well-formed and, when `report` holds (not a `let _ = …` discard and no
    /// diagnostic was already raised for the expression), diagnose the misuse.
    pub(crate) fn value_or_nil_word(
        &mut self,
        fw: &mut FnWalk,
        v: String,
        t: TyId,
        pos: &Pos,
        report: bool,
    ) -> String {
        if !v.is_empty() {
            return v;
        }
        if self.is_unit(t) && report {
            self.err(
                pos,
                "a `unit` expression has no value; expected a value".to_string(),
            );
        }
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        z
    }

    /// call-site arity check: mismatch is a user diagnostic, not an MLIR ICE
    pub(crate) fn check_call_arity(
        &mut self,
        pos: &Pos,
        ctx: &str,
        want: usize,
        got: usize,
    ) -> bool {
        if want != got {
            self.err(
                pos,
                format!(
                    "wrong number of arguments in {}: expected {}, got {}",
                    ctx, want, got
                ),
            );
            false
        } else {
            true
        }
    }

    /// A3 (store-face family): materialise a replayed store face on a produced
    /// word. `sem` decided promotion / narrowing / boxing in Pass 1; this only
    /// applies it. Integer targets go through `coerce_int_word` (whose word
    /// transform covers widening to plain `int` too), the rest through
    /// `coerce_word_to`.
    pub(crate) fn apply_store_face(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        vty: TyId,
        face: crate::sem::StoreFace,
    ) -> (String, TyId) {
        match face {
            crate::sem::StoreFace::Identity => (v.to_string(), vty),
            crate::sem::StoreFace::IntToFloat => {
                (self.int_to_f64_word(fw, v, vty), self.r.mk(Ty::F64))
            }
            crate::sem::StoreFace::Coerce(t) => {
                if self.is_int_like(t) && self.is_int_like(vty) {
                    (self.coerce_int_word(fw, v, t), t)
                } else {
                    self.coerce_word_to(fw, v, vty, t)
                }
            }
        }
    }

    /// A3 (let-initializer family): materialise a replayed `let`/`var`
    /// initializer plan: box the word into the declared `dyn` surface when
    /// `sem` chose to, apply the store face, then record the planned binding
    /// surface.
    pub(crate) fn apply_let_plan(
        &mut self,
        fw: &mut FnWalk,
        v: String,
        t: TyId,
        plan: crate::sem::LetPlan,
    ) -> (String, TyId) {
        let (v, t) = match plan.dyn_box {
            Some(dt) => {
                let tn = match self.r.get(dt).clone() {
                    Ty::Dyn(n) => n,
                    other => {
                        debug_assert!(
                            false,
                            "dyn box surface is not a dyn type: {}",
                            self.surface_name(&other)
                        );
                        String::new()
                    }
                };
                (self.emit_dyn_box(fw, &v, t, &tn), t)
            }
            None => (v, t),
        };
        if matches!(plan.face, crate::sem::StoreFace::Identity) {
            (v, plan.bind_ty)
        } else {
            let (v, _) = self.apply_store_face(fw, &v, t, plan.face);
            (v, plan.bind_ty)
        }
    }

    /// bind a word into a declared surface (patch 42/43 entry): value
    /// optionals box up, Weak targets wrap in a weak box, else as-is
    pub(crate) fn coerce_word_to(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
        to: TyId,
    ) -> (String, TyId) {
        // narrow/widen between integer surfaces: truncate at the store face
        if matches!(self.r.get(to), Ty::Int(_)) && self.is_int_like(from) {
            return (self.coerce_int_word(fw, v, to), to);
        }
        // auto-box a builtin value into a `dyn Trait` surface
        if let Ty::Dyn(tname) = self.r.get(to).clone() {
            if self.value_kind(from).is_some() && self.value_impls_trait(&tname) {
                let b = self.emit_dyn_box(fw, v, from, &tname);
                return (b, to);
            }
            return (v.to_string(), from);
        }
        // box into the `any` top-type surface
        if matches!(self.r.get(to), Ty::Any) {
            return self.coerce_into_any(fw, v, from);
        }
        if self.opt_inner(to).is_some() {
            return self.coerce_into_opt(fw, v, from, to);
        }
        if self.weak_inner(to).is_some() {
            return self.coerce_into_weak(fw, v, from, to);
        }
        (v.to_string(), from)
    }

    /// wrap a produced word into a Weak<T> surface (patch 43): a weakbox
    /// (rc-tracked, malloc'd) holding the (possibly boxed) target; nil
    /// passes through as word 0; already-weak words ride along
    pub(crate) fn coerce_into_weak(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
        to: TyId,
    ) -> (String, TyId) {
        let inner = match self.weak_inner(to) {
            Some(e) => e,
            None => return (v.to_string(), from),
        };
        let froms = self.r.get(from).clone();
        if matches!(froms, Ty::Unit) || froms == Ty::Weak(inner) {
            return (v.to_string(), to);
        }
        let check = self.r.mk(Ty::Opt(inner));
        let (targ, _tt) = self.coerce_word_to(fw, v, from, check);
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_weak_new({}) : (i64) -> i64",
            r, targ
        ));
        self.dangling_producer(fw, &r, to);
        (r, to)
    }

    /// box a value word into the `any` top type: `nil` (word 0 of a
    /// nil-capable surface) collapses to `any` nil, everything else becomes a
    /// runtime-typed rc box carrying the surface's structural descriptor.
    pub(crate) fn coerce_into_any(
        &mut self,
        fw: &mut FnWalk,
        v: &str,
        from: TyId,
    ) -> (String, TyId) {
        let any = self.r.mk(Ty::Any);
        if matches!(self.r.get(from), Ty::Any | Ty::Unit) {
            return (v.to_string(), any);
        }
        let d = self.emit_any_desc_ptr(fw, from);
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_any_from({}, {}) : (i64, i64) -> i64",
            r, d, v
        ));
        self.dangling_producer(fw, &r, any);
        (r, any)
    }
}

// ---- integer width coercions (emission side; predicates live in sem/types) ----

impl ModEmitter {
    /// convert an integer word to an f64 word, choosing unsigned promotion
    /// for `uint` (whose bit pattern reads negative as i64)
    pub(crate) fn int_to_f64_word(&mut self, fw: &mut FnWalk, v: &str, src: TyId) -> String {
        if matches!(self.r.get(src), Ty::Int(sloth_frontend::ty::IntKind::U64)) {
            iw_to_f64_word_u(fw, v)
        } else {
            iw_to_f64_word(fw, v)
        }
    }

    /// truncate/sign-extend an i64 word to a fixed-width integer surface
    /// (no-op for `int`/`i64`/`uint64` where the full 64 bits are kept)
    pub(crate) fn coerce_int_word(&mut self, fw: &mut FnWalk, v: &str, to: TyId) -> String {
        let (bits, signed) = match self.int_info(to) {
            Some(x) => x,
            None => return v.to_string(),
        };
        if bits >= 64 {
            return v.to_string();
        }
        if signed {
            // sign-extend the low `bits`: (v << (64-bits)) >>a (64-bits)
            let sh = (64 - bits) as i64;
            let a = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", a, sh));
            let l = fw.v();
            fw.op(&format!("    {} = arith.shli {}, {} : i64", l, v, a));
            let r = fw.v();
            fw.op(&format!("    {} = arith.shrsi {}, {} : i64", r, l, a));
            r
        } else {
            // zero-extend: mask the low `bits`
            let mask: i64 = ((1u64 << bits) - 1) as i64;
            let m = fw.v();
            fw.op(&format!("    {} = arith.constant {} : i64", m, mask));
            let r = fw.v();
            fw.op(&format!("    {} = arith.andi {}, {} : i64", r, v, m));
            r
        }
    }
}

// ---- closure capture writeback + checked assignment store routes ----

impl ModEmitter {
    /// write a captured scalar back into the closure frame so the value
    /// persists across calls (design §10.2; bug A4). Reference captures use
    /// [`lambda_capture_writeback_ref`] instead (ARC-aware).
    pub(crate) fn lambda_capture_writeback(&mut self, fw: &mut FnWalk, name: &str, val: &str) {
        let (env_name, idx) = match fw.lambda_env.as_ref() {
            Some((e, m)) => match m.get(name) {
                Some(i) => (e.clone(), *i),
                None => return,
            },
            None => return,
        };
        let (a, _t) = match fw.lookup(&env_name) {
            Some(x) => x,
            None => return,
        };
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        let env = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            env, a, z
        ));
        let zi = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            zi,
            enc_i_lit(idx as i64)
        ));
        fw.op(&format!(
            "    func.call @__sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
            env, zi, val
        ));
    }

    /// ARC-aware reference-capture writeback (bug B1): the frame owns one
    /// count per reference field (retained when the closure was built), so an
    /// assignment to a captured reference must retain the new owner for the
    /// frame, release the field's previous owner, then store. Retain-before-
    /// release keeps `cap = cap` safe. A miss (not a capture) emits nothing.
    pub(crate) fn lambda_capture_writeback_ref(&mut self, fw: &mut FnWalk, name: &str, val: &str) {
        let (env_name, idx) = match fw.lambda_env.as_ref() {
            Some((e, m)) => match m.get(name) {
                Some(i) => (e.clone(), *i),
                None => return,
            },
            None => return,
        };
        let (a, _t) = match fw.lookup(&env_name) {
            Some(x) => x,
            None => return,
        };
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        let env = fw.v();
        fw.op(&format!(
            "    {} = memref.load {}[{}] : memref<1xi64>",
            env, a, z
        ));
        let zi = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            zi,
            enc_i_lit(idx as i64)
        ));
        // field read has no ARC side effect (raw word); nil is inert
        let old = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_obj_field({}, {}) : (i64, i64) -> i64",
            old, env, zi
        ));
        let rv = self.emit_retain(fw, val);
        self.emit_release(fw, &old);
        fw.op(&format!(
            "    func.call @__sloth_obj_set_field({}, {}, {}) : (i64, i64, i64) -> i64",
            env, zi, rv
        ));
    }

    /// plain-name assignment checked against the declared/inferred surface
    /// type recorded at declare time (patch #22): float target promotes int
    /// words; float value into non-float target diagnosed; structurally
    /// different i64-word surfaces (int/str/bool/class/array/map) diagnosed;
    /// nil (word 0) accepted into any non-float target.
    ///
    /// A3 (store-face family): Pass 1 decides the promotion/narrowing/boxing
    /// ([`ModEmitter::named_store_face`]) and freezes it against `stmt_id`;
    /// Pass 2 replays it. The diagnostics stay Pass-1-only (the `err*` helpers
    /// drop them in Pass 2).
    pub(crate) fn check_named_assign(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        dt: TyId,
        v: &str,
        vty: TyId,
        pos: &Pos,
        stmt_id: u32,
    ) {
        // value-optional surfaces (patch 42): wrap bare scalars into boxes,
        // keep nil (Unit) / already-opt words as they are; `dyn T` surfaces
        // auto-box builtin values too. The common i64 store path below
        // releases the old and retains the new owner.
        let face = if self.check_mode {
            let f = self.named_store_face(dt, vty);
            self.record_assign_face(stmt_id, f);
            f
        } else if stmt_id == 0 {
            // synthetic (desugared compound assignment): no plan, re-derive
            self.named_store_face(dt, vty)
        } else {
            match self.planned_assign_face(stmt_id) {
                Some(f) => f,
                None => {
                    debug_assert!(
                        false,
                        "assign-face plan miss: frame={} stmt={}",
                        self.cur_frame, stmt_id
                    );
                    self.named_store_face(dt, vty)
                }
            }
        };
        let (v, vty) = self.apply_store_face(fw, v, vty, face);
        if self.is_float(dt) {
            let stored = if self.is_float(vty) {
                v.clone()
            } else {
                // int word -> f64 word (slot storage is always the word plane)
                self.int_to_f64_word(fw, &v, vty)
            };
            fw.assign(name, &stored, true);
            self.lambda_capture_writeback(fw, name, &stored);
            return;
        }
        if self.is_float(vty) && self.opt_inner(dt).is_none() {
            let dtn = sloth_frontend::ty::ty_name(self.r.get(dt));
            self.err_diff(pos, &format!("assignment to `{}`", name), "float", &dtn);
            // keep IR parseable: store with the value's own float spelling
            fw.assign(name, &v, true);
            self.lambda_capture_writeback(fw, name, &v);
            return;
        }
        let dts = self.r.get(dt).clone();
        let vts = self.r.get(vty).clone();
        if !self.surface_compat(&dts, &vts) {
            let dtn = self.surface_name(&dts);
            let vtn = self.surface_name(&vts);
            self.err_diff(pos, &format!("assignment to `{}`", name), &dtn, &vtn);
        }
        // rc patch B: release the overwritten word, retain the new owner's
        // copy (nil = rt no-op; value words are NOT rc-managed under the
        // de-tag word plane and must store raw). Loop variables are BORROWS
        // of container elements (patch C): their slot owns no count.
        // patch 42: transferred call-result words already carry their +1 —
        // bind them raw instead of retaining a second count
        if !self.is_ref(dt) {
            fw.assign(name, &v, false);
            self.lambda_capture_writeback(fw, name, &v);
            return;
        }
        let xferred = fw.rc_take_xfer(&v);
        // a borrowed formal's first assignment must NOT release the incoming
        // caller-owned count (design §9.2/§23.1 rule 3); the slot only becomes
        // an owner from this assignment on (and is then tracked for scope exit)
        let borrow_param = fw.params.contains(name) && !fw.param_owned.contains(&name.to_string());
        match fw.lookup(name) {
            Some((a, _)) if !fw.loopvars.contains(&name.to_string()) => {
                if borrow_param {
                    if xferred {
                        fw.assign(name, &v, false);
                    } else if fw.rc_consume(&v) {
                        // fresh producer: its +1 becomes the slot's ownership
                        fw.assign(name, &v, false);
                    } else {
                        let rv = self.emit_retain(fw, &v);
                        fw.assign(name, &rv, false);
                    }
                    fw.param_owned.insert(name.to_string());
                    // once owned, the slot must be released at function exit
                    if let Some(sc) = fw.scope_decls.first_mut() {
                        sc.insert(name.to_string(), a.clone());
                    }
                } else {
                    let old = self.load_slot(fw, &a);
                    self.emit_release(fw, &old);
                    if xferred {
                        fw.assign(name, &v, false);
                    } else {
                        let rv = self.emit_retain(fw, &v);
                        fw.assign(name, &rv, false);
                    }
                }
            }
            _ => {
                fw.assign(name, &v, false);
            }
        }
        // reference captures persist across calls too (bug B1): update the
        // closure frame's owned field with the new handle (ARC-aware); no-op
        // when `name` is not a capture
        self.lambda_capture_writeback_ref(fw, name, &v);
    }

    /// store a tagged word into a module-level global cell
    pub(crate) fn store_global(&self, fw: &mut FnWalk, gsym: &str, dt: TyId, val: &str) {
        let mty = memref_cell_ty(self, dt);
        let g = fw.v();
        fw.op(&format!(
            "    {} = memref.get_global @{} : {}",
            g, gsym, mty
        ));
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", z));
        fw.op(&format!("    memref.store {}, {}[{}] : {}", val, g, z, mty));
    }

    /// plain-name assignment to a module-level global cell: mirrors
    /// `check_named_assign` (coercion + surface check + rc overwrite) but
    /// stores through `memref.get_global` instead of a local slot. A3
    /// (store-face family): the coercion is decided in Pass 1 and replayed.
    pub(crate) fn check_global_assign(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        gsym: &str,
        dt: TyId,
        v: &str,
        vty: TyId,
        pos: &Pos,
        stmt_id: u32,
    ) {
        let face = if self.check_mode {
            let f = self.named_store_face(dt, vty);
            self.record_assign_face(stmt_id, f);
            f
        } else if stmt_id == 0 {
            self.named_store_face(dt, vty)
        } else {
            match self.planned_assign_face(stmt_id) {
                Some(f) => f,
                None => {
                    debug_assert!(
                        false,
                        "assign-face plan miss: frame={} stmt={}",
                        self.cur_frame, stmt_id
                    );
                    self.named_store_face(dt, vty)
                }
            }
        };
        let (v, vty) = self.apply_store_face(fw, v, vty, face);
        if self.is_float(dt) {
            let cv = if self.is_float(vty) {
                v.clone()
            } else {
                self.int_to_f64_word(fw, &v, vty)
            };
            self.store_global(fw, gsym, dt, &cv);
            return;
        }
        if self.is_float(vty) && self.opt_inner(dt).is_none() {
            let dtn = sloth_frontend::ty::ty_name(self.r.get(dt));
            self.err_diff(pos, &format!("assignment to `{}`", name), "float", &dtn);
            self.store_global(fw, gsym, dt, &v);
            return;
        }
        // unannotated global (`dt == Unit`) stays word-lenient
        let dts = self.r.get(dt).clone();
        if !matches!(dts, Ty::Unit) {
            let vts = self.r.get(vty).clone();
            if !self.surface_compat(&dts, &vts) {
                let dtn = self.surface_name(&dts);
                let vtn = self.surface_name(&vts);
                self.err_diff(pos, &format!("assignment to `{}`", name), &dtn, &vtn);
            }
        }
        // rc: read old, retain new (unless ownership transferred), release old,
        // then store. Retain-before-release keeps `g = g` self-assignment safe.
        // Value globals are not rc-managed (de-tag): store the raw word.
        if !self.is_ref(dt) {
            self.store_global(fw, gsym, dt, &v);
            return;
        }
        let (old, _) = self.emit_global_read(fw, gsym, dt);
        let stored = if fw.rc_take_xfer(&v) {
            v.clone()
        } else {
            self.emit_retain(fw, &v)
        };
        self.emit_release(fw, &old);
        self.store_global(fw, gsym, dt, &stored);
    }
}
