use pd_lex::Span;
use pd_syntax::{ast, T};

use super::*;
use crate::parse;
use crate::parser::{ParseError, ParseErrorKind};

/// Checks the conversion back to a string exactly matches the input string
#[macro_export]
macro_rules! parse_reflexive {
    ($ty:ty, $($tt:tt)*) => {{
        let src = stringify!($($tt)*);
        let parsed = $crate::parse::<$ty>(src);
        assert!(parsed.errors().is_empty());
        assert_eq!(src, parsed.syntax().to_string());
    }};
}

/// Asserts a successful parse (no errors) and returns the syntax node.
/// Includes a check of `parse_reflexive`
#[macro_export]
macro_rules! parse_syntax {
    ($ty:ty, $($tt:tt)*) => {{
        let src = stringify!($($tt)*);
        let parsed = $crate::parse::<$ty>(src);
        assert!(parsed.errors().is_empty());
        assert_eq!(src, parsed.syntax().to_string());
        parsed.syntax()
    }};
}

#[test]
fn test_parse_ok_fn() {
    let syntax = parse_syntax! {
        ast::Fn,
        fn main() {}
    };

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

    assert_eq!(expected, format!("{:#?}", syntax));
}

#[test]
fn test_parse_fn_missing_params() {
    let parsed = parse::<ast::Fn>(stringify!(
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

#[test]
fn test_parse_fn_missing_name() {
    let parsed = parse::<ast::Fn>(stringify!(
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
