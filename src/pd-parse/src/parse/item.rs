use super::*;

pub(crate) fn parse_item(p: &mut Parser<'_>) {
    if p.at(T![let]) { parse_value_def(p) } else { panic!() }
}

pub(crate) fn parse_value_def(p: &mut Parser<'_>) {
    p.enter(T![ValueDef], |p| {
        p.bump(T![let]);
        p.expect(T![Ident]);
        if p.accept(T![:]) {
            parse_type(p);
        }
        p.expect_recover(T![=], EXPR_FIRST);
        parse_expr(p)
    })
}
