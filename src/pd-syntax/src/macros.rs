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

            $(($tt) => {{ $crate::SyntaxKind::$variant }});*;

            // forward identifiers to keywords, and then SyntaxKind itself
            // i.e. T![false] == SyntaxKind::FalseKw;
            // then T![Ws] == SyntaxKind::Ws
            // and generally T![<X>] == SyntaxKind::<X>
            ($ident:ident) => {{
                $crate::SyntaxKind::$ident
            }};

        }
    };
}

macro_rules! make_kw {
    ($($kw:tt => $variant:ident,)*) => {
        // Lookup keyword either by literal token or the equivalent string
        #[macro_export]
        macro_rules! K {
            $(($kw) => {{ $crate::SyntaxKind::$variant }};)*
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

make_kw! {
    fn => FnKw,
    type => TypeKw,
    let => LetKw,
    false => FalseKw,
    true => TrueKw,
}

make_tokens! {
    fn => FnKw,
    type => TypeKw,
    let => LetKw,
    false => FalseKw,
    true => TrueKw,

    < => LeftAngle,
    > => RightAngle,
    = => Equal,
    , => Comma,
    : => Colon,
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
