use pd_ds::token_set::TokenSet;
use pd_lex::{Span, Token, TokenSource};
use pd_syntax::{AstNode, SyntaxKind, T};

use crate::parse::Parse;

pub struct Parser<'t> {
    source: &'t mut dyn TokenSource,
    builder: rowan::GreenNodeBuilder<'static>,
    errors: Vec<ParseError>,
    steps: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub span: Span,
    pub kind: ParseErrorKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
    Expected(SyntaxKind),
    ExpectedWithMsg(SyntaxKind, &'static str),
    Message(String),
    StaticMsg(&'static str),
}

impl<'t> Parser<'t> {
    pub fn new(source: &'t mut dyn TokenSource) -> Self {
        Self {
            source,
            builder: Default::default(),
            errors: Default::default(),
            steps: Default::default(),
        }
    }

    pub fn finish<T: AstNode>(mut self) -> Parse<T> {
        let node = self.builder.finish();
        self.errors.extend(
            self.source
                .errors()
                .into_iter()
                .map(|err| ParseError { span: err.span, kind: ParseErrorKind::Message(err.msg) }),
        );
        Parse::new(node, self.errors)
    }

    pub fn enter(&mut self, kind: SyntaxKind, f: impl FnOnce(&mut Self)) {
        self.builder.start_node(kind.to_raw());
        f(self);
        self.builder.finish_node();
    }

    pub fn in_parens(&mut self, kind: SyntaxKind, f: impl FnOnce(&mut Self)) {
        if !self.at(T!['(']) {
            return self.push_error(ParseErrorKind::Expected(T!['(']));
        }
        self.enter(kind, |p| {
            p.bump(T!['(']);
            f(p);
            p.expect(T![')']);
        })
    }

    pub fn in_braces(&mut self, kind: SyntaxKind, f: impl FnOnce(&mut Self)) {
        if !self.at(T!['{']) {
            return self.push_error(ParseErrorKind::Expected(T!['{']));
        }
        self.enter(kind, |p| {
            p.bump(T!['{']);
            f(p);
            p.expect(T!['}']);
        })
    }

    /// Asserts the current token kind matches `kind` and consumes the token
    pub fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.at(kind));
        self.bump_any();
    }

    pub fn bump_any(&mut self) {
        self.steps += 1;
        assert!(self.steps < 10000, "stuck");
        let current = self.current();
        self.builder.token(current.kind().to_raw(), &self.source.text()[current.span()]);
        self.source.bump();
        self.eat_trivia();
    }

    /// Consumes the current token if it matches
    pub fn accept(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        // TODO account for glued tokens (may need to advance more than once)
        self.bump_any();
        true
    }

    #[inline]
    pub fn expect(&mut self, kind: SyntaxKind) -> bool {
        self.expect_recover(kind, TokenSet::EMPTY)
    }

    pub(crate) fn error(&mut self, msg: &'static str) {
        self.push_error(ParseErrorKind::StaticMsg(msg));
    }

    pub(crate) fn expect_with_msg(&mut self, kind: SyntaxKind, msg: &'static str) -> bool {
        self.expect_recover_with_msg(kind, TokenSet::EMPTY, msg)
    }

    pub(crate) fn expect_recover_with_msg(
        &mut self,
        kind: SyntaxKind,
        recovery: TokenSet,
        msg: &'static str,
    ) -> bool {
        if !self.expect_recover(kind, recovery) {
            self.push_error(ParseErrorKind::ExpectedWithMsg(kind, msg));
            return false;
        }
        true
    }

    pub fn expect_recover(&mut self, kind: SyntaxKind, recovery: TokenSet) -> bool {
        if !self.accept(kind) {
            self.recover(kind, recovery);
            return false;
        }
        true
    }

    pub fn recover(&mut self, expected: SyntaxKind, recovery: TokenSet) {
        if recovery.contains(self.source.current().kind()) {
            return;
        }
        self.error_node(format!("todo error node"));
    }

    fn push_error(&mut self, kind: ParseErrorKind) {
        let offset = self.current().span().start();
        let span = Span::zero_sized(offset);
        self.errors.push(ParseError { span, kind })
    }

    pub fn error_node(&mut self, s: String) {
        self.enter(T![Error], |p| {
            let text = p.source.current_text();
            p.builder.token(T![Error].to_raw(), text);
            p.bump_any();
        })
    }

    pub fn at_any(&mut self, kinds: TokenSet) -> bool {
        kinds.contains(self.current().kind())
    }

    pub fn at(&mut self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub fn nth_at(&mut self, n: usize, kind: SyntaxKind) -> bool {
        // TODO glue tokens
        self.source.lookahead(n).kind() == kind
    }

    pub(crate) fn current(&self) -> Token {
        self.source.current()
    }

    fn text(&self) -> &str {
        self.source.text()
    }

    fn current_text(&mut self) -> &str {
        &self.text()[self.current().span()]
    }

    // We only have trailing trivia currently (never any leading)
    fn eat_trivia(&mut self) {
        loop {
            let current = self.source.current();
            if !current.kind().is_trivia() {
                break;
            }
            self.builder.token(current.kind().to_raw(), &self.source.text()[current.span()]);
            self.source.bump()
        }
    }
}
