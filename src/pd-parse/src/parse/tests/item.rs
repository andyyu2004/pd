use pd_syntax::ast;

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
