use pd_lex::TextTokenSource;

mod parse;
mod parser;

pub(crate) fn parse_source(src: &str) {
    let mut token_source = TextTokenSource::from_text(src);
    parser::parse_source_file(&mut token_source);
    let _syntax_errors = token_source.errors();
}

#[cfg(test)]
mod test;
