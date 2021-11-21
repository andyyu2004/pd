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
    ($ty:ty: $kind:ident) => {
        impl rowan::ast::AstNode for $ty {
            type Language = PdLanguage;

            #[inline]
            fn can_cast(kind: SyntaxKind) -> bool
            where
                Self: Sized,
            {
                kind == SyntaxKind::$kind
            }

            #[inline]
            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self>
            where
                Self: Sized,
            {
                Self::can_cast(node.kind()).then(|| Self { node })
            }

            #[inline]
            fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
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