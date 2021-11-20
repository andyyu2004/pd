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

impl_ast_node!(Fn: FnKw);
node_accessors!(Fn { body: BlockExpr });

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct BlockExpr {
    pub node: SyntaxNode,
}

impl_ast_node!(BlockExpr: BlockExpr);
