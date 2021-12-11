use super::*;
use SyntaxKind::*;

ast_node!(Fn);
node_accessors!(Fn { body: BlockExpr });

ast_node!(SourceFile);
ast_node!(BlockExpr);
ast_node!(Const: HasType, HasName, HasPat);
ast_node!(Literal);
ast_node!(BindingPat);
ast_node!(Name);
ast_node!(PathType);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Type {
    Path(PathType),
}

impl rowan::ast::AstNode for Type {
    type Language = PdLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            Literal => true,
            _ => todo!(),
        }
    }

    fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized,
    {
        let ty = match node.kind() {
            Path => Type::Path(PathType { node }),
            _ => return None,
        };
        Some(ty)
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            Type::Path(path) => &path.node,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Pat {
    Binding(BindingPat),
    Literal(Literal),
}

impl rowan::ast::AstNode for Pat {
    type Language = PdLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            Literal => true,
            _ => false,
        }
    }

    fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized,
    {
        if !Self::can_cast(node.kind()) {
            return None;
        }

        let pat = match node.kind() {
            Literal => Pat::Literal(Literal { node }),
            _ => todo!(),
        };
        Some(pat)
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            Pat::Literal(lit) => &lit.node,
            Pat::Binding(binding) => &binding.node,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
}

impl rowan::ast::AstNode for Expr {
    type Language = PdLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            Literal => true,
            _ => todo!(),
        }
    }

    fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized,
    {
        let expr = match node.kind() {
            Literal => Expr::Literal(Literal { node }),
            _ => todo!(),
        };
        Some(expr)
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            Expr::Literal(lit) => &lit.node,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Item {
    Const(Const),
}

impl rowan::ast::AstNode for Item {
    type Language = PdLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            SyntaxKind::Const => true,
            _ => false,
        }
    }

    fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized,
    {
        if !Self::can_cast(node.kind()) {
            return None;
        }
        let node = match node.kind() {
            SyntaxKind::Const => Self::Const(Const { node }),
            _ => unreachable!(),
        };
        Some(node)
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            Item::Const(node) => node.syntax(),
        }
    }
}
