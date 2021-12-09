use super::*;

pub(crate) fn parse_item(p: &mut Parser<'_>) {
    if p.at(T![let]) {
        parse_value_def(p)
    } else if p.at(T![fn]) {
        parse_fn_def(p)
    } else {
        panic!()
    }
}

/// `let <pattern> = <expr>`
/// Introduces a new non-recursive binding.
pub(crate) fn parse_value_def(p: &mut Parser<'_>) {
    p.enter(T![ValueDef], |p| {
        p.bump(T![let]);
        p.expect(T![Ident]);
        if p.accept(T![:]) {
            parse_type(p);
        }
        p.expect_recover(T![=], EXPR_FIRST);
        parse_expr(p);
    })
}

/// fn <name> <param>+ = <expr>
/// `fn` is essentially a restricted `let rec` where `<name>` must be a function (by requiring at least one parameter)
pub(crate) fn parse_fn_def(p: &mut Parser<'_>) {
    p.enter(T![Fn], |p| {
        p.bump(T![fn]);
        p.expect(T![Ident]);
        parse_params(p);
        p.expect_recover(T![=], EXPR_FIRST);
        parse_expr(p);
    });
}

pub(crate) fn parse_params(p: &mut Parser<'_>) {
    p.enter(T![Params], |p| {
        while !p.at(T![:]) || p.at(T![=]) {
            parse_pat(p);
        }
        p.expect_with_msg(T![:], "function definitions require at least one parameter");
        parse_type(p);
    })
}
