//! sloth2 lexer.
//!
//! Emits a token stream with source positions. String interpolation
//! `"a${expr}b"` is carried by a single `Tok::Str` token whose payload keeps
//! literal parts and the source ranges of embedded expressions; the parser
//! re-lexes/parses those snippets.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

/// A string literal split into parts ("pre", ${expr}, "mid", ...).
#[derive(Debug, Clone)]
pub enum StrPart {
    Lit(String),
    /// Raw source text of the interpolated expression and its start position.
    Expr(String, Pos),
    /// Parsed expression (filled by the parser; never produced by the lexer).
    ExprAst(Box<crate::ast::Expr>),
}

impl PartialEq for StrPart {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StrPart::Lit(a), StrPart::Lit(b)) => a == b,
            (StrPart::Expr(a, _), StrPart::Expr(b, _)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StrParts {
    pub parts: Vec<StrPart>,
}

impl StrParts {
    pub fn is_plain(&self) -> bool {
        self.parts.len() == 1 && matches!(self.parts[0], StrPart::Lit(_))
    }
    pub fn plain(&self) -> Option<&str> {
        if self.is_plain() {
            if let StrPart::Lit(s) = &self.parts[0] {
                return Some(s);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    // literals
    Int(i64),
    UInt(u64),
    Float(f64),
    True,
    False,
    Nil,
    // identifiers (context keywords arrive here; parser resolves)
    Ident(String),
    // punctuation / operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Dot,
    Comma,
    Colon,
    Semi,
    Assign,
    EqEq,
    NotEq,
    Lt,
    Gt,
    Le,
    Ge,
    Arrow,       // ->
    Range,       // ..
    RangeInc,    // ..=
    Variadic,    // ...
    Pipe,        // |  lambda start / bitwise or
    PipeOp,      // |>
    AmpAmp,      // &&
    PipePipe,    // ||
    PlusEq,      // +=
    MinusEq,     // -=
    Amp,         // &
    Caret,       // ^
    Tilde,       // ~
    Question,    // ?
    Elvis,       // ?:
    QuestionDot, // ?. (reserved; parser rejects)
    At,          // @
    Str(Box<StrParts>),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tok: Tok,
    pub pos: Pos,
}

#[derive(Debug)]
pub struct LexError {
    pub msg: String,
    pub pos: Pos,
}

pub struct Lexer {
    chars: Vec<char>,
    ptr: usize,
    line: usize,
    col: usize,
}

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    Lexer::new(src).scan()
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        let mut chars = Vec::new();
        for (_i, ch) in src.char_indices() {
            chars.push(ch);
        }
        Lexer {
            chars,
            ptr: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn scan(mut self) -> Result<Vec<Token>, LexError> {
        let mut out = Vec::new();
        while self.ptr < self.chars.len() {
            let c = self.chars[self.ptr];
            if c.is_whitespace() {
                self.advance();
                continue;
            }
            if c == '/' && self.peekn(1) == Some('/') {
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                continue;
            }
            if c == '/' && self.peekn(1) == Some('*') {
                self.advance();
                self.advance();
                loop {
                    if self.ptr >= self.chars.len() {
                        return Err(self.err("unterminated block comment"));
                    }
                    let c = self.chars[self.ptr];
                    if c == '*' && self.peekn(1) == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
                continue;
            }
            self.text(&mut out)?;
        }
        Ok(out)
    }

    fn text(&mut self, out: &mut Vec<Token>) -> Result<(), LexError> {
        let start = self.cur();
        let c = self.chars[self.ptr];
        match c {
            '0'..='9' => {
                let tok = self.number()?;
                out.push(Token { tok, pos: start });
                Ok(())
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                while let Some(c) = self.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        s.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                out.push(Token {
                    tok: Tok::Ident(s),
                    pos: start,
                });
                Ok(())
            }
            '"' => {
                let parts = self.string()?;
                out.push(Token {
                    tok: Tok::Str(Box::new(parts)),
                    pos: start,
                });
                Ok(())
            }
            _ => {
                let tok = self.punct()?;
                out.push(Token { tok, pos: start });
                Ok(())
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.ptr).copied()
    }
    fn peekn(&self, n: usize) -> Option<char> {
        self.chars.get(self.ptr + n).copied()
    }
    fn advance(&mut self) {
        if self.chars.get(self.ptr).copied() == Some('\n') {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.ptr += 1;
    }
    fn cur(&self) -> Pos {
        Pos {
            line: self.line,
            col: self.col,
        }
    }
    pub fn err(&self, msg: &str) -> LexError {
        LexError {
            msg: msg.to_string(),
            pos: self.cur(),
        }
    }

    fn number(&mut self) -> Result<Tok, LexError> {
        let start = self.ptr;
        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' {
                match self.peekn(1) {
                    Some('.') | None => break, // range op is punct, not part of number
                    _ => {
                        is_float = true;
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
        // exponent part: 1e0 / 1.5e-3 / 2E+4 (only when a digit follows the
        // optional sign, so a trailing `e` never eats an identifier)
        if matches!(self.peek(), Some('e') | Some('E')) {
            let mut k = 1usize;
            if matches!(self.peekn(k), Some('+') | Some('-')) {
                k += 1;
            }
            if matches!(self.peekn(k), Some(c) if c.is_ascii_digit()) {
                is_float = true;
                self.advance();
                if matches!(self.peek(), Some('+') | Some('-')) {
                    self.advance();
                }
                while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                    self.advance();
                }
            }
        }
        let s: String = self.chars[start..self.ptr].iter().collect();
        // `u`/`U` suffix: force an unsigned literal (must not start an ident)
        let mut unsigned = false;
        if !is_float && matches!(self.peek(), Some('u') | Some('U')) {
            if !matches!(self.peekn(1), Some(c) if c.is_alphanumeric() || c == '_') {
                self.advance();
                unsigned = true;
            }
        }
        if is_float {
            let f: f64 = s.parse().map_err(|_| self.err("invalid float literal"))?;
            Ok(Tok::Float(f))
        } else if !unsigned {
            match s.parse::<i64>() {
                Ok(i) => Ok(Tok::Int(i)),
                Err(_) => match s.parse::<u64>() {
                    // a decimal beyond i64::MAX can only be an unsigned word
                    Ok(u) => Ok(Tok::UInt(u)),
                    Err(_) => Err(self.err("int literal out of range")),
                },
            }
        } else {
            let u: u64 = s
                .parse()
                .map_err(|_| self.err("unsigned int literal out of range"))?;
            Ok(Tok::UInt(u))
        }
    }

    fn string(&mut self) -> Result<StrParts, LexError> {
        let mut parts = Vec::new();
        let mut lit = String::new();
        self.advance(); // eat opening quote
        loop {
            let Some(c) = self.peek() else {
                return Err(self.err("unterminated string literal"));
            };
            if c == '"' {
                self.advance();
                break;
            } else if c == '\\' {
                self.advance();
                let Some(e) = self.peek() else {
                    return Err(self.err("bad escape at end of string"));
                };
                let ch = match e {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '"' => '"',
                    '\\' => '\\',
                    '$' => '$',
                    '{' => '{',
                    '}' => '}',
                    _ => return Err(self.err("unsupported escape character")),
                };
                lit.push(ch);
                self.advance();
            } else if c == '$' && self.peekn(1) == Some('{') {
                self.advance();
                self.advance();
                let start_pos = self.cur();
                let expr_start = self.ptr;
                let mut depth = 1usize;
                loop {
                    match self.peek() {
                        None => return Err(self.err("unterminated interpolation")),
                        Some('{') => {
                            depth += 1;
                            self.advance();
                        }
                        Some('}') => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                            self.advance();
                        }
                        _ => self.advance(),
                    }
                }
                let expr_src: String = self.chars[expr_start..self.ptr].iter().collect();
                parts.push(StrPart::Lit(std::mem::take(&mut lit)));
                parts.push(StrPart::Expr(expr_src, start_pos));
                self.advance(); // eat closing '}'
            } else {
                lit.push(c);
                self.advance();
            }
        }
        parts.push(StrPart::Lit(lit));
        Ok(StrParts { parts })
    }

    fn slice3(&self) -> Option<[char; 3]> {
        Some([
            *self.chars.get(self.ptr)?,
            *self.chars.get(self.ptr + 1)?,
            *self.chars.get(self.ptr + 2)?,
        ])
    }

    fn punct(&mut self) -> Result<Tok, LexError> {
        // three-char first
        if let Some([a, b, c]) = self.slice3() {
            if a == '.' && b == '.' && c == '.' {
                self.advance();
                self.advance();
                self.advance();
                return Ok(Tok::Variadic);
            }
            if a == '.' && b == '.' && c == '=' {
                self.advance();
                self.advance();
                self.advance();
                return Ok(Tok::RangeInc);
            }
        }
        let cur = self.chars[self.ptr];
        let nxt = self.peekn(1);
        let pair = |want: (char, char)| cur == want.0 && nxt == Some(want.1);
        #[allow(clippy::redundant_closure)]
        if pair(('=', '=')) {
            self.advance();
            self.advance();
            return Ok(Tok::EqEq);
        }
        if cur == '!' && nxt == Some('=') {
            self.advance();
            self.advance();
            return Ok(Tok::NotEq);
        }
        if pair(('<', '=')) {
            self.advance();
            self.advance();
            return Ok(Tok::Le);
        }
        if pair(('>', '=')) {
            self.advance();
            self.advance();
            return Ok(Tok::Ge);
        }
        if pair(('.', '.')) {
            self.advance();
            self.advance();
            return Ok(Tok::Range);
        }
        if pair(('|', '>')) {
            self.advance();
            self.advance();
            return Ok(Tok::PipeOp);
        }
        if pair(('&', '&')) {
            self.advance();
            self.advance();
            return Ok(Tok::AmpAmp);
        }
        if pair(('|', '|')) {
            self.advance();
            self.advance();
            return Ok(Tok::PipePipe);
        }
        if pair(('-', '>')) {
            self.advance();
            self.advance();
            return Ok(Tok::Arrow);
        }
        if pair(('+', '=')) {
            self.advance();
            self.advance();
            return Ok(Tok::PlusEq);
        }
        if pair(('-', '=')) {
            self.advance();
            self.advance();
            return Ok(Tok::MinusEq);
        }
        let tok = match cur {
            '+' => Tok::Plus,
            '-' => Tok::Minus,
            '*' => Tok::Star,
            '/' => Tok::Slash,
            '%' => Tok::Percent,
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            '[' => Tok::LBracket,
            ']' => Tok::RBracket,
            '{' => Tok::LBrace,
            '}' => Tok::RBrace,
            '.' => Tok::Dot,
            ',' => Tok::Comma,
            ';' => Tok::Semi,
            ':' => Tok::Colon,
            '=' => Tok::Assign,
            '<' => Tok::Lt,
            '>' => Tok::Gt,
            '|' => Tok::Pipe,
            '&' => Tok::Amp,
            '^' => Tok::Caret,
            '~' => Tok::Tilde,
            '@' => Tok::At,
            '?' => {
                self.advance();
                return match self.peek() {
                    Some('.') => {
                        self.advance();
                        Ok(Tok::QuestionDot)
                    }
                    Some(':') => {
                        self.advance();
                        Ok(Tok::Elvis)
                    }
                    _ => Ok(Tok::Question),
                };
            }
            c => {
                return Err(LexError {
                    msg: format!("unexpected character {:?}", c),
                    pos: self.cur(),
                });
            }
        };
        self.advance();
        Ok(tok)
    }
}
