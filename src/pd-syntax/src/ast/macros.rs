use crate::ast;
use crate::AstNode;

#[macro_export]
macro_rules! node {
    ($node:ident) => {
        #[derive(Debug, Hash, Clone, PartialEq, Eq)]
        pub struct $node {
            pub node: $crate::SyntaxNode,
        }
    };
}

#[macro_export]
macro_rules! node_accessors {
    ($ty:ty {  $($ident:ident: $nodety:ty)* }) => {
        impl $ty {
            $(
                pub fn $ident(&self) -> Option<$nodety> {
                    self.find_child()
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! impl_ast_node {
    ($ty:ident) => {
        impl_ast_node!($ty: $ty);
    };
    ($ty:ty: $kind:ident) => {
        impl rowan::ast::AstNode for $ty {
            type Language = $crate::PdLanguage;

            #[inline]
            fn can_cast(kind: $crate::SyntaxKind) -> bool
            where
                Self: Sized,
            {
                kind == $crate::SyntaxKind::$kind
            }

            #[inline]
            fn cast(node: $crate::rowan::SyntaxNode<Self::Language>) -> Option<Self>
            where
                Self: Sized,
            {
                Self::can_cast(node.kind()).then(|| Self { node })
            }

            #[inline]
            fn syntax(&self) -> &$crate::rowan::SyntaxNode<Self::Language> {
                &self.node
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.syntax())
            }
        }
    };
}

#[macro_export]
macro_rules! ast_node {
    ($node:ident : $($trait:ident),+) => {
        #[derive(Debug, Hash, Clone, PartialEq, Eq)]
        pub struct $node {
            pub node: $crate::SyntaxNode,
        }

        impl_ast_node!($node);

        $(
            impl $crate::ast::$trait for $node {}
        )*
    };
    ($node:ident) => {
        node!($node);
        impl_ast_node!($node);
    };
}
