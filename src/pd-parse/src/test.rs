use pd_lex::Span;
use pd_syntax::T;

use super::*;
use crate::parse_source;
use crate::parser::{ParseError, ParseErrorKind};

fn parse_fn(src: &str) -> Parse<ast::Fn> {
    let mut token_source = TextTokenSource::from_text(src);
    let mut parser = Parser::new(&mut token_source);
    parse::parse_fn(&mut parser);
    parser.finish()
}

#[test]
fn test_parse_ok_fn() {
    let parsed = parse_fn(stringify!(
        fn main() {
        }
    ));

    let expected = r#"Fn@0..13
  FnKw@0..2 "fn"
  Whitespace@2..3 " "
  Ident@3..7 "main"
  Params@7..10
    OpenParen@7..8 "("
    CloseParen@8..9 ")"
    Whitespace@9..10 " "
  BlockExpr@10..13
    OpenBrace@10..11 "{"
    Whitespace@11..12 " "
    Stmts@12..12
    CloseBrace@12..13 "}"
"#;
    assert_eq!(expected, format!("{:#?}", parsed.syntax()));
    assert!(parsed.errors().is_empty());
}

#[test]
fn test_parse_fn_missing_params() {
    let parsed = parse_fn(stringify!(
        fn main {
        }
    ));

    let expected = r#"Fn@0..11
  FnKw@0..2 "fn"
  Whitespace@2..3 " "
  Ident@3..7 "main"
  Whitespace@7..8 " "
  BlockExpr@8..11
    OpenBrace@8..9 "{"
    Whitespace@9..10 " "
    Stmts@10..10
    CloseBrace@10..11 "}"
"#;

    assert_eq!(expected, format!("{:#?}", parsed.syntax()));
    assert_eq!(parsed.errors().len(), 1);
    assert_eq!(
        parsed.errors()[0],
        ParseError { span: Span::new(8, 8), kind: ParseErrorKind::Expected(T!['(']) }
    );
}
