use super::*;

macro_rules! token_source {
    ($src:expr) => {
        TextTokenSource::new(raw_tokens($src))
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
    next!(WS:2:1);
    next!(IDENT:3:4);
    next!('(':7:1);
    next!(')':8:1);
    next!(WS:9:1);
    next!('{':10:1);
    next!(WS:11:1);
    next!('}':12:1);
    next!(EOF:13:0);
    next!(EOF:13:0);
    next!(EOF:13:0);
}

#[test]
fn test_text_token_source() {
    let mut src = token_source!(stringify! {
        fn main() {
            x >> y
        }
    });

    assert_eq!(src.current(), Token { kind: T![fn], is_joint: false });
    // Current should not move source
    assert_eq!(src.current(), Token { kind: T![fn], is_joint: false });

    src.bump();
    assert_eq!(src.current(), Token { kind: T![WS], is_joint: false });

    assert_eq!(src.current(), src.lookahead(0));
    assert_eq!(src.lookahead(0), src.current());
    assert_eq!(src.lookahead(1), Token { kind: T![IDENT], is_joint: false });
    // Looking ahead multiple times shouldn't accumulate
    assert_eq!(src.lookahead(1), Token { kind: T![IDENT], is_joint: false });
    assert_eq!(src.lookahead(7), Token { kind: T![IDENT], is_joint: false });
    assert_eq!(src.lookahead(8), Token { kind: T![WS], is_joint: false });
    assert_eq!(src.lookahead(9), Token { kind: T![>], is_joint: true });
    assert_eq!(src.lookahead(10), Token { kind: T![>], is_joint: false });
    assert_eq!(src.lookahead(11), Token { kind: T![WS], is_joint: false });
}

#[test]
fn test_text_token_source_detect_joint_token() {
    let src = stringify!(x >> y);
    raw_tokens!(src);
    next!(IDENT:0:1);
    next!(WS:1:1);
    next!(>:2:1);
    next!(>:3:1);
    next!(WS:4:1);
    next!(IDENT:5:1);
    next!(EOF:6:0);

    let mut src = token_source!(src);
    assert_eq!(src.lookahead(2), Token { kind: T![>], is_joint: true });
    assert_eq!(src.lookahead(3), Token { kind: T![>], is_joint: false });
}

#[test]
fn test_trivia_are_not_joint() {
    let mut src = token_source!(" x   y ");
    assert_eq!(src.lookahead(0), Token { kind: T![WS], is_joint: false });
    assert_eq!(src.lookahead(1), Token { kind: T![IDENT], is_joint: false });
    // Multiple whitespaces get compressed by rustc_lexer
    assert_eq!(src.lookahead(2), Token { kind: T![WS], is_joint: false });
    assert_eq!(src.lookahead(3), Token { kind: T![IDENT], is_joint: false });
}
