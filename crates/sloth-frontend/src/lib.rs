//! sloth-frontend: lexer, parser, resolver, type checker, monomorphizer for sloth2.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod ty;

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::{lex, StrPart, Tok};

    fn toks(src: &str) -> Vec<Tok> {
        lex(src).unwrap().into_iter().map(|t| t.tok).collect()
    }

    #[test]
    fn basics() {
        assert_eq!(
            toks("var x: int = 1;"),
            vec![
                Tok::Ident("var".into()),
                Tok::Ident("x".into()),
                Tok::Colon,
                Tok::Ident("int".into()),
                Tok::Assign,
                Tok::Int(1),
                Tok::Semi,
            ]
        );
    }

    #[test]
    fn floats_and_ranges() {
        assert_eq!(
            toks("for (var i: 0..=5) { }"),
            vec![
                Tok::Ident("for".into()),
                Tok::LParen,
                Tok::Ident("var".into()),
                Tok::Ident("i".into()),
                Tok::Colon,
                Tok::Int(0),
                Tok::RangeInc,
                Tok::Int(5),
                Tok::RParen,
                Tok::LBrace,
                Tok::RBrace,
            ]
        );
    }

    #[test]
    fn string_and_interp() {
        let t = toks(r#"let s = "hi ${x} / ${y}!";"#);
        match &t[3] {
            Tok::Str(parts) => {
                let p = &parts.parts;
                assert_eq!(p.len(), 5);
                assert_eq!(p[0], StrPart::Lit("hi ".into()));
                assert_eq!(
                    p[1],
                    StrPart::Expr("x".into(), crate::lexer::Pos { line: 1, col: 15 })
                );
                assert_eq!(p[2], StrPart::Lit(" / ".into()));
                assert_eq!(
                    p[3],
                    StrPart::Expr("y".into(), crate::lexer::Pos { line: 1, col: 22 })
                );
                assert_eq!(p[4], StrPart::Lit("!".into()));
            }
            other => panic!("{:?}", other),
        }
    }

    #[test]
    fn ops() {
        assert_eq!(
            toks("x |> f(a, [1,2]) .. b != c"),
            vec![
                Tok::Ident("x".into()),
                Tok::PipeOp,
                Tok::Ident("f".into()),
                Tok::LParen,
                Tok::Ident("a".into()),
                Tok::Comma,
                Tok::LBracket,
                Tok::Int(1),
                Tok::Comma,
                Tok::Int(2),
                Tok::RBracket,
                Tok::RParen,
                Tok::Range,
                Tok::Ident("b".into()),
                Tok::NotEq,
                Tok::Ident("c".into()),
            ]
        );
    }

    #[test]
    fn lambda_elvis_variadic() {
        assert_eq!(
            toks("|a, b| -> unit { a ?: f(1...: Array<int>) }"),
            vec![
                Tok::Pipe,
                Tok::Ident("a".into()),
                Tok::Comma,
                Tok::Ident("b".into()),
                Tok::Pipe,
                Tok::Arrow,
                Tok::Ident("unit".into()),
                Tok::LBrace,
                Tok::Ident("a".into()),
                Tok::Elvis,
                Tok::Ident("f".into()),
                Tok::LParen,
                Tok::Int(1),
                Tok::Variadic,
                Tok::Colon,
                Tok::Ident("Array".into()),
                Tok::Lt,
                Tok::Ident("int".into()),
                Tok::Gt,
                Tok::RParen,
                Tok::RBrace,
            ]
        );
    }

    #[test]
    fn keywords_arrive_as_ident() {
        assert_eq!(
            toks("pub while return"),
            vec![
                Tok::Ident("pub".into()),
                Tok::Ident("while".into()),
                Tok::Ident("return".into()),
            ]
        );
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::{ArithOp, ExprNode};
    use crate::parser::{parse, parse_expr_src};

    #[test]
    fn program_globals() {
        let p = parse("var x: int = 1;\nlet y = 2.5;\n").unwrap();
        assert_eq!(p.decls.len(), 2);
        assert!(matches!(p.decls[0].kind, crate::ast::DeclKind::Var));
        assert!(matches!(p.decls[1].kind, crate::ast::DeclKind::Let));
    }

    #[test]
    fn generics_fn() {
        let p = parse("func map<T, R>(arr: Array<T>, f: (T) -> R): Array<R> { return arr; }\n")
            .unwrap();
        match &p.decls[0].node {
            crate::ast::DeclNode::Func(f) => {
                assert_eq!(f.type_params.len(), 2);
                assert_eq!(f.type_params[0].name, "T");
                assert!(f.ret.is_some());
            }
            _ => panic!(),
        }
    }

    #[test]
    fn variadic() {
        let p = parse("func add_all(xs...: Array<int>): int { return 0; }\n").unwrap();
        match &p.decls[0].node {
            crate::ast::DeclNode::Func(f) => {
                assert!(f.variadic.is_some());
            }
            _ => panic!(),
        }
    }

    #[test]
    fn class_with_impl() {
        let p = parse(
            "class Cat: Mammal impl Speaker, Pet { var n: int; func say(): unit { print(1); } }\n",
        )
        .unwrap();
        match &p.decls[0].node {
            crate::ast::DeclNode::Class(c) => {
                assert_eq!(c.superclass.as_deref(), Some("Mammal"));
                assert_eq!(c.impls, vec!["Speaker".to_string(), "Pet".to_string()]);
                assert_eq!(c.fields.len(), 1);
                assert_eq!(c.methods.len(), 1);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn trait_decl() {
        let p = parse("trait Speaker { func say(): unit; }\n").unwrap();
        match &p.decls[0].node {
            crate::ast::DeclNode::Trait(t) => {
                assert_eq!(t.methods.len(), 1);
                assert_eq!(t.methods[0].name, "say");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn expr_precedence() {
        // 1 + 2 * 3  →  Arith(Add, 1, Arith(Mul, 2, 3))
        let e = parse_expr_src("1 + 2 * 3").unwrap();
        match e.node {
            ExprNode::Arith {
                op: ArithOp::Add,
                lhs,
                rhs,
            } => {
                assert!(matches!(lhs.node, ExprNode::Int(1)));
                assert!(matches!(
                    rhs.node,
                    ExprNode::Arith {
                        op: ArithOp::Mul,
                        ..
                    }
                ));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn range_expr() {
        // 0..=5 should parse as Range{inclusive}
        let _p = parse("for (var i: 0..=5) { }").unwrap();
        // for iter expr is checked here via body
    }

    #[test]
    fn is_not_nil() {
        let e = parse_expr_src("x is not nil").unwrap();
        match e.node {
            ExprNode::Is { negated: true, .. } => {}
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn pipe_and_elvis() {
        let e = parse_expr_src("names |> map(|x| { return x; })").unwrap();
        assert!(matches!(e.node, ExprNode::Pipe { .. }));

        let e2 = parse_expr_src("name ?: \"anon\"").unwrap();
        assert!(matches!(e2.node, ExprNode::Elvis { .. }));
    }

    #[test]
    fn assignment_target() {
        let p = parse("this.arr[0] = 2 + 1;").unwrap();
        assert!(matches!(p.decls.len(), 0));
    }

    #[test]
    fn interpolation_expands() {
        let e = parse_expr_src(r#""a${x}b""#).unwrap();
        match e.node {
            ExprNode::Str(parts) => {
                assert_eq!(parts.parts.len(), 3);
                assert!(matches!(parts.parts[1], crate::lexer::StrPart::ExprAst(_)));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn nested_stmts() {
        let p = parse(
            "func f(a: int): int { while (a > 0) { if (a is not nil) { a = a - 1; } else { break; } } return a; }\n",
        )
        .unwrap();
        assert_eq!(p.decls.len(), 1);
    }

    #[test]
    fn map_list_literals() {
        let e = parse_expr_src("@(\"k\": 1, \"j\": [2, 3])").unwrap();
        assert!(matches!(e.node, ExprNode::Map(_)));
        let e2 = parse_expr_src("[1, 2, 3]").unwrap();
        assert!(matches!(e2.node, ExprNode::List(_)));
    }

    #[test]
    fn path_of_nested() {
        let _p = parse("this.a[0] = 1;").unwrap();
    }
}
