#![warn(rust_2018_idioms)]

pub use self::parse::Parse;

mod parse;

pub use parse::{parse, ParseNode};

mod parser;

#[cfg(test)]
mod test;
