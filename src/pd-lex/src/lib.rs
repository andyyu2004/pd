#![warn(rust_2018_idioms)]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

mod text_range;

use drop_bomb::DropBomb;
use peekmore::{PeekMore, PeekMoreIterator};
pub use text_range::TextRange;

use std::{fmt, iter};

use pd_syntax::{SyntaxKind, K, T};
use rustc_lexer::TokenKind;

pub trait TokenSource {
    fn bump(&mut self);
    fn lookahead(&mut self, n: usize) -> Token;
    fn text(&self) -> &str;
    fn current(&self) -> Token;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Token {
    raw: RawToken,
    /// Is the current token joint to the next one (`> >` vs `>>`).
    is_joint: bool,
}

impl Token {
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.raw.kind
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.raw.offset
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.raw.len
    }

    pub fn range(&self) -> TextRange {
        TextRange::new(self.offset(), self.offset() + self.len())
    }

    #[inline]
    pub fn is_joint(&self) -> bool {
        self.is_joint
    }
}

pub struct TextTokenSource<'t> {
    text: &'t str,
    raw_tokens: PeekMoreIterator<RawTokens<'t>>,
    curr: Option<Token>,
    errors: Vec<SyntaxError>,
    _bomb: DropBomb,
}

impl<'t> TextTokenSource<'t> {
    pub fn new(src: &'t str, raw_tokens: RawTokens<'t>) -> Self {
        let raw_tokens = raw_tokens.peekmore();
        let mut _bomb = DropBomb::new("Must call `errors` to consume the errors");
        // Allow not checking errors during tests
        #[cfg(test)]
        _bomb.defuse();
        let mut this = Self {
            text: src,
            raw_tokens,
            errors: Default::default(),
            curr: Default::default(),
            _bomb,
        };
        this.bump();
        this
    }

    pub fn from_text(text: &'t str) -> Self {
        Self::new(text, raw_tokens(text))
    }

    pub fn errors(mut self) -> Vec<SyntaxError> {
        self._bomb.defuse();
        self.errors
    }

    fn mk_token(&mut self, raw: RawToken) -> Token {
        let next = self.peek_mut();
        let is_joint =
            !raw.kind.is_trivia() && !next.kind.is_trivia() && raw.offset + 1 == next.offset;
        Token { raw, is_joint }
    }

    fn peek_mut(&mut self) -> RawToken {
        let (token, err) = self.raw_tokens.peek_mut().expect("expected not to be called after eof");
        if let Some(err) = err.take() {
            self.errors.push(err);
        }
        *token
    }
}

