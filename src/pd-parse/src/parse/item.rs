use super::*;

impl ParseError {
    pub(crate) const FN_DEF_REQUIRES_PARAM: &'static str =
        "function definitions require at least one parameter";
    pub(crate) const FN_DEF_REQUIRES_TYPE: &'static str =
        "function definitions require a type annotation";
}

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
        parse_pat(p);
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
        parse_name(p);
        parse_params(p);
        if p.expect_recover_with_msg(
            T![:],
            TYPE_FIRST.with(T![=]),
            ParseError::FN_DEF_REQUIRES_TYPE,
        ) {
            parse_type(p);
        }
        p.expect_recover(T![=], EXPR_FIRST);
        parse_expr(p);
    });
}
