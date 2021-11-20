#![warn(rust_2018_idioms)]

use pd_lex::{TextTokenSource, TokenSource};
use pd_syntax::ast;

use crate::parser::Parser;

use self::parse::Parse;

mod parse;
mod parser;

pub(crate) fn parse_source(src: &str) {
    let mut token_source = TextTokenSource::from_text(src);
    let parsed = parser::parse_source_file(&mut token_source);
    dbg!(parsed.syntax());
}

#[cfg(test)]
mod test;
