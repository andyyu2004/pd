#![warn(rust_2018_idioms)]

pub use self::parse::Parse;

pub mod parse;

mod parser;

#[cfg(test)]
mod test;
