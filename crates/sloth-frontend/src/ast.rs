//! sloth2 AST.

use crate::lexer::{Pos, StrParts};

pub type TypeId = usize;

pub fn eof_pos() -> Pos {
    Pos {
        line: usize::MAX,
        col: 0,
    }
}

pub fn tok_str(t: &crate::lexer::Tok) -> String {
    format!("{:?}", t)
}

/// Source-annotated program.
#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub decls: Vec<Decl>,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: String,
    pub alias: Option<String>,
    pub pos: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeclKind {
    Var,
    Let,
    Func,
    Class,
    Trait,
    ExternType,
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub kind: DeclKind,
    pub name: String,
    pub pos: Pos,
    pub visible: bool,
    pub node: DeclNode,
}

#[derive(Debug, Clone)]
pub enum DeclNode {
    Var {
        ty: Option<Type>,
        init: Expr,
    },
    Func(Box<FuncDef>),
    Class(Box<ClassDef>),
    Trait(Box<TraitDef>),
    /// extern type: opaque C-ABI reference word (no ctor/fields/methods)
    ExternType,
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub type_params: Vec<TypeParam>,
    pub params: Vec<Param>,
    pub variadic: Option<Variadic>,
    pub ret: Option<Type>,
    pub body: Box<Stmt>,
    /// extern func: C-ABI external declaration, no body
    pub is_extern: bool,
}

#[derive(Debug, Clone)]
pub struct Variadic {
    pub name: String,
    pub elem: Type,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct TypeParam {
    pub name: String,
    pub bound: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub superclass: Option<String>,
    pub impls: Vec<String>,
    pub type_params: Vec<TypeParam>,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<MethodDef>,
}

#[derive(Debug, Clone)]
pub struct MethodDef {
    pub name: String,
    pub visible: bool,
    pub fd: FuncDef,
}

#[derive(Debug, Clone)]
pub struct FieldDecl {
    pub mutable: bool,
    pub visible: bool,
    pub name: String,
    pub ty: Type,
    pub init: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub methods: Vec<MethodSig>,
}

#[derive(Debug, Clone)]
pub struct MethodSig {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Type,
    /// default method body (optional)
    pub body: Option<Box<Stmt>>,
}

// ---------------- Types ----------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    Bool,
    Int,
    Float,
    Str,
    Range,
}

#[derive(Debug, Clone)]
pub enum Type {
    /// primitive / built-in / type-alias / named class
    Simple(SimpleType),
    Optional(Box<Type>),
    Unit,
}

impl Type {
    pub fn prim(p: Prim) -> Type {
        Type::Simple(match p {
            Prim::Bool => SimpleType::Bool,
            Prim::Int => SimpleType::Int,
            Prim::Float => SimpleType::Float,
            Prim::Str => SimpleType::Str,
            Prim::Range => SimpleType::Range,
        })
    }
}

#[derive(Debug, Clone)]
pub enum SimpleType {
    Bool,
    Int,
    Float,
    Str,
    Range,
    /// fixed-width integer (`uint`/`int32`/`uint8`/…)
    FixedInt(crate::ty::IntKind),
    Any,
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    /// tensor extension TE-P1: `Tensor<T, R>` — element type + static rank
    /// (const-generic-lite; the rank is a non-negative integer literal)
    Tensor(Box<Type>, u32),
    Fn(Box<FnType>),
    Named(String, Vec<Type>), // user type with optional generic args
    Ident(String),
    Dyn(String),
}

#[derive(Debug, Clone)]
pub struct FnType {
    pub params: Vec<Type>,
    pub ret: Type,
}

pub fn tstr_like(t: &crate::lexer::Tok) -> String {
    format!("{:?}", t)
}

// ---------------- Expressions ----------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Gt,
    Lt,
    Ge,
    Le,
    EqEq,
    NotEq,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    /// int-only bitwise/shift operators (design §3.3)
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Not,
    Neg,
    BitNot,
}

#[derive(Debug, Clone)]
pub struct Expr {
    /// stable node id, assigned by `assign_ids` after parsing (0 = synthetic /
    /// unassigned; ignored by the type side table)
    pub id: u32,
    pub node: ExprNode,
    pub pos: Pos,
}

