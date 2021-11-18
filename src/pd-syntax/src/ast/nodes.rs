use super::*;

pub struct Fn {
    pub node: SyntaxNode,
}

impl_ast_node!(Fn: Fn);
node_accessors!(Fn { body: BlockExpr });

pub struct BlockExpr {
    pub node: SyntaxNode,
}

impl_ast_node!(BlockExpr: BlockExpr);
