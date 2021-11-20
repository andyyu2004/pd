use crate::parse_source;

#[test]
fn test_parse() {
    parse_source(stringify!(
        fn main() {
        }
    ))
}
