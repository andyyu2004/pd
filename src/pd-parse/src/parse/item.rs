use super::*;

pub(crate) fn parse_item(p: &mut Parser<'_>) {
    if p.at(T![let]) { parse_value_def(p) } else { panic!() }
}

pub(crate) fn parse_value_def(p: &mut Parser<'_>) {
    p.enter(T![ValueDef], |p| {
        p.bump(T![let]);
        p.expect(T![Ident]);
        p.expect(T![=]);
        parse_expr(p)
    })
}
