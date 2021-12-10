#![warn(rust_2018_idioms)]

pub use self::parse::Parse;
pub use db::{AstDatabase, AstDatabaseStorage};

mod db;
mod parse;

pub use parse::{parse, ParseNode};

mod parser;

#[cfg(test)]
mod test;
