use anyhow::Result;
use indexed_vec::Idx;

use super::*;

#[test]
fn test_ide_parse() -> Result<()> {
    let mut acx = AnalysisCtxt::default();
    let id = FileId::new(0);
    let src = stringify!(
        fn main() {
        }
    );
    acx.apply_change(Change::single(id, FileChange::Created(src.to_owned())));

    let analysis = acx.analysis();
    let parsed = analysis.parse(id)?;
    let expected = r#"SourceFile@0..13
  Fn@0..13
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
    Ok(())
}
