use pd_syntax::SyntaxKind;

pub(crate) struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.accept(kind));
    }

    pub fn accept(&mut self, kind: SyntaxKind) -> bool {
        todo!()
    }

    pub fn expect(&mut self, kind: SyntaxKind) {
        todo!()
    }
}
