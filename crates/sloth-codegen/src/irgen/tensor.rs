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

/// MLIR f64 literal spelling: MLIR requires a decimal point (so `1e-5` is
/// rejected while `1.0e-5` is accepted); Rust's `{:?}` drops the `.0`.
fn fmt_f64(v: f64) -> String {
    let s = format!("{:?}", v);
    if let Some(epos) = s.find(['e', 'E']) {
        if !s[..epos].contains('.') {
            return format!("{}.0{}", &s[..epos], &s[epos..]);
        }
    } else if !s.contains('.') {
        return format!("{}.0", s);
    }
    s
}

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

    /// runtime same-shape assertion (panics on mismatch, design §3.1)
    fn emit_shape_eq(&mut self, fw: &mut FnWalk, a: &str, b: &str) {
        fw.op(&format!(
            "    func.call @__sloth_tensor_shape_eq({}, {}) : (i64, i64) -> i64",
            a, b
        ));
    }

    /// runtime `dim(a, axa) == dim(b, axb)` assertion
    fn emit_dim_eq(&mut self, fw: &mut FnWalk, a: &str, axa: i64, b: &str, axb: i64) {
        let aa = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            aa,
            enc_i_lit(axa)
        ));
        let bb = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            bb,
            enc_i_lit(axb)
        ));
        fw.op(&format!(
            "    func.call @__sloth_tensor_dim_eq({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
            a, aa, b, bb
        ));
    }

    /// error-path zero word for an operator that could not be emitted
    pub(crate) fn tensor_bail(&mut self, fw: &mut FnWalk) -> (String, TyId) {
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::F64))
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
                "    {} = func.call @__sloth_arr_get({}, {}) : (i64, i64) -> i64",
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
            "    {} = func.call @__sloth_tensor_new_{}({}) : ({}) -> i64",
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
            "    func.call @__sloth_tensor_copy_from_array({}, {}) : (i64, i64) -> i64",
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
                    "    {} = func.call @__sloth_tensor_view({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
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
                        "    {} = func.call @__sloth_tensor_get1({}, {}) : (i64, i64) -> i64",
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
            "    {} = func.call @__sloth_tensor_view({}, {}, {}, {}) : (i64, i64, i64, i64) -> i64",
            r, av, iv, dropw, zero
        ));
        let ty = self.r.mk(Ty::Tensor(elem, rank - 1));
        self.dangling_producer(fw, &r, ty);
        (r, ty)
    }

    /// element write through a rank-1 tensor/view
    pub(crate) fn emit_tensor_set1(&mut self, fw: &mut FnWalk, av: &str, iw: &str, v: &str) {
        fw.op(&format!(
            "    func.call @__sloth_tensor_set1({}, {}, {}) : (i64, i64, i64) -> i64",
            av, iw, v
        ));
    }

    pub(crate) fn emit_tensor_copy_into(&mut self, fw: &mut FnWalk, dst: &str, src: &str) {
        fw.op(&format!(
            "    func.call @__sloth_tensor_copy_into({}, {}) : (i64, i64) -> i64",
            dst, src
        ));
    }

    // ---- TE-P2 channel B: memref-domain linalg operators -------------------
    //
    // The tagged tensor word is bridged into a `memref` via the runtime basis
    // descriptor + `memref.reinterpret_cast` using the *runtime* shape/stride
    // (R1). Operators then run `linalg.*` directly on memrefs, so they bypass
    // one-shot-bufferization entirely (design D2) and views keep working.

    /// MLIR element type of a tensor element surface
    fn tensor_elem_mlir(&self, elem: TyId) -> &'static str {
        if self.is_float(elem) {
            "f64"
        } else {
            "i64"
        }
    }

    fn tensor_basis_fn(&self, elem: TyId) -> &'static str {
        if self.is_float(elem) {
            "__sloth_tensor_basis_f64"
        } else {
            "__sloth_tensor_basis_i64"
        }
    }

    /// flat rank-1 strided memref over the tensor's element buffer
    pub(crate) fn emit_tensor_basis(&mut self, fw: &mut FnWalk, tv: &str, elem: TyId) -> String {
        let et = self.tensor_elem_mlir(elem);
        let f = self.tensor_basis_fn(elem);
        let r = fw.v();
        fw.op(&format!(
            "    {} = func.call @{}({}) : (i64) -> memref<?x{}, strided<[?], offset: ?>>",
            r, f, tv, et
        ));
        r
    }

    /// tagged dim word of `tv` along `axis`
    fn emit_tensor_dim_word(&mut self, fw: &mut FnWalk, tv: &str, axis: i64) -> String {
        let ax = fw.v();
        fw.op(&format!(
            "    {} = arith.constant {} : i64",
            ax,
            enc_i_lit(axis)
        ));
        let dw = fw.v();
        fw.op(&format!(
            "    {} = func.call @__sloth_tensor_dim({}, {}) : (i64, i64) -> i64",
            dw, tv, ax
        ));
        dw
    }

    /// allocate a contiguous tensor from tagged dim words (rank 1..=3)
    fn emit_tensor_alloc_dims(
        &mut self,
        fw: &mut FnWalk,
        dims: &[String],
        elem: TyId,
    ) -> (String, TyId) {
        let rank = dims.len() as u32;
        let kindw = self.tensor_kind_word(fw, elem);
        let mut vals = dims.to_vec();
        vals.push(kindw);
        let ty = self.r.mk(Ty::Tensor(elem, rank));
        let r = fw.v();
        let sig: Vec<&str> = (0..vals.len()).map(|_| "i64").collect();
        fw.op(&format!(
            "    {} = func.call @__sloth_tensor_new_{}({}) : ({}) -> i64",
            r,
            rank,
            vals.join(", "),
            sig.join(", ")
        ));
        self.dangling_producer(fw, &r, ty);
        (r, ty)
    }

    /// rank-1/2 strided memref view of `tv` with runtime sizes/strides
    pub(crate) fn emit_tensor_memref(
        &mut self,
        fw: &mut FnWalk,
        tv: &str,
        elem: TyId,
        rank: u32,
    ) -> (String, String) {
        let et = self.tensor_elem_mlir(elem).to_string();
        let flat = self.emit_tensor_basis(fw, tv, elem);
        let flat_ty = format!("memref<?x{}, strided<[?], offset: ?>>", et);
        let mut dims: Vec<String> = Vec::new();
        let mut strides: Vec<String> = Vec::new();
        for k in 0..rank {
            let ax = fw.v();
            fw.op(&format!(
                "    {} = arith.constant {} : i64",
                ax,
                enc_i_lit(k as i64)
            ));
            let dw = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_tensor_dim({}, {}) : (i64, i64) -> i64",
                dw, tv, ax
            ));
            let d = fw.v();
            fw.op(&format!(
                "    {} = arith.index_cast {} : i64 to index",
                d, dw
            ));
            dims.push(d);
            let sw = fw.v();
            fw.op(&format!(
                "    {} = func.call @__sloth_tensor_stride({}, {}) : (i64, i64) -> i64",
                sw, tv, ax
            ));
            let s = fw.v();
            fw.op(&format!(
                "    {} = arith.index_cast {} : i64 to index",
                s, sw
            ));
            strides.push(s);
        }
        let o = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", o));
        let target_ty = match rank {
            2 => format!("memref<?x?x{}, strided<[?, ?], offset: ?>>", et),
            _ => format!("memref<?x{}, strided<[?], offset: ?>>", et),
        };
        let r = fw.v();
        fw.op(&format!(
            "    {} = memref.reinterpret_cast {} to offset: [{}], sizes: [{}], strides: [{}] : {} to {}",
            r,
            flat,
            o,
            dims.join(", "),
            strides.join(", "),
            flat_ty,
            target_ty
        ));
        (r, target_ty)
    }

    /// `tensor.matvec(w: Tensor<T,2>, x: Tensor<T,1>): Tensor<T,1>` (channel B)
    pub(crate) fn emit_tensor_matvec(
        &mut self,
        fw: &mut FnWalk,
        we: &Expr,
        xe: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (wv, wt) = self.emit_expr(fw, we);
        let (xv, xt) = self.emit_expr(fw, xe);
        let wi = self.tensor_info(wt);
        let xi = self.tensor_info(xt);
        let (welem, wrank) = match wi {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.matvec` first operand must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        let (xelem, xrank) = match xi {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.matvec` second operand must be a tensor".into(),
                );
                return self.tensor_bail(fw);
            }
        };
        if wrank != 2 || xrank != 1 {
            self.err(
                pos,
                "`tensor.matvec` requires `Tensor<T,2>` and `Tensor<T,1>`".into(),
            );
            return self.tensor_bail(fw);
        }
        if welem != xelem {
            self.err(pos, "`tensor.matvec` element kind mismatch".into());
            return self.tensor_bail(fw);
        }
        if !self.is_float(welem) {
            self.err(
                pos,
                "`tensor.matvec` supports `float` elements only (TE-P2)".into(),
            );
            return self.tensor_bail(fw);
        }
        self.emit_dim_eq(fw, &wv, 1, &xv, 0);
        let d0 = self.emit_tensor_dim_word(fw, &wv, 0);
        let (yv, yt) = self.emit_tensor_alloc_dims(fw, &[d0], welem);
        let (wm, wty) = self.emit_tensor_memref(fw, &wv, welem, 2);
        let (xm, xty) = self.emit_tensor_memref(fw, &xv, xelem, 1);
        let (ym, yty) = self.emit_tensor_memref(fw, &yv, welem, 1);
        fw.op(&format!(
            "    linalg.matvec ins({}, {} : {}, {}) outs({} : {})",
            wm, xm, wty, xty, ym, yty
        ));
        (yv, yt)
    }

    /// encode a scalar MLIR value (f64/i64) into one word (de-tag: float
    /// results are raw f64 bits, int results are the native i64)
    fn emit_encode_scalar(&mut self, fw: &mut FnWalk, val: &str, elem: TyId) -> String {
        if self.is_float(elem) {
            let bv = fw.v();
            fw.op(&format!("    {} = arith.bitcast {} : f64 to i64", bv, val));
            bv
        } else {
            val.to_string()
        }
    }

    /// arithmetic op mnemonic for element type + operation stem
    fn arith_binop(&self, elem: TyId, stem: &str) -> String {
        if self.is_float(elem) {
            format!("{}f", stem)
        } else {
            match stem {
                "sub" => "subi",
                "mul" => "muli",
                "div" => "divsi",
                _ => "addi",
            }
            .to_string()
        }
    }

    fn parallel_maps(rank: u32) -> (String, String) {
        match rank {
            2 => (
                "affine_map<(d0, d1) -> (d0, d1)>".to_string(),
                "\"parallel\", \"parallel\"".to_string(),
            ),
            _ => (
                "affine_map<(d0) -> (d0)>".to_string(),
                "\"parallel\"".to_string(),
            ),
        }
    }

    /// elementwise `out = a OP b` over same-shape tensors (rank 1/2)
    pub(crate) fn emit_tensor_binop(
        &mut self,
        fw: &mut FnWalk,
        a: &Expr,
        b: &Expr,
        opname: &str,
        pos: &Pos,
    ) -> (String, TyId) {
        let stem = opname;
        let (av, at) = self.emit_expr(fw, a);
        let (bv, bt) = self.emit_expr(fw, b);
        let ai = self.tensor_info(at);
        let bi = self.tensor_info(bt);
        let (aelem, arank) = match ai {
            Some(x) => x,
            None => {
                self.err(pos, format!("`tensor.{}` operands must be tensors", opname));
                return self.tensor_bail(fw);
            }
        };
        let (belem, brank) = match bi {
            Some(x) => x,
            None => {
                self.err(pos, format!("`tensor.{}` operands must be tensors", opname));
                return self.tensor_bail(fw);
            }
        };
        if arank != brank || aelem != belem {
            self.err(
                pos,
                format!(
                    "`tensor.{}` operands must share element kind and rank",
                    opname
                ),
            );
            return self.tensor_bail(fw);
        }
        if !(1..=2).contains(&arank) {
            self.err(
                pos,
                format!("`tensor.{}` supports rank 1/2 (TE-P2)", opname),
            );
            return self.tensor_bail(fw);
        }
        self.emit_shape_eq(fw, &av, &bv);
        let mut dims = Vec::new();
        for k in 0..arank {
            dims.push(self.emit_tensor_dim_word(fw, &av, k as i64));
        }
        let (ov, ot) = self.emit_tensor_alloc_dims(fw, &dims, aelem);
        let (am, aty) = self.emit_tensor_memref(fw, &av, aelem, arank);
        let (bm, bty) = self.emit_tensor_memref(fw, &bv, belem, arank);
        let (om, oty) = self.emit_tensor_memref(fw, &ov, aelem, arank);
        let et = self.tensor_elem_mlir(aelem).to_string();
        let arith = self.arith_binop(aelem, stem);
        let (map, iters) = Self::parallel_maps(arank);
        let x = fw.v();
        let y = fw.v();
        let o = fw.v();
        let s = fw.v();
        fw.op(&format!(
            "    linalg.generic {{indexing_maps = [{}, {}, {}], iterator_types = [{}]}} ins({}, {} : {}, {}) outs({} : {}) {{",
            map, map, map, iters, am, bm, aty, bty, om, oty
        ));
        fw.op(&format!(
            "    ^bb0({}: {}, {}: {}, {}: {}):",
            x, et, y, et, o, et
        ));
        fw.op(&format!(
            "      {} = arith.{} {}, {} : {}",
            s, arith, x, y, et
        ));
        fw.op(&format!("      linalg.yield {} : {}", s, et));
        fw.op("    }");
        (ov, ot)
    }

    /// `tensor.matmul(a: Tensor<T,2>, b: Tensor<T,2>): Tensor<T,2>`
    pub(crate) fn emit_tensor_matmul(
        &mut self,
        fw: &mut FnWalk,
        ae: &Expr,
        be: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (av, at) = self.emit_expr(fw, ae);
        let (bv, bt) = self.emit_expr(fw, be);
        let (aelem, arank) = match self.tensor_info(at) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.matmul` first operand must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        let (belem, brank) = match self.tensor_info(bt) {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.matmul` second operand must be a tensor".into(),
                );
                return self.tensor_bail(fw);
            }
        };
        if arank != 2 || brank != 2 {
            self.err(
                pos,
                "`tensor.matmul` requires two `Tensor<T,2>` operands".into(),
            );
            return self.tensor_bail(fw);
        }
        if aelem != belem || !self.is_float(aelem) {
            self.err(
                pos,
                "`tensor.matmul` requires matching `float` elements (TE-P2)".into(),
            );
            return self.tensor_bail(fw);
        }
        self.emit_dim_eq(fw, &av, 1, &bv, 0);
        let d0 = self.emit_tensor_dim_word(fw, &av, 0);
        let d1 = self.emit_tensor_dim_word(fw, &bv, 1);
        let (ov, ot) = self.emit_tensor_alloc_dims(fw, &[d0, d1], aelem);
        let (am, aty) = self.emit_tensor_memref(fw, &av, aelem, 2);
        let (bm, bty) = self.emit_tensor_memref(fw, &bv, belem, 2);
        let (om, oty) = self.emit_tensor_memref(fw, &ov, aelem, 2);
        fw.op(&format!(
            "    linalg.matmul ins({}, {} : {}, {}) outs({} : {})",
            am, bm, aty, bty, om, oty
        ));
        (ov, ot)
    }

    /// `tensor.dot(a: Tensor<T,1>, b: Tensor<T,1>): T` (rank-1 reduction)
    pub(crate) fn emit_tensor_dot(
        &mut self,
        fw: &mut FnWalk,
        ae: &Expr,
        be: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (av, at) = self.emit_expr(fw, ae);
        let (bv, bt) = self.emit_expr(fw, be);
        let (aelem, arank) = match self.tensor_info(at) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.dot` first operand must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        let (belem, brank) = match self.tensor_info(bt) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.dot` second operand must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        if arank != 1 || brank != 1 {
            self.err(
                pos,
                "`tensor.dot` requires two `Tensor<T,1>` operands".into(),
            );
            return self.tensor_bail(fw);
        }
        if aelem != belem {
            self.err(pos, "`tensor.dot` element kind mismatch".into());
            return self.tensor_bail(fw);
        }
        self.emit_dim_eq(fw, &av, 0, &bv, 0);
        let (am, aty) = self.emit_tensor_memref(fw, &av, aelem, 1);
        let (bm, bty) = self.emit_tensor_memref(fw, &bv, belem, 1);
        let n = self.emit_tensor_dim_index(fw, &av, 0);
        let zero = self.emit_zero_scalar(fw, aelem);
        let res = self.emit_reduce_loop(fw, &[(am, aty), (bm, bty)], &n, &zero, aelem, true);
        let enc = self.emit_encode_scalar(fw, &res, aelem);
        (enc, aelem)
    }

    /// dim word decoded to an `index` (de-tag: identity)
    fn emit_tensor_dim_index(&mut self, fw: &mut FnWalk, tv: &str, axis: i64) -> String {
        let dw = self.emit_tensor_dim_word(fw, tv, axis);
        let d = fw.v();
        fw.op(&format!(
            "    {} = arith.index_cast {} : i64 to index",
            d, dw
        ));
        d
    }

    fn emit_zero_scalar(&mut self, fw: &mut FnWalk, elem: TyId) -> String {
        let zero = fw.v();
        if self.is_float(elem) {
            fw.op(&format!("    {} = arith.constant 0.0 : f64", zero));
        } else {
            fw.op(&format!("    {} = arith.constant 0 : i64", zero));
        }
        zero
    }

    /// `scf.for` reduction with a register accumulator (no `memref.alloca`,
    /// so repeated calls inside a loop do not grow the stack). `loads` are the
    /// rank-1 memrefs to read at index `i`; when `mul` is set the loaded
    /// elements are multiplied then added (dot), else simply added (sum).
    fn emit_reduce_loop(
        &mut self,
        fw: &mut FnWalk,
        loads: &[(String, String)],
        n: &str,
        zero: &str,
        elem: TyId,
        mul: bool,
    ) -> String {
        let et = self.tensor_elem_mlir(elem).to_string();
        let add = self.arith_binop(elem, "add");
        let mulf = self.arith_binop(elem, "mul");
        let c0 = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", c0));
        let c1 = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : index", c1));
        let res = fw.v();
        let i = fw.v();
        let acc = fw.v();
        fw.op(&format!(
            "    {} = scf.for {} = {} to {} step {} iter_args({} = {}) -> ({}) {{",
            res, i, c0, n, c1, acc, zero, et
        ));
        let mut loaded: Vec<String> = Vec::new();
        for (m, mty) in loads {
            let lv = fw.v();
            fw.op(&format!(
                "      {} = memref.load {}[{}] : {}",
                lv, m, i, mty
            ));
            loaded.push(lv);
        }
        let term = if mul && loaded.len() >= 2 {
            let p = fw.v();
            fw.op(&format!(
                "      {} = arith.{} {}, {} : {}",
                p, mulf, loaded[0], loaded[1], et
            ));
            p
        } else {
            loaded[0].clone()
        };
        let s = fw.v();
        fw.op(&format!(
            "      {} = arith.{} {}, {} : {}",
            s, add, acc, term, et
        ));
        fw.op(&format!("      scf.yield {} : {}", s, et));
        fw.op("    }");
        res
    }

    /// `tensor.sum(a: Tensor<T,1>): T` (rank-1 add-reduction)
    pub(crate) fn emit_tensor_sum(
        &mut self,
        fw: &mut FnWalk,
        ae: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (av, at) = self.emit_expr(fw, ae);
        let (aelem, arank) = match self.tensor_info(at) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.sum` operand must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        if arank != 1 {
            self.err(pos, "`tensor.sum` requires a `Tensor<T,1>` (TE-P2)".into());
            return self.tensor_bail(fw);
        }
        let (am, aty) = self.emit_tensor_memref(fw, &av, aelem, 1);
        let n = self.emit_tensor_dim_index(fw, &av, 0);
        let zero = self.emit_zero_scalar(fw, aelem);
        let res = self.emit_reduce_loop(fw, &[(am, aty)], &n, &zero, aelem, false);
        let enc = self.emit_encode_scalar(fw, &res, aelem);
        (enc, aelem)
    }

    /// `tensor.add_into(dst, src)`: in-place `dst += src` (no allocation)
    pub(crate) fn emit_tensor_add_into(
        &mut self,
        fw: &mut FnWalk,
        de: &Expr,
        se: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (dv, dt) = self.emit_expr(fw, de);
        let (sv, st) = self.emit_expr(fw, se);
        let (delem, drank) = match self.tensor_info(dt) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.add_into` destination must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        let (selem, srank) = match self.tensor_info(st) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.add_into` source must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        if drank != srank || delem != selem {
            self.err(
                pos,
                "`tensor.add_into` operands must share element kind and rank".into(),
            );
            return self.tensor_bail(fw);
        }
        self.emit_shape_eq(fw, &dv, &sv);
        let (dm, dty) = self.emit_tensor_memref(fw, &dv, delem, drank);
        let (sm, sty) = self.emit_tensor_memref(fw, &sv, selem, srank);
        let et = self.tensor_elem_mlir(delem).to_string();
        let arith = self.arith_binop(delem, "add");
        let (map, iters) = Self::parallel_maps(drank);
        let x = fw.v();
        let y = fw.v();
        let o = fw.v();
        let s = fw.v();
        fw.op(&format!(
            "    linalg.generic {{indexing_maps = [{}, {}, {}], iterator_types = [{}]}} ins({}, {} : {}, {}) outs({} : {}) {{",
            map, map, map, iters, dm, sm, dty, sty, dm, dty
        ));
        fw.op(&format!(
            "    ^bb0({}: {}, {}: {}, {}: {}):",
            x, et, y, et, o, et
        ));
        fw.op(&format!(
            "      {} = arith.{} {}, {} : {}",
            s, arith, o, y, et
        ));
        fw.op(&format!("      linalg.yield {} : {}", s, et));
        fw.op("    }");
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }

    // ---- TE-P3: fused elementwise / reduction kernels ----------------------

    /// open a same-shape `linalg.generic` map over `inputs` (`(value, type)`)
    /// writing `out`; returns the block-argument names (inputs then output)
    fn open_map(
        &mut self,
        fw: &mut FnWalk,
        inputs: &[(String, String)],
        out: &(String, String),
        rank: u32,
        elem: TyId,
    ) -> Vec<String> {
        let (map, iters) = Self::parallel_maps(rank);
        let maps: Vec<String> = (0..inputs.len() + 1).map(|_| map.clone()).collect();
        let names: Vec<String> = inputs.iter().map(|(n, _)| n.clone()).collect();
        let tys: Vec<String> = inputs.iter().map(|(_, t)| t.clone()).collect();
        let ins_clause = format!("ins({} : {})", names.join(", "), tys.join(", "));
        fw.op(&format!(
            "    linalg.generic {{indexing_maps = [{}], iterator_types = [{}]}} {} outs({} : {}) {{",
            maps.join(", "),
            iters,
            ins_clause,
            out.0,
            out.1
        ));
        let et = self.tensor_elem_mlir(elem).to_string();
        let mut names: Vec<String> = Vec::new();
        let mut sig: Vec<String> = Vec::new();
        for _ in inputs {
            let a = fw.v();
            sig.push(format!("{}: {}", a, et));
            names.push(a);
        }
        let o = fw.v();
        sig.push(format!("{}: {}", o, et));
        names.push(o);
        fw.op(&format!("    ^bb0({}):", sig.join(", ")));
        names
    }

    fn close_map(&mut self, fw: &mut FnWalk, yieldv: &str, elem: TyId) {
        let et = self.tensor_elem_mlir(elem).to_string();
        fw.op(&format!("      linalg.yield {} : {}", yieldv, et));
        fw.op("    }");
    }

    fn fconst(&mut self, fw: &mut FnWalk, v: f64) -> String {
        let c = fw.v();
        fw.op(&format!("    {} = arith.constant {} : f64", c, fmt_f64(v)));
        c
    }

    /// dim word decoded to a bare `i64` (de-tag: identity)
    fn emit_tensor_dim_i64(&mut self, fw: &mut FnWalk, tv: &str, axis: i64) -> String {
        self.emit_tensor_dim_word(fw, tv, axis)
    }

    /// allocate a same-shape output for `av` and return `(word, memref, ty, elem)`
    fn emit_same_shape_out(
        &mut self,
        fw: &mut FnWalk,
        av: &str,
        elem: TyId,
        rank: u32,
    ) -> (String, TyId, String, String) {
        let mut dims = Vec::new();
        for k in 0..rank {
            dims.push(self.emit_tensor_dim_word(fw, av, k as i64));
        }
        let (ov, ot) = self.emit_tensor_alloc_dims(fw, &dims, elem);
        let (om, oty) = self.emit_tensor_memref(fw, &ov, elem, rank);
        (ov, ot, om, oty)
    }

    /// `tensor.exp/sqrt/sin/cos/tan(a)`: elementwise `math.*` (rank 1/2)
    pub(crate) fn emit_tensor_unary(
        &mut self,
        fw: &mut FnWalk,
        ae: &Expr,
        op: &str,
        pos: &Pos,
    ) -> (String, TyId) {
        let (av, at) = self.emit_expr(fw, ae);
        let (elem, rank) = match self.tensor_info(at) {
            Some(x) => x,
            None => {
                self.err(pos, format!("`tensor.{}` operand must be a tensor", op));
                return self.tensor_bail(fw);
            }
        };
        if !(1..=2).contains(&rank) || !self.is_float(elem) {
            self.err(
                pos,
                format!("`tensor.{}` requires a rank 1/2 `float` tensor", op),
            );
            return self.tensor_bail(fw);
        }
        let (ov, ot, om, oty) = self.emit_same_shape_out(fw, &av, elem, rank);
        let (am, aty) = self.emit_tensor_memref(fw, &av, elem, rank);
        let a = self.open_map(fw, &[(am, aty)], &(om, oty), rank, elem);
        let r = fw.v();
        fw.op(&format!("      {} = math.{} {} : f64", r, op, a[0]));
        self.close_map(fw, &r, elem);
        (ov, ot)
    }

    /// `tensor.silu(a)`: `x * sigmoid(x)` fused into one generic (rank 1/2)
    pub(crate) fn emit_tensor_silu(
        &mut self,
        fw: &mut FnWalk,
        ae: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (av, at) = self.emit_expr(fw, ae);
        let (elem, rank) = match self.tensor_info(at) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.silu` operand must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        if !(1..=2).contains(&rank) || !self.is_float(elem) {
            self.err(
                pos,
                "`tensor.silu` requires a rank 1/2 `float` tensor".into(),
            );
            return self.tensor_bail(fw);
        }
        let (ov, ot, om, oty) = self.emit_same_shape_out(fw, &av, elem, rank);
        let (am, aty) = self.emit_tensor_memref(fw, &av, elem, rank);
        let a = self.open_map(fw, &[(am, aty)], &(om, oty), rank, elem);
        let s = self.emit_silu_body(fw, &a[0]);
        self.close_map(fw, &s, elem);
        (ov, ot)
    }

    /// `silu(x)` body value (fused: `x / (1 + exp(-x))`)
    fn emit_silu_body(&mut self, fw: &mut FnWalk, x: &str) -> String {
        let neg = fw.v();
        fw.op(&format!("      {} = arith.negf {} : f64", neg, x));
        let e = fw.v();
        fw.op(&format!("      {} = math.exp {} : f64", e, neg));
        let one = self.fconst(fw, 1.0);
        let den = fw.v();
        fw.op(&format!("      {} = arith.addf {}, {} : f64", den, one, e));
        let r = fw.v();
        fw.op(&format!("      {} = arith.divf {}, {} : f64", r, x, den));
        r
    }

    /// `tensor.silu_mul_into(a, b)`: in-place `a = silu(a) * b` (SwiGLU)
    pub(crate) fn emit_tensor_silu_mul_into(
        &mut self,
        fw: &mut FnWalk,
        ae: &Expr,
        be: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (av, at) = self.emit_expr(fw, ae);
        let (bv, bt) = self.emit_expr(fw, be);
        let (elem, rank) = match self.tensor_info(at) {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.silu_mul_into` first operand must be a tensor".into(),
                );
                return self.tensor_bail(fw);
            }
        };
        match self.tensor_info(bt) {
            Some((be, br)) if be == elem && br == rank => {}
            _ => {
                self.err(
                    pos,
                    "`tensor.silu_mul_into` operands must share element kind and rank".into(),
                );
                return self.tensor_bail(fw);
            }
        }
        if !self.is_float(elem) {
            self.err(
                pos,
                "`tensor.silu_mul_into` requires `float` elements".into(),
            );
            return self.tensor_bail(fw);
        }
        self.emit_shape_eq(fw, &av, &bv);
        let (am, aty) = self.emit_tensor_memref(fw, &av, elem, rank);
        let (bm, bty) = self.emit_tensor_memref(fw, &bv, elem, rank);
        let a = self.open_map(
            fw,
            &[(am.clone(), aty.clone()), (bm, bty)],
            &(am, aty),
            rank,
            elem,
        );
        let s = self.emit_silu_body(fw, &a[0]);
        let r = fw.v();
        fw.op(&format!("      {} = arith.mulf {}, {} : f64", r, s, a[1]));
        self.close_map(fw, &r, elem);
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }

    /// `tensor.rmsnorm(x, w): Tensor<float,1>` — one reduction plus a fused
    /// `x * inv * w` scale generic (no intermediate allocation)
    pub(crate) fn emit_tensor_rmsnorm(
        &mut self,
        fw: &mut FnWalk,
        xe: &Expr,
        we: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (xv, xt) = self.emit_expr(fw, xe);
        let (wv, wt) = self.emit_expr(fw, we);
        let (elem, rank) = match self.tensor_info(xt) {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.rmsnorm` first operand must be a tensor".into(),
                );
                return self.tensor_bail(fw);
            }
        };
        match self.tensor_info(wt) {
            Some((we2, wr2)) if we2 == elem && wr2 == rank => {}
            _ => {
                self.err(
                    pos,
                    "`tensor.rmsnorm` weight must share element kind and rank".into(),
                );
                return self.tensor_bail(fw);
            }
        }
        if rank != 1 || !self.is_float(elem) {
            self.err(
                pos,
                "`tensor.rmsnorm` requires rank-1 `float` tensors".into(),
            );
            return self.tensor_bail(fw);
        }
        self.emit_shape_eq(fw, &xv, &wv);
        let (am, aty) = self.emit_tensor_memref(fw, &xv, elem, 1);
        let (wm, wty) = self.emit_tensor_memref(fw, &wv, elem, 1);
        let n = self.emit_tensor_dim_index(fw, &xv, 0);
        let ni = self.emit_tensor_dim_i64(fw, &xv, 0);
        // sum of squares (reuse the dot loop as x*x)
        let zero = self.fconst(fw, 0.0);
        let ss = self.emit_reduce_loop(
            fw,
            &[(am.clone(), aty.clone()), (am.clone(), aty.clone())],
            &n,
            &zero,
            elem,
            true,
        );
        let nf = fw.v();
        fw.op(&format!("    {} = arith.sitofp {} : i64 to f64", nf, ni));
        let mean = fw.v();
        fw.op(&format!("    {} = arith.divf {}, {} : f64", mean, ss, nf));
        let eps = self.fconst(fw, 1e-5);
        let den = fw.v();
        fw.op(&format!("    {} = arith.addf {}, {} : f64", den, mean, eps));
        let root = fw.v();
        fw.op(&format!("    {} = math.sqrt {} : f64", root, den));
        let one = self.fconst(fw, 1.0);
        let inv = fw.v();
        fw.op(&format!("    {} = arith.divf {}, {} : f64", inv, one, root));
        let (ov, ot, om, oty) = self.emit_same_shape_out(fw, &xv, elem, 1);
        let a = self.open_map(fw, &[(am, aty), (wm, wty)], &(om, oty), 1, elem);
        let t = fw.v();
        fw.op(&format!("      {} = arith.mulf {}, {} : f64", t, a[0], inv));
        let r = fw.v();
        fw.op(&format!("      {} = arith.mulf {}, {} : f64", r, t, a[1]));
        self.close_map(fw, &r, elem);
        (ov, ot)
    }

    /// `tensor.softmax(x)` (fresh) / `tensor.softmax_into(x)` (in place)
    pub(crate) fn emit_tensor_softmax(
        &mut self,
        fw: &mut FnWalk,
        ae: &Expr,
        in_place: bool,
        pos: &Pos,
    ) -> (String, TyId) {
        let (av, at) = self.emit_expr(fw, ae);
        let (elem, rank) = match self.tensor_info(at) {
            Some(x) => x,
            None => {
                self.err(pos, "`tensor.softmax` operand must be a tensor".into());
                return self.tensor_bail(fw);
            }
        };
        if rank != 1 || !self.is_float(elem) {
            self.err(
                pos,
                "`tensor.softmax` requires a rank-1 `float` tensor".into(),
            );
            return self.tensor_bail(fw);
        }
        let (am, aty) = self.emit_tensor_memref(fw, &av, elem, 1);
        let n = self.emit_tensor_dim_index(fw, &av, 0);
        // pass 1: max (finite sentinel; MLIR rejects `-inf` literals)
        let neg_inf = self.fconst(fw, f64::MIN);
        let c0 = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : index", c0));
        let c1 = fw.v();
        fw.op(&format!("    {} = arith.constant 1 : index", c1));
        let mx = fw.v();
        let i1 = fw.v();
        let acc1 = fw.v();
        fw.op(&format!(
            "    {} = scf.for {} = {} to {} step {} iter_args({} = {}) -> (f64) {{",
            mx, i1, c0, n, c1, acc1, neg_inf
        ));
        let xi = fw.v();
        fw.op(&format!(
            "      {} = memref.load {}[{}] : {}",
            xi, am, i1, aty
        ));
        let cm = fw.v();
        fw.op(&format!(
            "      {} = arith.maximumf {}, {} : f64",
            cm, acc1, xi
        ));
        fw.op(&format!("      scf.yield {} : f64", cm));
        fw.op("    }");
        // output: fresh allocation or the operand itself
        let (ov, ot, om, oty) = if in_place {
            (av.clone(), at, am.clone(), aty.clone())
        } else {
            self.emit_same_shape_out(fw, &av, elem, 1)
        };
        // pass 2: exp(x-max) -> out, accumulate sum
        let zero = self.fconst(fw, 0.0);
        let sum = fw.v();
        let i2 = fw.v();
        let acc2 = fw.v();
        fw.op(&format!(
            "    {} = scf.for {} = {} to {} step {} iter_args({} = {}) -> (f64) {{",
            sum, i2, c0, n, c1, acc2, zero
        ));
        let xi2 = fw.v();
        fw.op(&format!(
            "      {} = memref.load {}[{}] : {}",
            xi2, am, i2, aty
        ));
        let dv = fw.v();
        fw.op(&format!("      {} = arith.subf {}, {} : f64", dv, xi2, mx));
        let ev = fw.v();
        fw.op(&format!("      {} = math.exp {} : f64", ev, dv));
        fw.op(&format!(
            "      memref.store {}, {}[{}] : {}",
            ev, om, i2, oty
        ));
        let sv = fw.v();
        fw.op(&format!("      {} = arith.addf {}, {} : f64", sv, acc2, ev));
        fw.op(&format!("      scf.yield {} : f64", sv));
        fw.op("    }");
        // pass 3: divide out by sum
        let i3 = fw.v();
        fw.op(&format!(
            "    scf.for {} = {} to {} step {} {{",
            i3, c0, n, c1
        ));
        let xo = fw.v();
        fw.op(&format!(
            "      {} = memref.load {}[{}] : {}",
            xo, om, i3, oty
        ));
        let qv = fw.v();
        fw.op(&format!("      {} = arith.divf {}, {} : f64", qv, xo, sum));
        fw.op(&format!(
            "      memref.store {}, {}[{}] : {}",
            qv, om, i3, oty
        ));
        fw.op("    }");
        (ov, ot)
    }

    /// `tensor.add_scaled_into(dst, src, a)`: in-place `dst += a * src`
    pub(crate) fn emit_tensor_add_scaled_into(
        &mut self,
        fw: &mut FnWalk,
        de: &Expr,
        se: &Expr,
        scale: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (dv, dt) = self.emit_expr(fw, de);
        let (sv, st) = self.emit_expr(fw, se);
        let (scv, sct) = self.emit_expr(fw, scale);
        let (elem, rank) = match self.tensor_info(dt) {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.add_scaled_into` destination must be a tensor".into(),
                );
                return self.tensor_bail(fw);
            }
        };
        match self.tensor_info(st) {
            Some((se2, sr2)) if se2 == elem && sr2 == rank => {}
            _ => {
                self.err(
                    pos,
                    "`tensor.add_scaled_into` operands must share element kind and rank".into(),
                );
                return self.tensor_bail(fw);
            }
        }
        if !self.is_float(elem) {
            self.err(
                pos,
                "`tensor.add_scaled_into` requires `float` elements".into(),
            );
            return self.tensor_bail(fw);
        }
        if !self.is_float(sct) {
            self.err(pos, "`tensor.add_scaled_into` scale must be `float`".into());
            return self.tensor_bail(fw);
        }
        self.emit_shape_eq(fw, &dv, &sv);
        let scf = emit_dec_f(fw, &scv);
        let (dm, dty) = self.emit_tensor_memref(fw, &dv, elem, rank);
        let (sm, sty) = self.emit_tensor_memref(fw, &sv, elem, rank);
        let a = self.open_map(fw, &[(sm, sty)], &(dm, dty), rank, elem);
        let m = fw.v();
        fw.op(&format!("      {} = arith.mulf {}, {} : f64", m, scf, a[0]));
        let r = fw.v();
        fw.op(&format!("      {} = arith.addf {}, {} : f64", r, a[1], m));
        self.close_map(fw, &r, elem);
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }

    /// `tensor.div_scalar_into(dst, s)`: in-place `dst /= s`
    pub(crate) fn emit_tensor_div_scalar_into(
        &mut self,
        fw: &mut FnWalk,
        de: &Expr,
        scale: &Expr,
        pos: &Pos,
    ) -> (String, TyId) {
        let (dv, dt) = self.emit_expr(fw, de);
        let (scv, sct) = self.emit_expr(fw, scale);
        let (elem, rank) = match self.tensor_info(dt) {
            Some(x) => x,
            None => {
                self.err(
                    pos,
                    "`tensor.div_scalar_into` destination must be a tensor".into(),
                );
                return self.tensor_bail(fw);
            }
        };
        if !self.is_float(elem) || !self.is_float(sct) {
            self.err(
                pos,
                "`tensor.div_scalar_into` requires `float` tensor and scalar".into(),
            );
            return self.tensor_bail(fw);
        }
        let scf = emit_dec_f(fw, &scv);
        let (dm, dty) = self.emit_tensor_memref(fw, &dv, elem, rank);
        let a = self.open_map(fw, &[(dm.clone(), dty.clone())], &(dm, dty), rank, elem);
        let r = fw.v();
        fw.op(&format!("      {} = arith.divf {}, {} : f64", r, a[0], scf));
        self.close_map(fw, &r, elem);
        let z = fw.v();
        fw.op(&format!("    {} = arith.constant 0 : i64", z));
        (z, self.r.mk(Ty::Unit))
    }
}
