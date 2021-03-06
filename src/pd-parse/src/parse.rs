mod expr;
mod item;
mod pat;
mod path;
mod ty;

use expr::*;
use item::*;
use pat::*;
use path::*;
use pd_ds::token_set::TokenSet;
use ty::*;

use std::marker::PhantomData;
use std::sync::Arc;

use pd_lex::TextTokenSource;
use pd_syntax::{ast, AstNodeExt, SyntaxKind, SyntaxNode, T};
use rowan::GreenNode;

use crate::parser::{ParseError, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parse<T> {
    node: GreenNode,
    errors: Arc<Vec<ParseError>>,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Parse<T> {
    pub fn new(node: GreenNode, errors: Vec<ParseError>) -> Self {
        Self { node, errors: Arc::new(errors), _marker: PhantomData }
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

pub trait ParseNode: AstNodeExt {
    fn parse(_: &mut Parser<'_>);
}

impl ParseNode for ast::SourceFile {
    #[inline]
    fn parse(parser: &mut Parser<'_>) {
        parse_source_file(parser)
    }
}

impl ParseNode for ast::Fn {
    #[inline]
    fn parse(parser: &mut Parser<'_>) {
        parse_fn_def(parser)
    }
}

impl ParseNode for ast::Item {
    #[inline]
    fn parse(parser: &mut Parser<'_>) {
        parse_item(parser)
    }
}

impl ParseNode for ast::Expr {
    fn parse(parser: &mut Parser<'_>) {
        parse_expr(parser)
    }
}

pub fn parse<N: ParseNode>(text: &str) -> Parse<N> {
    let mut token_source = TextTokenSource::from_text(text);
    let mut parser = Parser::new(&mut token_source);
    N::parse(&mut parser);
    parser.finish()
}

pub(crate) fn parse_source_file(p: &mut Parser<'_>) {
    p.enter(SyntaxKind::SourceFile, |p| {
        if p.at(T![fn]) {
            parse_fn_def(p)
        } else if p.at(T![let]) {
            parse_const(p)
        } else {
            p.expect(T![let]);
        }
    })
}

pub(crate) fn parse_name(p: &mut Parser<'_>) {
    parse_name_recover(p, TokenSet::EMPTY)
}

pub(crate) fn parse_name_recover(p: &mut Parser<'_>, recovery: TokenSet) {
    p.enter(T![Name], |p| {
        p.expect_recover(T![Ident], recovery);
    });
}

#[cfg(test)]
mod tests;
