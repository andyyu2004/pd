use pd_ds::token_set::TokenSet;
use pd_syntax::{SyntaxKind::*, T};

use crate::parser::Parser;

pub(crate) const EXPR_FIRST: TokenSet = LITERAL_FIRST;
pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[T![true], T![false]]);

pub(crate) fn literal(p: &mut Parser<'_>) -> bool {
    if !p.at_any(LITERAL_FIRST) {
        return false;
    }
    p.enter(Literal, |p| p.bump_any());
    return true;
}

pub(super) fn parse_expr(p: &mut Parser<'_>) {
    if literal(p) {
    } else {
        panic!("{:?}", p.current())
    }
}

pub(crate) fn parse_block(p: &mut Parser<'_>) {
    p.in_braces(T![BlockExpr], |p| {
        p.enter(T![Exprs], |p| {
            while p.at(T!['}']) {
                parse_expr(p);
            }
            p.bump(T!['}']);
        })
    });
}
