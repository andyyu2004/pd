use std::marker::PhantomData;

use pd_lex::SyntaxError;
use pd_syntax::{SyntaxNode, T};
use rowan::GreenNode;

use crate::parser::{ParseError, Parser};

#[derive(Debug)]
pub struct Parse<T> {
    node: GreenNode,
    errors: Vec<ParseError>,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Parse<T> {
    pub fn new(node: GreenNode, errors: Vec<ParseError>) -> Self {
        Self { node, errors, _marker: PhantomData }
    }

    #[inline]
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.node.clone())
    }

    #[inline]
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }
}

pub(crate) fn parse_fn(p: &mut Parser<'_>) {
    p.enter(T![FN], |p| {
        p.bump(T![fn]);
        p.expect(T![IDENT]);
        p.in_parens(T![PARAMS], parse_params);
        p.in_braces(T![BLOCK_EXPR], parse_block);
    });
}

pub(crate) fn parse_params(p: &mut Parser<'_>) {
}

pub(crate) fn parse_block(p: &mut Parser<'_>) {
    p.enter(T![STMTS], |_| {});
}
