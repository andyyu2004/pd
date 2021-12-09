use super::*;

pub(crate) fn parse_pat(p: &mut Parser<'_>) {
    // p.enter(T![PAT])
    p.accept(T![Ident]);
}
