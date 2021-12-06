pub mod ast;
mod macros;

use rowan::ast::AstNode as _;
use rowan::Language;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Ident,
    Underscore,
    LeftAngle,
    RightAngle,
    Equal,
    Comma,
    Colon,

    // Keywords
    FnKw,
    TypeKw,
    LetKw,
    FalseKw,
    TrueKw,

    // Misc
    Comment,
    Whitespace,
    Error,
    Eof,

    // Nodes
    Params,
    Literal,

    // Items
    Fn,
    ValueDef,

    // Statements
    Stmts,

    // Expressions
    BlockExpr,
    Expr,

    SourceFile,

    #[doc(hidden)]
    __Last,
}

impl SyntaxKind {
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::Whitespace | SyntaxKind::Comment)
    }

    #[inline]
    pub fn to_raw(self) -> rowan::SyntaxKind {
        rowan::SyntaxKind(self as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdLanguage;

impl Language for PdLanguage {
    type Kind = SyntaxKind;

    #[inline]
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::__Last as u16);
        unsafe { std::mem::transmute(raw.0) }
    }

    #[inline]
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.to_raw()
    }
}

pub trait AstNode: rowan::ast::AstNode<Language = PdLanguage> {
    #[inline]
    fn find_child<N: AstNode>(&self) -> Option<N> {
        self.syntax().children().find_map(N::cast)
    }
}

impl<N: rowan::ast::AstNode<Language = PdLanguage>> AstNode for N {
}

pub type SyntaxNode = rowan::SyntaxNode<PdLanguage>;
