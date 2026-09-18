//! Tensor extension TE-P1: construction, indexing/slicing views and view
//! assignment. The ABI stays one tagged rc word (D1); the runtime owns the
//! descriptor and the (non-tracked) element buffer, codegen only computes
//! element offsets and calls the `sloth_tensor_*` helpers.

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
    /// `(elem, rank)` of a tensor surface, if it is one
    pub(crate) fn tensor_info(&self, t: TyId) -> Option<(TyId, u32)> {
        match self.r.get(t) {
            Ty::Tensor(e, r) => Some((*e, *r)),
            _ => None,
        }
    }

    /// tagged int word for the runtime element-kind flag (0 int / 1 float)
    fn tensor_kind_word(&mut self, fw: &mut FnWalk, elem: TyId) -> String {
        let kind = if self.is_float(elem) { 1 } else { 0 };
        let c = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            c,
            enc_i_lit(kind)
        ));
        c
    }

    /// rank 1..=3 checked; the runtime constructor family is rank-specific
    fn tensor_rank_ok(&mut self, pos: &Pos, rank: u32) -> bool {
        if (1..=3).contains(&rank) {
            true
        } else {
            self.err(
                pos,
                format!(
                    "tensor rank {} unsupported (TE-P1 supports rank 1..=3)",
                    rank
                ),
            );
            false
        }
    }

    /// allocate a zeroed tensor whose dims are read from a shape `Array<int>`
    /// word; `rank` comes from the declared target surface
    pub(crate) fn emit_tensor_new_from_shape(
        &mut self,
        fw: &mut FnWalk,
        shape_w: &str,
        rank: u32,
        elem: TyId,
        pos: &Pos,
    ) -> (String, TyId) {
        if !self.tensor_rank_ok(pos, rank) {
            let z = fw.v();
            fw.op(&format!("    {} = arith.constant 0 : i64", z));
            return (z, self.r.mk(Ty::Tensor(elem, rank)));
        }
        let mut vals: Vec<String> = Vec::new();
        for k in 0..rank {
            let kw = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                kw,
                enc_i_lit(k as i64)
            ));
            let dw = fw.v();
            fw.op(&format!(
                "    {} = call @sloth_arr_get({}, {}) : (i64, i64) -> i64",
                dw, shape_w, kw
            ));
            vals.push(dw);
        }
        let kindw = self.tensor_kind_word(fw, elem);
        vals.push(kindw);
        let ty = self.r.mk(Ty::Tensor(elem, rank));
        let r = fw.v();
        let sig: Vec<&str> = (0..vals.len()).map(|_| "i64").collect();
        fw.op(&format!(
            "    {} = call @sloth_tensor_new_{}({}) : ({}) -> i64",
            r,
            rank,
            vals.join(", "),
            sig.join(", ")
        ));
        self.dangling_producer(fw, &r, ty);
        (r, ty)
    }

    /// `tensor.zeros(shape)`: elem/rank from the declared target surface
    pub(crate) fn emit_tensor_zeros(
        &mut self,
        fw: &mut FnWalk,
        shape: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let hint = self.exp_ret.last().copied();
        let (elem, rank) = match hint.and_then(|h| self.tensor_info(h)) {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.zeros` requires a declared `Tensor<T, R>` target".to_string(),
                );
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                return (z, self.r.mk(Ty::Unit));
            }
        };
        let (sv, _st) = self.emit_expr(fw, shape);
        self.emit_tensor_new_from_shape(fw, &sv, rank, elem, pos)
    }

    /// `tensor.from_array(data, shape)`: element kind from the source array
    /// (falling back to the target surface), rank from the target surface
    pub(crate) fn emit_tensor_from_array(
        &mut self,
        fw: &mut FnWalk,
        data: &Expr,
        shape: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let hint = self.exp_ret.last().copied();
        let (helem, rank) = match hint.and_then(|h| self.tensor_info(h)) {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.from_array` requires a declared `Tensor<T, R>` target".to_string(),
                );
                let z = fw.v();
                fw.op(&format!("    {} = arith.constant 0 : i64", z));
                return (z, self.r.mk(Ty::Unit));
            }
        };
        let (dv, dt) = self.emit_expr(fw, data);
        let elem = match self.r.get(dt).clone() {
            Ty::Array(e) => e,
            _ => {
                self.err(pos, "`tensor.from_array` data must be an Array".to_string());
                helem
            }
        };
        let (sv, _st) = self.emit_expr(fw, shape);
        let (tw, ty) = self.emit_tensor_new_from_shape(fw, &sv, rank, elem, pos);
        fw.op(&format!(
            "    call @sloth_tensor_copy_from_array({}, {}) : (i64, i64) -> i64",
            tw, dv
        ));
        (tw, ty)
    }

    /// `t[i]` / `t[a..b]` on a tensor: rank-1 integer index yields the
    /// element scalar; otherwise a shared-storage view is produced
    pub(crate) fn emit_tensor_index(
        &mut self,
        fw: &mut FnWalk,
        av: &str,
        elem: TyId,
        rank: u32,
        idx: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        match &idx.node {
            ExprNode::Range {
                low,
                high,
                inclusive,
            } => {
                if !self.tensor_rank_ok(pos, rank) {
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    return (z, self.r.mk(Ty::Tensor(elem, rank)));
                }
                // keep-rank slice: off = lo, len = hi(+1) - lo
                let (lo, _) = self.emit_expr(fw, low);
                let (hi0, _) = self.emit_expr(fw, high);
                let hi = if *inclusive {
                    let one = fw.v();
                    fw.op(&format!(
                        "    {} = arith.constant {} : i64",
                        one,
                        enc_i_lit(1)
                    ));
                    let h = fw.v();
                    fw.op(&format!("    {} = arith.addi {}, {} : i64", h, hi0, one));
                    h
                } else {
                    hi0
                };
                let len = fw.v();
                fw.op(&format!("    {} = arith.subi {}, {} : i64", len, hi, lo));
                let dropw = fw.v();
                fw.op(&format!(
                    "    {} = arith.constant {} : i64",
                    dropw,
                    enc_i_lit(0)
                ));
                let r = fw.v();
                fw.op(&format!(
                    "    {} = call @sloth_tensor_view({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
                    r, av, lo, dropw, len
                ));
                let ty = self.r.mk(Ty::Tensor(elem, rank));
                self.dangling_producer(fw, &r, ty);
                (r, ty)
            }
            _ => {
                let (iv, _) = self.emit_expr(fw, idx);
                if rank == 1 {
                    let r = fw.v();
                    fw.op(&format!(
                        "    {} = call @sloth_tensor_get1({}, {}) : (i64, i64) -> i64",
                        r, av, iv
                    ));
                    (r, elem)
                } else if self.tensor_rank_ok(pos, rank) {
                    self.emit_tensor_view_drop(fw, av, elem, rank, &iv, pos)
                } else {
                    let z = fw.v();
                    fw.op(&format!("    {} = arith.constant 0 : i64", z));
                    (z, self.r.mk(Ty::Tensor(elem, rank)))
                }
            }
        }
    }

    /// `t[i]` view dropping dim 0 (rank R -> R-1)
    pub(crate) fn emit_tensor_view_drop(
        &mut self,
        fw: &mut FnWalk,
        av: &str,
        elem: TyId,
        rank: u32,
        iv: &str,
        _pos: &Pos,
    ) -> (String, TyId) {
        let dropw = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            dropw,
            enc_i_lit(1)
        ));
        let zero = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            zero,
            enc_i_lit(0)
        ));
        let r = fw.v();
        fw.op(&format!(
            "    {} = call @sloth_tensor_view({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
            r, av, iv, dropw, zero
        ));
        let ty = self.r.mk(Ty::Tensor(elem, rank - 1));
        self.dangling_producer(fw, &r, ty);
        (r, ty)
    }

    /// element write through a rank-1 tensor/view
    pub(crate) fn emit_tensor_set1(&mut self, fw: &mut FnWalk, av: &str, iw: &str, v: &str) {
        fw.op(&format!(
            "    call @sloth_tensor_set1({}, {}, {}) : (i64, i64, i64) -> i64",
            av, iw, v
        ));
    }

    pub(crate) fn emit_tensor_copy_into(&mut self, fw: &mut FnWalk, dst: &str, src: &str) {
        fw.op(&format!(
            "    call @sloth_tensor_copy_into({}, {}) : (i64, i64) -> i64",
            dst, src
        ));
    }
}
