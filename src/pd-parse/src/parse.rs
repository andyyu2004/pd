use pd_ds::token_set::TokenSet;
use pd_syntax::T;

use crate::parser::Parser;

pub(crate) fn parse_fn(parser: &mut Parser) {
    parser.bump(T![fn]);
    parser.expect(T![IDENT]);
    parser.expect(T!['(']);
    parser.expect(T![')']);
    parser.expect(T!['{']);
    parser.expect(T!['}']);
}
