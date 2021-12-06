use pd_syntax::ast;

use crate::parse_success;

#[test]
fn test_parse_literal() {
    let syntax = parse_success!(ast::Expr, false);
    let expected = r#"Literal@0..5
  FalseKw@0..5 "false"
"#;
    assert_eq!(format!("{:#?}", syntax), expected);
}
