mod ast;
mod macros;

pub use ast::*;

use rowan::ast::AstNode as _;
use rowan::Language;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Ident,
    Fn,
    Underscore,

    Comment,
    Whitespace,

    BlockExpr,

    #[doc(hidden)]
    __Last,
}

impl SyntaxKind {
    pub fn from_keyword(s: &str) -> Option<Self> {
        let kw = match s {
            "fn" => Self::Fn,
            _ => return None,
        };
        Some(kw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdLanguage;

impl Language for PdLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::__Last as u16);
        unsafe { std::mem::transmute(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub trait AstNode: rowan::ast::AstNode<Language = PdLanguage> {
    fn find_child<N: AstNode>(&self) -> Option<N> {
        self.syntax().children().find_map(N::cast)
    }
}

impl<N: rowan::ast::AstNode<Language = PdLanguage>> AstNode for N {
}

pub type SyntaxNode = rowan::SyntaxNode<PdLanguage>;
