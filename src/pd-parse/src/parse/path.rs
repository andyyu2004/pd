use pd_ds::token_set::TokenSet;

use super::*;

pub(crate) const PATH_FIRST: TokenSet = TokenSet::new(&[T![Ident]]);

pub(crate) fn parse_path(p: &mut Parser<'_>) {
    p.enter(T![Path], |p| {
        p.expect(T![Ident]);
    })
}
