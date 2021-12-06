mod expr;
mod item;

use expr::*;
use item::*;

use std::marker::PhantomData;
use std::sync::Arc;

use pd_lex::TextTokenSource;
use pd_syntax::{ast, AstNode, SyntaxKind, SyntaxNode, T};
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

pub trait ParseNode: AstNode {
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
        parse_fn(parser)
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
            parse_fn(p)
        } else if p.at(T![let]) {
            parse_value_def(p)
        } else {
            p.expect(T![let]);
        }
    })
}

pub(crate) fn parse_fn(p: &mut Parser<'_>) {
    p.enter(T![Fn], |p| {
        p.bump(T![fn]);
        p.expect(T![Ident]);
        p.in_parens(T![Params], parse_params);
        p.in_braces(T![BlockExpr], parse_block);
    });
}

pub(crate) fn parse_params(p: &mut Parser<'_>) {
}

pub(crate) fn parse_block(p: &mut Parser<'_>) {
    p.enter(T![Stmts], |_| {});
}

#[cfg(test)]
mod tests;
