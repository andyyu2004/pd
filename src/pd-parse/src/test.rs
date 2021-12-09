/// Checks the conversion back to a string exactly matches the input string
#[macro_export]
macro_rules! parse_reflexive {
    ($ty:ty, $($tt:tt)*) => {{
        let src = stringify!($($tt)*);
        let parsed = $crate::parse::<$ty>(src);
        assert!(parsed.errors().is_empty());
        assert_eq!(src, parsed.syntax().to_string());
    }};
}

/// Asserts a successful parse (no errors) and returns the syntax node.
/// Includes a check of `parse_reflexive`
#[macro_export]
macro_rules! parse_syntax {
    ($ty:ty, $($tt:tt)*) => {{
        let src = stringify!($($tt)*);
        let parsed = $crate::parse::<$ty>(src);
        assert!(parsed.errors().is_empty());
        assert_eq!(src, parsed.syntax().to_string());
        parsed.syntax()
    }};
}
