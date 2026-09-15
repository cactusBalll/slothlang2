//! Recursive-descent + Pratt parser for sloth2 (§3.9 EBNF).

use crate::ast::*;
use crate::lexer::{Lexer, Pos, StrPart, StrParts, Tok, Token};

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub pos: Pos,
}

type PResult<T> = Result<T, ParseError>;

pub fn parse(src: &str) -> PResult<Program> {
    let toks = Lexer::new(src).scan().map_err(|e| ParseError {
        msg: format!("lex error: {}", e.msg),
        pos: e.pos,
    })?;
    Parser { toks, ptr: 0 }.program()
}

struct Parser {
    toks: Vec<Token>,
    ptr: usize,
}

// binding powers: tighter binds higher
const P_PIPE: u8 = 1;
const P_ELVIS: u8 = 2;
const P_OR: u8 = 3;
const P_AND: u8 = 4;
const P_CMP: u8 = 5;
const P_RANGE: u8 = 6;
const P_ADD: u8 = 7;
const P_MUL: u8 = 8;
const P_UNARY: u8 = 9;

impl Parser {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.ptr).map(|t| &t.tok)
    }
    fn peek_at(&self, n: usize) -> Option<&Tok> {
        self.toks.get(self.ptr + n).map(|t| &t.tok)
    }
    fn pos(&self) -> Pos {
        self.toks
            .get(self.ptr)
            .map(|t| t.pos.clone())
            .unwrap_or_else(eof_pos)
    }
    fn advance(&mut self) -> Option<Token> {
        let t = self.toks.get(self.ptr).cloned();
        if t.is_some() {
            self.ptr += 1;
        }
        t
    }
    fn is_kw(&self, kw: &str) -> bool {
        matches!(self.peek(), Some(Tok::Ident(s)) if s == kw)
    }
    fn is_kw_at(&self, n: usize, kw: &str) -> bool {
        matches!(self.peek_at(n), Some(Tok::Ident(s)) if s == kw)
    }
    fn eat(&mut self, t: Tok) -> bool {
        if self.peek() == Some(&t) {
            self.ptr += 1;
            true
        } else {
            false
        }
    }
    fn eat_kw(&mut self, kw: &str) -> bool {
        if self.is_kw(kw) {
            self.ptr += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, t: Tok, what: &str) -> PResult<()> {
        if self.eat(t) {
            Ok(())
        } else {
            Err(self.err_want(what))
        }
    }
    fn expect_kw(&mut self, kw: &str) -> PResult<()> {
        if self.eat_kw(kw) {
            Ok(())
        } else {
            Err(self.err_want(kw))
        }
    }
    fn err_want(&self, what: &str) -> ParseError {
        ParseError {
            msg: format!("expected {}", what),
            pos: self.pos(),
        }
    }
    fn err(&self, msg: &str) -> ParseError {
        ParseError {
            msg: msg.to_string(),
            pos: self.pos(),
        }
    }
    fn ident(&mut self, what: &str) -> PResult<(String, Pos)> {
        let pos = self.pos();
        match self.advance() {
            Some(Token { tok: Tok::Ident(s), .. }) => Ok((s, pos)),
            _ => Err(self.err_want(what)),
        }
    }
    fn eof(&self) -> bool {
        self.ptr >= self.toks.len()
    }

    // ---------------- program ----------------

    pub fn program(&mut self) -> PResult<Program> {
        let mut imports = Vec::new();
        let mut decls = Vec::new();
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            if self.is_kw("import") {
                imports.push(self.import_decl()?);
            } else if self.looks_like_decl() {
                decls.push(self.decl()?);
            } else {
                stmts.push(self.stmt()?);
            }
        }
        Ok(Program { imports, decls, stmts })
    }

    /// heuristic: these keyword starts begin toplevel declarations
    fn looks_like_decl(&self) -> bool {
        if self.is_kw("pub") {
            // pub var/let/func/class/trait
            return matches!(
                self.peek_at(1),
                Some(Tok::Ident(s)) if s == "var" || s == "let" || s == "func" || s == "class" || s == "trait"
            );
        }
        matches!(
            self.peek(),
            Some(Tok::Ident(s)) if s == "func" || s == "class" || s == "trait" || s == "var" || s == "let"
        )
    }

    fn import_decl(&mut self) -> PResult<Import> {
        let pos = self.pos();
        self.expect_kw("import")?;
        let path = match self.advance() {
            Some(Token { tok: Tok::Str(s), .. }) => match s.plain() {
                Some(p) => p.to_string(),
                None => return Err(self.err("interpolation not allowed in import path")),
            },
            _ => return Err(self.err_want("import path string")),
        };
        let mut alias = None;
        if self.eat_kw("as") {
            alias = Some(self.ident("module alias")?.0);
        }
        self.expect(Tok::Semi, "';' after import")?;
        Ok(Import { path, alias, pos })
    }

    fn decl(&mut self) -> PResult<Decl> {
        let visible = self.eat_kw("pub");
        match self.peek().cloned() {
            Some(Tok::Ident(s)) if s == "func" => {
                self.ptr += 1;
                let (name, pos, f) = self.func_after_kw()?;
                Ok(Decl { kind: DeclKind::Func, name, pos, visible, node: DeclNode::Func(Box::new(f)) })
            }
            Some(Tok::Ident(s)) if s == "var" || s == "let" => self.var_let_decl(visible),
            Some(Tok::Ident(s)) if s == "class" => self.class_decl(visible),
            Some(Tok::Ident(s)) if s == "trait" => self.trait_decl(visible),
            Some(other) => Err(ParseError {
                msg: format!(
                    "unexpected token in toplevel declaration: {}",
                    crate::ast::tstr_like(&other)
                ),
                pos: self.pos(),
            }),
            None => Err(self.err("unexpected end of file")),
        }
    }

    fn var_let_decl(&mut self, visible: bool) -> PResult<Decl> {
        let pos = self.pos();
        let kind = match self.advance() {
            Some(Token { tok: Tok::Ident(s), .. }) if s == "var" => DeclKind::Var,
            Some(Token { tok: Tok::Ident(s), .. }) if s == "let" => DeclKind::Let,
            _ => unreachable!(),
        };
        let (name, _) = self.ident("variable name")?;
        let ty = if self.eat(Tok::Colon) { Some(self.ty()?) } else { None };
        self.expect(Tok::Assign, "'=' in declaration")?;
        let init = self.expr(0)?;
        self.expect(Tok::Semi, "';'")?;
        Ok(Decl { kind, name, pos, visible, node: DeclNode::Var { ty, init } })
    }

    fn type_params(&mut self) -> PResult<Vec<TypeParam>> {
        if !self.eat(Tok::Lt) {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        while self.peek().is_some() && !matches!(self.peek(), Some(Tok::Gt)) {
            let (name, _) = self.ident("type parameter")?;
            let bound = if self.eat(Tok::Colon) {
                Some(self.ident("trait bound")?.0)
            } else {
                None
            };
            out.push(TypeParam { name, bound });
            if !self.eat(Tok::Comma) {
                break;
            }
        }
        self.expect(Tok::Gt, "'>' closing type parameter list")?;
        Ok(out)
    }

    fn params(&mut self) -> PResult<(Vec<Param>, Option<Variadic>)> {
        self.expect(Tok::LParen, "'('")?;
        let mut out = Vec::new();
        let mut variadic = None;
        while !matches!(self.peek(), Some(Tok::RParen)) {
            if self.peek().is_none() {
                return Err(self.err("unexpected EOF in parameter list"));
            }
            let (name, _) = self.ident("parameter name")?;
            if self.eat(Tok::Variadic) {
                self.expect(Tok::Colon, "':' after variadic parameter")?;
                self.expect_kw("Array")?;
                self.expect(Tok::Lt, "'<'")?;
                let elem = self.ty()?;
                self.expect(Tok::Gt, "'>'")?;
                variadic = Some(Variadic { name, elem });
                if self.eat(Tok::Comma) {
                    continue;
                }
                break;
            }
            let ty = if self.eat(Tok::Colon) { Some(self.ty()?) } else { None };
            out.push(Param { name, ty });
            if !self.eat(Tok::Comma) {
                break;
            }
        }
        self.expect(Tok::RParen, "')'")?;
        Ok((out, variadic))
    }

    // ---------------- types ----------------

    fn ty(&mut self) -> PResult<Type> {
        let base = self.ty_base()?;
        if self.eat(Tok::Question) {
            if matches!(base, Type::Optional(_)) {
                return Err(self.err("nested optional T?? is not allowed"));
            }
            Ok(Type::Optional(Box::new(base)))
        } else {
            Ok(base)
        }
    }

    fn ty_base(&mut self) -> PResult<Type> {
        if self.eat(Tok::LParen) {
            let mut ps = Vec::new();
            if !self.eat(Tok::RParen) {
                loop {
                    ps.push(self.ty()?);
                    if !self.eat(Tok::Comma) {
                        break;
                    }
                }
                self.expect(Tok::RParen, "')'")?;
            }
            self.expect(Tok::Arrow, "'->' in function type")?;
            let ret = self.ty()?;
            return Ok(Type::Simple(SimpleType::Fn(Box::new(FnType { params: ps, ret }))));
        }
        let (name, _) = match self.peek().cloned() {
            Some(Tok::Ident(n)) => {
                self.ptr += 1;
                (n, self.pos())
            }
            _ => return Err(self.err_want("type")),
        };
        let t = match name.as_str() {
            "unit" => Type::Unit,
            "int" | "i64" => Type::prim(Prim::Int),
            "float" | "f64" => Type::prim(Prim::Float),
            "bool" => Type::prim(Prim::Bool),
            "str" => Type::prim(Prim::Str),
            "range" => Type::prim(Prim::Range),
            "Array" => {
                self.expect(Tok::Lt, "'<'")?;
                let el = self.ty()?;
                self.expect(Tok::Gt, "'>'")?;
                Type::Simple(SimpleType::Array(Box::new(el)))
            }
            "Map" => {
                self.expect(Tok::Lt, "'<'")?;
                let k = self.ty()?;
                self.expect(Tok::Comma, "','")?;
                let v = self.ty()?;
                self.expect(Tok::Gt, "'>'")?;
                Type::Simple(SimpleType::Map(Box::new(k), Box::new(v)))
            }
            "dyn" => {
                let tr = self.ident("trait name after dyn")?.0;
                Type::Simple(SimpleType::Dyn(tr))
            }
            rest => {
                if self.eat(Tok::Lt) {
                    let mut args = Vec::new();
                    loop {
                        args.push(self.ty()?);
                        if !self.eat(Tok::Comma) {
                            break;
                        }
                    }
                    self.expect(Tok::Gt, "'>'")?;
                    Type::Simple(SimpleType::Named(rest.to_string(), args))
                } else {
                    Type::Simple(SimpleType::Ident(rest.to_string()))
                }
            }
        };
        Ok(t)
    }

    // ---------------- functions/classes/traits ----------------

    /// after `func` keyword consumed
    fn func_after_kw(&mut self) -> PResult<(String, Pos, FuncDef)> {
        let pos = self.pos();
        let (name, _) = self.ident("function name")?;
        let type_params = self.type_params()?;
        let (params, variadic) = self.params()?;
        let ret = if self.eat(Tok::Arrow) || self.eat(Tok::Colon) {
            if self.eat_kw("var") {
                // arrow into var decl: ignore type
                let _ = self.eat(Tok::Colon);
            }
            let r = self.ty()?;
            Some(r)
        } else {
            None
        };
        let body = self.block()?;
        Ok((
            name,
            pos,
            FuncDef { type_params, params, variadic, ret, body: Box::new(body) },
        ))
    }

    fn class_decl(&mut self, visible: bool) -> PResult<Decl> {
        let pos = self.pos();
        self.expect_kw("class")?;
        let (name, _) = self.ident("class name")?;
        let type_params = self.type_params()?;
        let superclass = if self.eat(Tok::Colon) {
            Some(self.ident("superclass")?.0)
        } else {
            None
        };
        let mut impls = Vec::new();
        if self.eat_kw("impl") {
            loop {
                impls.push(self.ident("trait name")?.0);
                if !self.eat(Tok::Comma) {
                    break;
                }
            }
        }
        self.expect(Tok::LBrace, "'{' to begin class body")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        while !matches!(self.peek(), Some(Tok::RBrace)) {
            if self.peek().is_none() {
                return Err(self.err("unexpected EOF in class body"));
            }
            let _ = self.eat_kw("pub");
            if self.is_kw("var") || self.is_kw("let") {
                let mutable = self.is_kw("var");
                self.ptr += 1;
                let (fname, _) = self.ident("field name")?;
                self.expect(Tok::Colon, "':' after field name (fields require types)")?;
                let fty = self.ty()?;
                let finit = if self.eat(Tok::Assign) {
                    Some(self.expr(0)?)
                } else {
                    None
                };
                self.expect(Tok::Semi, "';' after field")?;
                fields.push(FieldDecl { mutable, name: fname, ty: fty, init: finit });
            } else if self.is_kw("func") {
                self.ptr += 1;
                let (mname, _, f) = self.func_after_kw()?;
                methods.push(MethodDef { name: mname, fd: f });
            } else {
                return Err(self.err("expected field or method in class body"));
            }
        }
        self.ptr += 1; // eat }
        Ok(Decl {
            kind: DeclKind::Class,
            name,
            pos,
            visible,
            node: DeclNode::Class(Box::new(ClassDef {
                superclass,
                impls,
                type_params,
                fields,
                methods,
            })),
        })
    }

    fn trait_decl(&mut self, visible: bool) -> PResult<Decl> {
        let pos = self.pos();
        self.expect_kw("trait")?;
        let (name, _) = self.ident("trait name")?;
        self.expect(Tok::LBrace, "'{' to begin trait body")?;
        let mut methods = Vec::new();
        while !matches!(self.peek(), Some(Tok::RBrace)) {
            if self.peek().is_none() {
                return Err(self.err("unexpected EOF in trait body"));
            }
            self.expect_kw("func")?;
            let (mname, _) = self.ident("method name")?;
            let (params, variadic) = self.params()?;
            if variadic.is_some() {
                return Err(self.err("variadic in trait method is unsupported"));
            }
            self.expect(Tok::Colon, "':' return type")?;
            let ret = self.ty()?;
            self.expect(Tok::Semi, "';'")?;
            methods.push(MethodSig { name: mname, params, ret });
        }
        self.ptr += 1; // eat }
        Ok(Decl {
            kind: DeclKind::Trait,
            name,
            pos,
            visible,
            node: DeclNode::Trait(Box::new(TraitDef { methods })),
        })
    }

    // ---------------- statements ----------------

    pub fn block(&mut self) -> PResult<Stmt> {
        let pos = self.pos();
        self.expect(Tok::LBrace, "'{'")?;
        let mut out = Vec::new();
        while !matches!(self.peek(), Some(Tok::RBrace)) {
            if self.peek().is_none() {
                return Err(self.err("unexpected EOF in block"));
            }
            out.push(self.stmt()?);
        }
        self.ptr += 1;
        Ok(Stmt { node: StmtNode::Block(out), pos })
    }

    pub fn stmt(&mut self) -> PResult<Stmt> {
        let pos = self.pos();
        if self.is_kw("var") || self.is_kw("let") {
            let mutable = self.is_kw("var");
            self.ptr += 1;
            let (name, _) = self.ident("variable name")?;
            let ty = if self.eat(Tok::Colon) { Some(self.ty()?) } else { None };
            self.expect(Tok::Assign, "'=' in declaration")?;
            let init = self.expr(0)?;
            self.expect(Tok::Semi, "';'")?;
            return Ok(Stmt { node: StmtNode::Let { mutable, name, ty, init }, pos });
        }
        match self.peek().cloned() {
            Some(Tok::LBrace) => self.block(),
            Some(Tok::Ident(s)) if s == "if" => self.if_stmt(),
            Some(Tok::Ident(s)) if s == "while" => self.while_stmt(),
            Some(Tok::Ident(s)) if s == "for" => self.for_stmt(),
            Some(Tok::Ident(s)) if s == "return" => {
                self.ptr += 1;
                let e = if matches!(self.peek(), Some(Tok::Semi)) || self.peek().is_none() {
                    None
                } else {
                    Some(self.expr(0)?)
                };
                if self.peek().is_some() {
                    self.expect(Tok::Semi, "';'")?;
                }
                Ok(Stmt { node: StmtNode::Return(e), pos })
            }
            Some(Tok::Ident(s)) if s == "break" => {
                self.ptr += 1;
                self.expect(Tok::Semi, "';'")?;
                Ok(Stmt { node: StmtNode::Break, pos })
            }
            Some(Tok::Ident(s)) if s == "continue" => {
                self.ptr += 1;
                self.expect(Tok::Semi, "';'")?;
                Ok(Stmt { node: StmtNode::Continue, pos })
            }
            _ => {
                let e = self.expr(0)?;
                if self.eat(Tok::Assign) {
                    let value = self.expr(0)?;
                    self.expect(Tok::Semi, "';'")?;
                    let target =
                        expr_to_path(&e).ok_or_else(|| self.err("invalid assignment target"))?;
                    Ok(Stmt { node: StmtNode::Assign { target, value }, pos })
                } else {
                    self.expect(Tok::Semi, "';'")?;
                    Ok(Stmt { node: StmtNode::Expr(e), pos })
                }
            }
        }
    }

    fn if_stmt(&mut self) -> PResult<Stmt> {
        let pos = self.pos();
        self.expect_kw("if")?;
        if self.eat(Tok::LParen) {
            let cond = self.expr(0)?;
            self.expect(Tok::RParen, "')'")?;
            let then_ = Box::new(self.stmt()?);
            let else_ = if self.eat_kw("else") {
                Some(Box::new(self.stmt()?))
            } else {
                None
            };
            return Ok(Stmt { node: StmtNode::If { cond, then_, else_ }, pos });
        }
        let cond = self.expr(0)?;
        let then_ = Box::new(self.stmt()?);
        let else_ = if self.eat_kw("else") {
            Some(Box::new(self.stmt()?))
        } else {
            None
        };
        Ok(Stmt { node: StmtNode::If { cond, then_, else_ }, pos })
    }

    fn while_stmt(&mut self) -> PResult<Stmt> {
        let pos = self.pos();
        self.expect_kw("while")?;
        if let Some(Tok::LParen) = self.peek() {
            let _ = self.eat(Tok::LParen);
            let cond = self.expr(0)?;
            self.expect(Tok::RParen, "')'")?;
            let body = Box::new(self.stmt()?);
            return Ok(Stmt { node: StmtNode::While { cond, body }, pos });
        }
        let cond = self.expr(0)?;
        let body = Box::new(self.stmt()?);
        Ok(Stmt { node: StmtNode::While { cond, body }, pos })
    }

    fn for_stmt(&mut self) -> PResult<Stmt> {
        let pos = self.pos();
        self.expect_kw("for")?;
        // form A: for (var x: T in it) {...}
        if self.eat(Tok::LParen) {
            self.expect_kw("var")?;
            let (var, _) = self.ident("iterator variable")?;
            self.expect(Tok::Colon, "':' after for variable")?;
            let iter = self.expr(0)?;
            self.expect(Tok::RParen, "')'")?;
            let body = Box::new(self.stmt()?);
            return Ok(Stmt { node: StmtNode::For { var, iter, body }, pos });
        }
        // form B: for x in it {...}
        let (var, _) = self.ident("iterator variable")?;
        self.expect_kw("in")?;
        let iter = self.expr(0)?;
        let body = Box::new(self.stmt()?);
        Ok(Stmt { node: StmtNode::For { var, iter, body }, pos })
    }

    // ---------------- expressions ----------------

    pub fn expr(&mut self, min_bp: u8) -> PResult<Expr> {
        let mut lhs = self.unary()?;
        loop {
            let pos = self.pos();
            let Some(t) = self.peek().cloned() else { break };
            if let Tok::Ident(kw) = &t {
                match kw.as_str() {
                    "is" => {
                        if P_CMP < min_bp {
                            break;
                        }
                        let is_not = self.is_kw_at(1, "not");
                        self.ptr += if is_not { 2 } else { 1 };
                        let rhs = self.expr(P_CMP + 1)?;
                        lhs = Expr {
                            pos,
                            node: ExprNode::Is {
                                negated: is_not,
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                            },
                        };
                        continue;
                    }
                    "and" => {
                        if P_AND < min_bp {
                            break;
                        }
                        self.ptr += 1;
                        let rhs = self.expr(P_AND + 1)?;
                        lhs = bin_expr(pos, BinOp::And, lhs, rhs);
                        continue;
                    }
                    "or" => {
                        if P_OR < min_bp {
                            break;
                        }
                        self.ptr += 1;
                        let rhs = self.expr(P_OR + 1)?;
                        lhs = bin_expr(pos, BinOp::Or, lhs, rhs);
                        continue;
                    }
                    _ => break,
                }
            }
            use ArithOp as A;
            if let Some((op, bp)) = match &t {
                Tok::Plus => Some((A::Add, P_ADD)),
                Tok::Minus => Some((A::Sub, P_ADD)),
                Tok::Star => Some((A::Mul, P_MUL)),
                Tok::Slash => Some((A::Div, P_MUL)),
                Tok::Percent => Some((A::Mod, P_MUL)),
                _ => None,
            } {
                if bp < min_bp {
                    break;
                }
                self.ptr += 1;
                let rhs = self.expr(bp + 1)?;
                lhs = Expr {
                    pos,
                    node: ExprNode::Arith {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                };
                continue;
            }
            // range operators .. / ..=
            let range_inc: Option<bool> = match &t {
                Tok::Range => Some(false),
                Tok::RangeInc => Some(true),
                _ => None,
            };
            if let Some(inclusive) = range_inc {
                if P_RANGE < min_bp {
                    break;
                }
                self.ptr += 1;
                let high = self.expr(P_RANGE + 1)?;
                lhs = Expr {
                    pos,
                    node: ExprNode::Range {
                        low: Box::new(lhs),
                        high: Box::new(high),
                        inclusive,
                    },
                };
                continue;
            }
            let (op, bp): (BinOp, u8) = match &t {
                Tok::Lt => (BinOp::Lt, P_CMP),
                Tok::Gt => (BinOp::Gt, P_CMP),
                Tok::Le => (BinOp::Le, P_CMP),
                Tok::Ge => (BinOp::Ge, P_CMP),
                Tok::EqEq => (BinOp::EqEq, P_CMP),
                Tok::NotEq => (BinOp::NotEq, P_CMP),
                Tok::AmpAmp => (BinOp::And, P_AND),
                Tok::PipePipe => (BinOp::Or, P_OR),
                _ => break,
            };
            if bp < min_bp {
                break;
            }
            self.ptr += 1;
            let rhs = self.expr(bp + 1)?;
            lhs = Expr {
                pos,
                node: ExprNode::Bin {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            };
        }
        // trailing `?:` (elvis) and `|>` (pipe), lower precedence than and/or
        while let Some(pos) = {
            let p = self.pos();
            match self.peek() {
                Some(Tok::Elvis) if P_ELVIS >= min_bp => Some(p),
                Some(Tok::PipeOp) if P_PIPE >= min_bp => Some(p),
                _ => None,
            }
        } {
            let is_elvis = matches!(self.peek(), Some(Tok::Elvis));
            self.ptr += 1;
            // pipe rhs binds left: chain groups as (a |> f) |> g, never f |> (g)
            let rhs = if is_elvis {
                self.expr(0)?
            } else {
                self.expr(P_PIPE + 1)?
            };
            lhs = if is_elvis {
                Expr { pos, node: ExprNode::Elvis { lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            } else {
                Expr { pos, node: ExprNode::Pipe { lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            };
        }
        Ok(lhs)
    }

    fn unary(&mut self) -> PResult<Expr> {
        let pos = self.pos();
        if self.is_kw("not") {
            self.ptr += 1;
            let e = self.expr(P_UNARY)?;
            return Ok(Expr {
                pos,
                node: ExprNode::Un {
                    op: UnOp::Not,
                    expr: Box::new(e),
                },
            });
        }
        if self.eat(Tok::Minus) {
            let e = self.expr(P_UNARY)?;
            return Ok(Expr {
                pos,
                node: ExprNode::Un {
                    op: UnOp::Neg,
                    expr: Box::new(e),
                },
            });
        }
        let atom = self.primary()?;
        self.postfix_chain(atom)
    }

    fn postfix_chain(&mut self, mut e: Expr) -> PResult<Expr> {
        loop {
            let pos = self.pos();
            match self.peek().cloned() {
                Some(Tok::LParen) => {
                    self.ptr += 1;
                    let mut args = Vec::new();
                    while !matches!(self.peek(), Some(Tok::RParen)) {
                        if self.peek().is_none() {
                            return Err(self.err("unexpected EOF in argument list"));
                        }
                        args.push(self.expr(0)?);
                        if !self.eat(Tok::Comma) {
                            break;
                        }
                    }
                    self.expect(Tok::RParen, "')'")?;
                    e = Expr {
                        pos,
                        node: ExprNode::Call {
                            callee: Box::new(e),
                            args,
                        },
                    };
                }
                Some(Tok::LBracket) => {
                    self.ptr += 1;
                    let idx = self.expr(0)?;
                    self.expect(Tok::RBracket, "']'")?;
                    e = Expr {
                        pos,
                        node: ExprNode::Index {
                            obj: Box::new(e),
                            idx: Box::new(idx),
                        },
                    };
                }
                Some(Tok::Dot) => {
                    self.ptr += 1;
                    let (name, _) = self.ident("field or method name")?;
                    e = Expr {
                        pos,
                        node: ExprNode::Field {
                            obj: Box::new(e),
                            name,
                        },
                    };
                }
                _ => break,
            }
        }
        Ok(e)
    }

    fn primary(&mut self) -> PResult<Expr> {
        let pos = self.pos();
        let Some(t) = self.peek().cloned() else {
            return Err(self.err("unexpected end of file in expression"));
        };
        match t {
            Tok::Int(v) => {
                self.ptr += 1;
                Ok(Expr { pos, node: ExprNode::Int(v) })
            }
            Tok::Float(v) => {
                self.ptr += 1;
                Ok(Expr { pos, node: ExprNode::Float(v) })
            }
            Tok::True => {
                self.ptr += 1;
                Ok(Expr { pos, node: ExprNode::Bool(true) })
            }
            Tok::False => {
                self.ptr += 1;
                Ok(Expr { pos, node: ExprNode::Bool(false) })
            }
            Tok::Nil => {
                self.ptr += 1;
                Ok(Expr { pos, node: ExprNode::Nil })
            }
            Tok::Str(parts) => {
                self.ptr += 1;
                let expanded = self.expand_str(&parts)?;
                Ok(Expr { pos, node: ExprNode::Str(expanded) })
            }
            Tok::Ident(name) => match name.as_str() {
                "this" => {
                    self.ptr += 1;
                    Ok(Expr { pos, node: ExprNode::This })
                }
                "super" => {
                    self.ptr += 1;
                    Ok(Expr { pos, node: ExprNode::Super })
                }
                "nil" => {
                    self.ptr += 1;
                    Ok(Expr { pos, node: ExprNode::Nil })
                }
                "and" | "or" | "not" | "if" | "else" | "while" | "for" | "return" | "break"
                | "continue" | "func" | "class" | "trait" | "pub" | "impl" | "as" | "var"
                | "let" | "is" => Err(self.err("keyword not allowed as identifier")),
                _ => {
                    self.ptr += 1;
                    Ok(Expr { pos, node: ExprNode::Ident(name) })
                }
            },
            Tok::LBracket => {
                self.ptr += 1;
                let mut out = Vec::new();
                while !matches!(self.peek(), Some(Tok::RBracket)) {
                    if self.peek().is_none() {
                        return Err(self.err("unexpected EOF in list literal"));
                    }
                    out.push(self.expr(0)?);
                    if !self.eat(Tok::Comma) {
                        break;
                    }
                }
                self.expect(Tok::RBracket, "']'")?;
                Ok(Expr { pos, node: ExprNode::List(out) })
            }
            Tok::At => {
                self.ptr += 1;
                self.expect(Tok::LParen, "'(' after @")?;
                let mut out = Vec::new();
                while !matches!(self.peek(), Some(Tok::RParen)) {
                    if self.peek().is_none() {
                        return Err(self.err("unexpected EOF in map literal"));
                    }
                    let k = self.expr(0)?;
                    self.expect(Tok::Colon, "':' in map entry")?;
                    let v = self.expr(0)?;
                    out.push((k, v));
                    if !self.eat(Tok::Comma) {
                        break;
                    }
                }
                self.expect(Tok::RParen, "')'")?;
                Ok(Expr { pos, node: ExprNode::Map(out) })
            }
            Tok::Pipe => self.lambda(),
            Tok::LParen => {
                self.ptr += 1;
                let e = self.expr(0)?;
                self.expect(Tok::RParen, "')'")?;
                Ok(e)
            }
            other => Err(ParseError {
                msg: format!(
                    "unexpected token in expression: {}",
                    crate::ast::tstr_like(&other)
                ),
                pos,
            }),
        }
    }

    /// Re-lex + re-parse each `${}` snippet; splice parsed exprs back in.
    fn expand_str(&self, s: &StrParts) -> PResult<StrParts> {
        let mut out_parts = Vec::new();
        for part in &s.parts {
            match part {
                StrPart::Lit(l) => out_parts.push(StrPart::Lit(l.clone())),
                StrPart::Expr(src, pos) => {
                    let e = parse_expr_src(src).map_err(|pe| ParseError {
                        msg: pe.msg,
                        pos: pos.clone(),
                    })?;
                    out_parts.push(StrPart::ExprAst(Box::new(e)));
                }
                StrPart::ExprAst(e) => out_parts.push(StrPart::ExprAst(e.clone())),
            }
        }
        Ok(StrParts { parts: out_parts })
    }

    fn lambda(&mut self) -> PResult<Expr> {
        let pos = self.pos();
        self.expect(Tok::Pipe, "'|' starting lambda parameters")?;
        let mut params = Vec::new();
        while !matches!(self.peek(), Some(Tok::Pipe)) {
            let (name, _) = self.ident("lambda parameter")?;
            let ty = if self.eat(Tok::Colon) { Some(self.ty()?) } else { None };
            params.push(Param { name, ty });
            if !self.eat(Tok::Comma) {
                break;
            }
        }
        self.expect(Tok::Pipe, "'|' closing lambda parameter list")?;
        let ret = if self.eat(Tok::Arrow) { Some(self.ty()?) } else { None };
        let body = self.block()?;
        Ok(Expr {
            pos,
            node: ExprNode::Lambda(Box::new(Lambda { params, ret, body: Box::new(body) })),
        })
    }
}

fn rhs_of(e: Expr) -> Expr {
    e
}

fn bin_expr(pos: Pos, op: BinOp, lhs: Expr, rhs: Expr) -> Expr {
    Expr {
        pos,
        node: ExprNode::Bin {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
    }
}

/// Stub to keep old references compiling until full rewrite lands.
#[cfg(test)]
fn _marker() -> usize {
    0
}

// ---------------- helpers appended ----------------

pub fn expr_to_path(e: &Expr) -> Option<Vec<PathSeg>> {
    let mut segs = Vec::new();
    let mut cur = e;
    loop {
        match &cur.node {
            ExprNode::Ident(name) => {
                segs.push(PathSeg::Name(name.clone()));
                segs.reverse();
                return Some(segs);
            }
            ExprNode::This => {
                segs.push(PathSeg::Name("this".to_string()));
                segs.reverse();
                return Some(segs);
            }
            ExprNode::Super => {
                segs.push(PathSeg::Name("super".to_string()));
                segs.reverse();
                return Some(segs);
            }
            ExprNode::Field { obj, name } => {
                segs.push(PathSeg::Name(name.clone()));
                cur = obj;
            }
            ExprNode::Index { obj, idx } => {
                segs.push(PathSeg::Index((**idx).clone()));
                cur = obj;
            }
            _ => return None,
        }
    }
}

pub fn parse_expr_src(src: &str) -> PResult<Expr> {
    let toks = Lexer::new(src).scan().map_err(|e| ParseError {
        msg: e.msg,
        pos: e.pos,
    })?;
    Parser { toks, ptr: 0 }.expr(0)
}
