#![feature(type_alias_impl_trait, trait_alias)]

pub mod ast;
mod macros;

// This needs to be imported for rust-analyzers sake (autocomplete etc) as trait aliases are not fully implemented
pub use rowan::ast::AstNode as AstMethods;

use rowan::{self, Language};

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
    Type,
    Path,

    // Items
    Item,
    Fn,
    ValueDef,

    // Expressions
    Exprs,
    BlockExpr,

    // Patterns
    Binding,

    // Types
    PathType,

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

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::__Last as u16);
        unsafe { std::mem::transmute(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.to_raw()
    }
}

pub trait HasName: AstNode + Sized {
    fn name(&self) -> Option<ast::Ident> {
        self.find_child()
    }
}

pub trait HasType: AstNode + Sized {
    fn ty(&self) -> Option<ast::Type> {
        self.find_child()
    }
}

pub type AstChildren<N: AstNode> = impl Iterator<Item = N> + std::fmt::Debug;

pub trait AstNodeExt: rowan::ast::AstNode<Language = PdLanguage> {
    fn find_child<N: AstNode>(&self) -> Option<N> {
        self.syntax().children().find_map(N::cast)
    }

    fn find_children<N: AstNode>(&self) -> AstChildren<N> {
        self.syntax().children().filter_map(N::cast)
    }
}

impl<N: rowan::ast::AstNode<Language = PdLanguage>> AstNodeExt for N {
}

pub trait AstNode = rowan::ast::AstNode<Language = PdLanguage>;

pub type SyntaxNode = rowan::SyntaxNode<PdLanguage>;

pub trait SyntaxNodeExt: Sized {
    fn cast<N: AstNode>(self) -> Option<N>;
}

impl SyntaxNodeExt for SyntaxNode {
    fn cast<N: AstNode>(self) -> Option<N> {
        N::cast(self)
    }
}