impl TokenSource for TextTokenSource<'_> {
    fn bump(&mut self) {
        let raw_token = self.peek_mut();
        self.curr = Some(self.mk_token(raw_token));
        self.raw_tokens.next();
    }

    fn lookahead(&mut self, n: usize) -> Token {
        if n == 0 {
            return self.current();
        }
        let &(raw_token, _) = self.raw_tokens.peek_forward(n - 1).unwrap();
        self.raw_tokens.advance_cursor();
        let token = self.mk_token(raw_token);
        self.raw_tokens.reset_cursor();
        token
    }

    #[inline]
    fn current(&self) -> Token {
        self.curr.expect("should only be None on construction")
    }

    #[inline]
    fn text(&self) -> &str {
        self.text
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawToken {
    pub kind: SyntaxKind,
    pub offset: usize,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxError {
    msg: String,
    range: TextRange,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl SyntaxError {
    pub fn new(msg: impl Into<String>, range: TextRange) -> Self {
        Self { msg: msg.into(), range }
    }
}

type RawTokens<'t> = impl Iterator<Item = (RawToken, Option<SyntaxError>)> + 't;

/// Returns an iterator over raw tokens
/// This stream has an infinite tail full of EOF tokens
pub fn raw_tokens(src: &str) -> RawTokens<'_> {
    let mut tokens = rustc_lexer::tokenize(src);
    let mut offset = 0;

    iter::from_fn(move || {
        let rustc_lexer::Token { kind, len } = match tokens.next() {
            Some(token) => token,
            None => return Some((RawToken { offset, kind: T![EOF], len: 0 }, None)),
        };
        let range = TextRange::at(offset, len);
        let (kind, err) = token_kind_to_syntax_kind(kind, &src[range]);
        let token = RawToken { offset, kind, len };
        offset += token.len;
        Some((token, err.map(|err| SyntaxError::new(err, range))))
    })
}

// copied from RA
fn token_kind_to_syntax_kind(
    kind: rustc_lexer::TokenKind,
    token_text: &str,
) -> (SyntaxKind, Option<&'static str>) {
    use SyntaxKind::*;
    let syntax_kind = {
        match kind {
            TokenKind::LineComment { doc_style: _ } => Comment,
            TokenKind::BlockComment { doc_style: _, terminated: false } => {
                return (
                    Comment,
                    Some("Missing trailing `*/` symbols to terminate the block comment"),
                );
            }
            TokenKind::Whitespace => Whitespace,
            TokenKind::Ident =>
                if token_text == "_" {
                    Underscore
                } else {
                    K![token_text].unwrap_or(Ident)
                },

            TokenKind::RawIdent => Ident,
            TokenKind::OpenParen => T!['('],
            TokenKind::CloseParen => T![')'],
            TokenKind::OpenBrace => T!['{'],
            TokenKind::CloseBrace => T!['}'],
            TokenKind::OpenBracket => T!['['],
            TokenKind::CloseBracket => T![']'],
            TokenKind::Lt => T![<],
            TokenKind::Gt => T![>],
            // TokenKind::Semi => T![;],
            // TokenKind::Comma => T![,],
            // TokenKind::Dot => T![.],
            // TokenKind::At => T![@],
            // TokenKind::Pound => T![#],
            // TokenKind::Tilde => T![~],
            // TokenKind::Question => T![?],
            // TokenKind::Colon => T![:],
            // TokenKind::Dollar => T![$],
            // TokenKind::Eq => T![=],
            // TokenKind::Bang => T![!],
            // TokenKind::Minus => T![-],
            // TokenKind::And => T![&],
            // TokenKind::Or => T![|],
            // TokenKind::Plus => T![+],
            // TokenKind::Star => T![*],
            // TokenKind::Slash => T![/],
            // TokenKind::Caret => T![^],
            // TokenKind::Percent => T![%],
            // TokenKind::Unknown => ERROR,
            // TokenKind::UnknownPrefix => todo!(),
            // TokenKind::Literal { kind, suffix_start } => todo!(),
            // TokenKind::Lifetime { starts_with_number } => todo!(),
            kind => todo!("{:?}", kind),
        }
    };

    return (syntax_kind, None);

    // fn match_literal_kind(kind: &rustc_lexer::LiteralKind) -> (SyntaxKind, Option<&'static str>) {
    //     let mut err = "";
    //     let syntax_kind = match *kind {
    //         rustc_lexer::LiteralKind::Int { empty_int, base: _ } => {
    //             if empty_int {
    //                 err = "Missing digits after the integer base prefix";
    //             }
    //             INT_NUMBER
    //         }
    //         rustc_lexer::LiteralKind::Float { empty_exponent, base: _ } => {
    //             if empty_exponent {
    //                 err = "Missing digits after the exponent symbol";
    //             }
    //             FLOAT_NUMBER
    //         }
    //         rustc_lexer::LiteralKind::Char { terminated } => {
    //             if !terminated {
    //                 err = "Missing trailing `'` symbol to terminate the character literal";
    //             }
    //             CHAR
    //         }
    //         rustc_lexer::LiteralKind::Byte { terminated } => {
    //             if !terminated {
    //                 err = "Missing trailing `'` symbol to terminate the byte literal";
    //             }
    //             BYTE
    //         }
    //         rustc_lexer::LiteralKind::Str { terminated } => {
    //             if !terminated {
    //                 err = "Missing trailing `\"` symbol to terminate the string literal";
    //             }
    //             STRING
    //         }
    //         rustc_lexer::LiteralKind::ByteStr { terminated } => {
    //             if !terminated {
    //                 err = "Missing trailing `\"` symbol to terminate the byte string literal";
    //             }
    //             BYTE_STRING
    //         }
    //         rustc_lexer::LiteralKind::RawStr { err: raw_str_err, .. } => {
    //             if let Some(raw_str_err) = raw_str_err {
    //                 err = match raw_str_err {
    //                     RawStrError::InvalidStarter { .. } => "Missing `\"` symbol after `#` symbols to begin the raw string literal",
    //                     RawStrError::NoTerminator { expected, found, .. } => if expected == found {
    //                         "Missing trailing `\"` to terminate the raw string literal"
    //                     } else {
    //                         "Missing trailing `\"` with `#` symbols to terminate the raw string literal"
    //                     },
    //                     RawStrError::TooManyDelimiters { .. } => "Too many `#` symbols: raw strings may be delimited by up to 65535 `#` symbols",
    //                 };
    //             };
    //             STRING
    //         }
    //         rustc_lexer::LiteralKind::RawByteStr { err: raw_str_err, .. } => {
    //             if let Some(raw_str_err) = raw_str_err {
    //                 err = match raw_str_err {
    //                     RawStrError::InvalidStarter { .. } => "Missing `\"` symbol after `#` symbols to begin the raw byte string literal",
    //                     RawStrError::NoTerminator { expected, found, .. } => if expected == found {
    //                         "Missing trailing `\"` to terminate the raw byte string literal"
    //                     } else {
    //                         "Missing trailing `\"` with `#` symbols to terminate the raw byte string literal"
    //                     },
    //                     RawStrError::TooManyDelimiters { .. } => "Too many `#` symbols: raw byte strings may be delimited by up to 65535 `#` symbols",
    //                 };
    //             };

    //             BYTE_STRING
    //         }
    //     };

    //     let err = if err.is_empty() { None } else { Some(err) };

    //     (syntax_kind, err)
}

#[cfg(test)]
mod tests;
