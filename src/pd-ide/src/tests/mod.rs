use anyhow::Result;
use expect_test::expect;
use indexvec::Idx;

use super::*;

#[test]
fn test_ide_parse() -> Result<()> {
    let mut acx = AnalysisCtxt::default();
    let id = FileId::new(0);
    let src = stringify!(
      let x = false
    );
    acx.apply_change(Change::single(id, FileChange::Created(src.to_owned())));

    let analysis = acx.analysis();
    let parsed = analysis.parse(id)?;
    let expected = expect![[r#"
        SourceFile@0..13
          ValueDef@0..13
            LetKw@0..3 "let"
            Whitespace@3..4 " "
            BindingPat@4..6
              Name@4..6
                Ident@4..5 "x"
                Whitespace@5..6 " "
            Equal@6..7 "="
            Whitespace@7..8 " "
            Literal@8..13
              FalseKw@8..13 "false"

    "#]];
    expected.assert_debug_eq(&parsed.syntax());
    assert!(parsed.errors().is_empty());
    Ok(())
}
