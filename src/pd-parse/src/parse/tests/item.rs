use expect_test::expect;
use pd_lex::Span;
use pd_syntax::{ast, T};

use crate::parser::{ParseError, ParseErrorKind};
use crate::{parse, parse_syntax};

#[test]
fn test_parse_item_value_def() {
    let syntax = parse_syntax!(ast::Item, let x = false);
    let expected = r#"ValueDef@0..13
  LetKw@0..3 "let"
  Whitespace@3..4 " "
  Ident@4..5 "x"
  Whitespace@5..6 " "
  Equal@6..7 "="
  Whitespace@7..8 " "
  Literal@8..13
    FalseKw@8..13 "false"
"#;
    assert_eq!(format!("{:#?}", syntax), expected);
}

#[test]
fn test_parse_item_value_def_with_ty() {
    let parsed = parse::<ast::Item>(stringify!(let x: bool = false));
    let expected = r#"ValueDef@0..20
  LetKw@0..3 "let"
  Whitespace@3..4 " "
  Ident@4..5 "x"
  Whitespace@5..6 " "
  Colon@6..7 ":"
  Whitespace@7..8 " "
  Type@8..13
    Path@8..13
      Ident@8..12 "bool"
      Whitespace@12..13 " "
  Equal@13..14 "="
  Whitespace@14..15 " "
  Literal@15..20
    FalseKw@15..20 "false"
"#;
    assert_eq!(format!("{:#?}", parsed.syntax()), expected);
}

#[test]
fn test_parse_item_value_def_missing_equal() {
    let parsed = parse::<ast::Item>(stringify!(let x false));
    let expected = r#"ValueDef@0..11
  LetKw@0..3 "let"
  Whitespace@3..4 " "
  Ident@4..5 "x"
  Whitespace@5..6 " "
  Literal@6..11
    FalseKw@6..11 "false"
"#;
    assert_eq!(format!("{:#?}", parsed.syntax()), expected);
}

#[test]
fn test_parse_fn_def() {
    let parsed = parse::<ast::Item>(stringify!(fn f x y: a = false));
    let expected = expect![[r#"
        Fn@0..20
          FnKw@0..2 "fn"
          Whitespace@2..3 " "
          Ident@3..4 "f"
          Whitespace@4..5 " "
          Params@5..9
            Binding@5..7
              Ident@5..6 "x"
              Whitespace@6..7 " "
            Binding@7..9
              Ident@7..8 "y"
              Whitespace@8..9 " "
          Colon@9..10 ":"
          Whitespace@10..11 " "
          Type@11..13
            Path@11..13
              Ident@11..12 "a"
              Whitespace@12..13 " "
          Equal@13..14 "="
          Whitespace@14..15 " "
          Literal@15..20
            FalseKw@15..20 "false"

    "#]];
    expected.assert_debug_eq(&parsed.syntax());
}

#[test]
fn test_parse_ok_fn() {
    let syntax = parse_syntax! {
        ast::Fn,
        fn f x: t = false
    };

    let expected = expect![[r#"
        Fn@0..18
          FnKw@0..2 "fn"
          Whitespace@2..3 " "
          Ident@3..4 "f"
          Whitespace@4..5 " "
          Params@5..7
            Binding@5..7
              Ident@5..6 "x"
              Whitespace@6..7 " "
          Colon@7..8 ":"
          Whitespace@8..9 " "
          Type@9..11
            Path@9..11
              Ident@9..10 "t"
              Whitespace@10..11 " "
          Equal@11..12 "="
          Whitespace@12..13 " "
          Literal@13..18
            FalseKw@13..18 "false"

    "#]];

    expected.assert_debug_eq(&syntax);
}

#[test]
fn test_parse_fn_no_params() {
    let parsed = parse::<ast::Fn>(stringify!(
        fn x: t = false
    ));

    let expected = expect![[r#"
        Fn@0..16
          FnKw@0..2 "fn"
          Whitespace@2..3 " "
          Ident@3..4 "x"
          Whitespace@4..5 " "
          Params@5..5
          Colon@5..6 ":"
          Whitespace@6..7 " "
          Type@7..9
            Path@7..9
              Ident@7..8 "t"
              Whitespace@8..9 " "
          Equal@9..10 "="
          Whitespace@10..11 " "
          Literal@11..16
            FalseKw@11..16 "false"

    "#]];

    assert_eq!(parsed.errors().len(), 1);
    assert_eq!(
        parsed.errors()[0],
        ParseError {
            span: Span::new(5, 5),
            kind: ParseErrorKind::StaticMsg(ParseError::FN_DEF_REQUIRES_PARAM)
        }
    );
    expected.assert_debug_eq(&parsed.syntax());
}

#[test]
fn test_parse_fn_no_params_no_type() {
    let parsed = parse::<ast::Fn>(stringify!(
        fn x = false
    ));

    let expected = expect![[r#"
        Fn@0..12
          FnKw@0..2 "fn"
          Whitespace@2..3 " "
          Ident@3..4 "x"
          Whitespace@4..5 " "
          Params@5..5
          Equal@5..6 "="
          Whitespace@6..7 " "
          Literal@7..12
            FalseKw@7..12 "false"

    "#]];

    assert_eq!(parsed.errors().len(), 2);
    assert_eq!(
        parsed.errors()[0],
        ParseError {
            span: Span::new(5, 5),
            kind: ParseErrorKind::StaticMsg(ParseError::FN_DEF_REQUIRES_PARAM)
        },
    );
    assert_eq!(
        parsed.errors()[1],
        ParseError {
            span: Span::new(5, 5),
            kind: ParseErrorKind::ExpectedWithMsg(T![:], ParseError::FN_DEF_REQUIRES_TYPE)
        },
    );
    expected.assert_debug_eq(&parsed.syntax());
}
