use pd_ds::token_set::TokenSet;
use pd_lex::TokenSource;
use pd_syntax::SyntaxKind;

use crate::parse;

pub(crate) fn parse_source_file(token_source: &mut dyn TokenSource) {
    let mut parser = Parser::new(token_source);
    parse::parse_fn(&mut parser);
}

pub(crate) struct Parser<'t> {
    source: &'t mut dyn TokenSource,
    builder: rowan::GreenNodeBuilder<'static>,
}

impl<'t> Parser<'t> {
    pub fn new(source: &'t mut dyn TokenSource) -> Self {
        Self { source, builder: Default::default() }
    }

    /// Asserts the current token kind matches `kind` and consumes the token
    pub fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.expect(kind));
        self.eat_trivia();
    }

    fn eat_trivia(&mut self) {
        loop {
            let current = self.source.current();
            if !current.kind().is_trivia() {
                break;
            }
            self.builder.token(current.kind().to_raw(), &self.source.text()[current.range()]);
            self.source.bump()
        }
    }

    /// Consumes the current token if it matches
    pub fn accept(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        // TODO account for glued tokens (may need to advance more than once)
        self.source.bump();
        self.eat_trivia();
        true
    }

    #[inline]
    pub fn expect(&mut self, kind: SyntaxKind) -> bool {
        self.expect_recover(kind, TokenSet::EMPTY)
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
        todo!("expected {:?} but found {:?}", expected, self.source.current().kind());
    }

    pub fn at(&mut self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub fn nth_at(&mut self, n: usize, kind: SyntaxKind) -> bool {
        // TODO glue tokens
        self.source.lookahead(n).kind() == kind
    }
}
