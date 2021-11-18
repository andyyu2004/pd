macro_rules! make {
    ($($tt:tt => $variant:ident,)*) => {
        #[macro_export]
        macro_rules! T {
            ('{') => {{ $crate::SyntaxKind::OpenBrace }};
            ('}') => {{ $crate::SyntaxKind::CloseBrace }};
            ('(') => {{ $crate::SyntaxKind::OpenParen }};
            (')') => {{ $crate::SyntaxKind::CloseParen }};

            $(($tt) => {{ $crate::SyntaxKind::$variant }})*;
        }
    };
}

make! {
    fn => Fn,
}

#[cfg(test)]
#[test]
fn test_t_macro() {
    assert_eq!(T!['('], super::SyntaxKind::OpenParen);
}
