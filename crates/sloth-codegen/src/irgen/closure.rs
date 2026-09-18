//! First-class function values (design §2.3/§2.6): a closure is a 2-word
//! object `{ tagged fnptr, env }`. Every value is callable through a
//! per-target *bridge* with the uniform ABI `(i64 env, i64 args...) -> i64`,
//! so call sites need no static knowledge of captures or the target symbol.

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

/// bridge call kinds
pub(crate) const BR_LAMBDA: u8 = 0;
pub(crate) const BR_NAMED: u8 = 1;
pub(crate) const BR_METHOD: u8 = 2;

impl ModEmitter {
    /// raw tagged function-pointer word for `sym` (bit0 set)
    pub(crate) fn emit_fnptr_word(&mut self, fw: &mut FnWalk, sym: &str) -> String {
        let p = fw.v();
        fw.op(&format!(
            "    {} = llvm.mlir.addressof @{} : !llvm.ptr",
            p, sym
        ));
        let iw = fw.v();
        fw.op(&format!(
            "    {} = llvm.ptrtoint {} : !llvm.ptr to i64",
            iw, p
        ));
        let one = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : i64", one));
        let w = fw.v();
        fw.op(&format!("    {} = arith.ori {}, {} : i64", w, iw, one));
        w
    }

    /// wrap `{ tagged fnptr, env }` into a fresh closure object; the box owns
    /// its own count on a ref-typed env
    pub(crate) fn emit_closure_box(&mut self, fw: &mut FnWalk, fnptr: &str, env: &str) -> String {
        // dedicated ctor: field 0 is a tagged (non-rc) fn pointer, so the
        // generic object cascade must not walk it (rt closure_dtor handles
        // the env field only)
        let rev = self.emit_retain(fw, env);
        let obj = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_closure_new({}, {}) : (i64, i64) -> i64",
            obj, fnptr, rev
        ));
        obj
    }

    /// build (once) the trampoline that adapts `target` to the uniform
    /// `(env, args...)` closure ABI and return its symbol
    pub(crate) fn fn_bridge(
        &mut self,
        key: &str,
        target: &str,
        ncap: usize,
        nargs: usize,
        kind: u8,
        target_is_llvm: bool,
        target_ret_unit: bool,
    ) -> String {
        if let Some(b) = self.bridges.get(key) {
            return b.clone();
        }
        let bname = format!("sloth_{}__clo{}", self.name, self.bridges.len());
        self.bridges.insert(key.to_string(), bname.clone());
        let mut body = String::new();
        let mut args: Vec<String> = Vec::new();
        let mut vc = 0usize;
        let fresh = |vc: &mut usize| -> String {
            *vc += 1;
            format!("%v{}", *vc)
        };
        if kind == BR_LAMBDA {
            for j in 0..ncap {
                let jw = fresh(&mut vc);
                // the frame stores capture `j` at the *tagged* index word
                // (`enc_i_lit(j)`); sloth_obj_field decodes it, so the bridge
                // must hand over the same encoding (multi-capture fix)
                body.push_str(&format!(
                    "    {} = arith.constant {} : i64\n",
                    jw,
                    enc_i_lit(j as i64)
                ));
                let c = fresh(&mut vc);
                body.push_str(&format!(
                    "    {} = func.call @sloth_obj_field(%p0, {}) : (i64, i64) -> i64\n",
                    c, jw
                ));
                args.push(c);
            }
        } else if kind == BR_METHOD {
            args.push("%p0".to_string());
        }
        for i in 0..nargs {
            args.push(format!("%p{}", i + 1));
        }
        let ckw = if target_is_llvm {
            "llvm.call"
        } else {
            "func.call"
        };
        let tys: Vec<String> = args.iter().map(|_| "i64".to_string()).collect();
        if target_ret_unit {
            body.push_str(&format!(
                "    {} @{}({}) : ({}) -> ()\n",
                ckw,
                target,
                args.join(", "),
                tys.join(", ")
            ));
            let z = fresh(&mut vc);
            body.push_str(&format!("    {} = arith.constant 0 : i64\n", z));
            body.push_str(&format!("    llvm.return {} : i64\n", z));
        } else {
            let r = fresh(&mut vc);
            body.push_str(&format!(
                "    {} = {} @{}({}) : ({}) -> i64\n",
                r,
                ckw,
                target,
                args.join(", "),
                tys.join(", ")
            ));
            body.push_str(&format!("    llvm.return {} : i64\n", r));
        }
        let named: Vec<String> = (0..=nargs).map(|i| format!("%p{}: i64", i)).collect();
        // bridges must be llvm.func so their address is addressable by
        // llvm.mlir.addressof (the closure stores a raw fn pointer)
        self.out.push_str(&format!(
            "  llvm.func @{}({}) -> i64 {{\n{}}}\n",
            bname,
            named.join(", "),
            body
        ));
        bname
    }

    /// a named (non-generic) function used as a first-class value: bridge it
    /// behind a fresh closure box with an empty environment
    pub(crate) fn emit_fn_named_value(
        &mut self,
        fw: &mut FnWalk,
        name: &str,
        pos: &Pos,
    ) -> Option<(String, TyId)> {
        let fd = self.funcs.get(name)?.clone();
        if fd.is_extern || fd.variadic.is_some() || !fd.type_params.is_empty() {
            self.err(
                pos,
                format!(
                    "`{}` cannot be used as a value (generic/variadic/extern function)",
                    name
                ),
            );
            return None;
        }
        let plan = self.plan_func(name, None, &fd, None);
        let sym = plan.mangled.clone();
        let nargs = fd.params.len();
        let bkey = format!("named:{}", sym);
        let bridge = self.fn_bridge(
            &bkey,
            &sym,
            0,
            nargs,
            BR_NAMED,
            false,
            self.is_unit(plan.ret),
        );
        let fp = self.emit_fnptr_word(fw, &bridge);
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        let cbox = self.emit_closure_box(fw, &fp, &z);
        let pty: Vec<TyId> = plan.params.iter().map(|p| p.1).collect();
        let ft = self.r.mk(Ty::Fn(FnTy {
            params: pty,
            ret: plan.ret,
            lam: Some(LamMeta {
                sym: bridge,
                caps: Vec::new(),
            }),
        }));
        self.dangling_producer(fw, &cbox, ft);
        Some((cbox, ft))
    }

    /// a method reference `obj.method` as a first-class value: bridge the
    /// resolved method, with the receiver as the environment (design §6)
    pub(crate) fn emit_method_ref_value(
        &mut self,
        fw: &mut FnWalk,
        defcls: &str,
        mname: &str,
        fd: &FuncDef,
        recv: &str,
        pos: &Pos,
    ) -> (String, TyId) {
        let plan = self.plan_for_class(mname, defcls, fd);
        let is_ll = self
            .llvm_method
            .contains(&(defcls.to_string(), mname.to_string()));
        let nargs = fd.params.len().saturating_sub(1);
        let bkey = format!("meth:{}", plan.mangled);
        let bridge = self.fn_bridge(
            &bkey,
            &plan.mangled,
            0,
            nargs,
            BR_METHOD,
            is_ll,
            self.is_unit(plan.ret),
        );
        let fp = self.emit_fnptr_word(fw, &bridge);
        let cbox = self.emit_closure_box(fw, &fp, recv);
        let pty: Vec<TyId> = plan.params.iter().skip(1).map(|p| p.1).collect();
        let ft = self.r.mk(Ty::Fn(FnTy {
            params: pty,
            ret: plan.ret,
            lam: Some(LamMeta {
                sym: bridge,
                caps: Vec::new(),
            }),
        }));
        self.dangling_producer(fw, &cbox, ft);
        let _ = pos;
        (cbox, ft)
    }

    /// indirect call through a closure word; `fts` is the value's Fn shape
    pub(crate) fn emit_fn_value_call(
        &mut self,
        fw: &mut FnWalk,
        clo: &str,
        ft: &FnTy,
        args: &Vec<Expr>,
        pos: &Pos,
    ) -> (String, TyId) {
        let f0 = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            f0,
            enc_i_lit(0)
        ));
        let fp = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
            fp, clo, f0
        ));
        let f1 = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            f1,
            enc_i_lit(1)
        ));
        let env = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_obj_field({}, {}) : (i64, i64) -> i64",
            env, clo, f1
        ));
        let fm2 = fw.v();
        fw.op(&format!("    {} = arith.constant -2 : i64", fm2));
        let fpr = fw.v();
        fw.op(&format!("    {} = arith.andi {}, {} : i64", fpr, fp, fm2));
        let vp = fw.v();
        fw.op(&format!(
            "    {} = llvm.inttoptr {} : i64 to !llvm.ptr",
            vp, fpr
        ));
        if args.len() != ft.params.len() {
            self.err_diff(
                pos,
                "function call arity",
                &format!("{} arg(s)", ft.params.len()),
                &format!("{} arg(s)", args.len()),
            );
        }
        let mut vals: Vec<String> = vec![env];
        let mut tys: Vec<String> = vec!["i64".to_string()];
        for (i, a) in args.iter().enumerate() {
            let (v, at) = self.emit_expr(fw, a);
            if let Some(pt) = ft.params.get(i).copied() {
                // design §2.1: no implicit int/float conversion on args
                if self.is_float(pt) != self.is_float(at) {
                    let an = sloth_frontend::ty::ty_name(self.r.get(pt));
                    let bn = sloth_frontend::ty::ty_name(self.r.get(at));
                    self.err_diff(&a.pos, "function argument", &an, &bn);
                }
            }
            vals.push(v);
            tys.push("i64".to_string());
        }
        let ret = ft.ret;
        if self.is_unit(ret) {
            fw.op(&format!(
                "    llvm.call {}({}) : !llvm.ptr, ({}) -> ()",
                vp,
                vals.join(", "),
                tys.join(", ")
            ));
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            (z, ret)
        } else {
            let r = fw.v();
            fw.op(&format!(
                "    {} = llvm.call {}({}) : !llvm.ptr, ({}) -> i64",
                r,
                vp,
                vals.join(", "),
                tys.join(", ")
            ));
            // §5.1.1 rule 5: the bridge returns the callee's owned +1
            if self.is_ref(ret) {
                fw.rc_mark_xfer(&r);
            }
            (r, ret)
        }
    }
}
