macro_rules! make_tokens {
    ($($tt:tt => $variant:ident,)*) => {
        #[macro_export]
        macro_rules! T {
            ('{') => {{ $crate::SyntaxKind::OpenBrace }};
            ('}') => {{ $crate::SyntaxKind::CloseBrace }};
            ('(') => {{ $crate::SyntaxKind::OpenParen }};
            (')') => {{ $crate::SyntaxKind::CloseParen }};
            ('[') => {{ $crate::SyntaxKind::OpenBracket }};
            (']') => {{ $crate::SyntaxKind::CloseBracket }};

            $(($tt) => {{ $crate::SyntaxKind::$variant }});*
        }
    };
}

macro_rules! make_kw {
    ($($kw:tt => $variant:ident,)*) => {
        #[macro_export]
        macro_rules! K {
            ($expr:expr) => {
                match $expr {
                    $(
                        stringify!($kw) => Some($crate::SyntaxKind::$variant),
                    )*
                    _ => None,
                }
            };
        }
    };
}

make_tokens! {
    fn => FnKw,
    type => TypeKw,

    < => LeftAngle,
    > => RightAngle,
    IDENT => Ident,
    PARAMS => Params,
    WS => Whitespace,
    EOF => Eof,
    ERROR => Error,

    FN => Fn,
    STMTS => Stmts,

    BLOCK_EXPR => BlockExpr,
}

make_kw! {
    fn => FnKw,
    type => TypeKw,
}

#[cfg(test)]
mod test {
    use crate::SyntaxKind;

    #[test]
    fn test_t_macro() {
        assert_eq!(T!['('], SyntaxKind::OpenParen);
    }

    #[test]
    fn test_k_macro() {
        use crate::SyntaxKind;
        let fn_kw = "fn";
        assert_eq!(K![fn_kw], Some(SyntaxKind::FnKw));
        assert_eq!(K!["type"], Some(SyntaxKind::TypeKw));
    }
}
