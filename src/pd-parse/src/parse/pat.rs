use pd_ds::token_set::TokenSet;

use super::*;

pub(crate) const PAT_FIRST: TokenSet = LITERAL_FIRST.union(TokenSet::new(&[T![Ident]]));

pub(crate) fn parse_params(p: &mut Parser<'_>) {
    p.enter(T![Params], |p| {
        if !p.at_any(PAT_FIRST) {
            p.error(ParseError::FN_DEF_REQUIRES_PARAM);
            return;
        }

        while !p.at(T![:]) && !p.at(T![=]) {
            parse_pat(p);
        }
    })
}

pub(crate) fn parse_pat(p: &mut Parser<'_>) {
    // p.enter(T![PAT])
    p.expect(T![Ident]);
}
