//! Radix (base) handling for numeric literals.
//!
//! The dispatcher at [`crate::lexer::inner::scanner::parse::number::parse_number`]
//! routes a leading character to the right per-radix parser:
//!
//! - `0b` / `0B` → [`binary`]
//! - `0o` / `0O` → [`octal`]
//! - `0x` / `0X` → [`hex`]
//! - any other digit → [`decimal`]
//!
//! Every per-radix module exposes a function of the shape
//! `parse_<radix>(scanner, prefix_consumed: bool) -> Result<IntegerLiteral,
//! LexerError>`. The `prefix_consumed` flag tells the parser whether
//! the caller has already advanced past the `0b`/`0o`/`0x` prefix
//! (the dispatcher does this so it can fall back to a plain decimal
//! `0` on a bare `0`).
//!
//! # Underscore policy
//!
//! All four radix parsers accept `_` between digits and **between
//! the prefix and the first digit** for hex/octal/binary. Multiple
//! underscores in a row are accepted. The presence of any underscore
//! is recorded via `IntegerLiteral::has_underscore`; the digits
//! themselves are stored stripped.

/// Binary (base-2) parser.
pub mod binary;

/// Octal (base-8) parser.
pub mod octal;

/// Decimal (base-10) parser.
pub mod decimal;

/// Hexadecimal (base-16) parser.
pub mod hex;

// Submodules are in the same module, accessible directly

/// Numeric base types.
///
/// Tag enum that the dispatcher uses to communicate the chosen base
/// back to callers. The active parser does not have to return this —
/// the dispatch path is fixed by prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Radix {
    Binary,
    Octal,
    Decimal,
    Hex,
}

/// Get numeric value of radix.
pub fn radix_value(radix: Radix) -> u32 {
    match radix {
        Radix::Binary => 2,
        Radix::Octal => 8,
        Radix::Decimal => 10,
        Radix::Hex => 16,
    }
}

/// Detect radix from string prefix.
pub fn detect_radix(input: &str) -> Radix {
    if input.starts_with("0b") || input.starts_with("0B") {
        Radix::Binary
    } else if input.starts_with("0o") || input.starts_with("0O") {
        Radix::Octal
    } else if input.starts_with("0x") || input.starts_with("0X") {
        Radix::Hex
    } else {
        Radix::Decimal
    }
}

/// Parse a number with given radix.
pub fn parse_radix(input: &str, radix: Radix) -> Option<u64> {
    // Remove underscore separators
    let cleaned: String = input.chars().filter(|&c| c != '_').collect();

    match radix {
        Radix::Binary => binary::parse(&cleaned),
        Radix::Octal => octal::parse(&cleaned),
        Radix::Decimal => decimal::parse(&cleaned),
        Radix::Hex => hex::parse(&cleaned),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_value_binary() {
        assert_eq!(radix_value(Radix::Binary), 2);
    }
    #[test]
    fn test_radix_value_octal() {
        assert_eq!(radix_value(Radix::Octal), 8);
    }
    #[test]
    fn test_radix_value_hex() {
        assert_eq!(radix_value(Radix::Hex), 16);
    }
    #[test]
    fn test_detect_binary() {
        assert_eq!(detect_radix("0b101"), Radix::Binary);
    }
    #[test]
    fn test_detect_octal() {
        assert_eq!(detect_radix("0o77"), Radix::Octal);
    }
    #[test]
    fn test_detect_hex() {
        assert_eq!(detect_radix("0xFF"), Radix::Hex);
    }
    #[test]
    fn test_detect_decimal() {
        assert_eq!(detect_radix("42"), Radix::Decimal);
    }
}
