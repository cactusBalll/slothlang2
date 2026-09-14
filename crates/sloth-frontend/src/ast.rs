//! sloth2 AST.

use crate::lexer::{Pos, StrParts};

pub type TypeId = usize;

pub fn eof_pos() -> Pos {
    Pos { line: usize::MAX, col: 0 }
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
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub type_params: Vec<TypeParam>,
    pub params: Vec<Param>,
    pub variadic: Option<Variadic>,
    pub ret: Option<Type>,
    pub body: Box<Stmt>,
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
    pub fd: FuncDef,
}

#[derive(Debug, Clone)]
pub struct FieldDecl {
    pub mutable: bool,
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
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub node: ExprNode,
    pub pos: Pos,

}

#[derive(Debug, Clone)]
pub enum ExprNode {
    Int(i64),
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
