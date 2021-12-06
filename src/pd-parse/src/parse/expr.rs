use pd_ds::token_set::TokenSet;
use pd_syntax::{SyntaxKind::*, T};

use crate::parser::Parser;

pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[T![true], T![false]]);

pub(crate) fn literal(p: &mut Parser<'_>) -> bool {
    if !p.at_any(LITERAL_FIRST) {
        return false;
    }
    p.enter(Literal, |p| p.bump_any());
    return true;
}

pub(super) fn expr(p: &mut Parser<'_>) {
    literal(p);
}
