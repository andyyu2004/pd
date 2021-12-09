use super::*;

pub(crate) fn parse_path(p: &mut Parser<'_>) {
    p.enter(T![Path], |p| {
        p.expect(T![Ident]);
    })
}
