use pd_ds::token_set::TokenSet;

use super::*;

pub(crate) const TYPE_FIRST: TokenSet = PATH_FIRST;

pub(crate) fn parse_type(p: &mut Parser<'_>) {
    // TODO type shouldn't be it's own syntaxkind
    p.enter(T![Type], |p| {
        parse_path(p);
    });
}
