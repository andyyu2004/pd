use super::*;
use SyntaxKind::*;

node!(SourceFile);
impl_ast_node!(SourceFile: SourceFile);

ast_node!(Fn);
node_accessors!(Fn { body: BlockExpr });

ast_node!(BlockExpr);
ast_node!(ValueDef);
ast_node!(Literal);

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
    ValueDef(ValueDef),
}

impl rowan::ast::AstNode for Item {
    type Language = PdLanguage;

    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        match kind {
            SyntaxKind::ValueDef => true,
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
            SyntaxKind::ValueDef => Self::ValueDef(ValueDef { node }),
            _ => unreachable!(),
        };
        Some(node)
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        match self {
            Item::ValueDef(node) => node.syntax(),
        }
    }
}
