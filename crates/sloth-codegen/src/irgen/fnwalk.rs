//! `FnWalk`: per-function body CFG emission state machine + id-use walkers.

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

pub(crate) struct FnWalk {
    /// current (open) basic block text
    pub(crate) cur: String,
    pub(crate) vcount: usize,
    pub(crate) scopes: Vec<HashMap<String, (String, TyId)>>,
    /// per-scope bindovable map: value true = immutable (let)
    pub(crate) imms: Vec<HashMap<String, bool>>,
    /// rc patch B: per-scope slots DECLARED here whose word is ref-shaped
    /// (pre-resolved at declare time): name -> alloca; released at pop_scope
    pub(crate) scope_decls: Vec<HashMap<String, String>>,
    /// rc patch B: statement-dangling producer temps (fresh handles still
    /// owned by the producer) — ref-typed only; released at statement end or
    /// flushed before a conditional terminator
    pub(crate) dangling: Vec<String>,
    /// loop label stack for break/continue: (break_target, continue_target)
    pub(crate) loops: Vec<(String, String)>,
    /// parallel to `loops`: scope_decls depth at loop-body entry. break/continue
    /// release the ref locals declared in the body scopes being abandoned
    /// (their pop_scope is skipped because the block is terminated)
    pub(crate) loop_bases: Vec<usize>,
    /// rc patch C: loop-var names currently borrowed (for-in slots hold
    /// borrows of container elements; overwriting them must NOT release the
    /// element the slot does not own)
    pub(crate) loopvars: Vec<String>,
    /// parallel to `loops`: the SSA word of the loop's *owned* per-iteration
    /// element (a fresh char `str` from `for c in s`, or an owned `next()`
    /// result), `None` when the element is a borrow / already dropped. A
    /// `return` abandons the frame without reaching the loop's `cont`/`brk`
    /// release, so the return face drains these instead (the scope-exit path
    /// never owns them).
    pub(crate) loop_owned_elems: Vec<Option<String>>,
    /// patch 42: words whose count transferred across a callee return edge
    /// (rc_consume): the receiver binds them as slot owners WITHOUT an
    /// extra retain; consumed on first binding (exact ownership)
    pub(crate) xfer: Vec<String>,
    pub(crate) ret: TyId,
    pub(crate) ret_alloca: String,
    pub(crate) ret_flag: String,
    /// running count of emitter basic blocks
    pub(crate) bb: usize,
    /// true if current block already ends with terminator
    pub(crate) term: bool,
    /// shared return/end block label for this function
    pub(crate) end_label: String,
    /// class this function/method body belongs to (for this/super resolution)
    pub(crate) cur_cls: Option<String>,
}

impl FnWalk {
    pub(crate) fn v(&mut self) -> String {
        let n = self.vcount;
        self.vcount += 1;
        format!("%v{}", n)
    }
}

impl FnWalk {
    /// emit instruction line(s) into current block
    pub(crate) fn op(&mut self, s: &str) {
        self.cur.push_str(s);
        self.cur.push('\n');
    }
    pub(crate) fn noterm(&self) -> bool {
        !self.term
    }
    /// unconditional jump `cf.br ^L` (closes current block)
    pub(crate) fn jump(&mut self, t: &str) {
        if self.noterm() {
            self.op(&format!("    cf.br {}", t));
        }
        self.term = true;
    }
    /// conditional jump; pending owned temps (producers + transferred call
    /// results) are flushed HERE. Mid-expression CFG splits must save/clear
    /// `dangling`/`xfer` around their branch so enclosing operands survive.
    pub(crate) fn cjump(&mut self, c: &str, t: &str, f: &str) {
        let c1 = self.v();
        let pending = std::mem::take(&mut self.dangling);
        for h in pending {
            self.op(&format!("    sloth.rc_release {} : i64", h));
        }
        let pending_x = std::mem::take(&mut self.xfer);
        for h in pending_x {
            self.op(&format!("    sloth.rc_release {} : i64", h));
        }
        // tag migration: `c` is an encoded bool word (false=0, true=enc(1)=2);
        // a trunci would read the tag bit and always yield false — compare
        // against zero instead so both 0/1 and 0/2 conventions stay correct
        let zc = self.v();
        self.op(&format!("    {} = arith.constant 0 : i64", zc));
        self.op(&format!("    {} = arith.cmpi ne, {}, {} : i64", c1, c, zc));
        self.op(&format!("    cf.cond_br {}, {}, {}", c1, t, f));
        self.term = true;
    }
    /// statement-close for owned temps: release the +1 of (a) fresh producers
    /// that were not stored anywhere and (b) transferred call results that no
    /// owner (slot/field/container/return) claimed (§5.1.1 rule 7)
    pub(crate) fn rc_flush(&mut self) {
        let pending = std::mem::take(&mut self.dangling);
        for h in pending {
            self.op(&format!("    sloth.rc_release {} : i64", h));
        }
        let pending_x = std::mem::take(&mut self.xfer);
        for h in pending_x {
            self.op(&format!("    sloth.rc_release {} : i64", h));
        }
    }
    /// consume a dangling producer whose ownership TRANSFERS out of the
    /// statement (return face): cancel its pending flush release so the
    /// +1 travels to the result instead of freeing a value still in use.
    /// Returns true when the word held a pending producer +1.
    pub(crate) fn rc_consume(&mut self, h: &str) -> bool {
        let had = self.dangling.iter().any(|x| x == h);
        self.dangling.retain(|x| x != h);
        had
    }
    /// mark a transferred returned word (patch 42): the +1 already rides in
    /// it; the receiver must not retain it again
    pub(crate) fn rc_mark_xfer(&mut self, h: &str) {
        if !self.xfer.contains(&h.to_string()) {
            self.xfer.push(h.to_string());
        }
    }
    /// bind a transferred word: extra +1 already lives in it, so take it
    /// raw (returns true when the mark was present and consumed)
    pub(crate) fn rc_take_xfer(&mut self, h: &str) -> bool {
        if self.xfer.iter().any(|x| x == h) {
            self.xfer.retain(|x| x != h);
            true
        } else {
            false
        }
    }
    /// owned per-iteration elements of every enclosing loop (innermost last),
    /// for the return face to release after the result word is transferred.
    /// Not destructive: sibling branches each need the same list.
    pub(crate) fn rc_loop_elems(&self) -> Vec<String> {
        self.loop_owned_elems
            .iter()
            .flatten()
            .cloned()
            .collect()
    }

