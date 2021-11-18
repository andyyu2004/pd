use std::{fmt, iter};

use pd_syntax::{SyntaxKind, T};
use rustc_lexer::TokenKind;
use text_size::{TextRange, TextSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    kind: SyntaxKind,
    size: TextSize,
}

pub struct SyntaxError {
    msg: String,
    range: TextRange,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl SyntaxError {
    pub fn new(msg: impl Into<String>, range: TextRange) -> Self {
        Self { msg: msg.into(), range }
    }
}

pub fn lex(src: &str) -> impl Iterator<Item = (Token, Option<SyntaxError>)> + '_ {
    let mut tokens = rustc_lexer::tokenize(src);
    let mut offset = 0;

    iter::from_fn(move || {
        let token = tokens.next()?;
        let len = TextSize::try_from(token.len).unwrap();
        let range = TextRange::at(offset.try_into().unwrap(), len);
        let (kind, err) = token_kind_to_syntax_kind(token.kind, &src[range]);
        offset += token.len;
        Some((Token { kind, size: len }, err.map(|err| SyntaxError::new(err, range))))
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
                    SyntaxKind::from_keyword(token_text).unwrap_or(Ident)
                },

            TokenKind::RawIdent => Ident,
            // TokenKind::Semi => T![;],
            // TokenKind::Comma => T![,],
            // TokenKind::Dot => T![.],
            // TokenKind::OpenParen => T!['('],
            // TokenKind::CloseParen => T![')'],
            // TokenKind::OpenBrace => T!['{'],
            // TokenKind::CloseBrace => T!['}'],
            // TokenKind::OpenBracket => T!['['],
            // TokenKind::CloseBracket => T![']'],
            // TokenKind::At => T![@],
            // TokenKind::Pound => T![#],
            // TokenKind::Tilde => T![~],
            // TokenKind::Question => T![?],
            // TokenKind::Colon => T![:],
            // TokenKind::Dollar => T![$],
            // TokenKind::Eq => T![=],
            // TokenKind::Bang => T![!],
            // TokenKind::Lt => T![<],
            // TokenKind::Gt => T![>],
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
            _ => todo!(),
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
