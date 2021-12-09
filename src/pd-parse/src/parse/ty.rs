use super::*;

pub(crate) fn parse_type(p: &mut Parser<'_>) {
    p.enter(T![Type], |p| {
        parse_path(p);
    });
}
