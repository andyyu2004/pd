use super::*;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub node: SyntaxNode,
}

impl_ast_node!(SourceFile: SourceFile);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Fn {
    pub node: SyntaxNode,
}

impl_ast_node!(Fn: Fn);
node_accessors!(Fn { body: BlockExpr });

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct BlockExpr {
    pub node: SyntaxNode,
}

impl_ast_node!(BlockExpr: BlockExpr);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Expr {
    pub node: SyntaxNode,
}

impl_ast_node!(Expr: Expr);
