use super::*;

macro_rules! token_source {
    ($src:expr) => {
        TextTokenSource::from_text($src)
    };
}

macro_rules! raw_tokens {
    ($src:expr) => {
        let mut raw_tokens = raw_tokens($src);
        macro_rules! next {
            ($kind:tt:$offset:literal:$len:literal) => {
                assert_eq!(
                    raw_tokens.next().unwrap(),
                    (RawToken { kind: T![$kind], offset: $offset, len: $len }, None)
                );
            };
        }
    };
}

#[test]
fn test_raw_lexer() {
    raw_tokens!(stringify! {
        fn main() {}
    });

    next!(fn:0:2);
    next!(Whitespace:2:1);
    next!(Ident:3:4);
    next!('(':7:1);
    next!(')':8:1);
    next!(Whitespace:9:1);
    next!('{':10:1);
    next!('}':11:1);
    next!(Eof:12:0);
    next!(Eof:12:0);
    next!(Eof:12:0);
}

macro_rules! check_eq {
    ($expr:expr, $kind:tt:$joint:literal) => {{
        let token = $expr;
        assert_eq!(token.raw.kind, T![$kind]);
        assert_eq!(token.is_joint, $joint);
    }};
}

#[test]
fn test_text_token_source() {
    let mut src = token_source!(stringify! {
        fn main() {
            x >> y
        }
    });

    check_eq!(src.current(), fn:false);
    // Current should not move source
    check_eq!(src.current(), fn:false);
    src.bump();
    check_eq!(src.current(), Whitespace:false);

    assert_eq!(src.current(), src.lookahead(0));
    assert_eq!(src.lookahead(0), src.current());
    check_eq!(src.lookahead(1), Ident:false);
    // looking ahead multiple times shouldn't accumulate
    check_eq!(src.lookahead(1), Ident:false);
    check_eq!(src.lookahead(7), Ident:false);
    check_eq!(src.lookahead(8), Whitespace:false);
    check_eq!(src.lookahead(9), >:true);
    check_eq!(src.lookahead(10), >:false);
    check_eq!(src.lookahead(11), Whitespace:false);
}

#[test]
fn test_text_token_source_detect_joint_token() {
    let src = stringify!(x >> y);
    raw_tokens!(src);
    next!(Ident:0:1);
    next!(Whitespace:1:1);
    next!(>:2:1);
    next!(>:3:1);
    next!(Whitespace:4:1);
    next!(Ident:5:1);
    next!(Eof:6:0);

    let mut src = token_source!(src);
    check_eq!(src.lookahead(2), >:true);
    check_eq!(src.lookahead(3), >:false);
}

#[test]
fn test_trivia_are_not_joint() {
    let mut src = token_source!(" x   y ");
    check_eq!(src.lookahead(0), Whitespace:false);
    check_eq!(src.lookahead(1), Ident:false);
    // Multiple whitespaces get compressed by rustc_lexer
    check_eq!(src.lookahead(2), Whitespace:false);
    check_eq!(src.lookahead(3), Ident:false);
}
