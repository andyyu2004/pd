use pd_syntax::ast;

use crate::{parse_reflexive, parse_syntax};

#[test]
fn test_parse_literal_expr() {
    let syntax = parse_syntax!(ast::Expr, false);
    let expected = r#"Literal@0..5
  FalseKw@0..5 "false"
"#;
    assert_eq!(format!("{:#?}", syntax), expected);

    parse_reflexive!(ast::Expr, true);
}
