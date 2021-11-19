use pd_syntax::T;

use crate::parser::Parser;

pub(crate) fn function(parser: &mut Parser) {
    parser.bump(T![fn]);
    parser.expect(T![IDENT]);
    parser.expect(T!['{']);
    parser.expect(T!['}']);
}