#[derive(Debug, Clone)]
pub enum ExprNode {
    Int(i64),
    /// unsigned integer literal (spelled `123u` or a decimal > `i64::MAX`)
    UInt(u64),
    Float(f64),
    Bool(bool),
    Nil,
    Str(StrParts),
    Ident(String),
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    /// explicit generic call `f<A,B>(args)`
    GenCall {
        callee: Box<Expr>,
        targs: Vec<Type>,
        args: Vec<Expr>,
    },
    Index {
        obj: Box<Expr>,
        idx: Box<Expr>,
    },
    Field {
        obj: Box<Expr>,
        name: String,
    },
    Arith {
        op: ArithOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Bin {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Un {
        op: UnOp,
        expr: Box<Expr>,
    },
    Lambda(Box<Lambda>),
    Range {
        low: Box<Expr>,
        high: Box<Expr>,
        inclusive: bool,
    },
    Pipe {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Elvis {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Is {
        negated: bool,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    This,
    Super,
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Box<Stmt>,
}

// ---------------- Statements ----------------

#[derive(Debug, Clone)]
pub struct Stmt {
    /// stable node id, assigned by `assign_ids` after parsing (0 = synthetic /
    /// unassigned; ignored by the type side table)
    pub id: u32,
    pub node: StmtNode,
    pub pos: Pos,
}

#[derive(Debug, Clone)]
pub enum StmtNode {
    Expr(Expr),
    Let {
        mutable: bool,
        name: String,
        ty: Option<Type>,
        init: Expr,
    },
    Assign {
        target: Vec<PathSeg>,
        value: Expr,
    },
    /// compound assignment `target += value` / `target -= value`
    AssignOp {
        target: Vec<PathSeg>,
        op: ArithOp,
        value: Expr,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    If {
        cond: Expr,
        then_: Box<Stmt>,
        else_: Option<Box<Stmt>>,
    },
    For {
        var: String,
        iter: Expr,
        body: Box<Stmt>,
    },
    Break,
    Continue,
    Return(Option<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum PathSeg {
    Name(String),
    Index(Expr),
}

// ---------------- Node id assignment ----------------

/// Assign a unique id to every `Expr`/`Stmt` node reachable from `prog`
/// (pre-order). The semantic pass records expression types in a side table
/// keyed by these ids; the emitter consults it. Synthetic nodes created during
/// emission keep id 0 and are ignored.
pub fn assign_ids(prog: &mut Program) {
    let mut c = IdAssigner { next: 1 };
    for s in &mut prog.stmts {
        c.stmt(s);
    }
    for d in &mut prog.decls {
        match &mut d.node {
            DeclNode::Var { init, .. } => c.expr(init),
            DeclNode::Func(f) => c.stmt(&mut f.body),
            DeclNode::Class(cl) => {
                for fd in &mut cl.fields {
                    if let Some(init) = &mut fd.init {
                        c.expr(init);
                    }
                }
                for m in &mut cl.methods {
                    c.stmt(&mut m.fd.body);
                }
            }
            DeclNode::Trait(t) => {
                for m in &mut t.methods {
                    if let Some(b) = &mut m.body {
                        c.stmt(b);
                    }
                }
            }
            DeclNode::ExternType => {}
        }
    }
}

struct IdAssigner {
    next: u32,
}

impl IdAssigner {
    fn fresh(&mut self, slot: &mut u32) {
        *slot = self.next;
        self.next += 1;
    }
    fn stmt(&mut self, s: &mut Stmt) {
        self.fresh(&mut s.id);
        match &mut s.node {
            StmtNode::Expr(e) => self.expr(e),
            StmtNode::Let { init, .. } => self.expr(init),
            StmtNode::Assign { target, value } => {
                self.path(target);
                self.expr(value);
            }
            StmtNode::AssignOp { target, value, .. } => {
                self.path(target);
                self.expr(value);
            }
            StmtNode::While { cond, body } => {
                self.expr(cond);
                self.stmt(body);
            }
            StmtNode::If { cond, then_, else_ } => {
                self.expr(cond);
                self.stmt(then_);
                if let Some(e) = else_ {
                    self.stmt(e);
                }
            }
            StmtNode::For { iter, body, .. } => {
                self.expr(iter);
                self.stmt(body);
            }
            StmtNode::Break | StmtNode::Continue => {}
            StmtNode::Return(Some(e)) => self.expr(e),
            StmtNode::Return(None) => {}
            StmtNode::Block(ss) => {
                for st in ss {
                    self.stmt(st);
                }
            }
        }
    }
    fn path(&mut self, segs: &mut [PathSeg]) {
        for seg in segs {
            if let PathSeg::Index(e) = seg {
                self.expr(e);
            }
        }
    }
    fn expr(&mut self, e: &mut Expr) {
        self.fresh(&mut e.id);
        match &mut e.node {
            ExprNode::Int(_)
            | ExprNode::UInt(_)
            | ExprNode::Float(_)
            | ExprNode::Bool(_)
            | ExprNode::Nil
            | ExprNode::Ident(_)
            | ExprNode::This
            | ExprNode::Super => {}
            ExprNode::Str(sp) => {
                for p in &mut sp.parts {
                    if let crate::lexer::StrPart::ExprAst(x) = p {
                        self.expr(x);
                    }
                }
            }
            ExprNode::List(xs) => {
                for x in xs {
                    self.expr(x);
                }
            }
            ExprNode::Map(pairs) => {
                for (k, v) in pairs {
                    self.expr(k);
                    self.expr(v);
                }
            }
            ExprNode::Call { callee, args } => {
                self.expr(callee);
                for a in args {
                    self.expr(a);
                }
            }
            ExprNode::GenCall {
                callee,
                targs: _,
                args,
            } => {
                self.expr(callee);
                for a in args {
                    self.expr(a);
                }
            }
            ExprNode::Index { obj, idx } => {
                self.expr(obj);
                self.expr(idx);
            }
            ExprNode::Field { obj, .. } => self.expr(obj),
            ExprNode::Arith { lhs, rhs, .. } | ExprNode::Bin { lhs, rhs, .. } => {
                self.expr(lhs);
                self.expr(rhs);
            }
            ExprNode::Un { expr, .. } => self.expr(expr),
            ExprNode::Lambda(l) => {
                self.stmt(&mut l.body);
            }
            ExprNode::Range { low, high, .. } => {
                self.expr(low);
                self.expr(high);
            }
            ExprNode::Pipe { lhs, rhs } | ExprNode::Elvis { lhs, rhs } => {
                self.expr(lhs);
                self.expr(rhs);
            }
            ExprNode::Is { lhs, rhs, .. } => {
                self.expr(lhs);
                self.expr(rhs);
            }
        }
    }
}