    /// release the owned locals of the loop-body scopes about to be abandoned
    /// by a `break`/`continue` (their block pop_scope is skipped once the
    /// terminator is emitted). Scopes from the loop-body base upward are
    /// settled; the loop's own scope is left to the loop teardown.
    pub(crate) fn rc_release_loop_body(&mut self) {
        let base = match self.loop_bases.last() {
            Some(b) => *b,
            None => return,
        };
        if base >= self.scope_decls.len() {
            return;
        }
        let slots: Vec<String> = self.scope_decls[base..]
            .iter()
            .flat_map(|d| d.values().cloned())
            .collect();
        for a in slots {
            let z = self.v();
            self.op(&format!("    {} = arith.constant 0 : index", z));
            let w = self.v();
            self.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                w, a, z
            ));
            self.op(&format!("    sloth.rc_release {} : i64", w));
            self.untrack_slot(&a);
        }
    }

    /// release every owned local still in scope. A function return abandons
    /// its frame: the normal scope-exit path is skipped (the block is
    /// terminated), so the return face settles the owned slots here instead.
    /// The returned word is retained/transferred before this runs, so
    /// releasing its source slot (if any) never frees the result.
    pub(crate) fn rc_release_scope_slots(&mut self) {
        let slots: Vec<String> = self
            .scope_decls
            .iter()
            .flat_map(|d| d.values().cloned())
            .collect();
        for a in slots {
            let z = self.v();
            self.op(&format!("    {} = arith.constant 0 : index", z));
            let w = self.v();
            self.op(&format!(
                "    {} = memref.load {}[{}] : memref<1xi64>",
                w, a, z
            ));
            self.op(&format!("    sloth.rc_release {} : i64", w));
            self.untrack_slot(&a);
        }
    }

    /// start a new labelled block
    pub(crate) fn label(&mut self, name: &str) {
        self.op(&format!("  {}:", name));
        self.term = false;
    }
    /// labeled block that closes a brace-free fallthrough explicitly (for
    /// mid-loop increment labels reached by fallthrough and by continue)
    pub(crate) fn label_br(&mut self, name: &str) {
        if self.noterm() {
            self.op(&format!("    cf.br {}", name));
        }
        self.label(name);
    }
    /// fresh label name
    pub(crate) fn newlabel(&mut self, p: &str) -> String {
        self.bb += 1;
        format!("^{}_{}", p, self.bb)
    }
    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.imms.push(HashMap::new());
        self.scope_decls.push(HashMap::new());
    }
    /// scope teardown: release every ref-typed slot declared in this scope
    /// (pre-resolved into scope_decls at declare time), then pop. Params and
    /// synthetic shadow inserts never enter scope_decls => skipped correctly.
    pub(crate) fn pop_scope(&mut self) {
        if let Some(d) = self.scope_decls.pop() {
            // a terminated block means control already left this scope
            // (return/break): releases would land after the terminator and
            // break MLIR; ownership falls to the enclosing exit path then
            if self.noterm() {
                for (_n, a) in d {
                    let z = self.v();
                    self.op(&format!("    {} = arith.constant 0 : index", z));
                    let w = self.v();
                    self.op(&format!(
                        "    {} = memref.load {}[{}] : memref<1xi64>",
                        w, a, z
                    ));
                    self.op(&format!("    sloth.rc_release {} : i64", w));
                    self.untrack_slot(&a);
                }
            }
        }
        self.scopes.pop();
        self.imms.pop();
    }
    pub(crate) fn declare(&mut self, name: &str, t: TyId, fl: bool, mutable: bool) -> String {
        let a = self.declare_raw(name, t, fl);
        self.imms
            .last_mut()
            .unwrap()
            .insert(name.to_string(), !mutable);
        a
    }
    pub(crate) fn declare_raw(&mut self, _name: &str, _t: TyId, _fl: bool) -> String {
        let a = self.v();
        // tag migration: every slot holds one tagged i64 word
        self.op(&format!("    {} = memref.alloca() : memref<1xi64>", a));
        self.scopes
            .last_mut()
            .unwrap()
            .insert(_name.to_string(), (a.clone(), _t));
        a
    }
    pub(crate) fn lookup(&self, name: &str) -> Option<(String, TyId)> {
        for sc in self.scopes.iter().rev() {
            if let Some(v) = sc.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
    /// CE: register an owned-local slot so a fiber unwinding past this frame
    /// (error/cancel/abandon) can release its current word. No-op on main.
    pub(crate) fn track_slot(&mut self, a: &str) {
        let pi = self.v();
        self.op(&format!(
            "    {} = memref.extract_aligned_pointer_as_index {} : memref<1xi64> -> index",
            pi, a
        ));
        let pw = self.v();
        self.op(&format!(
            "    {} = arith.index_cast {} : index to i64",
            pw, pi
        ));
        self.op(&format!(
            "    func.call @__sloth_fiber_track({}) : (i64) -> i64",
            pw
        ));
    }

    /// CE: unregister a slot released by its owning scope/path
    pub(crate) fn untrack_slot(&mut self, a: &str) {
        let pi = self.v();
        self.op(&format!(
            "    {} = memref.extract_aligned_pointer_as_index {} : memref<1xi64> -> index",
            pi, a
        ));
        let pw = self.v();
        self.op(&format!(
            "    {} = arith.index_cast {} : index to i64",
            pw, pi
        ));
        self.op(&format!(
            "    func.call @__sloth_fiber_untrack({}) : (i64) -> i64",
            pw
        ));
    }

    pub(crate) fn assign(&mut self, name: &str, val: &str, _fl: bool) {
        if let Some((a, _t)) = self.lookup(name) {
            let z = self.v();
            self.op(&format!("    {} = arith.constant 0 : index", z));
            self.op(&format!(
                "    memref.store {}, {}[{}] : memref<1xi64>",
                val, a, z
            ));
        }
    }
}

pub(crate) fn walk_ids_stmt(
    s: &Stmt,
    push_use: &mut dyn FnMut(&String),
    decls: &mut std::collections::HashSet<String>,
) {
    match &s.node {
        StmtNode::Expr(e) => walk_ids_expr(e, push_use, decls),
        StmtNode::Let { name, init, .. } => {
            walk_ids_expr(init, push_use, decls);
            decls.insert(name.clone());
        }
        StmtNode::Assign { target, value } => {
            walk_ids_expr(value, push_use, decls);
            for seg in target {
                if let PathSeg::Name(n) = seg {
                    push_use(n);
                }
                if let PathSeg::Index(e) = seg {
                    walk_ids_expr(e, push_use, decls);
                }
            }
        }
        StmtNode::AssignOp { target, value, .. } => {
            walk_ids_expr(value, push_use, decls);
            for seg in target {
                if let PathSeg::Name(n) = seg {
                    push_use(n);
                }
                if let PathSeg::Index(e) = seg {
                    walk_ids_expr(e, push_use, decls);
                }
            }
        }
        StmtNode::While { cond, body } => {
            walk_ids_expr(cond, push_use, decls);
            walk_ids_stmt(body, push_use, decls);
        }
        StmtNode::If { cond, then_, else_ } => {
            walk_ids_expr(cond, push_use, decls);
            walk_ids_stmt(then_, push_use, decls);
            if let Some(els) = else_ {
                walk_ids_stmt(els, push_use, decls);
            }
        }
        StmtNode::For { var, iter, body } => {
            walk_ids_expr(iter, push_use, decls);
            walk_ids_stmt(body, push_use, decls);
            decls.insert(var.clone());
        }
        StmtNode::Return(Some(e)) => {
            walk_ids_expr(e, push_use, decls);
        }
        StmtNode::Return(None) | StmtNode::Break | StmtNode::Continue => {}
        StmtNode::Block(ss) => {
            for st in ss {
                walk_ids_stmt(st, push_use, decls);
            }
        }
    }
}

pub(crate) fn walk_ids_expr(
    e: &Expr,
    push_use: &mut dyn FnMut(&String),
    decls: &mut std::collections::HashSet<String>,
) {
    match &e.node {
        ExprNode::Ident(n) => push_use(n),
        ExprNode::Call { callee, args } => {
            walk_ids_expr(callee, push_use, decls);
            for a in args {
                walk_ids_expr(a, push_use, decls);
            }
        }
        ExprNode::GenCall { callee, args, .. } => {
            walk_ids_expr(callee, push_use, decls);
            for a in args {
                walk_ids_expr(a, push_use, decls);
            }
        }
        ExprNode::Map(pairs) => {
            for (k, v) in pairs {
                walk_ids_expr(k, push_use, decls);
                walk_ids_expr(v, push_use, decls);
            }
        }
        ExprNode::Range { low, high, .. } => {
            walk_ids_expr(low, push_use, decls);
            walk_ids_expr(high, push_use, decls);
        }
        ExprNode::Pipe { lhs, rhs } | ExprNode::Elvis { lhs, rhs } => {
            walk_ids_expr(lhs, push_use, decls);
            walk_ids_expr(rhs, push_use, decls);
        }
        ExprNode::Is { lhs, rhs, .. } => {
            walk_ids_expr(lhs, push_use, decls);
            walk_ids_expr(rhs, push_use, decls);
        }
        ExprNode::Field { obj, .. } => walk_ids_expr(obj, push_use, decls),
        ExprNode::Index { obj, idx } => {
            walk_ids_expr(obj, push_use, decls);
            walk_ids_expr(idx, push_use, decls);
        }
        ExprNode::Arith { op: _, lhs, rhs } | ExprNode::Bin { op: _, lhs, rhs } => {
            walk_ids_expr(lhs, push_use, decls);
            walk_ids_expr(rhs, push_use, decls);
        }
        ExprNode::Un { expr, .. } => walk_ids_expr(expr, push_use, decls),
        ExprNode::Str(sp) => {
            // interpolation `${expr}` parts are uses of the enclosing scope
            for p in &sp.parts {
                if let sloth_frontend::lexer::StrPart::ExprAst(e) = p {
                    walk_ids_expr(e, push_use, decls);
                }
            }
        }
        ExprNode::List(xs) => {
            for x in xs {
                walk_ids_expr(x, push_use, decls);
            }
        }
        // a nested lambda is transparent to capture analysis: identifiers it
        // uses that are not bound by it (or an enclosing local) are free in
        // the enclosing lambda too, so they must be captured transitively
        ExprNode::Lambda(l) => {
            let mut nested = decls.clone();
            for p in &l.params {
                nested.insert(p.name.clone());
            }
            let mut all: Vec<String> = Vec::new();
            {
                let mut collect = |n: &String| all.push(n.clone());
                walk_ids_stmt(&l.body, &mut collect, &mut nested);
            }
            for n in all {
                if !nested.contains(&n) {
                    push_use(&n);
                }
            }
        }
        ExprNode::Int(_)
        | ExprNode::UInt(_)
        | ExprNode::Float(_)
        | ExprNode::Bool(_)
        | ExprNode::Nil
        | ExprNode::This
        | ExprNode::Super => {}
    }
}

pub(crate) fn fresh_walk(me: &mut ModEmitter) -> FnWalk {
    FnWalk {
        cur: String::new(),
        vcount: 1000,
        scopes: vec![HashMap::new()],
        imms: vec![HashMap::new()],
        scope_decls: vec![HashMap::new()],
        dangling: Vec::new(),
        loops: Vec::new(),
        loop_bases: Vec::new(),
        ret: me.r.mk(Ty::Unit),
        ret_alloca: String::new(),
        ret_flag: String::new(),
        bb: 0,
        term: false,
        end_label: "^ginit".to_string(),
        cur_cls: None,
        loopvars: Vec::new(),
        loop_owned_elems: Vec::new(),
        xfer: Vec::new(),
    }
}

impl FnWalk {
    /// true when the outermost binding of `name` is immutable (let)
    pub(crate) fn imm_of(&self, name: &str) -> bool {
        for m in self.imms.iter().rev() {
            if let Some(v) = m.get(name) {
                return *v;
            }
        }
        false
    }
}
