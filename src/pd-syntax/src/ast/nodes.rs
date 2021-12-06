use super::*;

node!(SourceFile);
impl_ast_node!(SourceFile: SourceFile);

node!(Fn);
impl_ast_node!(Fn: Fn);
node_accessors!(Fn { body: BlockExpr });

node!(BlockExpr);
impl_ast_node!(BlockExpr: BlockExpr);

node!(Expr);
impl_ast_node!(Expr: Expr);

node!(ValueDef);
impl_ast_node!(ValueDef: ValueDef);

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
